// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Action>
use bevy::prelude::*;

use super::qtable::{QTable, Action};
use crate::observer::system::Observation;

const QTABLE_PATH: &str = "assets/qtable.csv";
const ALPHA: f32 = 0.1;
const GAMMA: f32 = 0.99;

#[derive(Default)]
pub struct LastState {
    s: Option<[usize; 4]>,   // [x, y, vx, vy]
}

#[derive(Default)]
pub struct LastAction {
    a: Option<Action>,
}

pub fn qlearning_update(
    mut obs_r: EventReader<Observation>,
    mut q: ResMut<QTable>,
    mut step: Local<u32>,
    mut last_state: Local<LastState>,
    mut last_action: Local<LastAction>,
) {
    let mut updated = false;

    for obs in obs_r.read() {
        let x = obs.observation[0] as usize;
        let y = obs.observation[1] as usize;
        let vx = (obs.observation[2] + 1) as usize; // -1,0,1 -> 0,1,2
        let vy = (obs.observation[3] + 1) as usize;

        let s = [x, y, vx, vy];
        // info!("Receive observation {}, {}, {}, {}", x, y, vx, vy);

        if let (Some(s_pre), Some(a_pre)) = (&last_state.s, &last_action.a) {
            let reward = f_reward(s);
            let old_q = q.get(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre);
            let max_q = q.max_q(s[0], s[1], s[2], s[3]);
            let target = reward + GAMMA * max_q;
            let new_q = old_q + ALPHA * (target - old_q);
            q.set(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre, new_q);
            updated = true;
        }
        last_state.s = Some(s);
        last_action.a = Some(Action::I);
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

fn f_reward(state: [usize; 4]) -> f32 {
    state[1] as f32
}