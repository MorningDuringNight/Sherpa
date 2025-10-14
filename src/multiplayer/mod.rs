use bevy::prelude::*;
pub mod server;
pub mod client;
mod handshake_data;


use client::*;
use server::*;

pub struct UdpServerPlugin;


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
    // pick your player with (btn 1 or 2) after picking lobby.
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

impl Plugin for UdpServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_udp_server.after(crate::player::load_players::spawn_players))
            .add_systems(
                FixedUpdate,
                (
                    process_remote_inputs_system,
                    send_snapshots_system.after(process_remote_inputs_system),
                )
            );
    }

}

pub struct UdpClientPlugin {
    pub server_addr: String,
}

impl Plugin for UdpClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ServerAddress(self.server_addr.clone()))
            .add_systems(Startup, client_handshake)
            .add_systems(
                FixedUpdate,
                send_input_state_system
                    .before(apply_snapshot_system)
                    .chain()
            )
            .add_systems(
                FixedUpdate,
                (apply_snapshot_system.run_if(resource_exists::<ClientNetChannels>)).chain(),
            )
            .add_systems(
                Update,
                smooth_interpolation_system,
            );
    }
}
