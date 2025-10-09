use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use async_channel::{Sender, Receiver};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use crate::{app::MainPlayer, player::{player_control::PlayerInputEvent, Player}};

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


// make registry
// init async_channels
pub fn setup_udp_server(mut commands: Commands, main_player_q: Query<Entity, With<MainPlayer>>, other_player_q: Query<Entity, (With<Player>, Without<MainPlayer>)>,
    ) {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Failed to bind UDP socket");
    socket.set_nonblocking(true).unwrap();
    println!("[UDP Server] Listening on 0.0.0.0:5000");

    let registry = ClientRegistry::default();
    let socket_clone = socket.try_clone().unwrap();

    commands.insert_resource(UdpServerSocket { socket });
    commands.insert_resource(registry.clone());

    let task_pool = IoTaskPool::get();


    let (tx_snapshots, rx_snapshots) = async_channel::unbounded::<SnapshotMsg>();
    let (tx_inputs, rx_inputs) = async_channel::unbounded::<RemoteInputEvent>();

    // this could cause race conditions I need to think a bit more about it.
    let tx_inputs = tx_inputs.clone();

    let main_player_entity = main_player_q.single().expect("Expected a MainPlayer entity");
    let other_player_entity = other_player_q.single().expect("Expected a secondary Player entity");
    // Recieve from client
    // send inputs from clients to main ecs thread.
    {
        // shouldn't cause race issues; I am only setting on connection
        // and not mutating at all.
        //
        //
        // Instad of sending MAIN / Player f
        let recv_socket = socket_clone.try_clone().unwrap();
        let recv_clients = registry.clone();
        task_pool.spawn(async move {
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
                                main_player_entity,
                                other_player_entity,
                            );
                        }
                        else if data == b"KEEPALIVE" {
                            // client is sending a periodic NAT keepalive
                            if let Some(mut client_map) = recv_clients.clients.write().ok() {
                                if let Some(session) = client_map.get_mut(&addr) {
                                    session.last_seen = Instant::now();
                                    // (optional) println! for debugging:
                                    println!("[Server] Keepalive from {}, updated last_seen", addr);
                                }
                            }
                        }
                        else {
                            // we received some packet which was not a hankshake acknowledgement
                            if let Some(event) = parse_input_packet(addr, data, &recv_clients) {
                                // send input event to ECS thread through the async_channel.
                                if let Err(e) = tx_inputs.try_send(event) {
                                    eprintln!("[Server] Failed to send input event to ECS: {}", e);
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // if I could use a library for async udp socket we wouldn't have to yield
                        // here explicitly.
                        std::thread::yield_now();
                        continue;
                    }
                    Err(e) => {
                        eprintln!("[UDP Server] Recv error: {:?}", e);
                        break;
                    }
                }
            }
        }).detach();
    }

    // wait for game state / snapshot messages from ECS thread
    // brodcast changes to all clients.
    // there could be sepatate threads for each client. (but maybe this was is easier and less
    // complicated)
    {
        let broadcast_socket = socket_clone;
        let broadcast_clients = registry.clone();

        task_pool.spawn(async move {
            while let Ok(msg) = rx_snapshots.recv().await {
                let clients_guard = broadcast_clients.clients.read().unwrap();
                for addr in clients_guard.keys() {
                    if let Err(e) = broadcast_socket.send_to(&msg.data, addr) {
                        eprintln!("[UDP Server] Failed to send snapshot to {}: {}", addr, e);
                    }
                }
            }
        }).detach();
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

pub fn send_snapshots_system(
    players: Query<&Transform, With<Player>>,
    channels: Res<NetChannels>,
    mut tick: Local<u32>,
) {
    *tick += 1;

    let decimals = 1; // truncate to 1 decimal place
    let player_count = players.iter().len() as u16;

    // tick (4 bytes) + player_count (2 bytes) + N*(x:4, y:4)
    let mut buf = Vec::with_capacity(4 + 2 + player_count as usize * 8);
    buf.extend_from_slice(&tick.to_be_bytes());
    buf.extend_from_slice(&player_count.to_be_bytes());

    for transform in players.iter() {
        let x = truncate_f32(transform.translation.x, decimals);
        let y = truncate_f32(transform.translation.y, decimals);

        buf.extend_from_slice(&x.to_be_bytes());
        buf.extend_from_slice(&y.to_be_bytes());
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
    main_entity: Entity,
    other_entity: Entity,
) {
    let player_entity = if msg == b"MAIN" {
        println!("[Server] {} identified as MAIN player", addr);
        main_entity
    } else {
        println!("[Server] {} identified as regular PLAYER", addr);
        other_entity
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

// validates packet containing user input data and returns player input state struct
// which then gets sent through the channel to the ecs thread.
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
        left:  mask & (1 << 1) != 0,
        right: mask & (1 << 3) != 0,
        jump_pressed,
        jump_just_released,
    })
}
