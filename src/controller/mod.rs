// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Policy mod>
use bevy::prelude::*;

use crate::policy::action::RLAction;
use crate::policy::qtable::Action;
use crate::player::{Player, ControlMode};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controller_update);
        app.add_systems(Update, controller_update2);
    }
}

fn controller_update (
    mut events: EventReader<RLAction>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    query: Query<(&Player, &ControlMode)>,
) {
    // Check if player1 is AI-controlled
    let p1_is_ai = query.iter()
        .find(|(player, _)| matches!(player, Player::Local(0)))
        .map(|(_, mode)| *mode == ControlMode::AI)
        .unwrap_or(false);
    
    for event in events.read() {
        // Only process player1 events, and only when AI-controlled
        if !event.is_p1 || !p1_is_ai {
            continue;
        }
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

fn controller_update2 (
    mut events: EventReader<RLAction>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    query: Query<(&Player, &ControlMode)>,
) {
    // Check if player2 is AI-controlled
    let p2_is_ai = query.iter()
        .find(|(player, _)| matches!(player, Player::Local(1)))
        .map(|(_, mode)| *mode == ControlMode::AI)
        .unwrap_or(false);
    
    for event in events.read() {
        // Only process player2 events, and only when AI-controlled
        if event.is_p1 || !p2_is_ai {
            continue;
        }
        let action = event.action;
        // info!("Action {}", action.index());
        keys.release(KeyCode::ArrowLeft);
        keys.release(KeyCode::ArrowRight);
        keys.release(KeyCode::ArrowUp);

        match action {
            Action::L => {
                keys.press(KeyCode::ArrowLeft);
            },
            Action::R => {
                keys.press(KeyCode::ArrowRight);
            },
            Action::J => {
                keys.press(KeyCode::ArrowUp);
            },
            Action::LJ => {
                keys.press(KeyCode::ArrowLeft);
                keys.press(KeyCode::ArrowUp);
            },
            Action::RJ => {
                keys.press(KeyCode::ArrowRight);
                keys.press(KeyCode::ArrowUp);
            },
            _ => {}
        }
    }
}