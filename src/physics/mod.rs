// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

pub mod integrate;
pub mod gravity;
pub mod rope_force;
pub mod collision;


use self::integrate::clean_force_system;
use self::integrate::integrate_force_system;
use self::integrate::integrate_momentum_system;
use self::integrate::integrate_velocity_system;
use self::integrate::boundary;
use self::gravity::gravity_system;
use self::rope_force::clean_rope_force_system;
use self::rope_force::rope_tension_system;
use self::rope_force::rope_force_to_system;

use self::collision::platform_collider_system;
use self::collision::player_collider_system;
use self::collision::update_coyote_timer_system;
use self::collision::update_wall_jump_timer_system;
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
                    clean_rope_force_system,
                    rope_tension_system,
                    rope_force_to_system,
                    integrate_force_system,
                    integrate_momentum_system,
                    integrate_velocity_system,
                    player_collider_system,
                    platform_collider_system,
                    update_coyote_timer_system,
                    update_wall_jump_timer_system,
                    on_collision,
                    boundary,
                ).chain()
            );
    }
}
