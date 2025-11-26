use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[states(scoped_entities)]
pub enum MyAppState {
    #[default]
    InGame,
    MainMenu,
    EndCredit,
}