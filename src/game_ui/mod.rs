use bevy::prelude::*;
use crate::config::MyAppState;
pub mod ui;
pub mod leaderboard;

use ui::*;
use leaderboard::*;

pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource( TotalCoin {amount:0,})
            .insert_resource(MaxHeight{amount:0,})
            .add_systems(Startup, load_ui_camera)
            .add_systems(OnEnter(MyAppState::InGame), load_ui_game)
            .add_systems(OnEnter(MyAppState::MainMenu), load_main_menu)
            .add_systems(Update, main_menu_input
                .run_if(in_state(MyAppState::MainMenu)))
            .add_systems(Update, main_menu_input
                .run_if(in_state(MyAppState::EndCredit)))
            .add_systems(Update, update_height
                .run_if(in_state(MyAppState::InGame)))
            .add_systems(Update, update_ui
                .run_if(in_state(MyAppState::InGame)))

            .add_systems(OnExit(MyAppState::MainMenu), despawn_ui)
            .add_systems(OnExit(MyAppState::InGame), despawn_ui)
            .add_systems(OnExit(MyAppState::EndCredit), despawn_ui)

            .add_systems(Update, game_death
                .run_if(in_state(MyAppState::InGame)))

            .add_systems(OnEnter(MyAppState::EndCredit), update_leaderboard)
            .add_systems(OnEnter(MyAppState::EndCredit), load_ui_leaderboard)
            .add_systems(OnEnter(MyAppState::EndCredit), update_end.after(load_ui_leaderboard));
    }
}

