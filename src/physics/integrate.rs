// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for physics integration>
use bevy::prelude::*;
use crate::components::motion::{Velocity, NetForce, Momentum, Mass };
use crate::player::Player;

pub fn clean_force_system(
    mut query: Query<&mut NetForce>,
) {
    for mut net_force in query.iter_mut() {
        net_force.0 = Vec2::ZERO;
    }
}

pub fn integrate_force_system(
    time: Res<Time>,
    mut query: Query<(&mut Momentum, &NetForce)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut momentum, net_force) in query.iter_mut() {
        momentum.0 += net_force.0 * delta_seconds;
    }
}

pub fn integrate_momentum_system(
    mut query: Query<(&mut Velocity, &Momentum, &Mass)>,
) {
    for (mut velocity, momentum, mass) in query.iter_mut() {
        velocity.0 = momentum.0 / mass.0;
    }
}

pub fn integrate_velocity_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * delta_seconds;
        transform.translation.y += velocity.0.y * delta_seconds;
    }
}

// Force to give a windows boundary
pub fn boundary(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Momentum)>,
) {
    let width = 1280.0 - 64.0; // minus player width
    let height = 720.0 - 64.0; // minus player height
    for (mut transform, mut velocity, mut momentum) in query.iter_mut() {
        if transform.translation.x < 32.0{
            transform.translation.x = 32.0;
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.x > width + 32.0{
            transform.translation.x = width + 32.0;
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.y < 32.0 {
            transform.translation.y = 32.0;
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
        }
        if transform.translation.y > (64.0*32.0) {
            transform.translation.y = height + 32.0;
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
        }
    }
}
