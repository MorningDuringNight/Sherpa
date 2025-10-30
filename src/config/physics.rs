// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics configuration>

use bevy::prelude::*;

// Physics world scale
pub const SCALE: f32 = 64.0; // 64 pixels = 1 meter

// Gravity constant
pub const GRAVITY: Vec2 = Vec2::new(0.0, -9.81 * SCALE); // in pixel/s^2

// Player control parameters
pub const PLAYER_CONTROL_SPEED_LIMIT: f32 = 10.0 * SCALE; // in pixel/s
pub const PLAYER_MOVE_FORCE: f32 = 4500.0 * SCALE; // in Newton = kg*pixel/s^2
pub const PLAYER_JUMP_FORCE: f32 = 12000.0 * SCALE; // in Newton

// Rope parameters
pub const ROPE_REST_LENGTH: f32 = 5.0 * SCALE;  // in pixel
pub const ROPE_MAX_EXTENSION: f32 = 50.0;  // Maximum extension beyond rest length
pub const SPRING_CONSTANT: f32 = 80000.0;  // in Newton/pixel = kg/s^2