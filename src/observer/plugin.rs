use super::system::Observation;
use crate::{
    app::BotActive,
    observer::{state::ObservationState, system::update_observation_system},
};
use bevy::prelude::*;

/// The main plugin entry for the observer system.
/// This plugin spawns and updates the observation state,
/// which can be accessed by AI or logging systems.
pub struct ObserverPlugin;

impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Observation>()
            .add_systems(Update, update_observation_system);
    }
}
