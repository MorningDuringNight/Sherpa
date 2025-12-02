use bevy::prelude::*;

pub mod enemylogic;
pub mod bundle;

use enemylogic::{spawn_enemy_system, update_enemy_system};

use crate::physics::collision::{enemy_player_collision_system, enemy_platform_collision_system, on_enemy_player_collision_system, on_enemy_platform_collision_system};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_enemy_system)
        .add_systems(FixedUpdate, (
            update_enemy_system, 
            on_enemy_player_collision_system,
            on_enemy_platform_collision_system,
            enemy_player_collision_system,
            enemy_platform_collision_system,
        ).chain());
    }
}