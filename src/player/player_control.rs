// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>

use bevy::prelude::*;
use crate::config::physics::{
    PLAYER_MOVE_FORCE,
    PLAYER_JUMP_FORCE,
    PLAYER_CONTROL_SPEED_LIMIT,
};
use crate::player::{Player, ControlMode};
use crate::player::bundle::PlayerControls;
use crate::components::motion::{
    ControlForce,
    GroundState,
    JumpController,
    NetForce,
    Velocity,
};
use crate::stateMachine::Bot;
use crate::app::PlayerKeyMappings;

/// Cleans up key states when switching back to player control.
/// Immediately releases keys that Bot may have pressed and handles RLAction events.
pub fn cleanup_ai_keys_system(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    query: Query<(&Player, &ControlMode, &PlayerControls), Changed<ControlMode>>,
    mut rl_events: EventReader<crate::policy::action::RLAction>,
) {
    for (player, control_mode, player_controls) in query.iter() {
        // If just switched to player control, clean up all keys for this player
        if *control_mode == ControlMode::Player {
            match player {
                Player::Local(0) | Player::Local(1) => {
                    // Release all keys for this player to avoid residual key states from Bot
                    keys.release(player_controls.left);
                    keys.release(player_controls.right);
                    keys.release(player_controls.up);
                    keys.release(player_controls.down);
                    
                    // Clear RLAction event queue for this player to avoid residual events
                    // Note: EventReader can only read, not clear, but we can ignore them through ControllerPlugin's checks
                }
                _ => {}
            }
        }
    }
}

/// discrete per-frame input state for one player entity.
#[derive(Event)]
pub struct PlayerInputEvent {
    pub entity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump_pressed: bool,
    pub jump_just_released: bool,
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
) {
    for event in reader.read() {
        if let Ok((
            velocity,
            mut control_force,
            mut net_force,
            mut jump_controller,
            ground_state,
        )) = query.get_mut(event.entity)
        {
            control_force.0.y = 0.0;

            apply_horizontal_movement(&velocity, &mut control_force, event);

            apply_jump(&time, &mut control_force, &mut jump_controller, &ground_state, event);

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
/// Collects keyboard input every `Update` frame and emits `PlayerInputEvent`s.
/// Processes both Player and AI controlled entities.
/// For Player mode: reads actual keyboard input
/// For AI mode: reads ButtonInput that Bot has manipulated
pub fn player_input_collection_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Player, &PlayerControls, &ControlMode)>,
    mut writer: EventWriter<PlayerInputEvent>,
) {
    for (entity, _, player_controls, _control_mode) in &query {
        // Process both player-controlled and AI-controlled entities
        // For AI-controlled entities, Bot manipulates ButtonInput, and we read these key states
        writer.write(PlayerInputEvent {
            entity,
            left: keyboard_input.pressed(player_controls.left),
            right: keyboard_input.pressed(player_controls.right),
            jump_pressed: keyboard_input.pressed(player_controls.up),
            jump_just_released: keyboard_input.just_released(player_controls.up),
        });
    }
}

/// System to toggle player control mode.
/// Press key 1 to toggle player1, press key 2 to toggle player2.
pub fn toggle_player_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &Player, Option<&ControlMode>, Option<&PlayerControls>)>,
    key_mappings: Res<PlayerKeyMappings>,
) {
    // Detect key 1 - toggle player1
    if keyboard.just_pressed(KeyCode::Digit1) {
        for (entity, player, control_mode, _player_controls) in query.iter_mut() {
            if let Player::Local(0) = player {
                let current_mode = control_mode.copied().unwrap_or(ControlMode::Player);
                
                match current_mode {
                    ControlMode::Player => {
                        // Switch to AI control
                        // Keep PlayerControls (so Bot knows which keys to operate)
                        // Add Bot component
                        commands.entity(entity)
                            .insert(Bot::new())
                            .insert(ControlMode::AI);
                    }
                    ControlMode::AI => {
                        // Switch back to player control
                        // Remove Bot component, restore PlayerControls (if removed)
                        commands.entity(entity)
                            .remove::<Bot>()
                            .insert(key_mappings.p1.clone())
                            .insert(ControlMode::Player);
                        
                        // Clean up keys that Bot may have pressed to avoid residual key states
                        // Note: Cannot directly access ButtonInput here, need to clean in next frame's system
                    }
                }
                break; // Exit after finding player1
            }
        }
    }
    
    // Detect key 2 - toggle player2
    if keyboard.just_pressed(KeyCode::Digit2) {
        for (entity, player, control_mode, _player_controls) in query.iter_mut() {
            if let Player::Local(1) = player {
                let current_mode = control_mode.copied().unwrap_or(ControlMode::Player);
                
                match current_mode {
                    ControlMode::Player => {
                        // Switch to AI control
                        // Keep PlayerControls (so Bot knows which keys to operate)
                        commands.entity(entity)
                            .insert(Bot::new())
                            .insert(ControlMode::AI);
                    }
                    ControlMode::AI => {
                        // Switch back to player control
                        commands.entity(entity)
                            .remove::<Bot>()
                            .insert(key_mappings.p2.clone())
                            .insert(ControlMode::Player);
                        
                        // Clean up keys that Bot may have pressed to avoid residual key states
                        // Note: Cannot directly access ButtonInput here, need to clean in next frame's system
                    }
                }
                break; // Exit after finding player2
            }
        }
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
    let can_jump = ground_state.is_grounded || !ground_state.coyote_timer.finished();

    // Start jump on press
    if event.jump_pressed && !jump_controller.is_jumping && can_jump {
        control_force.0.y = PLAYER_JUMP_FORCE;
        jump_controller.is_jumping = true;
        jump_controller.jump_time_elapsed = 0.0;
    }

    // While holding, apply extra force until max duration
    if jump_controller.is_jumping
        && event.jump_pressed
        && jump_controller.jump_time_elapsed < jump_controller.max_jump_duration
    {
        jump_controller.jump_time_elapsed += time.delta_secs();
        control_force.0.y += PLAYER_JUMP_FORCE * jump_controller.jump_multiplier;
    }

    // Stop jumping if button released OR jump duration expired
    if jump_controller.is_jumping
        && (event.jump_just_released
            || jump_controller.jump_time_elapsed >= jump_controller.max_jump_duration)
    {
        jump_controller.is_jumping = false;
    }
}

