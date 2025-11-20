// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Policy mod>
use bevy::prelude::*;

pub mod action;
pub mod qtable;
mod debug;

use self::action::qlearning_update;
use self::action::RLAction;
use self::debug::qtable_gizmos;
use self::qtable::QTable;

pub struct PolicyPlugin;
impl Plugin for PolicyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RLAction>()
           .add_systems(Startup, qtable_load_system)
           .add_systems(Update, qlearning_update)
           .add_systems(Update, qtable_gizmos);
    }
}

const QTABLE_PATH: &str = "assets/qtable.csv";

fn qtable_load_system(mut commands: Commands) {
    let table = QTable::load_from_csv(QTABLE_PATH)
        .unwrap_or_else(|_| {
            println!("qtable.csv missing or fail to read, create a new '0' QTable");
            QTable::new()
        });

    commands.insert_resource(table);
}
