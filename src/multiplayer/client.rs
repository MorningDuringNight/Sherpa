use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use super::handshake_data::*;
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

pub fn parse_snapshot(snapshot: &[u8]) -> (u32, Vec<(f32, f32)>) {
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
    (tick, positions)
}

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
    let (net_snapshots_in, snapshot_receiver) = async_channel::unbounded::<SnapshotUpdate>();

    println!("[Client] Sending HELLO to {}", server_addr);
    let mut buf = [0u8; 1024];

    // let responses: Vec<_> = 
    // tick + player selection + characterSelection + player name.
    let mut packet_number = 0;
    while packet_number < 15  {

        let handshake_message = HandshakeData::new(packet_number,0, 0, "blue_fish", "Sean").encode();
        socket
            .send_to(handshake_message.as_slice(), server_addr)
            .expect("Failed to send handshake message");


        // out of order packets 
        // number each packet
        // resolve on the server-side;
        match socket.recv_from(&mut buf) {
            // recieve messages inlcuding. time sent from server.
            Ok((len, addr)) => {
                // calculate ping.
                // the client sends the time that it sent Handshake message
                // the server diff-message.send-now from when it was received.
                // send back ACK+PlayerNumber from server.
                //
                // client computes RTT
                // make an array of size 15 match the server responses
                // 15 handshake messages -> 15 acks (or less)
                // then process the data. and begin returning a rolling average of rtt.
                // client sends packet numbers 
                // server sends instant with response
                let msg = &buf[..len];
                if let Some(resp) = HandshakeResponse::decode(msg){
                    println!("{:#?}", resp);
                }
                // parse handshake response
                // server ACK + PlayerNumber + PacketNumber + Server
            }
            Err(e) => {
                eprintln!("[Client] Handshake packet failed.: {}", e);
            }
        }
        packet_number += 1;
    }
    let msg = if is_main_player.as_ref().0 {
        b"MAIN"
    } else {
        b"PLAY"
    };


    // Expect new data's -> {instant, tick, playerNumber (for assigning controls),
    // characterSelection for assigning sprite,} | also playerName | later on for
    // leaderboard stretch goal.


    // expect the next message you receive from the server to be "ACK" in response to main or hello
    // it could be lost with udp
    // instead we should send 60 packets and calculate ping.
    // the function could have a funny name udp_client_packet_gun 
    // send ack in response from server, if we didnt recieve any response, the client failed
    // handshake and they can't play (but there are a lot of reason's this could happen, and there
    // could be a tcp fallback maybe)
    // server should send ack to the correct client. or send a unique response to each client.
    // structure is bad, lets pull out handshake logic.
    match socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
            let msg = &buf[..len];
            println!("[Client] Handshake successful with server {}", addr);

            // clone socket so it can live in both sending and receiving tasks
            // shouldn't cause race conditions because I am sending inputs and recieving
            // positions
            // this data should have a tick number attached so we can check if it stale or not.
            // send ack to every single one
            let socket_clone = socket.try_clone().expect("Failed to clone client socket");

            let task_pool = IoTaskPool::get();
            // send keys. 
            // potentially spawn this thread in a different system.
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

                            let (tick, positions) = parse_snapshot(snapshot);

                            if let Err(e) = net_snapshots_in.try_send(SnapshotUpdate { tick, positions }) {
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
            commands.insert_resource(ClientNetChannels { rx_snapshots: snapshot_receiver });
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
