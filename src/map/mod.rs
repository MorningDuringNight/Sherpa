use bevy::prelude::*;
mod game_object_builder;
mod loader;
mod mapdata;
mod util;
<<<<<<< Updated upstream
=======
pub mod scroller;
use scroller::camera_follow;
use crate::config::MyAppState;

>>>>>>> Stashed changes

pub use game_object_builder::Collider;
pub use loader::{Coin, Platform};
pub use mapdata::MapFile;

use loader::{load_background_layers, load_map, load_map_resouces};

const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (load_map_resouces, load_background_layers, load_map).chain(),
        );
=======
            OnEnter(MyAppState::InGame),
            (load_map_resouces, load_background_layers, load_map,).chain(),
        )
        .add_systems(PostUpdate, camera_follow
            .run_if(in_state(MyAppState::InGame)));
>>>>>>> Stashed changes
    }
}
