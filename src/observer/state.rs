use bevy::prelude::*;

/// Stores the summarized game observation for AI use.
#[derive(Resource, Default, Debug)]
pub struct ObservationState {
    pub player_positions: Vec<Vec2>,
    pub rope_tension: f32,
    pub map_region: Option<String>,
}

impl ObservationState {
    /// Convert the observation into a numerical vector for AI (Q-learning).
    pub fn as_vector(&self) -> Vec<f32> {
        let mut vec = vec![self.rope_tension];
        for pos in &self.player_positions {
            vec.push(pos.x);
            vec.push(pos.y);
        }
        vec
    }

    /// Reset the state each frame before updates.
    pub fn clear(&mut self) {
        self.player_positions.clear();
        self.rope_tension = 0.0;
        self.map_region = None;
    }
}
