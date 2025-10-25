// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;
use bevy::time::Fixed;

mod gravity;
mod clear;
mod integrate;

pub(super) mod bundle;
pub(super) mod config;
pub(super) mod component;

use self::gravity::gravity_system;
use self::clear::clean_force;
use self::clear::clean_impulse;
use self::integrate::netforce_to_momentum;
use self::integrate::impulse_to_momentum;
use self::integrate::momentum_to_velocity;
use self::integrate::velocity_to_transform;
use self::integrate::boundary;
// use self::net_force::collect_force_events_system;

use super::schedule::PhysicsSet;

pub(super) struct PhysicsCorePlugin;
impl Plugin for PhysicsCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                PhysicsSet::Clear,
                PhysicsSet::Emit,
                PhysicsSet::Integrate,
            ).chain(),
        )
        .add_systems(
            FixedUpdate, 
            (
                clean_force,
                clean_impulse,
            ).in_set(PhysicsSet::Clear)
        )
        .add_systems(
            FixedUpdate, 
            (
                gravity_system,
            ).in_set(PhysicsSet::Emit)
        )
        .add_systems(
            FixedUpdate, 
            (
                netforce_to_momentum,
                impulse_to_momentum,
                momentum_to_velocity,
                velocity_to_transform,
                boundary,
            ).in_set(PhysicsSet::Integrate).chain()
        );
    }
}