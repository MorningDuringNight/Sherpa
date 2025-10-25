// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Control components>
use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub(super) struct ControlScheme {
    pub move_axis: f32,
    pub jump_just: bool,
}

#[derive(Component, Clone, Debug)]
pub(super) struct KeyboardControlled {
    pub up: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}