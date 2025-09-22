// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player configuration>

use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(100.0, 200.0, 0.0);

#[derive(Resource, Clone, Copy)]
pub struct PlayerSpawnPoint {
    pub position: Vec3,
}