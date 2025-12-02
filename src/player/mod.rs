use bevy::prelude::*;
pub mod bundle;
pub mod load_players;
pub mod player_control;

use crate::config::MyAppState;

use crate::config::MyAppState;

use self::player_control::{player_movement_input_system, player_input_collection_system, platform_spawn_system, despawn_platform_system, PlayerInputEvent};

use crate::player::load_players::reset_player;

pub use self::load_players::{Player, spawn_players};
pub use bundle::PlayerCollider;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>();

        #[cfg(feature = "server")]
        {
            app.add_systems(Startup, spawn_players);
        }
        #[cfg(feature = "client")]
        {
            app.add_systems(OnEnter(MyAppState::InGame), spawn_players);
        }

        app.add_systems(FixedUpdate, (player_movement_input_system).chain());
        app.add_systems(Update, (platform_spawn_system, despawn_platform_system).run_if(in_state(MyAppState::InGame)));

        #[cfg(feature = "client")]
        // doesn't do much at all when running with client+server
        // kind sorta client side prediction already.
        app.add_systems(Update, player_input_collection_system);
    }
}
