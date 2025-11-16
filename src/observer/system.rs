use bevy::prelude::*;
use crate::player::Player;
use crate::components::motion::Velocity;
use crate::components::motion::GroundState; 
use crate::observer::state::ObservationState;
use crate::game_ui::ui::TotalCoin;

#[derive(Event, Debug)]
pub struct Observation {
    pub observation: Vec<i32>,
    pub coin: usize,
    pub height: f32,
}

/// System that collects live data from the world each frame.
pub fn update_observation_system(
    mut obs: ResMut<ObservationState>,
    c: ResMut<TotalCoin>,
    query_players: Query<(&Transform, &Velocity), With<Player>>,
    mut obs_w: EventWriter<Observation>,
) {
    obs.clear();

    let coin = c.amount as usize;

    // Gather player positions
    if let Some((transform, velocity)) = query_players.iter().next() {
        obs.positions = transform.translation.truncate();
        obs.velocities = velocity.0; 
    }
    let height = obs.positions.y;

    // estimate rope tension by measuring distance between players
    let obs_vec: Vec<_> = obs.as_vector();
    // info!("Observer updated: {:?}", &obs_vec);

    obs_w.write(Observation {
        observation: obs_vec,
        coin,
        height,
    });
}
