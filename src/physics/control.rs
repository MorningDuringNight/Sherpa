// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;
use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{ControlForce, GroundState, JumpController, NetForce, PlayerInput, Velocity};

pub fn player_movement_input_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut ControlForce, &mut NetForce, &Player, &mut JumpController, &mut GroundState, &PlayerInput), With<Player>>,
) {
    for (mut velocity, mut control_force, mut net_force, player, mut jump_controller, mut ground_state, input) in &mut query {
        // Calculate the resistance parameter
        // f = c*v => c = f/v
        let resistance = PLAYER_MOVE_FORCE / PLAYER_CONTROL_SPEED_LIMIT; 

        // Calculate the resistance force (speed-dependent)
        let resistance_force = resistance * velocity.0.x.abs();

        // Reset control force
        control_force.0 = Vec2::ZERO;

        // Horizontal force
        if input.move_left {
            if velocity.0.x > -PLAYER_CONTROL_SPEED_LIMIT { // speed up if not reached limit
                control_force.0.x = - PLAYER_MOVE_FORCE;
                if velocity.0.x < 0.0 {
                    control_force.0.x += resistance_force;
                }
            }
        }
        if input.move_right {
            if velocity.0.x < PLAYER_CONTROL_SPEED_LIMIT { // don't speed up if reached limit
                control_force.0.x = PLAYER_MOVE_FORCE;
                if velocity.0.x > 0.0 {
                    control_force.0.x -= resistance_force;
                }
            }
        }
        let can_jump = ground_state.is_grounded || !ground_state.coyote_timer.finished();

        // Vertical force
        if input.jump_pressed && !jump_controller.is_jumping && can_jump {
            control_force.0.y = PLAYER_JUMP_FORCE;
            jump_controller.is_jumping = true;
            jump_controller.jump_time_elapsed = 0.0;
        }
        // Check if player is holding the jump key
        if jump_controller.is_jumping && input.jump_pressed && jump_controller.jump_time_elapsed < jump_controller.max_jump_duration {
            jump_controller.jump_time_elapsed += time.delta_secs();

            // Apply smaller force while holding
            control_force.0.y += PLAYER_JUMP_FORCE * jump_controller.jump_multiplier;
        }
        // End the jump either by letting go or time running out
        if jump_controller.is_jumping && input.jump_released || jump_controller.jump_time_elapsed >= jump_controller.max_jump_duration {
            jump_controller.is_jumping = false;
        }

        net_force.0 += control_force.0;
    }
}

pub fn keyboard_player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut PlayerInput)>
) {
    for (player, mut input) in &mut query {
        input.move_left = keyboard_input.pressed(player.controls.left);
        input.move_right = keyboard_input.pressed(player.controls.right);
        input.jump_pressed = keyboard_input.pressed(player.controls.up);
        input.jump_released = keyboard_input.just_released(player.controls.up);
    }
}

pub fn ai_player_input_system(){

}
// Server system to process inputs that are received
pub fn receive_player_input_system() {
    // Need to receive the message from the network
    // Find which player the input is for
    // Update the player input component to the message
}
// Client system to send player inputs to the server
pub fn send_player_input_system() {
    // Create message with input data
    // Send to server
}