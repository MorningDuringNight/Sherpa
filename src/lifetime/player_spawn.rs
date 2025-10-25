// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Spawn System>
use bevy::prelude::*;

use super::component::SpawnTrack;

use crate::event::{Lifetime2PlayerSpawn, ControlType};

pub(super) fn queue_for_player_setup_event(
    asset_server: Res<AssetServer>,
    mut track: ResMut<SpawnTrack>,
    mut events: EventWriter::<Lifetime2PlayerSpawn>,
) {
    let player_count = 3;
    track.expected_players = player_count;
    track.spawned_players = 0;
    track.node_to_entity = vec![None; player_count];
    track.is_rope = false;

    let tex: Handle<Image> = asset_server.load("portrait_rainey.png");
    events.write(Lifetime2PlayerSpawn {
        node: 0,
        texture: tex,
        position: Vec2::new(-100.0,-200.0),
        controls: ControlType::Keyboard {
            up: KeyCode::KeyW,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        },
        mass: Some(50.0),
    });

    let tex: Handle<Image> = asset_server.load("portrait_shawn.png");
    events.write(Lifetime2PlayerSpawn {
        node: 1,
        texture: tex,
        position: Vec2::new(-300.0,-200.0),
        controls: ControlType::Keyboard {
            up: KeyCode::ArrowUp,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
        },
        mass: Some(50.0),
    });

    let tex: Handle<Image> = asset_server.load("portrait_jagger.png");
    events.write(Lifetime2PlayerSpawn {
        node: 2,
        texture: tex,
        position: Vec2::new(-500.0,-200.0),
        controls: ControlType::Keyboard {
            up: KeyCode::KeyI,
            left: KeyCode::KeyJ,
            right: KeyCode::KeyL,
        },
        mass: Some(50.0),
    });
}