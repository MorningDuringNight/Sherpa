// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for physics integration>
use crate::components::motion::{Mass, Momentum, NetForce, Velocity};
use crate::config::player::{PLAYER_LENGTH, PLAYER_WIDTH};
use crate::player::Player;
use bevy::prelude::*;

pub fn clean_force_system(mut query: Query<&mut NetForce>) {
    for mut net_force in query.iter_mut() {
        net_force.0 = Vec2::ZERO;
    }
}

pub fn integrate_force_system(time: Res<Time>, mut query: Query<(&mut Momentum, &NetForce)>) {
    let delta_seconds = time.delta_secs();
    for (mut momentum, net_force) in query.iter_mut() {
        momentum.0 += net_force.0 * delta_seconds;
    }
}

pub fn integrate_momentum_system(mut query: Query<(&mut Velocity, &Momentum, &Mass)>) {
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
    mut max_height_ev: EventWriter<super::MaxHeightReached>,
) {
    let width = 1280.0 - PLAYER_WIDTH; // minus player width
    let height = 64.0 * 64.0 - PLAYER_LENGTH; // minus player height
    for (mut transform, mut velocity, mut momentum) in query.iter_mut() {
        if transform.translation.x < PLAYER_WIDTH / 2. {
            transform.translation.x = PLAYER_WIDTH / 2.;
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.x > width + (PLAYER_WIDTH / 2.) {
            transform.translation.x = width + (PLAYER_WIDTH / 2.);
            velocity.0.x = 0.0;
            momentum.0.x = 0.0;
        }
        if transform.translation.y < PLAYER_LENGTH / 2. {
            transform.translation.y = PLAYER_LENGTH / 2.;
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
        }
        if transform.translation.y > (height) {
            transform.translation.y = height + (PLAYER_LENGTH / 2.);
            velocity.0.y = 0.0;
            momentum.0.y = 0.0;
            max_height_ev.write(super::MaxHeightReached {
                height: transform.translation.y,
            });
        }
    }
}
