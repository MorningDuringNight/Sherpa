// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <AI policy>
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
// TODO-AI Bot: Choose policy

// the algorithm is
// Q(s,a) = q(s,a) + a(R + yi * maxQ(s`, a`) - Q(s,a))
//Q(s,a) is the q value for the state s and action a
// alpha is the learning rate , controlling how much new information overrides old infor
// R is the immediate reward for taking action a in states
//yi is the discount factor, represeting the importance of future rewards
//max_a Q(s`,a`) is the max Q valie for the next state s` represeting best possible reward achievable from that state


pub struct QTable {
    table: HashMap<(State, Action), f64>,
    learning: f64,
    discount: f64,
}

impl QTable {
    pub fn new(learning: f64, discount: f64) -> Self {
        Self {
            table: HashMap::new(),
            learning,
            discount,
        }
    }
    pub fn get_q(&self, state: &State, action: &Action) -> f64{
        
    }
}