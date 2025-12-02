use crate::components::motion::GroundState;
use crate::components::motion::Velocity;
use crate::game_ui::ui::TotalCoin;
use crate::observer::state::ObservationState;
use crate::player::Player;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct Observation {
    pub observation: Vec<i32>,
    pub coin: usize,
    pub height: f32,
    pub level: f32,
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
    query_players: Query<(&Transform, &Velocity, &GroundState), With<Player>>,
    mut obs_w: EventWriter<Observation>,
) {
    obs.clear();

    let coin = c.amount as usize;
    let mut gs = false;

    // Gather player positions
    if let Some((transform, velocity, ground_state)) = query_players.iter().next() {
        obs.positions = transform.translation.truncate();
        obs.velocities = velocity.0;
        gs = ground_state.is_grounded || !ground_state.coyote_timer.finished();
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
    let height = obs.positions.y; // * (obs_vec[1] as f32) / 2.0;

    let h = obs_vec[1];
    let step_bonus = if h > 16 {
        100.0
    } else if h > 11 {
        10.0
    } else if h > 9 {
        5.0
    } else if h > 8 {
        4.0
    } else if h > 6 {
        3.0
    } else if h > 5 {
        2.5
    } else if h > 3 {
        1.0
    } else if h > 1 {
        0.5
    } else {
        0.0
    };
    let level = step_bonus * if gs { 1.0 } else { 0.0 };
    // info!("Level reward: {}", level);

    let x = obs.positions.x;
    let dist_left = (x - LEFT_WALL).abs();
    let dist_right = (x - RIGHT_WALL).abs();
    let d = dist_left.min(dist_right);
    let t = (d / TILE).min(MAX_TILES);
    let mut is_wall = 1.0 / (1.0 + t);
    if (obs_vec[2] == -1 && obs_vec[0] == 0) || (obs_vec[2] == 1 && obs_vec[0] == 31) {
        is_wall += 2.0;
    }
    // info!("is wall: {}", is_wall);

    obs_w.write(Observation {
        observation: obs_vec,
        coin,
        height,
        level,
        is_wall,
    });
}
