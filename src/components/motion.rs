// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player bundle and components>

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Velocity (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct NetForce (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Momentum (pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct Mass (pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Gravity (pub bool);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(true)
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct ControlForce (pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct RopeForce (pub Vec2);

#[derive(Component)]
pub struct JumpController {
    pub is_jumping: bool,
    pub jump_time_elapsed: f32,
    pub max_jump_duration: f32,
    pub jump_multiplier: f32,
    pub can_wall_jump: bool,
    pub wall_jump_timer: Timer,
}

impl Default for JumpController {
    fn default() -> Self {
        Self {
            is_jumping: false,
            jump_time_elapsed: 0.0,
            max_jump_duration: 0.25,
            jump_multiplier: 0.35,
            can_wall_jump: true,
            wall_jump_timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct GroundState {
    pub is_grounded: bool,
    pub coyote_timer: Timer,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            is_grounded: false,
            coyote_timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}
