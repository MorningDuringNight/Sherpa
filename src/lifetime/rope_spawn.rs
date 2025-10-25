// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime - Player Spawn>
use bevy::prelude::*;

use crate::event::{Player2LifetimeSpawned, Lifetime2RopeSpawn};
use super::component::SpawnTrack;

pub(super) fn wait_for_player_spawn(
    mut track: ResMut<SpawnTrack>,
    mut events: EventReader<Player2LifetimeSpawned>,
    mut rope_events: EventWriter<Lifetime2RopeSpawn>,
) {
    for event in events.read() {
        let node = event.node as usize;
        if track.node_to_entity[node].is_none() {
            track.node_to_entity[node] = Some(event.entity);
            track.spawned_players += 1;
        }
    }

    if !track.is_rope && track.spawned_players == track.expected_players {
        let total_nodes = track.node_to_entity.len();

        // spawn rope between players
        for i in 0..total_nodes - 1 {
            if let (Some(head), Some(tail)) = (track.node_to_entity[i], track.node_to_entity[i + 1]) {
                rope_events.write(Lifetime2RopeSpawn {
                    head_entity: head,
                    tail_entity: tail,
                });
            }
        }

        // all players spawned, now spawn rope
        track.is_rope = true;
    }
}