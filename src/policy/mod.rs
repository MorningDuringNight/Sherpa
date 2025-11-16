// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Policy mod>
use bevy::prelude::*;

pub mod action;
pub mod qtable;

use self::action::player_move;
use self::qtable::QTable;

pub struct PolicyPlugin;
impl Plugin for PolicyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, qtable_load_system)
           .add_systems(Update, player_move);
    }
}

const QTABLE_PATH: &str = "qtable.csv";

fn qtable_load_system(mut commands: Commands) {
    let table = QTable::load_from_csv(QTABLE_PATH)
        .unwrap_or_else(|_| {
            println!("qtable.csv missing or fail to read, create a new '0' QTable");
            QTable::new()
        });

    commands.insert_resource(table);
}
