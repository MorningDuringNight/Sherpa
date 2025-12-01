use bevy::prelude::*;
pub mod bundle;
pub mod load_players;
pub mod player_control;


use crate::components::motion::Mass;

use self::player_control::{player_movement_input_system, player_input_collection_system, PlayerInputEvent, toggle_prediction_system, PredictionOn};

pub use self::load_players::{spawn_players, Player, PlayerMass};
pub use bundle::{PlayerCollider};

pub struct PlayerPlugin;

pub fn toggle_player_mass(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Mass, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        for mut mass in &mut query {
            if mass.0 == 120.0 {
                mass.0 = 60.0;
            } else {
                mass.0 = 120.0;
            }
            info!("Player mass toggled at runtime: {}", mass.0);
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PredictionOn(true));
        app.insert_resource(PlayerMass(120.0));
        app.add_event::<PlayerInputEvent>();
        app.add_systems(Startup, spawn_players);
        app.add_systems(FixedUpdate, (player_movement_input_system).chain());

        #[cfg(feature = "client")]
        // doesn't do much at all when running with client+server
        // kind sorta client side prediction already.
        app.add_systems(Update, player_input_collection_system);
        app.add_systems(Update, toggle_prediction_system);
        app.add_systems(Update, toggle_player_mass);
    }


}
