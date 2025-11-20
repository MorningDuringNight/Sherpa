// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Action>
use bevy::prelude::*;
use rand::Rng;

use super::qtable::{QTable, Action};
use crate::observer::system::Observation;

const QTABLE_PATH: &str = "assets/qtable.csv";
const ALPHA: f32 = 0.05;
const GAMMA: f32 = 0.97;
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
    r: Option<(usize, f32, f32, f32)>, // coin, height, level, is_wall
}

#[derive(Default)]
pub struct ActionCommit {
    pub frames_left: u8,
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
    mut commit: Local<ActionCommit>,
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
        let r = (obs.coin, obs.height, obs.level, obs.is_wall);
        // info!("Receive observation {}, {}, {}, {}", x, y, vx, vy);

        if let (Some(s_pre), Some(a_pre), Some(r_pre)) = (&last_state.s, &last_action.a, &last_reward.r) {
            if s != *s_pre { // Plan A: update only when move
                let reward = f_reward(*r_pre, r);
                let old_q = q.get(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre);
                let max_q = q.max_q(s[0], s[1], s[2], s[3]);
                let target = reward + GAMMA * max_q;
                let new_q = old_q + ALPHA * (target - old_q);
                q.set(s_pre[0], s_pre[1], s_pre[2], s_pre[3], *a_pre, new_q);
                updated = true;
                // if commit.frames_left == 0 {
                info!("Reward: {}, {}, {}, {}, {}", obs.coin, obs.height, obs.level, obs.is_wall, reward);
                // }
            }
        }
        
        // if *step % 10 == 0 {
        //     let action = epsilon_greedy(&q, s);
        //     last_action.a = Some(action);
        //     e_act.write(RLAction { action });
        // } else {
        //     if let Some(a) = last_action.a {
        //         e_act.write(RLAction { action: a });
        //     } else {
        //         let a = Action::I;
        //         last_action.a = Some(a);
        //         e_act.write(RLAction { action: a });
        //     }
        // }

        let greedy = epsilon_greedy(&q, s);

        let action_to_do = if commit.frames_left == 0 {
            match greedy {
                Action::I => {
                    Action::I
                }
                a_non_idle => {
                    commit.frames_left = 9;
                    a_non_idle
                }
            }
        } else {
            if greedy == Action::I {
                commit.frames_left = 0;
                Action::I
            } else {
                commit.frames_left -= 1;
                last_action.a.unwrap_or(greedy)
            }
        };

        e_act.write(RLAction { action: action_to_do });
        last_action.a = Some(action_to_do);

        last_state.s = Some(s);
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

const COIN_R: f32 = 40.0;
const UP_R: f32 = 1.0;
const DOWN_R: f32 = 0.5;
const LEVEL_R: f32 = 6.0;
const WALL_R: f32 = 10.0;
const GOOD_PLACE: f32 = 2.0;
const HEIGHT_SCORE: f32 = 0.002;
const AWAY_WALL: f32 = 2.0;
const TIME_PENALTY: f32 = 0.5;

fn f_reward(r_pre: (usize, f32, f32, f32), r: (usize, f32, f32, f32)) -> f32 {
    let (c_prev, h_prev, level_prev, wall_prev) = r_pre;
    let (c,      h,      level,      wall)      = r;

    let coin_diff  = (c as i32 - c_prev as i32) as f32;
    let h_diff     = h - h_prev;
    let level_diff = level - level_prev;
    let wall_diff  = wall - wall_prev;

    let r_coin  = COIN_R * coin_diff;
    let r_jump  = UP_R * h_diff.max(0.0) + DOWN_R * h_diff.min(0.0);
    let r_level = LEVEL_R * level_diff;
    let r_wall  = - WALL_R * wall_diff;
    
    let r_height_state = HEIGHT_SCORE * (h + 32.0);
    let r_level_state = GOOD_PLACE * level;
    let r_wall_state = - AWAY_WALL * wall;
    
    let time_penalty = - TIME_PENALTY;

    let reward = r_coin + r_jump + r_level + r_wall +
                 r_height_state + r_level_state + r_wall_state +
                 time_penalty;
    // info!("{}, {}, {}, {}, total{}", coin, height, level, wall, rr);
    // info!("{}, {}, {}, {}, total{}", 50.0 * (r.0 as f32), 0.3 * r.1, 8.0 * r.2, - 200.0 * r.3, r1);
    reward
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