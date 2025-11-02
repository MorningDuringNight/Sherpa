use bevy::prelude::*;
use crate::player::Player;
use crate::components::motion::Velocity;
use crate::components::motion::GroundState; 
use crate::observer::state::ObservationState;

/// System that collects live data from the world each frame.
pub fn update_observation_system(
    mut obs: ResMut<ObservationState>,
    query_players: Query<&Transform, With<Player>>,
) {
    obs.clear();

    // Gather player positions
    for transform in query_players.iter() {
        let pos = transform.translation.truncate();
        obs.player_positions.push(pos);
    }

    // estimate rope tension by measuring distance between players
    if obs.player_positions.len() == 2 {
        let dist = obs.player_positions[0].distance(obs.player_positions[1]);
        obs.rope_tension = dist.clamp(0.0, 1000.0);
    }

    info!("Observer updated: {:?}", obs.as_vector());
}
