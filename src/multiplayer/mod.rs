use bevy::prelude::*;
pub mod server;
pub mod client;


use client::*;
use server::*;
use crate::config::MyAppState;

pub struct UdpServerPlugin;

impl Plugin for UdpServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientRegistry::default())
            .add_systems(Startup, setup_udp_server.after(crate::player::load_players::spawn_players))
            .add_systems(FixedUpdate, send_snapshots_system.run_if(has_clients)
            .run_if(in_state(MyAppState::InGame)))
            .add_systems(FixedUpdate, process_remote_inputs_system
                .run_if(in_state(MyAppState::InGame)));
    }

}

pub struct UdpClientPlugin {
    pub server_addr: String,
}

impl Plugin for UdpClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerAddress(self.server_addr.clone()))
            .add_systems(Startup, client_handshake)
            .add_systems(FixedUpdate, (
                send_input_state_system,
            ).chain()
                .run_if(in_state(MyAppState::InGame)))
            .add_systems(
                FixedUpdate,
                apply_snapshot_system.run_if(resource_exists::<ClientNetChannels>)
                .run_if(in_state(MyAppState::InGame)));
    }
}
