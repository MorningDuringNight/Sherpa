// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;
use bevy::time::Fixed;

mod physics_core;
mod player;
mod control;

mod config;
mod schedule;
mod collision;

use self::physics_core::PhysicsCorePlugin;
use self::player::player_insert_physics;
use self::control::player_intent_to_force;
use self::collision::resolve_collision_system;
use self::schedule::PhysicsSet;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
           .configure_sets(
                FixedUpdate,
                (
                    PhysicsSet::Clear,
                    PhysicsSet::Emit,
                    PhysicsSet::Integrate,
                ).chain(),
            )
           .add_plugins(PhysicsCorePlugin)
           .add_systems(
                FixedUpdate,
                (
                    player_insert_physics,
                    player_intent_to_force,
                ).in_set(PhysicsSet::Emit).chain()
            )
            .add_systems(
                FixedUpdate,
                resolve_collision_system
                    .in_set(PhysicsSet::Integrate)
            );
    }
}