// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Action>
use bevy::prelude::*;

use super::qtable::QTable;
use crate::observer::system::Observation;

const QTABLE_PATH: &str = "assets/qtable.csv";

pub fn qlearning_update(
    mut obs_r: EventReader<Observation>,
    mut q: ResMut<QTable>,
    mut step: Local<u32>,
) {
    let mut updated = false;

    for obs in obs_r.read() {
        let x = obs.observation[0];
        let y = obs.observation[1];
        let vx = obs.observation[2] + 1;
        let vy = obs.observation[3] + 1;

        // info!("Receive observation {}, {}, {}, {}", x, y, vx, vy);
        updated = true;
    }

    if updated {
        *step += 1;
        // info!("P {}", *step);
        if *step % 200 == 0 {
            info!("Update {}", *step);
            if let Err(e) = q.save_to_csv(QTABLE_PATH) {
                eprintln!("Failed to save qtable: {e}");
            } else {
                println!("QTable saved at step {}", *step);
            }
        }
    }
}