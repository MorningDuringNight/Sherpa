use bevy::prelude::*;
mod game_object_builder;
mod loader;
mod mapdata;
mod util;
use crate::config::MyAppState;

mod platformfunction;

pub use game_object_builder::Collider;
pub use loader::Coin;
pub use mapdata::MapFile;

use platformfunction::linear_move_with_easing;
use loader::{load_background_layers, load_game_objects, load_map_data, load_render_resources};

const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "client")]
        app.add_systems(
            OnEnter(MyAppState::InGame),
            (
                load_map_data,
                load_render_resources,
                load_background_layers,
                load_game_objects,
            )
            .chain(),
        );
      
        #[cfg(feature = "server")]
        app.add_systems( OnEnter(MyAppState::InGame), (load_map_data, load_game_objects).chain());
        
        app.add_systems(Update, linear_move_with_easing
             .run_if(in_state(MyAppState::InGame)));
    }
}
