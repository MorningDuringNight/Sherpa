// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision physics>
use crate::event::{Collision2PhysicsDetected, CollisionType};
use crate::physics::physics_core::component::Velocity;
use crate::player::component::Player;
use bevy::math::bounding::BoundingVolume;
use bevy::prelude::*;

const PLATFORM_FRICTION: f32 = 0.88;

pub(super) fn resolve_collision_system(
    mut collision_events: EventReader<Collision2PhysicsDetected>,
    mut players: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for event in collision_events.read() {
        // Get minimum and maximum corners of both AABBs
        let min_x = event.aabb_a.min.x.max(event.aabb_b.min.x);
        let max_x = event.aabb_a.max.x.min(event.aabb_b.max.x);
        let min_y = event.aabb_b.min.y.max(event.aabb_b.min.y);
        let max_y = event.aabb_b.max.y.min(event.aabb_b.max.y);

        let overlap_x = max_x - min_x;
        let overlap_y = max_y - min_y;
        let overlap = Vec2::new(overlap_x, overlap_y);

        // Calculate normal for direction of collision
        let center_a = event.aabb_a.center();
        let center_b = event.aabb_b.center();
        let delta = center_a - center_b;
        let normal = delta.normalize_or_zero();

        match event.collision_type {
            CollisionType::Platform => {
                // Only modify or correct the player entity
                if let Ok((mut transform, mut velocity)) = players.get_mut(event.entity_a) {
                    resolve_platform_collision(&mut transform, &mut velocity, overlap, normal);
                };
            }
            // Resolve collisions for BOTH player entities
            CollisionType::Player => {
                // Modify collision for entity a
                if let Ok((mut transform, mut velocity)) = players.get_mut(event.entity_a) {
                    resolve_player_collision(
                        &mut transform,
                        &mut velocity, 
                        overlap, 
                        normal,
                    );  
                };
                // Modify for entity b
                if let Ok((mut transform, mut velocity)) = players.get_mut(event.entity_a) {
                    resolve_player_collision(
                        &mut transform, 
                        &mut velocity, 
                        overlap, 
                        normal,
                    );  
                };
            }
            CollisionType::Collectible => {
                // Don't need physics resolution for collectibles
            }
        }
    }
}

fn resolve_platform_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    overlap: Vec2,
    normal: Vec2,
) {
    // Determine if horizontal or vertical collision
    let is_horizontal = overlap.x < overlap.y;

    if is_horizontal {
        // Wall collision
        let direction = if normal.x > 0.0 { 1.0 } else { -1.0 };
        transform.translation.x -= overlap.x * direction;
        velocity.0.x = 0.0;

        // Need to add wall jump check if not grounded and can wall jump
    } else {
        // Floor/ceiling collision
        let direction = if normal.y > 0.0 { 1.0 } else { -1.0 };
        transform.translation.y -= overlap.y * direction;
        velocity.0.y = 0.0;

        // Check if landing on floor
        if normal.y > 0.0 {
            // Need to add resets for landing on ground and can wall jump
        }
    }
}

fn resolve_player_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    overlap: Vec2,
    normal: Vec2,
) {
    let is_horizontal = overlap.x < overlap.y;

    if is_horizontal {
        let direction = if normal.x > 0.0 { 1.0 } else { -1.0 };
        transform.translation.x -= overlap.x / 2.0 * direction;
        velocity.0.x = 0.0;
    } else {
        let direction = if normal.x > 0.0 { 1.0 } else { -1.0 };
        transform.translation.y -= overlap.y / 2.0 * direction;
        velocity.0.y = 0.0;
    }
}