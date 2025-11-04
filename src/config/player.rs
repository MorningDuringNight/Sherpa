// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player configuration>

use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(50.0, 0.0, 0.0);
pub const PLAYER_INITIAL_VELOCITY: Vec2 = Vec2::new(0.0, 0.0);
pub const PLAYER_LENGTH: f32 = 64.;
pub const PLAYER_WIDTH: f32 = 64.;
pub const PLAYER_SPAWN_MASS: f32 = 120.0; // in kg


#[derive(Resource, Clone, Copy)]
pub struct PlayerSpawnPoint {
    pub position: Vec3,
}

#[derive(Resource, Clone, Copy)]
pub struct PlayerSpawnVelocity {
    pub velocity: Vec2,
}
