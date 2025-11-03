// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <CoinPlugin>
use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource( TotalCoin {amount:0,});
        app.add_systems(Update, spawn_coin)
           .add_systems(Update, coin_detect);
    }
}