// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics configuration>

use bevy::prelude::*;

// Physics world scale
pub(super) const SCALE: f32 = 64.0; // 64 pixels = 1 meter

// Player control speed limit
pub(super) const PLAYER_CONTROL_SPEED_LIMIT: f32 = 10.0 * SCALE; // in pixel/s
// Player control parameters
pub(super) const PLAYER_MOVE_FORCE: f32 = 1500.0 * SCALE; // in Newton = kg*pixel/s^2
// pub(super) const PLAYER_JUMP_FORCE: f32 = 20000.0 * SCALE; // in Newton
pub(super) const PLAYER_JUMP_IMPULSE: f32 = 500.0 * SCALE; // jump impulse in 60hz