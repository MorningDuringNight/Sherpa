use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;

pub struct qtable {
    qvalue: Vec2<Vec2<position>>,
    config: QCreate,
}

#[derive(serde::Serialize, serde::Deserialize)]

pub struct QCreate{
    pub state_size: i32,
    pub action_size: i32,
    pub discount: f64,
    pub learning_rate: f64,
    pub explore: f64,
}

impl Default for QCreate{
    fn default() -> Self{
        Self {
            state_size: 24,
            action_size: 4,
            discount: 0.99,
            learning_rate: 0.4,
            explore: 0.4,

        }
    }

}

impl qtable{
    pub fn new() -> Self{
        Self::new_with(Default::default())
    }

    pub fn sized(&self) -> Self{
        self.config..state_size
    }

    pub fn pick_state(&self, index: usize) -> Option<State> {
        State::new_on(self, index).ok()
    }
}