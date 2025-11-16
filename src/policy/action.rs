// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Action>
use bevy::prelude::*;
use rand::Rng;

use super::qtable::{QTable, Action};
use crate::observer::system::Observation;

const QTABLE_PATH: &str = "assets/qtable.csv";
const ALPHA: f32 = 0.1;
const GAMMA: f32 = 0.99;
const EPSILON: f32 = 0.3;

#[derive(Default)]
pub struct LastState {
    s: Option<[usize; 4]>,   // [x, y, vx, vy]
}

#[derive(Default)]
pub struct LastAction {
    a: Option<Action>,
}

#[derive(Default)]
pub struct LastReward {
    r: Option<(usize, f32)>, // coin, height
}

#[derive(Event, Debug)]
pub struct RLAction {
    pub action: Action,
}

pub fn qlearning_update(
    mut obs_r: EventReader<Observation>,
    mut q: ResMut<QTable>,
    mut step: Local<u32>,
    mut last_state: Local<LastState>,
    mut last_action: Local<LastAction>,
    mut last_reward: Local<LastReward>,
    mut e_act: EventWriter<RLAction>,
) {
    let mut updated = false;

    for obs in obs_r.read() {
        let x = obs.observation[0] as usize;
        let y = obs.observation[1] as usize;
        let vx = (obs.observation[2] + 1) as usize; // -1,0,1 -> 0,1,2
        let vy = (obs.observation[3] + 1) as usize;

        // Q(s, a) <- Q(s, a) + α(r + γmaxQ(s', a') - Q(s, a))
        // Q(s, a) <- (1 - α)Q(s, a) + α(r + γmaxQ(s', a'))
        let s = [x, y, vx, vy];
        let r = (obs.coin, obs.height);
        // info!("Receive observation {}, {}, {}, {}", x, y, vx, vy);

        if let (Some(s_pre), Some(a_pre), Some(r_pre)) = (&last_state.s, &last_action.a, &last_reward.r) {
            let reward = f_reward(*r_pre, r);
            let old_q = q.get(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre);
            let max_q = q.max_q(s[0], s[1], s[2], s[3]);
            let target = reward + GAMMA * max_q;
            let new_q = old_q + ALPHA * (target - old_q);
            q.set(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre, new_q);
            updated = true;
        }
        let action = epsilon_greedy(&q, s);

        e_act.write(RLAction {
            action,
        });

        last_state.s = Some(s);
        last_action.a = Some(action);
        last_reward.r = Some(r);
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

fn f_reward(r_pre: (usize, f32), r: (usize, f32)) -> f32 {
    let coin_diff = r.0 - r_pre.0;
    let y_diff = r.1 - r_pre.1;
    100.0 * (coin_diff as f32) + 1.0 * y_diff - 0.1
}

fn epsilon_greedy(q: &QTable, s: [usize; 4]) -> Action {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < EPSILON {
        // info!("Lucy");
        let idx = rng.gen_range(0..6);
        Action::from_index(idx)
    } else {
        let action = q.best_a(s[0], s[1], s[2], s[3]);
        // info!("state {}, {}, {}, {}, action {}", s[0], s[1], s[2], s[3], action.index());
        action
    }
}