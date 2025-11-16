// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Action>
use bevy::prelude::*;

use crate::observer::system::Observation;

pub fn player_move(
    mut obs_r: EventReader<Observation>,
) {
    for obs in obs_r.read() {
        let x = obs.observation[0];
        let y = obs.observation[1];
        let vx = obs.observation[2];
        let vy = obs.observation[3];

        info!("Receive observation {}, {}, {}, {}", x, y, vx, vy);
    }
}