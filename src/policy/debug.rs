// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Debug>
use bevy::prelude::*;

use super::qtable::{QTable, Action};
use super::qtable::{X_N, Y_N};

const CELL_SIZE: f32 = 64.0;
const MAX_ARROW_LEN: f32 = 20.0;

pub fn qtable_gizmos(
    q: Res<QTable>,
    mut gizmos: Gizmos,
) {
    for x in 0..X_N {
        for y in 0..Y_N {
            let qs = q.avg_q_xy(x, y);

            let dirs: &[(Action, Vec2)] = &[
                (Action::L,  Vec2::new(-1.0,  0.0)),
                (Action::R,  Vec2::new( 1.0,  0.0)),
                (Action::J,  Vec2::new( 0.0,  1.0)),
                (Action::LJ, Vec2::new(-0.7, 0.7)),
                (Action::RJ, Vec2::new( 0.7, 0.7)),
            ];

            let mut max_abs: f32 = 0.0;
            for (a, _) in dirs {
                let qv = qs[a.index()];
                max_abs = max_abs.max(qv.abs());
            }
            if max_abs == 0.0 {
                continue;
            }

            let cx = (x as f32 + 0.5) * CELL_SIZE;
            let cy = (y as f32 + 0.5) * CELL_SIZE;
            let center = Vec2::new(cx, cy);

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