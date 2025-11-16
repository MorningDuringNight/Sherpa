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
    pub is_wall: f32,
}

const LEFT_WALL: f32 = 32.0;
const RIGHT_WALL: f32 = 1248.0;
const TILE: f32 = 64.0;
const MAX_TILES: f32 = 4.0;

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

    // estimate rope tension by measuring distance between players
    let obs_vec: Vec<_> = obs.as_vector();
    // info!("Observer updated: {:?}", &obs_vec);

    // let is_wall = if obs_vec[0] == 0 || obs_vec[0] == 31 {
    //     2.0
    // } else {
    //     if obs_vec[0] == 1 || obs_vec[0] == 30 {
    //         1.0
    //     } else {
    //         0.0
    //     }
    // };
    let height = obs.positions.y * (obs_vec[1] as f32) / 2.0;

    let x = obs.positions.x;
    let dist_left = (x - LEFT_WALL).abs();
    let dist_right = (x - RIGHT_WALL).abs();
    let d = dist_left.min(dist_right);
    let t = (d / TILE).min(MAX_TILES);
    let mut is_wall = 1.0 / (1.0 + t * t);
    if (obs_vec[2] == -1 && obs_vec[0] == 0) || (obs_vec[2] == 1 && obs_vec[0] == 31) {
        is_wall += 2.0;
    }
    // info!("is wall {}", is_wall);

    obs_w.write(Observation {
        observation: obs_vec,
        coin,
        height,
        is_wall,
    });
}
