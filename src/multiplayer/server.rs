use crate::{
    player::{Player, player_control::PlayerInputEvent},
};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, TaskPool, TaskPoolBuilder};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

// A snapshot message built on the ECS thread and sent to network task
#[derive(Debug)]
pub struct SnapshotMsg {
    pub data: Vec<u8>,
}

// data associated with a socket mapping.
#[derive(Debug)]
pub struct ClientSession {
    pub last_seen: Instant,
    pub player: Entity,
    pub prev_mask: u8,
}

// we might not need a lock here, we build the client registry relatively Synchronously
// we do mutate the client session though. and possibly indirectly read from in from multiple threads.
#[derive(Resource, Default, Clone)]
pub struct ClientRegistry {
    pub clients: Arc<RwLock<HashMap<SocketAddr, ClientSession>>>,
}

#[derive(Resource)]
pub struct UdpServerSocket {
    pub socket: UdpSocket,
}

#[derive(Debug)]
// not actually an event very bad name oops.
pub struct RemoteInputEvent {
    pub player: Entity,
    pub left: bool,
    pub right: bool,
    pub jump_pressed: bool,
    pub jump_just_released: bool,
}

// tx_snapshots for getting state out of the simulation -> UDP thread
// rx_inputs for sending input events | UDP thread -> main game loop (inputs)
#[derive(Resource)]
pub struct NetChannels {
    pub tx_snapshots: Sender<SnapshotMsg>,
    pub rx_inputs: Receiver<RemoteInputEvent>,
}

fn custom_network_pool() -> TaskPool {
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(2)
        .max(4);

    let pool = TaskPoolBuilder::default()
        .num_threads(threads)
        .thread_name("udp-network".into())
        .build();

    println!(
        "[Init] Custom network pool with {} threads",
        pool.thread_num()
    );
    pool
}

