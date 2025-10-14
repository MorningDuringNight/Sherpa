// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;
use crate::config::MyAppState;

pub mod integrate;
pub mod control;
pub mod gravity;
pub mod rope_force;
pub mod collision;

use self::integrate::clean_force_system;
use self::integrate::integrate_force_system;
use self::integrate::integrate_momentum_system;
use self::integrate::integrate_velocity_system;
use self::integrate::update_position;
use self::integrate::boundary;
use self::control::player_movement_input_system;
use self::gravity::gravity_system;
use self::rope_force::clean_rope_force_system;
use self::rope_force::rope_tension_system;
use self::rope_force::rope_force_to_system;

use self::collision::platform_collider_system;
use self::collision::player_collider_system;
use self::collision::update_coyote_timer_system;
use self::collision::on_collision;
use self::collision::PlayerCollisionEvent;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerCollisionEvent>()
            .add_systems(
                FixedUpdate, 
                (
                    clean_force_system,
                    gravity_system,
                    player_movement_input_system,
                    clean_rope_force_system,
                    rope_tension_system,
                    rope_force_to_system,
                    integrate_force_system,
                    integrate_momentum_system,
                    integrate_velocity_system,
                    player_collider_system,
                    platform_collider_system,
                    update_coyote_timer_system,
                    on_collision,
                    boundary,
                ).chain()
                .run_if(in_state(MyAppState::InGame))
            )
            .add_systems(Update, update_position
                .run_if(in_state(MyAppState::InGame)));
    }
}
