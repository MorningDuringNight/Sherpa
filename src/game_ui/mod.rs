use bevy::prelude::*;
use crate::config::MyAppState;
pub mod ui;

use ui::*;

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
            .add_systems(Update, update_height
                .run_if(in_state(MyAppState::InGame)))
            .add_systems(Update, update_ui
                .run_if(in_state(MyAppState::InGame)))
            .add_systems(OnExit(MyAppState::MainMenu), despawn_ui);
    }
}
