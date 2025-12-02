use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    #[default]
    InGame,
    MainMenu,
    EndCredit,
}