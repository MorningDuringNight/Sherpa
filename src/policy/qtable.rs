// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Q table Resource>
use bevy::prelude::*;
use std::fs::File;
use std::io;
use csv::ReaderBuilder;
use csv::WriterBuilder;

const X_N: usize = 20;
const Y_N: usize = 32;
const VX_N: usize = 3;
const VY_N: usize = 3;
const ACTION_N: usize = 6;
const NUM_STATES: usize = X_N * Y_N * VX_N * VY_N;

#[derive(Resource, Debug)]
pub struct QTable {
    pub qtable: Vec<[f32; 6]>,
}

impl QTable {
    pub fn new() -> Self {
        Self {
            qtable: vec![[0.0; ACTION_N]; NUM_STATES],
        }
    }

    fn state_index(x: usize, y: usize, vx: usize, vy: usize) -> usize {
        (((x * Y_N) + y) * VX_N + vx) * VY_N + vy
    }

    fn index_to_state(mut idx: usize) -> (usize, usize, usize, usize) {
        let vy = idx % VY_N;
        idx /= VY_N;
        let vx = idx % VX_N;
        idx /= VX_N;
        let y = idx % Y_N;
        idx /= Y_N;
        let x = idx;
        (x, y, vx, vy)
    }

    pub fn get(&self, x: usize, y: usize, vx: usize, vy: usize, a: Action) -> f32 {
        let idx = Self::state_index(x, y, vx, vy);
        self.qtable[idx][a.index()]
    }

    pub fn set(&mut self, x: usize, y: usize, vx: usize, vy: usize, a: Action, value: f32) {
        let idx = Self::state_index(x, y, vx, vy);
        self.qtable[idx][a.index()] = value;
    }

    pub fn best_a(&self, x: usize, y: usize, vx: usize, vy: usize) -> Action {
        self.best_a_and_q(x, y, vx, vy).0
    }

    pub fn max_q(&self, x: usize, y: usize, vx: usize, vy: usize) -> f32 {
        self.best_a_and_q(x, y, vx, vy).1
    }

    pub fn best_a_and_q(&self, x: usize, y: usize, vx: usize, vy: usize) -> (Action, f32) {
        let idx = Self::state_index(x, y, vx, vy);
        let row = &self.qtable[idx];

        let (best_idx, &best_q) = row
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        (Action::from_index(best_idx), best_q)
    }

    pub fn save_to_csv(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(file);

        wtr.write_record(&["x", "y", "vx", "vy", "I", "L", "R", "J", "LJ", "RJ"])?;

        for idx in 0..NUM_STATES {
            let (x, y, vx, vy) = Self::index_to_state(idx);
            let row = &self.qtable[idx];
            wtr.write_record(&[
                x.to_string(),
                y.to_string(),
                vx.to_string(),
                vy.to_string(),
                row[0].to_string(),
                row[1].to_string(),
                row[2].to_string(),
                row[3].to_string(),
                row[4].to_string(),
                row[5].to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn load_from_csv(path: &str) -> io::Result<Self> {
        let mut table = QTable::new();

        let file = File::open(path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in rdr.records() {
            let record = result
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let x: usize = record[0].parse().unwrap();
            let y: usize = record[1].parse().unwrap();
            let vx: usize = record[2].parse().unwrap();
            let vy: usize = record[3].parse().unwrap();

            let i_q: f32  = record[4].parse().unwrap();
            let l_q: f32  = record[5].parse().unwrap();
            let r_q: f32  = record[6].parse().unwrap();
            let j_q: f32  = record[7].parse().unwrap();
            let lj_q: f32 = record[8].parse().unwrap();
            let rj_q: f32 = record[9].parse().unwrap();

            let idx = QTable::state_index(x, y, vx, vy);

            table.qtable[idx] = [i_q, l_q, r_q, j_q, lj_q, rj_q];
        }

        Ok(table)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    I,
    L,
    R,
    J,
    LJ,
    RJ,
}

impl Action {
    pub fn index(self) -> usize {
        match self {
            Action::I  => 0,
            Action::L  => 1,
            Action::R  => 2,
            Action::J  => 3,
            Action::LJ => 4,
            Action::RJ => 5,
        }
    }
    
    pub const fn from_index(i: usize) -> Self {
        match i {
            0 => Action::I,
            1 => Action::L,
            2 => Action::R,
            3 => Action::J,
            4 => Action::LJ,
            5 => Action::RJ,
            _ => Action::I,
        }
    }
}