mod app;
mod components;
mod config;
mod game_ui;
mod map;
mod multiplayer;
mod physics;
mod player;
mod util;
mod stateMachine;

use std::env;
fn main() {
    let mut player_number = None;

    for arg in env::args().skip(1) { // skip binary name
        match arg.as_str() {
            "--p1" => {
                player_number = Some(0);
            }
            "--p2" => {
                player_number = Some(1);
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
            }
        }
    }
    app::run(player_number);
}
