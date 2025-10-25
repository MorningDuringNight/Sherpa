// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player configuration>
use bevy::prelude::*;

const SCALE: f32 = 64.0; // 64 pixels = 1 meter

// Player spawn parameters
pub(super) const PLAYER_SIZE: Vec2 = Vec2::new(SCALE, SCALE);
pub(super) const PLAYER_SPAWN_MASS: f32 = 120.0; // in kg