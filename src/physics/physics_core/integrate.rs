// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for physics integration>
use bevy::prelude::*;
use super::component::{Velocity, Momentum, Impulse, NetForce, Mass};

pub(super) fn netforce_to_momentum(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Momentum, &NetForce)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut momentum, net_force) in query.iter_mut() {
        momentum.0 += net_force.0 * delta_seconds;
    }
}

pub(super) fn impulse_to_momentum(
    mut query: Query<(&mut Momentum, &Impulse)>,
) {
    for (mut momentum, impulse) in query.iter_mut() {
        momentum.0 += impulse.0;
    }
}

pub(super) fn momentum_to_velocity(
    mut query: Query<(&mut Velocity, &Momentum, &Mass)>,
) {
    for (mut velocity, momentum, mass) in query.iter_mut() {
        velocity.0 = momentum.0 / mass.0;
    }
}

pub(super) fn velocity_to_transform(
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
pub(super) fn boundary(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Momentum)>,
) {
    let width = 1280.0 - 64.0; // minus player width
    let height = 720.0 - 64.0; // minus player height
    for (mut transform, mut velocity, mut momentum) in query.iter_mut() {
        if transform.translation.x < - width / 2.0 {
            transform.translation.x = - width / 2.0;
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.x > width / 2.0 {
            transform.translation.x = width / 2.0;
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.y < - height / 2.0 {
            transform.translation.y = - height / 2.0;
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
        }
        if transform.translation.y > height / 2.0 {
            transform.translation.y = height / 2.0;
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
        }
    }
}