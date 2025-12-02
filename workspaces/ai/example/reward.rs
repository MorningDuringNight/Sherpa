const COIN_R: f32 = 40.0;
const UP_R: f32 = 1.0;
const DOWN_R: f32 = 0.5;
const LEVEL_R: f32 = 6.0;
const WALL_R: f32 = 10.0;
const GOOD_PLACE: f32 = 2.0;
const HEIGHT_SCORE: f32 = 0.002;
const AWAY_WALL: f32 = 2.0;
const TIME_PENALTY: f32 = 0.5;

let coin_diff  = c - c_prev;
let h_diff     = h - h_prev;
let level_diff = level - level_prev;
let wall_diff  = wall - wall_prev;

let r_coin  = COIN_R * coin_diff;
let r_jump  = UP_R * h_diff.max(0.0) + DOWN_R * h_diff.min(0.0);
let r_level = LEVEL_R * level_diff;
let r_wall  = - WALL_R * wall_diff;

let r_height_state = HEIGHT_SCORE * h;
let r_level_state = GOOD_PLACE * level;
let r_wall_state = - AWAY_WALL * wall;

let time_penalty = - TIME_PENALTY;

let reward = r_coin + r_jump + r_level + r_wall +
             r_height_state + r_level_state + r_wall_state +
             time_penalty;
reward