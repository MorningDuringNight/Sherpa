// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime - Player Spawn>
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub(super) struct SpawnTrack {
    pub expected_players: usize,
    pub spawned_players: usize,
    pub node_to_entity: Vec<Option<Entity>>, //std::collections::HashMap<f32, Entity>,
    pub is_rope: bool,
}

impl Default for SpawnTrack {
    fn default() -> Self {
        SpawnTrack {
            expected_players: 2,
            spawned_players: 0,
            node_to_entity: vec![None; 2],
            is_rope: false,
        }
    }
}