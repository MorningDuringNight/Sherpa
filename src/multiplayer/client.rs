use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::net::UdpSocket;
use std::time::Duration;
use async_channel::{Sender, Receiver};

use crate::{app::{GameMode}, player::{Player}};

/// Resource to hold the client socket after handshake
#[derive(Resource)]
pub struct UdpClientSocket {
    pub socket: UdpSocket,
    pub server_addr: std::net::SocketAddr,
}


#[derive(Debug)]
pub struct SnapshotUpdate {
    pub tick: u32,
    pub positions: Vec<(f32, f32)>,
}

#[derive(Resource)]
pub struct ClientNetChannels {
    pub rx_snapshots: Receiver<SnapshotUpdate>,
}

// send input to the socket in the main bevy ecs thread. Synchronously.
pub fn send_input_state_system(
    mut seq: Local<u32>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Option<Res<UdpClientSocket>>,
) {
    if client.is_none() { return; }
    let client = client.unwrap();

    let mut mask = 0u8;
    if keyboard.pressed(KeyCode::KeyW) { mask |= 1 << 0; }
    if keyboard.pressed(KeyCode::KeyA) { mask |= 1 << 1; }
    if keyboard.pressed(KeyCode::KeyS) { mask |= 1 << 2; }
    if keyboard.pressed(KeyCode::KeyD) { mask |= 1 << 3; }

    *seq += 1;
    let mut buf = Vec::with_capacity(5);
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.push(mask);

    if let Err(e) = client.socket.send_to(&buf, client.server_addr) {
        eprintln!("[Client] Failed to send input state: {}", e);
    }
}

/// resource to temporarily store the server address before handshake
#[derive(Resource)]
pub struct ServerAddress(pub String);

pub fn client_handshake(mut commands: Commands, server_addr: Res<ServerAddress>, gamemode: Res<GameMode>) {

    // Hostname resolution
    // let addr_str = &server_addr.0;
    // let mut addrs_iter = addr_str
    //     .to_socket_addrs()
    //     .expect("Failed to resolve hostname via DNS");
    //
    // let server_addr = addrs_iter
    //     .next()
    //     .expect("No addresses returned for server hostname");
    let server_addr: std::net::SocketAddr = server_addr
        .0
        .parse()
        .expect("Failed to parse server address");

    // create client UDP socket and bind to a random available port on localhost
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP client socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Failed to set read timeout");

    let (tx_snapshots, rx_snapshots) = async_channel::unbounded::<SnapshotUpdate>();
    let tx_snapshots_clone = tx_snapshots.clone();

    println!("[Client] Sending HELLO to {}", server_addr);

    let msg = match *gamemode {
        GameMode::NetCoop(id) if id == 0 => b"MAIN",
        GameMode::NetCoop(id) if id == 1 => b"PLAY",
        _ => b"ERRR",
    };

    socket
        .send_to(msg, server_addr)
        .expect("Failed to send handshake message");

    let mut buf = [0u8; 1024];
    // asynchronously recieve snapshots from the server
    match socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
            let msg = &buf[..len];
            if msg == b"ACK" {
                println!("[Client] Handshake successful with server {}", addr);

                // clone socket so it can live in both sending and receiving tasks
                // shouldn't cause race conditions because I am sending inputs and recieving
                // positions
                // this data should have a tick number attached so we can check if it stale or not.
                let socket_clone = socket.try_clone().expect("Failed to clone client socket");

                let task_pool = IoTaskPool::get();
                // send keys. 
                task_pool.spawn(async move {
                    let mut buf = [0u8; 1500];
                    loop {
                        match socket_clone.recv_from(&mut buf) {
                            Ok((len, from)) => {
                                let snapshot = &buf[..len];

                                // parse the state out of the snapshot packet recieved from the server
                                if snapshot.len() < 6 {
                                    eprintln!("[Client] Invalid snapshot length {}", snapshot.len());
                                    continue;
                                }

                                let tick = u32::from_be_bytes(snapshot[0..4].try_into().unwrap());
                                let player_count = u16::from_be_bytes(snapshot[4..6].try_into().unwrap()) as usize;

                                let mut offset = 6;
                                // println!("Tick {} with {} players", tick, player_count);

                                let mut positions = Vec::with_capacity(player_count);

                                // iterate through list of players and their positions.
                                for i in 0..player_count {
                                    if offset + 8 > snapshot.len() {
                                        eprintln!("[Client] Truncated snapshot for player {}", i);
                                        break;
                                    }

                                    let x = f32::from_be_bytes(snapshot[offset..offset+4].try_into().unwrap());
                                    let y = f32::from_be_bytes(snapshot[offset+4..offset+8].try_into().unwrap());
                                    offset += 8;

                                    positions.push((x, y));
                                }
                                if let Err(e) = tx_snapshots_clone.try_send(SnapshotUpdate { tick, positions }) {
                                    eprintln!("[Client] Failed to enqueue snapshot: {}", e);
                                }
                            }

                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                std::thread::yield_now();
                                continue;
                            }
                            Err(e) => {
                                eprintln!("[Client] Snapshot recv error: {}", e);
                                break;
                            }
                        }
                    }
                }).detach();

                // insert client socket for later use; (sending inputs to the socket)
                commands.insert_resource(UdpClientSocket {
                    socket,
                    server_addr,
                });
                // channel for receiving snapshots from the server into the main thread and
                // processing with apply_snapshot_system
                commands.insert_resource(ClientNetChannels { rx_snapshots });
            }
        }
        Err(e) => {
            eprintln!("[Client] Handshake failed: {}", e);
        }
    }
}

pub fn apply_snapshot_system(
    channels: Res<ClientNetChannels>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    while let Ok(snapshot) = channels.rx_snapshots.try_recv() {
        if snapshot.positions.is_empty() {
            continue;
        }

        // iterate through all players
        // snapshots must come back in player_number order.
        for (mut transform, player) in players.iter_mut() {
            match player {
                Player::Local(id) => {
                    if let Some((x, y)) = snapshot.positions.get(*id) {
                        // Local players are applied directly
                        transform.translation.x = *x;
                        transform.translation.y = *y;
                    }
                }
                Player::Net(id) => {
                    if let Some((x, y)) = snapshot.positions.get(*id) {
                        // Net players will be interpolated later
                        // TODO: replace with interpolation logic
                        transform.translation.x = *x;
                        transform.translation.y = *y;
                    }
                }
                Player::Npc(_) => {
                    // nothing yet.
                }
            }
        }
    }
}