// make registry
// init async_channels
pub fn setup_udp_server(
    mut commands: Commands,
    players: Query<(Entity, &Player)>,
) {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Failed to bind UDP socket");
    socket.set_nonblocking(false).unwrap();
    println!("[UDP Server] Listening on 0.0.0.0:5000");

    let registry = ClientRegistry::default();
    let socket_clone = socket.try_clone().unwrap();

    commands.insert_resource(UdpServerSocket { socket });
    commands.insert_resource(registry.clone());

    let (tx_snapshots, rx_snapshots) = async_channel::unbounded::<SnapshotMsg>();
    let (tx_inputs, rx_inputs) = async_channel::unbounded::<RemoteInputEvent>();

    // this could cause race conditions I need to think a bit more about it.
    let tx_inputs = tx_inputs.clone();

    let mut locals: Vec<(Entity, usize)> = players
        .iter()
        .filter_map(|(e, p)| match p { Player::Local(id) => Some((e, *id)), _ => None })
        .collect();

    locals.sort_by_key(|&(_, id)| id);

    let (p1, p2) = (locals[0].0, locals[1].0);
    // Recieve from client
    // send inputs from clients to main ecs thread.
    {
        // shouldn't cause race issues; I am only setting on connection
        // and not mutating at all.
        let recv_socket = socket_clone.try_clone().unwrap();
        let recv_clients = registry.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match recv_socket.recv_from(&mut buf) {
                    Ok((len, addr)) => {
                        let data = &buf[..len];

                        if data == b"MAIN" || data == b"PLAY" {
                            // builds client session and creates mapping in ClientRegistry
                            handle_handshake(
                                &recv_socket,
                                &recv_clients,
                                addr,
                                data,
                                p1,
                                p2,
                            );
                        } else {
                            // we received some packet which was not a hankshake acknowledgement
                            if let Some(event) = parse_input_packet(addr, data, &recv_clients) {
                                // send input event to ECS thread through the async_channel.
                                if let Err(e) = tx_inputs.try_send(event) {
                                    eprintln!("[Server] Failed to send input event to ECS: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[UDP Server] Recv error: {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    // wait for game state / snapshot messages from ECS thread
    // brodcast changes to all clients.
    // there could be sepatate threads for each client. (but maybe this was is easier and less
    // complicated)
    {
        let broadcast_socket = socket_clone;
        let broadcast_clients = registry.clone();

        thread::spawn(move || {
            let mut tick_count: u64 = 0;
            println!("[Thread] UDP broadcast thread started");

            while let Ok(msg) = rx_snapshots.recv_blocking() {
                tick_count += 1;

                // Decode tick number if present
                let tick = if msg.data.len() >= 4 {
                    u32::from_be_bytes(msg.data[0..4].try_into().unwrap_or_default())
                } else {
                    0
                };

                let clients_guard = broadcast_clients.clients.read().unwrap();
                let client_count = clients_guard.len();

                println!(
                    "[Broadcast] tick={} | snapshot_size={} bytes | connected_clients={}",
                    tick,
                    msg.data.len(),
                    client_count
                );

                if client_count == 0 {
                    println!("[Broadcast] No clients yet — skipping tick={}", tick);
                    continue;
                }

                for (i, addr) in clients_guard.keys().enumerate() {
                    println!("  [{}] Sending snapshot tick={} to {}", i, tick, addr);

                    match broadcast_socket.send_to(&msg.data, addr) {
                        Ok(sent) => {
                            println!(
                                "  [{}] ✅ Sent {} bytes successfully to {}",
                                i, sent, addr
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "  [{}] ❌ Failed to send snapshot to {}: {}",
                                i, addr, e
                            );
                        }
                    }
                }

                println!(
                    "[Broadcast] Finished tick={} ({} clients)\n",
                    tick, client_count
                );
            }

            println!("[Thread] UDP broadcast thread exited unexpectedly!");
        });
    }
    commands.insert_resource(NetChannels {
        tx_snapshots,
        rx_inputs,
    });
}

pub fn truncate_f32(v: f32, decimals: u32) -> f32 {
    let factor = 10f32.powi(decimals as i32);
    (v * factor).trunc() / factor
}

// listen for structs (RemoteInputEvent) sent through the channel in the (async receiving task).
// apply input state to player via events (meh solution maybe needs refactor).
pub fn process_remote_inputs_system(
    channels: Res<NetChannels>,
    mut writer: EventWriter<PlayerInputEvent>,
) {
    while let Ok(remote) = channels.rx_inputs.try_recv() {
        writer.write(PlayerInputEvent {
            entity: remote.player,
            left: remote.left,
            right: remote.right,
            jump_pressed: remote.jump_pressed,
            jump_just_released: remote.jump_just_released,
        });
    }
}

pub fn has_clients(registry: Option<Res<ClientRegistry>>) -> bool {
    if let Some(reg) = registry {
        let map = reg.clients.read().unwrap();
        !map.is_empty()
    } else {
        false
    }
}

pub fn send_snapshots_system(
    players: Query<(&Transform, &Player)>,
    channels: Res<NetChannels>,
    mut tick: Local<u32>,
) {
    *tick += 1;

    let decimals = 1;
    let mut locals: [Option<&Transform>; 2] = [None, None];

    // Collect references to Player::Local(0) and Player::Local(1)
    for (transform, player) in &players {
        if let Player::Local(id) = player {
            locals[*id] = Some(transform);
        }
    }

    // Fallback: only count players we actually found
    let player_count = locals.iter().flatten().count() as u16;

    // tick (4 bytes) + player_count (2 bytes) + N*(x:4, y:4)
    let mut buf = Vec::with_capacity(4 + 2 + player_count as usize * 8);
    buf.extend_from_slice(&tick.to_be_bytes());
    buf.extend_from_slice(&player_count.to_be_bytes());

    // Write Player::Local(0) first, then Player::Local(1)
    for id in 0..2 {
        if let Some(transform) = locals[id] {
            let x = truncate_f32(transform.translation.x, decimals);
            let y = truncate_f32(transform.translation.y, decimals);

            buf.extend_from_slice(&x.to_be_bytes());
            buf.extend_from_slice(&y.to_be_bytes());
        }
    }

    if let Err(e) = channels.tx_snapshots.try_send(SnapshotMsg { data: buf }) {
        eprintln!("[Server] Failed to send snapshot to net task: {}", e);
    }
}

fn handle_handshake(
    socket: &UdpSocket,
    registry: &ClientRegistry,
    addr: SocketAddr,
    msg: &[u8],
    p1: Entity,
    p2: Entity,
) {
    let player_entity = if msg == b"MAIN" {
        println!("[Server] {} identified as P1 player", addr);
        p1
    } else {
        println!("[Server] {} identified as P2 player", addr);
        p2
    };

    let mut map = registry.clients.write().unwrap();
    map.insert(
        addr,
        ClientSession {
            last_seen: Instant::now(),
            prev_mask: 0,
            player: player_entity,
        },
    );

    let _ = socket.send_to(b"ACK", addr);
}

// validates packet and returns player input state struct to send to the bevy ecs thread.
fn parse_input_packet(
    addr: SocketAddr,
    data: &[u8],
    clients: &ClientRegistry,
) -> Option<RemoteInputEvent> {
    if data.len() < 5 {
        return None;
    }

    let _seq = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let mask = data[4];

    let mut map = clients.clients.write().unwrap();
    // get client via their address.
    let client = map.get_mut(&addr)?;

    let prev_mask = client.prev_mask;

    // detect changes between prev packet and this packet to detect if you should
    // emit a PlayerInputEvent when receiving inputs from the thread to ecs loop via the channel.
    let jump_pressed = mask & (1 << 0) != 0;
    let jump_prev_pressed = prev_mask & (1 << 0) != 0;

    // not needed yet.
    let jump_just_pressed = jump_pressed && !jump_prev_pressed;
    let jump_just_released = !jump_pressed && jump_prev_pressed;

    // update session in client registry.
    // maybe this prev data should be stored somewhere else.
    client.prev_mask = mask;
    client.last_seen = Instant::now();

    Some(RemoteInputEvent {
        player: client.player,
        left: mask & (1 << 1) != 0,
        right: mask & (1 << 3) != 0,
        jump_pressed,
        jump_just_released,
    })
}
