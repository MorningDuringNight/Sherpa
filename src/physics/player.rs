// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player physics>
use bevy::prelude::*;

use super::physics_core::bundle::PhysicsBundle;
use crate::event::Player2PhysicsAttach;

pub(super) fn player_insert_physics(
    mut commands: Commands,
    mut events: EventReader<Player2PhysicsAttach>,
) {
    for event in events.read() {
        commands.entity(event.entity).insert(PhysicsBundle::new(event.mass, true));
    }
}