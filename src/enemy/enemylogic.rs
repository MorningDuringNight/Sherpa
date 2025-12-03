use bevy::prelude::*;

use crate::enemy::bundle::{Enemy, EnemyBundle, EnemyMovement};

pub fn spawn_enemy_system(mut commands: Commands) {
    let spawnx = 500.0;
    let spawny = 1000.0;

    commands.spawn(EnemyBundle::new(spawnx, spawny));
}

pub fn update_enemy_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemyMovement), With<Enemy>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        let change = movement.speed * time.delta_secs();
        // Move enemy based on direction and speed
        if movement.down {
            transform.translation.y -= change;
        } else {
            transform.translation.y += change;
        }
    }
}
