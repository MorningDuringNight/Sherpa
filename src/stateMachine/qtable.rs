use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;
use serde::{Deserialize,Serialize};
use crate::observer::{plugin,state,system};
use std::collections::HashMap;

pub struct qtable {
    qvalue: Vec<Vec<f64>>,
    config: QCreate,
}

#[derive(serde::Serialize, serde::Deserialize)]

pub struct QCreate{
    pub table: HashMap<(Vec<i32>,BotState), f64>,
    pub action_size: i32,
    pub discount: f64,
    pub learning_rate: f64,
    pub explore: f64,
}

impl Default for QCreate{
    fn default() -> Self{
        let mut table = HashMap::new();
        table.insert(
            (vec![0, 0, 0, 0], BotState::new()), 0.0);
        Self {
            table: table,
            action_size: 4,
            discount: 0.99,
            learning_rate: 0.4,
            explore: 0.4,

        }
    }

}

impl qtable{
    pub fn new() -> Self {
        Self::new_with(Default::default())
    }

    pub fn sized(&self) -> Self{
        self.config..state_size
    }

    pub fn pick_state(&self, index: usize) -> Option<State> {
        State::new_on(self, index).ok()
    }
}