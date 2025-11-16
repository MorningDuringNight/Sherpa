use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;
use serde::{Deserialize,Serialize};
use crate::observer::{plugin,state,system};
use std::collections::HashMap;
use std::fs;
use std::io;


#[derive(Serialize, Deserialize)]

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
        //the state is the vector that is made from observer and returned here
        //only do the positions 
        table.insert(
            (vec![0, 0, 0, 0], BotState::idel), 0.0);
        Self {
            table: table,
            action_size: 6,
            discount: 0.99,
            learning_rate: 0.4,
            explore: 0.4,
        }

    }
}

impl QCreate {
    pub fn new(mut HashMap: HashMap<Vec<i32>,f64>, action_size: i32, discount: f64, learning_rate: f64, explore:f64) -> Self{

                Self{
            table: HashMap::new(),
            action_size,
            discount,
            learning_rate,
            explore,
        }
    }

    pub fn add_state_overwrite(&mut self, state:Vec<i32>,action: BotState,q_value:f64){
        self.table.insert((state, action), q_value);
    }

    pub fn get_qvalue(&mut self, state:Vec<i32>, action: BotState) -> (){
        self.table.get(&(state,action)).unwrap_or(&0.0);
    }

    // pub fn get_total(mut HashMap: HashMap<Vec<i32>,f64>, state:Vec<i32>){
    //    let idels = HashMap.get(&(state,BotState::idel)).unwrap_or(&0.0);
    // }

}