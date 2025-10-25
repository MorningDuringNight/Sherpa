// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Gravity system>
use bevy::prelude::*;
use super::config::GRAVITY;
use super::component::{NetForce, Gravity, Mass};

pub(super) fn gravity_system(
    mut query: Query<(&mut NetForce, &Gravity, &Mass)>,
) {
    for (mut net_force, gravity, mass) in &mut query {
        if gravity.0 {
            // F = m * g
            net_force.0 += GRAVITY * mass.0;
        }
    }
}
