// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Coin detect>
use bevy::prelude::*;


pub fn coin_detect(
    mut coinCount: ResMut<TotalCoin>,
    mut players: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut Momentum,
        &PlayerCollider,
        &mut GroundState,
    ), With<Player>>,
    colliders: Query<(Entity, &Transform, &Collider), With<Coin>, Without<Player>>,
){
    let dt = time.delta_secs();

    for (player, mut transform, mut velocity, mut momentum, player_collider, mut ground) in players.iter_mut() {
        let mut player_aabb = predicted_aabb(&transform, &velocity, player_collider, dt);
        ground.is_grounded = false;

        for (game_object, collider_transform, collider) in colliders.iter() {
            let collider_pos = collider_transform.translation.truncate();
            let collider_aabb = collider.aabb.translated_by(collider_pos);

            if player_aabb.intersects(&collider_aabb) {
                commands.entity(ev.game_object).despawn();
                coinCount.amount += 1; 
            }
        }
    }
}