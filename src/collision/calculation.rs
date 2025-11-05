// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Jagger Hershey
// Description: <Collision calculation>

use bevy::prelude::*;
use crate::collision::component::{Collider, PlayerCollider};
use crate::event::{Collision2PhysicsDetected, CollisionType};
use crate::player::component::Player;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

pub fn detect_platform_collisions_system(
    players: Query<(Entity, &Transform, &PlayerCollider), With<Player>>,
    platforms: Query<(Entity, &Transform, &Collider), Without<Player>>,
    mut event_writer: EventWriter<Collision2PhysicsDetected>,
) {
    // Check for each player against every platform
    for (player_entity, transform, player_collider) in players.iter() {
        let player_pos = transform.translation.truncate();
        let player_aabb = player_collider.aabb.translated_by(player_pos);

        for (platform_entity, platform_transform, collider) in platforms.iter() {
            let platform_pos = platform_transform.translation.truncate();
            let platform_aabb = collider.aabb.translated_by(platform_pos);

            // Check intersection
            if player_aabb.intersects(&platform_aabb) {
                // Send event to physics to handle
                event_writer.write(Collision2PhysicsDetected {
                    entity_a: player_entity,
                    entity_b: platform_entity,
                    collision_type: CollisionType::Platform,
                    aabb_a: player_aabb,
                    aabb_b: platform_aabb,
                });
            }
        }
    }
}

pub fn detect_player_collisions_system(
    players: Query<(Entity, &Transform, &PlayerCollider), With<Player>>,
    mut event_writer: EventWriter<Collision2PhysicsDetected>,
) {
    let players_vec: Vec<_> = players.iter().collect();

    for i in 0..players_vec.len() {
        for j in (i + 1)..players_vec.len() {
            let (entity1, transform1, collider1) = players_vec[i];
            let (entity2, transform2, collider2) = players_vec[j];

            // Get both positions
            let pos1 = transform1.translation.truncate();
            let pos2 = transform2.translation.truncate();

            let aabb1 = collider1.aabb.translated_by(pos1);
            let aabb2 = collider2.aabb.translated_by(pos2);

            if aabb1.intersects(&aabb2) {
                // Send event to physics
                event_writer.write(Collision2PhysicsDetected {
                    entity_a: entity1,
                    entity_b: entity2,
                    collision_type: CollisionType::Player,
                    aabb_a: aabb1,
                    aabb_b: aabb2,
                });
            }
        }
    }
}
