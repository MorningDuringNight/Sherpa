// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Net force module>
use bevy::prelude::*;
use super::component::{NetForce, Impulse};

pub(super) fn clean_force(
    mut query: Query<&mut NetForce>,
) {
    for mut net_force in query.iter_mut() {
        net_force.0 = Vec2::ZERO;
    }
}

pub(super) fn clean_impulse(
    mut query: Query<&mut Impulse>,
) {
    for mut impulse in query.iter_mut() {
        impulse.0 = Vec2::ZERO;
    }
}
