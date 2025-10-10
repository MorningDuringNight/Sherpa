use async_std::{net::UdpSocket};
use crate::{
    app::MainPlayer,
    player::{Player, player_control::PlayerInputEvent},
};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, TaskPool, TaskPoolBuilder};
use std::{
    collections::HashMap,
    net::{SocketAddr},
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

// Multiplayer improvements
// There is non-determinism
// Say that the game is starting
// We need to run the simulation loop in a way where we can 
// apply player 1 or player input to a specific simulation frame
// in the past or present.
// we should run the simulation loop ahead of the client by some proportion of the round trip time.
// so that the client recieves inputs around the right moment. 
// How would we set this up to prevent input delay on the user end.
// Maybe we run the simulation behind the current player state?
// Whichever way results in the client and server talking at around the same time after accounting
// for ping
//
// Create a system for storing the last 4 states and applying inputs to them.
// There is a data structure that fits well here. linked list or array ; don't over complicate
// if we apply inputs in the past (if they we're missed) it impacts our current state so we can
// just drop all of the states after we apply missed or late/out of order inputs.
// 

// A snapshot message built on the ECS thread (game simulation) and sent to network task
#[derive(Debug)]
pub struct SnapshotMsg {
    pub data: Vec<u8>,
}

// data associated with a socket mapping in the client registry.
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
    pub clients: Arc<std::sync::RwLock<HashMap<SocketAddr, ClientSession>>>,
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

#[derive(Resource)]
pub struct NetChannels {
    // get snapshots from simulation to send over the net to all clients.
    pub ecs_snapshot_sender: Sender<SnapshotMsg>,
    // get inputs from net to simulate
    pub user_input_receiver: Receiver<RemoteInputEvent>,
}

// make registry
// init async_channels
pub fn setup_udp_server(mut commands: Commands, main_q: Query<Entity, With<MainPlayer>>, other_q: Query<Entity, (With<Player>, Without<MainPlayer>)>,
    ) {

    let (ecs_snapshot_sender, snapshot_receiver) = async_channel::unbounded::<SnapshotMsg>();
    let (user_input_sender, user_input_receiver) = async_channel::unbounded::<RemoteInputEvent>();

    let main_entity = main_q.single().expect("MainPlayer entity not found");
    let other_entity = other_q.single().expect("Secondary Player entity not found");

    let registry = ClientRegistry::default();
    let socket = Arc::new(async_std::task::block_on(UdpSocket::bind("0.0.0.0:5000"))
        .expect("Failed to bind UDP socket"),
    );


    println!("[UDP] Listening on 0.0.0.0:5000");

    // receive network inputs from clients
    // either handshake or player input state packets.
    {
        let socket   = Arc::clone(&socket);
        let registry = registry.clone();
        let user_input = user_input_sender.clone();
        let main_entity  = main_entity;   // capture real entities
        let other_entity = other_entity;

        IoTaskPool::get().spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let (len, addr) = match socket.recv_from(&mut buf).await {
                    Ok(v) => v,
                    Err(e) => { eprintln!("[UDP recv] error: {}", e); continue; }
                };

                let data = &buf[..len];

                if data == b"MAIN" || data == b"PLAY" {
                    handle_handshake(&socket, &registry, addr, data, main_entity, other_entity).await;
                } else if let Some(evt) = parse_input_packet(addr, data, &registry) {
                    // unbounded channel: try_send never blocks, only fails if closed
                    if let Err(e) = user_input.try_send(evt) {
                        eprintln!("[UDP recv] Failed to send input to ECS: {}", e);
                    }
                }
            }
        }).detach();
    }

    // receive snapshots from bevy ecs; send to all clients (as fast as you recieved them).
    // the are sent at 1x per game frame on the client side.
    {
        let socket = Arc::clone(&socket);
        let registry = registry.clone();

        IoTaskPool::get().spawn(async move {
            println!("[UDP send] Broadcast task started");
            while let Ok(msg) = snapshot_receiver.recv().await {
                let addrs: Vec<_> = {
                    let guard = registry.clients.read().unwrap();
                    guard.keys().cloned().collect()
                };

                for addr in addrs {
                    if let Err(e) = socket.send_to(&msg.data, addr).await {
                        eprintln!("[UDP send] failed to {}: {}", addr, e);
                    }
                }
            }
        }).detach();
    }
    commands.insert_resource(NetChannels { ecs_snapshot_sender, user_input_receiver });
    commands.insert_resource(registry.clone());
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
    let mut n = 0;
    while let Ok(remote) = channels.user_input_receiver.try_recv() {
        n += 1;
        // quick sanity print
        // println!("[ECS] input for entity {:?}", remote.player);
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

// get game state from the ecs world and send it over the channel to the thread.
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

        println!("{} {}", x, y);
        buf.extend_from_slice(&x.to_be_bytes());
        buf.extend_from_slice(&y.to_be_bytes());
    }

    if let Err(e) = channels.ecs_snapshot_sender.try_send(SnapshotMsg { data: buf }) {
        eprintln!("[Server] Failed to send snapshot to net task: {}", e);
    }
}

