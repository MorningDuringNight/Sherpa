use bevy::prelude::*;
pub mod bundle;
pub mod load_players;
pub mod player_control;


use self::player_control::{player_movement_input_system, player_input_collection_system, PlayerInputEvent};

pub use self::load_players::{spawn_players, Player};
pub use bundle::{PlayerCollider};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>();
        app.add_systems(Startup, spawn_players);
        app.add_systems(FixedUpdate, (player_movement_input_system).chain());

        #[cfg(feature = "client")]
        // doesn't do much at all when running with client+server
        // kind sorta client side prediction already.
        app.add_systems(Update, player_input_collection_system);
    }


}
