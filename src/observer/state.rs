use bevy::prelude::*;

/// Stores the summarized game observation for AI use.
#[derive(Resource, Default, Debug, Clone)]
pub struct ObservationState {
    pub positions: Vec2,
    pub velocities: Vec2,
    pub map_region: Option<String>,
}

fn discretize_velocity(v: &f32) -> i32 {
        let threshold = 10.0;
        if *v < -threshold {
            -1
        } else if *v > threshold  {
            1
        } else {
            0
        }
    }

impl ObservationState {
    /// Convert the observation into a numerical vector for AI (Q-learning).
    pub fn as_vector(&self) -> Vec<i32> {
        
        let x: i32 = (self.positions.x as i32) / 64;
        let y: i32 = (self.positions.y as i32) / 64;

        let vx_disc = discretize_velocity(&self.velocities.x);
        let vy_disc = discretize_velocity(&self.velocities.y);
        vec![
            x,
            y,
            vx_disc,
            vy_disc
        ]
    }

    /// Reset the state each frame before updates.
    pub fn clear(&mut self) {
        self.positions = Vec2::ZERO;
        self.velocities = Vec2:: ZERO;
        self.map_region = None;
    }
}
