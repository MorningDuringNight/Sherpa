use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::net::UdpSocket;
use std::time::Duration;
use async_channel::{Sender, Receiver};

use crate::{app::{IsMainPlayer, MainPlayer}, player::Player};

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

pub fn client_handshake(mut commands: Commands, server_addr: Res<ServerAddress>, is_main_player: Res<IsMainPlayer>) {

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
    let socket = UdpSocket::bind(format!("0.0.0.0:0")).expect("Failed to bind UDP client socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Failed to set read timeout");

    // ecs world and networking IO world each own different ends of this channel
    // snapshots_in -> producer: is the snapshot receiving network task  | consumer: apply_snapshot_system, which recieves on server_snapshots.recv at a fixed rate.
    // most systems are running at about 60pps / 60 fps
    let (ecs_snapshots_in, server_snapshots) = async_channel::unbounded::<SnapshotUpdate>();

    println!("[Client] Sending HELLO to {}", server_addr);

    // todo player refactor.
    // default collider on all platforms by default.
    // It can be hardcoded to 2 players.
    // You should be able to pick player 1 or player 2
    // I could run some terminal commands before starting
    // which player are you; what server ip?
    // network mode vs local mode vs ai mode?;
    // insert it as a resource that will conditionally
    // impact systems and decide which systems are included
    // instead of the server annotations (maybe still for asset loading?).
    //  The solution is split out the parts which are reused and compose new functions and systems
    //  and plugins
    //  The player can be controlled by sending an event each frame; events are sent while idle;
    //  the ovehead is okay; but a state or a resouce is a better solution; although we need to
    //  maintain the ability to send the player input state at any frame.
    //  component enum struct Player(LocalPlayer, NetworkedPlayer, AiPlayer)
        //  result | we can query for Player and access the player type from within the player
    //  player control mapping; "w'a's'd" in NetworkedMode for all LocalPlayer
    //  for LocalPlay mode | wasd for player 1 | up down left right for player 2
    //  for AiMode(Training, P2) player 1 is wasd | Ai sends movement events to the main event loop.
    //  2 Ai | should probably run in headless mode.
    //  How does the server know which players to simulate???
        // 3 lobby's with Predetermined names
        // no create new lobby functionality yet
        // pick your player with (btn 1 or 2)
        // how is player choice communicated to the server?
        // what needs to be communicated?
        // how should I query for a specific player on the simulation side that maps to a networked 
        // I could use an index but better to use a name;
        // send this message in the handshake establishment.
        // this is when you map client socket to entity.
        // in server mode you shouldn't initialize the entities on the server side until you agree to
        // start the game
        // Third player?
        //
        // how do we start the game at the same time;
        // we say what tick we are on and we agree to start the game at some
        // time in the future.
        // or the server tells us to;
        // your inputs should not be registered during this time
        // the server should not be receiving packets yet
    let msg = if is_main_player.as_ref().0 {
        b"MAIN"
    } else {
        b"PLAY"
    };

    socket
        .send_to(msg, server_addr)
        .expect("Failed to send handshake message");

    let mut buf = [0u8; 1024];
    // asynchronously recieve snapshotsrom the server
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
                                    println!("[Client] Invalid snapshot length {}", snapshot.len());
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
                                if let Err(e) = ecs_snapshots_in.try_send(SnapshotUpdate { tick, positions }) {
                                    eprintln!("[Client] Failed to enqueue snapshot: {}", e);
                                }
                            }

                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
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
                commands.insert_resource(ClientNetChannels { rx_snapshots: server_snapshots });
            }
        }
        Err(e) => {
            eprintln!("[Client] Handshake failed: {}", e);
        }
    }
}

pub fn apply_snapshot_system(
    channels: Res<ClientNetChannels>,
    mut main_query: Query<&mut Transform, (With<Player>, With<MainPlayer>)>,
    mut other_query: Query<&mut Transform, (With<Player>, Without<MainPlayer>)>,
) {
    while let Ok(snapshot) = channels.rx_snapshots.try_recv() {
        if snapshot.positions.is_empty() {
            continue;
        }

        if let Ok(mut main_transform) = main_query.single_mut() {
            if let Some((x, y)) = snapshot.positions.get(0) {
                main_transform.translation.x = *x;
                main_transform.translation.y = *y;
            }
        }

        if let Ok(mut other_transform) = other_query.single_mut() {
            if let Some((x, y)) = snapshot.positions.get(1) {
                other_transform.translation.x = *x;
                other_transform.translation.y = *y;
            }
        }
    }
}
