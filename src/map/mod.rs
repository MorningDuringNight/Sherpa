use bevy::prelude::*;
mod game_object_builder;
mod loader;
mod mapdata;
pub mod scroller;
mod util;
use scroller::camera_follow;

pub use game_object_builder::Collider;
pub use loader::Coin;
pub use mapdata::MapFile;

use loader::{load_background_layers, load_game_objects, load_map_data, load_render_resources};

const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "client")]
        app.add_systems(
            Startup,
            (
                load_map_data,
                load_render_resources,
                load_background_layers,
                load_game_objects,
            )
                .chain(),
        );
        #[cfg(feature = "server")]
        app.add_systems(Startup, (load_map_data, load_game_objects).chain());
    }
}
