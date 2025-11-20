use bevy::prelude::*;
use crate::observer::{state::ObservationState, system::update_observation_system};
use super::system::Observation;

/// The main plugin entry for the observer system.
/// This plugin spawns and updates the observation state,
/// which can be accessed by AI or logging systems.
pub struct ObserverPlugin;

impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Observation>()
            .init_resource::<ObservationState>() 
            .add_systems(Update, update_observation_system);
    }
}
