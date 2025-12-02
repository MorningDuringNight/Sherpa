use core::f32;

use bevy::prelude::*;

use crate::enemy::bundle::{Enemy, EnemyBundle, EnemySpeed};
use crate::player::Player;

const ENEMY_SPAWN_HEIGHT: f32 = 800.0;
const ENEMY_SPAWN_TIME: f32 = 8.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_enemy_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    query: Query<&Transform, With<Player>>
) {
    spawn_timer.timer.tick(time.delta());
    
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut highest = f32::MIN;
    let mut found = false;
    let mut horizontal = f32::MIN;

    for transform in query.iter() {
        found = true;
        if transform.translation.y > highest {
            highest = transform.translation.y;
            horizontal = transform.translation.x;
        }
    }

    if found {
        commands.spawn(EnemyBundle::new(horizontal, highest + ENEMY_SPAWN_HEIGHT));
    }
}

pub fn despawn_enemy_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in query.iter() {
        // Check if the enemy is at the bottom of the screen
        if transform.translation.y < 0.0 {
            // Despawn if the entity reaches the bottom of the game world
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_enemy_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemySpeed), With<Enemy>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        // Move enemy downward
        transform.translation.y -= speed.speed * time.delta_secs();
    }
}