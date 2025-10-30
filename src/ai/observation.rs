#![cfg(feature = "obs")]  

use bevy::prelude::*;

pub struct ObservationPlugin;

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_observations_system)
           .add_systems(Update, debug_observation_log);
    }
}

// -------------------------
// Observation components
// -------------------------

/// Holds numeric state inputs for the AI policy network.
#[derive(Component, Debug, Default)]
pub struct ObservationOut {
    pub vec: Vec<f32>,
    pub mask: Vec<f32>,
    pub version: u32,
}

/// Configures sensing range and ray settings.
#[derive(Component, Debug, Clone)]
pub struct SenseConfig {
    pub ray_max: f32,
    pub sense_range: f32,
    pub k_nearest: usize,
}

impl Default for SenseConfig {
    fn default() -> Self {
        Self {
            ray_max: 16.0,
            sense_range: 250.0,
            k_nearest: 3,
        }
    }
}

/// Small utility flag for visual debugging
#[derive(Resource, Default)]
pub struct ObservationTick(pub u32);

// -------------------------
// Systems (mock logic for now)
// -------------------------

/// Example system: simulate sensing nearby entities
fn update_observations_system(
    time: Res<Time>,
    mut tick: ResMut<ObservationTick>,
    mut query: Query<&mut ObservationOut>,
) {
    tick.0 += 1;

    for mut obs in query.iter_mut() {
        // fake observation update
        let t = time.elapsed_seconds().sin();
        obs.vec = vec![t, tick.0 as f32];
        obs.mask = vec![1.0, 0.0];
        obs.version = tick.0;
    }
}

/// Example debug print — shows that system works when feature enabled
fn debug_observation_log(
    tick: Res<ObservationTick>,
    query: Query<&ObservationOut>,
) {
    if tick.0 % 60 == 0 {
        let count = query.iter().count();
        info!("(obs) Tick {}, updated {} entities", tick.0, count);
    }
}
