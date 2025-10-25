// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;

mod scheme;
mod intent;
mod binding;

mod component;

use self::scheme::keyboard_control_system;
use self::intent::scheme_to_intent_writer;
use self::binding::on_request_control;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_request_control)
        .add_systems(Update, keyboard_control_system)
        .add_systems(Update, scheme_to_intent_writer);
    }
}