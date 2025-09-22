// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player bundle and components>

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Velocity (pub Vec2);