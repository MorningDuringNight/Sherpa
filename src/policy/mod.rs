// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Policy mod>
use bevy::prelude::*;

pub mod action;
mod debug;
pub mod qtable;

use crate::app::BotActive;

use self::action::RLAction;
use self::action::RLAction2;
use self::action::make_qlearning_system;
use self::debug::make_gizmos;
use self::qtable::QTable;

const QTABLE_PATH: &str = "assets/qtable.csv";
const QTABLE_P2: &str = "assets/qtableSec.csv";
pub struct PolicyPlugin;
impl Plugin for PolicyPlugin {
    fn build(&self, app: &mut App) {
        // let q1 = PathWay{path: "assets/qtable.csv"};
        // let q2 = PathWay{path: "assets/qtable.csv"};
        app.add_systems(Startup, qtable_load_system)
            //    .add_systems(Update, qlearning_update)
            .add_systems(Update, make_qlearning_system(QTABLE_PATH.to_string(), true))
            .add_systems(Update, make_qlearning_system(QTABLE_P2.to_string(), false));
    }
}

#[derive(Resource)]
pub struct Tupper(pub QTable, pub QTable);

fn qtable_load_system(mut commands: Commands) {
    let table = QTable::load_from_csv(QTABLE_PATH).unwrap_or_else(|_| {
        println!("qtable.csv missing or fail to read, create a new '0' QTable");
        QTable::new()
    });

    let table_p2 = QTable::load_from_csv(QTABLE_P2).unwrap_or_else(|_| {
        println!("qtable.csv missing or fail to read, create a new '0' QTable");
        QTable::new()
    });

    let mut QTtup = Tupper(table, table_p2);
    commands.insert_resource(QTtup);
}