// send ACK to client when they send an handshake packet (MAIN OR PLAY).
async fn handle_handshake(
    socket: &Arc<UdpSocket>,
    registry: &ClientRegistry,
    addr: SocketAddr,
    msg: &[u8],
    main_entity: Entity,
    other_entity: Entity,
) {
    let assigned = if msg == b"MAIN" { main_entity } else { other_entity };
    println!("[Server] Handshake from {} -> {:?}", addr, if msg == b"MAIN" { "MAIN" } else { "PLAY" });

    {
        let mut map = registry.clients.write().unwrap();
        map.insert(addr, ClientSession {
            last_seen: Instant::now(),
            prev_mask: 0,
            player: assigned,
        });
    }

    if let Err(e) = socket.send_to(b"ACK", addr).await {
        eprintln!("[Server] Failed to send ACK: {}", e);
    }
}

// validates input packets from players returns player input state struct to send to the bevy ecs thread for simulation
fn parse_input_packet(
    addr: SocketAddr,
    data: &[u8],
    clients: &ClientRegistry,
) -> Option<RemoteInputEvent> {
    println!(
        "[UDP parse] incoming packet from {} ({} bytes): {:?}",
        addr,
        data.len(),
        data
    );

    if data.len() < 5 {
        println!(
            "[UDP parse] ❌ too short ({} bytes, expected >= 5) from {}",
            data.len(),
            addr
        );
        return None;
    }

    // Attempt to lock registry and find client
    let mut map = match clients.clients.write() {
        Ok(m) => m,
        Err(e) => {
            // eprintln!("[UDP parse] ❌ failed to lock client registry: {}", e);
            return None;
        }
    };

    if !map.contains_key(&addr) {
        println!(
            "[UDP parse] ⚠️  ignoring input from unknown client {} (registry has {} clients)",
            addr,
            map.len()
        );
        for key in map.keys() {
            println!("    - known client: {}", key);
        }
        return None;
    }

    let client = map.get_mut(&addr)?;
    let prev_mask = client.prev_mask;

    // decode sequence + mask
    let seq_bytes = &data[0..4];
    let mask = data[4];
    let seq = u32::from_be_bytes(seq_bytes.try_into().unwrap());

    // decode button states
    let jump_pressed = mask & (1 << 0) != 0;
    let left = mask & (1 << 1) != 0;
    let right = mask & (1 << 3) != 0;

    // println!(
    //     "[UDP parse] state → left={} right={} jump_pressed={}",
    //     left, right, jump_pressed
    // );

    // compare vs previous recieved input to see if jump was initiated or stopped.
    let jump_prev_pressed = prev_mask & (1 << 0) != 0;
    let jump_just_pressed = jump_pressed && !jump_prev_pressed;
    let jump_just_released = !jump_pressed && jump_prev_pressed;

    // update client session
    client.prev_mask = mask;
    client.last_seen = Instant::now();

    // event will be sent over the channel from the receiving task to the bevy thread.
    let evt = RemoteInputEvent {
        player: client.player,
        left,
        right,
        jump_pressed,
        jump_just_released,
    };

    Some(evt)
}
