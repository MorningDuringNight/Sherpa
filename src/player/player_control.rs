// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>

use crate::components::motion::{ControlForce, GroundState, JumpController, NetForce, Velocity};
use crate::config::physics::{PLAYER_CONTROL_SPEED_LIMIT, PLAYER_JUMP_FORCE, PLAYER_MOVE_FORCE};
use crate::player::Player;
use bevy::prelude::*;

/// discrete per-frame input state for one player entity.
#[derive(Event)]
pub struct PlayerInputEvent {
    pub entity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump_pressed: bool,
    pub jump_just_released: bool,
}

#[derive(Resource)]
pub struct PredictionOn(pub bool);

pub fn toggle_prediction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut prediction_on: ResMut<PredictionOn>,
) {
    // When 'P' is just pressed, toggle the value
    if keyboard.just_pressed(KeyCode::KeyP) {
        prediction_on.0 = !prediction_on.0;
        info!("Prediction toggled: {}", prediction_on.0);
    }
}

pub fn player_movement_input_system(
    time: Res<Time>,
    mut reader: EventReader<PlayerInputEvent>,
    mut query: Query<(
        &mut Velocity,
        &mut ControlForce,
        &mut NetForce,
        &mut JumpController,
        &mut GroundState,
    )>,
    prediction_on: Res<PredictionOn>
) {
    if !prediction_on.as_ref().0 {
        return;
    }
    for event in reader.read() {
        if let Ok((velocity, mut control_force, mut net_force, mut jump_controller, ground_state)) =
            query.get_mut(event.entity)
        {
            control_force.0.y = 0.0;

            apply_horizontal_movement(&velocity, &mut control_force, event);

            apply_jump(
                &time,
                &mut control_force,
                &mut jump_controller,
                &ground_state,
                event,
            );

            net_force.0 += control_force.0;
        }
    }
}

/// Collects keyboard input every `Update` frame and emits `PlayerInputEvent`s.
/// This ensures we never miss `just_released` frames.
/// this could potentially be replaced with a state based system.
/// where we still write every frame but instead of reading events we pass in the input state.
/// The previous solution also checked all of these 'key' properties every frame.
/// Any improvement I tried to make to this made it worse.
pub fn player_input_collection_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Player, &super::bundle::PlayerControls)>,
    mut writer: EventWriter<PlayerInputEvent>,
) {
    for (entity, _, player_controls) in &query {
        writer.write(PlayerInputEvent {
            entity,
            left: keyboard_input.pressed(player_controls.left),
            right: keyboard_input.pressed(player_controls.right),
            jump_pressed: keyboard_input.pressed(player_controls.up),
            jump_just_released: keyboard_input.just_released(player_controls.up),
        });
    }
}

fn apply_horizontal_movement(
    velocity: &Velocity,
    control_force: &mut ControlForce,
    event: &PlayerInputEvent,
) {
    control_force.0.x = 0.0;

    let resistance = PLAYER_MOVE_FORCE / PLAYER_CONTROL_SPEED_LIMIT;
    let resistance_force = resistance * velocity.0.x.abs();

    if event.left {
        if velocity.0.x > -PLAYER_CONTROL_SPEED_LIMIT {
            control_force.0.x = -PLAYER_MOVE_FORCE;
            if velocity.0.x < 0.0 {
                control_force.0.x += resistance_force;
            }
        }
    }

    if event.right {
        if velocity.0.x < PLAYER_CONTROL_SPEED_LIMIT {
            control_force.0.x = PLAYER_MOVE_FORCE;
            if velocity.0.x > 0.0 {
                control_force.0.x -= resistance_force;
            }
        }
    }
}

fn apply_jump(
    time: &Time,
    control_force: &mut ControlForce,
    jump_controller: &mut JumpController,
    ground_state: &GroundState,
    event: &PlayerInputEvent,
) {

    let can_ground_jump = ground_state.is_grounded || !ground_state.coyote_timer.finished();
    let can_wall_jump = !ground_state.is_grounded
        && jump_controller.can_wall_jump
        && !jump_controller.wall_jump_timer.finished();

    // Vertical force
    if event.jump_pressed && !jump_controller.is_jumping {
        // Check grounded jump first
        if can_ground_jump {
            control_force.0.y = PLAYER_JUMP_FORCE;
            jump_controller.is_jumping = true;
            jump_controller.jump_time_elapsed = 0.0;
        } else if can_wall_jump {
            control_force.0.y = PLAYER_JUMP_FORCE;

            // Consume wall jump
            jump_controller.can_wall_jump = false;

            // Start variable jump height tracking
            jump_controller.is_jumping = true;
            jump_controller.jump_time_elapsed = 0.0;
        }
    }
    // Check if player is holding the jump key
    if jump_controller.is_jumping
        && event.jump_pressed
        && jump_controller.jump_time_elapsed < jump_controller.max_jump_duration
    {
        jump_controller.jump_time_elapsed += time.delta_secs();

        // Apply smaller force while holding
        control_force.0.y += PLAYER_JUMP_FORCE * jump_controller.jump_multiplier;
    }
    // End the jump either by letting go or time running out
    if jump_controller.is_jumping && event.jump_just_released
        || jump_controller.jump_time_elapsed >= jump_controller.max_jump_duration
    {
        jump_controller.is_jumping = false;
    }
}