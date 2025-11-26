use bevy::prelude::*;

pub mod enemylogic;
pub mod bundle;

use enemylogic::{spawn_enemy_system, despawn_enemy_system, update_enemy_system, EnemySpawnTimer};

use crate::physics::collision::{enemy_player_collision_system, on_enemy_collision_system};
use crate::config::MyAppState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_systems(FixedUpdate, (
            spawn_enemy_system, 
            update_enemy_system, 
            on_enemy_collision_system,
            enemy_player_collision_system,
            despawn_enemy_system,
        ).chain()
        .run_if(in_state(MyAppState::InGame)));
    }
}