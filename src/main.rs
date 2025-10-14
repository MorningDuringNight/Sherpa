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
    let mut is_main_player = false;

    for arg in env::args().skip(1) { // skip binary name
        match arg.as_str() {
            "--main" => {
                is_main_player = true;
            }
            "--player" => {
                is_main_player = false;
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
            }
        }
    }
    app::run(is_main_player);
}
