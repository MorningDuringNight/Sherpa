// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Policy mod>
use bevy::prelude::*;

use crate::policy::action::RLAction;
use crate::policy::qtable::Action;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controller_update);
    }
}

fn controller_update (
    mut events: EventReader<RLAction>,
    mut commands: Commands,
) {
    for event in events.read() {
        let action = event.action;
        info!("Action {}", action.index());
    }
}