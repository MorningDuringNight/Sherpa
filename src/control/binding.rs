// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Deal with Control Request>
use bevy::prelude::*;

use super::component::{KeyboardControlled, ControlScheme};
use crate::event::{Player2ControlAttach, ControlType};

fn clean_control(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity)
        .remove::<ControlScheme>();
}

pub(super) fn on_request_control(
    mut commands: Commands,
    mut reqs: EventReader<Player2ControlAttach>,
) {
    for req in reqs.read() {
        clean_control(&mut commands, req.entity);
        match &req.spec {
            ControlType::Keyboard {up, left, right} => {
                commands.entity(req.entity).insert((
                    KeyboardControlled { up: *up, left: *left, right: *right },
                    ControlScheme::default(),
                ));
            }
            ControlType::Aibot {..} => {
                commands.entity(req.entity).insert((
                    ControlScheme::default(),
                ));
            }
        }
    }
}