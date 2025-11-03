// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Coin Spawn>
use bevy::prelude::*;
use map::game_object_builder::{GameObject};

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct TotalCoin{
    pub amount: u32,
}

#[derive(Component, Default)]
pub struct Coin;

#[derive(Event, Debug)]
pub struct SpawnCoinEvent {
    pub game_object: GameObject;
}


// TODO-Coin System: Spawn coin.

    

pub fn spawn_coin(
    mut commands: Commands,
    mut events: EventReader<SpawnCoinEvent>,
){
     for ev in events.read() {
        commands.spawn(ev.game_object);
     }
}