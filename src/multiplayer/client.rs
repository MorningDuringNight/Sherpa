use async_channel::{Receiver, Sender};
use async_io::Timer;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::net::UdpSocket;
use std::time::Duration;

use crate::{app::GameMode, player::Player};

// -----------------------------------------------------------
//                 CLIENT SOCKET (UDP)
// -----------------------------------------------------------
#[derive(Resource)]
pub struct UdpClientSocket {
    pub socket: UdpSocket,
    pub server_addr: std::net::SocketAddr,
}

// -----------------------------------------------------------
//              INPUT HISTORY (for rollback)
// -----------------------------------------------------------
#[derive(Resource, Default)]
pub struct InputHistory {
    pub entries: Vec<InputEntry>,
}

pub struct InputEntry {
    pub tick: u32,
    pub mask: u8,
}

const MAX_HISTORY: usize = 200;

// -----------------------------------------------------------
//             CLIENT PREDICTION STATE
// -----------------------------------------------------------
#[derive(Resource, Default)]
pub struct ClientPredictionState {
    pub last_server_tick: u32,
    pub authoritative_pos: Vec2,
    pub predicted_pos: Vec2,

    pub input_history: Vec<(u32, u8)>,
}

// -----------------------------------------------------------
//                  SNAPSHOT UPDATE
// -----------------------------------------------------------
#[derive(Debug)]
pub struct SnapshotUpdate {
    pub tick: u32,
    pub positions: Vec<(f32, f32)>,
}

// -----------------------------------------------------------
//              INPUT COMMAND (TX INTO ECS)
// -----------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct InputCommand {
    pub seq: u32,
    pub mask: u8,
}

// -----------------------------------------------------------
//                CHANNELS FOR CLIENT
// -----------------------------------------------------------
#[derive(Resource)]
pub struct ClientNetChannels {
    pub rx_snapshots: Receiver<SnapshotUpdate>,
    pub tx_inputs: Sender<InputCommand>,
}

// -----------------------------------------------------------
//          SEND INPUT (CALLED EVERY FRAME ON CLIENT)
// -----------------------------------------------------------
pub fn send_input_state_system(
    mut seq: Local<u32>,
    keyboard: Res<ButtonInput<KeyCode>>,

    channels: Option<Res<ClientNetChannels>>,
    mut prediction_state: Option<ResMut<ClientPredictionState>>,
    mut history: ResMut<InputHistory>,

    client: Option<Res<UdpClientSocket>>,
) {
    let channels = match channels {
        Some(c) => c,
        None => return,
    };
    let client = match client {
        Some(c) => c,
        None => return,
    };

    // -------- Construct input bitmask --------
    let mut mask = 0u8;
    if keyboard.pressed(KeyCode::KeyW) {
        mask |= 1 << 0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        mask |= 1 << 1;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        mask |= 1 << 2;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        mask |= 1 << 3;
    }

    *seq += 1;

    // -------- Store in InputHistory --------
    history.entries.push(InputEntry { tick: *seq, mask });
    if history.entries.len() > MAX_HISTORY {
        history.entries.remove(0);
    }

    // -------- UDP packet --------
    let mut buf = Vec::with_capacity(5);
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.push(mask);

    if let Err(e) = client.socket.send_to(&buf, client.server_addr) {
        eprintln!("[Client] Failed to send input state: {}", e);
    }

    // -------- Prediction local storage --------
    if let Some(mut pred) = prediction_state.as_mut() {
        pred.input_history.push((*seq, mask));
        if pred.input_history.len() > MAX_HISTORY {
            pred.input_history.remove(0);
        }
    }

    // -------- Send to ECS input channel --------
    let cmd = InputCommand { seq: *seq, mask };
    if let Err(e) = channels.tx_inputs.try_send(cmd) {
        eprintln!("[Client] Failed to enqueue input: {}", e);
    }
}

// -----------------------------------------------------------
//               CLIENT HANDSHAKE + NETWORK SETUP
// -----------------------------------------------------------
#[derive(Resource)]
pub struct ServerAddress(pub String);

