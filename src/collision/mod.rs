// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Jagger Hershey
// Description: <Collision mod>
use bevy::prelude::*;

mod component;
mod calculation;

pub use component::{PlayerCollider, Collider};
use crate::event::{Collision2PhysicsDetected, CollisionType};

use calculation::{detect_platform_collisions_system, detect_player_collisions_system};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // Register the collision detection event
        app.add_event::<Collision2PhysicsDetected>();
        
        // Run detection systems in FixedUpdate
        app.add_systems(
            FixedUpdate,
            (
                detect_platform_collisions_system,
                detect_player_collisions_system,
            ).chain()
        );
    }
}