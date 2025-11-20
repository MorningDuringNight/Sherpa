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
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        let action = event.action;
        // info!("Action {}", action.index());
        keys.release(KeyCode::KeyA);
        keys.release(KeyCode::KeyD);
        keys.release(KeyCode::KeyW);

        match action {
            Action::L => {
                keys.press(KeyCode::KeyA);
            },
            Action::R => {
                keys.press(KeyCode::KeyD);
            },
            Action::J => {
                keys.press(KeyCode::KeyW);
            },
            Action::LJ => {
                keys.press(KeyCode::KeyA);
                keys.press(KeyCode::KeyW);
            },
            Action::RJ => {
                keys.press(KeyCode::KeyD);
                keys.press(KeyCode::KeyW);
            },
            _ => {}
        }
    }
}