pub fn client_handshake(
    mut commands: Commands,
    server_addr: Res<ServerAddress>,
    gamemode: Res<GameMode>,
) {
    let server_addr: std::net::SocketAddr = server_addr.0.parse().expect("Invalid server address");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP client");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Failed to set read timeout");

    let (tx_snapshots, rx_snapshots) = async_channel::unbounded::<SnapshotUpdate>();
    let (tx_inputs, rx_inputs) = async_channel::unbounded::<InputCommand>();

    let msg = match *gamemode {
        GameMode::NetCoop(id) if id == 0 => b"MAIN",
        GameMode::NetCoop(id) if id == 1 => b"PLAY",
        _ => b"ERRR",
    };

    socket.send_to(msg, server_addr).ok();

    // -------- SYN-ACK handshake --------
    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
            if &buf[..len] == b"ACK" {
                println!("[Client] Handshake OK with {}", addr);

                commands.insert_resource(ClientPredictionState::default());
                commands.insert_resource(InputHistory::default());

                // -------- SPAWN SNAPSHOT RECEIVER TASK --------
                let sock_clone = socket.try_clone().unwrap();
                let tx_snapshots_clone = tx_snapshots.clone();

                IoTaskPool::get()
                    .spawn(async move {
                        let mut buf = [0u8; 1500];

                        loop {
                            match sock_clone.recv_from(&mut buf) {
                                Ok((len, _)) => {
                                    let data = &buf[..len];
                                    if data.len() < 6 {
                                        continue;
                                    }

                                    let tick = u32::from_be_bytes(data[0..4].try_into().unwrap());
                                    let count =
                                        u16::from_be_bytes(data[4..6].try_into().unwrap()) as usize;

                                    let mut offset = 6;
                                    let mut positions = Vec::with_capacity(count);

                                    for _ in 0..count {
                                        let x = f32::from_be_bytes(
                                            data[offset..offset + 4].try_into().unwrap(),
                                        );
                                        let y = f32::from_be_bytes(
                                            data[offset + 4..offset + 8].try_into().unwrap(),
                                        );
                                        offset += 8;
                                        positions.push((x, y));
                                    }

                                    tx_snapshots_clone
                                        .try_send(SnapshotUpdate { tick, positions })
                                        .ok();
                                }
                                Err(_) => {}
                            }
                        }
                    })
                    .detach();

                // -------- INPUT SENDER TASK --------
                let sock_clone = socket.try_clone().unwrap();
                let addr_clone = server_addr;

                IoTaskPool::get()
                    .spawn(async move {
                        while let Ok(input) = rx_inputs.recv().await {
                            let mut buf = Vec::with_capacity(5);
                            buf.extend_from_slice(&input.seq.to_be_bytes());
                            buf.push(input.mask);

                            let sock = sock_clone.try_clone().unwrap();
                            IoTaskPool::get()
                                .spawn(async move {
                                    Timer::after(Duration::from_millis(10)).await;
                                    sock.send_to(&buf, addr_clone).ok();
                                })
                                .detach();
                        }
                    })
                    .detach();

                // -------- INSERT RESOURCES --------
                commands.insert_resource(UdpClientSocket {
                    socket,
                    server_addr,
                });
                commands.insert_resource(ClientNetChannels {
                    rx_snapshots,
                    tx_inputs,
                });
            }
        }
        Err(e) => {
            eprintln!("[Client] Handshake failed: {}", e);
        }
    }
}

// -----------------------------------------------------------
//                SNAPSHOT APPLICATION + PREDICTION
// -----------------------------------------------------------
pub fn apply_snapshot_system(
    channels: Res<ClientNetChannels>,
    mut players: Query<(&mut Transform, &Player)>,
    mut prediction: ResMut<ClientPredictionState>,
    history: Res<InputHistory>,
) {
    while let Ok(snapshot) = channels.rx_snapshots.try_recv() {
        let tick = snapshot.tick;

        // Ignore stale snapshots
        if tick <= prediction.last_server_tick {
            continue;
        }
        prediction.last_server_tick = tick;

        // -----------------------------------------------------
        // 1. APPLY AUTHORITATIVE POSITION FOR LOCAL PLAYER
        // -----------------------------------------------------
        let mut local_id = None;

        for (_, player) in players.iter() {
            if let Player::Local(id) = player {
                local_id = Some(*id);
                break;
            }
        }

        let local_id = local_id.expect("Local player missing?!");

        let (auth_x, auth_y) = snapshot.positions[local_id];
        let authoritative = Vec2::new(auth_x, auth_y);

        prediction.authoritative_pos = authoritative;
        prediction.predicted_pos = authoritative;

        // -----------------------------------------------------
        // 2. ROLLBACK & REPLAY INPUTS FOR LOCAL PLAYER
        // -----------------------------------------------------
        for entry in history.entries.iter().filter(|e| e.tick > tick) {
            simulate_input(&mut prediction.predicted_pos, entry.mask);
        }

        // -----------------------------------------------------
        // 3. APPLY POSITIONS TO ALL PLAYERS
        // -----------------------------------------------------
        for (mut transform, player) in players.iter_mut() {
            match player {
                // ----------------------
                // LOCAL PREDICTED PLAYER
                // ----------------------
                Player::Local(id) => {
                    let current = transform.translation.truncate();
                    let target = prediction.predicted_pos;

                    // smooth correction avoid snapping
                    let blend = 0.12;
                    let new = current + (target - current) * blend;

                    transform.translation.x = new.x;
                    transform.translation.y = new.y;
                }

                // -----------------------
                // REMOTE NETWORK PLAYERS
                // -----------------------
                Player::Net(id) => {
                    if let Some((x, y)) = snapshot.positions.get(*id) {
                        let target = Vec2::new(*x, *y);
                        let current = transform.translation.truncate();

                        // Optional interpolation (looks smooth!)
                        let blend = 0.30;
                        let new = current + (target - current) * blend;

                        transform.translation.x = new.x;
                        transform.translation.y = new.y;
                    }
                }

                // -----------------------
                // NPCs ignore for now
                // -----------------------
                Player::Npc(_) => {}
            }
        }
    }
}

// -----------------------------------------------------------
//            SIMULATE INPUT (placeholder prediction)
// -----------------------------------------------------------
fn simulate_input(pos: &mut Vec2, mask: u8) {
    let dt = 1.0 / 60.0;

    let mut vel = Vec2::ZERO;

    if mask & (1 << 0) != 0 {
        vel.y += 450.0;
    }
    if mask & (1 << 1) != 0 {
        vel.x -= 300.0;
    }
    if mask & (1 << 2) != 0 {
        vel.y -= 300.0;
    }
    if mask & (1 << 3) != 0 {
        vel.x += 300.0;
    }

    *pos += vel * dt;
}
