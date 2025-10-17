// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <General physics bundle>
use bevy::prelude::*;

use super::component::{Velocity, NetForce, Gravity, Mass, Momentum};

#[derive(Bundle, Clone)]
pub struct PhysicsBundle {
    velocity: Velocity,
    net_force: NetForce,
    gravity: Gravity,
    mass: Mass,
    momentum: Momentum,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        PhysicsBundle {
            velocity: Velocity::default(),
            net_force: NetForce::default(),
            gravity: Gravity::default(),
            mass: Mass::default(),
            momentum: Momentum::default(),
        }
    }
}

impl PhysicsBundle {
    pub fn new(mass: f32, gravity: bool) -> Self {
        PhysicsBundle {
            velocity: Velocity::default(),
            net_force: NetForce::default(),
            gravity: Gravity(gravity),
            mass: Mass(mass),
            momentum: Momentum::default(),
        }
    }
}