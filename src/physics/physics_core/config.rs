// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics configuration>

use bevy::prelude::*;

// Physics world scale
pub(super) const SCALE: f32 = 64.0; // 64 pixels = 1 meter

// Gravity constant
pub(super) const GRAVITY: Vec2 = Vec2::new(0.0, -9.81 * SCALE); // in pixel/s^2