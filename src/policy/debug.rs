// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Debug>
use bevy::prelude::*;

use super::qtable::{QTable, Action};
use super::qtable::{X_N, Y_N};
use super::Tupper;

const CELL_SIZE: f32 = 64.0;
const MAX_ARROW_LEN: f32 = 24.0;
const MAX_IDLE_RADIUS: f32 = 12.0;

pub fn make_gizmos(is_P1: bool) -> impl FnMut(
    Res<Tupper>,
    Gizmos,
) {
    move | qt, gizmos| {
        qtable_gizmos(
            qt,
            gizmos,
            is_P1,
        );
    }
}

pub fn qtable_gizmos(
    qt: Res<Tupper>,
    mut gizmos: Gizmos,
    is_P1: bool,
) {

    let Tupper(q1, q2) = qt.as_ref();
    
    let q = if is_P1 {
        q1
    } else {
        q2
    };
    

    for x in 0..X_N {
        for y in 0..Y_N {
            let qs = q.avg_q_xy(x, y);

            let mut max_abs: f32 = 0.0;
            for v in qs.iter() {
                max_abs = max_abs.max(v.abs());
            }
            if max_abs == 0.0 {
                continue;
            }

            let cx = (x as f32 + 0.5) * CELL_SIZE;
            let cy = (y as f32 + 0.5) * CELL_SIZE;
            let center = Vec2::new(cx, cy);

            let q_idle = qs[Action::I.index()];
            if q_idle.abs() > 1e-5 {
                let radius = (q_idle / max_abs).abs() * MAX_IDLE_RADIUS;

                let idle_color = if q_idle >= 0.0 {
                    Color::srgb(0.0, 1.0, 0.0)
                } else {
                    Color::srgb(1.0, 0.0, 0.0)
                };

                let r = radius.max(1.0);
                gizmos.circle_2d(center, r, idle_color);
            }

            let dirs: &[(Action, Vec2)] = &[
                (Action::L,  Vec2::new(-1.0,  0.0)),
                (Action::R,  Vec2::new( 1.0,  0.0)),
                (Action::J,  Vec2::new( 0.0,  1.0)),
                (Action::LJ, Vec2::new(-0.7, 0.7)),
                (Action::RJ, Vec2::new( 0.7, 0.7)),
            ];

            for (a, dir) in dirs {
                let qv = qs[a.index()];
                if qv.abs() < 1e-5 {
                    continue;
                }

                let len = (qv / max_abs) * MAX_ARROW_LEN;

                let color = if qv >= 0.0 {
                    Color::srgb(0.0, 1.0, 0.0)
                } else {
                    Color::srgb(1.0, 0.0, 0.0)
                };

                let end = center + *dir * len;

                gizmos.arrow_2d(center, end, color);
            }
        }
    }
}