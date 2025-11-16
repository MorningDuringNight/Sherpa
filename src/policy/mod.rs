// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

pub mod action;

use self::action::player_move;

pub struct PolicyPlugin;
impl Plugin for PolicyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_move);
    }
}
