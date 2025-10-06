// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

pub mod integrate;
pub mod control;
pub mod gravity;
pub mod rope_force;
pub mod collision;

use crate::physics::collision::update_coyote_timer_system;

use self::integrate::clean_force_system;
use self::integrate::integrate_force_system;
use self::integrate::integrate_momentum_system;
use self::integrate::integrate_velocity_system;
use self::integrate::boundary;
use self::control::player_movement_input_system;
use self::control::keyboard_player_input_system;
use self::gravity::gravity_system;
use self::rope_force::clean_rope_force_system;
use self::rope_force::rope_tension_system;
use self::rope_force::rope_force_to_system;
use self::collision::platform_collider_system;
use self::collision::player_collider_system;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                clean_force_system,
                gravity_system,
                keyboard_player_input_system,
                player_movement_input_system,
                clean_rope_force_system,
                rope_tension_system,
                rope_force_to_system,
                integrate_force_system,
                integrate_momentum_system,
                integrate_velocity_system,
                update_coyote_timer_system,
                player_collider_system,
                platform_collider_system,
                boundary,
            ).chain()
        );
    }
}
