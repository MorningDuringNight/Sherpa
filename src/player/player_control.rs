// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>

use crate::components::motion::{ControlForce, GroundState, JumpController, NetForce, Velocity};
use crate::config::physics::{PLAYER_CONTROL_SPEED_LIMIT, PLAYER_JUMP_FORCE, PLAYER_MOVE_FORCE};
use crate::map::Collider;
use crate::map::Platform;
use crate::player::Player;
use bevy::math::VectorSpace;
use bevy::math::bounding::Aabb2d;
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

#[derive(Component)]
pub struct SpawnedPlatform;

#[derive(Component)]
pub struct DespawnTimer {
    timer: Timer,
}

impl Default for DespawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

pub fn platform_spawn_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut JumpController, &GroundState, &super::bundle::PlayerControls), With<Player>>,
) {
    for (player, transform, mut jump_controller, grounded, player_controls) in query.iter_mut() {
        if jump_controller.ability_available && !grounded.is_grounded && keyboard_input.just_pressed(player_controls.down) {
            let offset = Vec2::new(0.0, -50.0);
            let platform_pos = transform.translation.truncate() + offset;
            
            let platform_width = 150.0;
            let platform_height = 30.0;

            let collider = Collider {
                aabb: Aabb2d::new(
                    Vec2::ZERO,
                    Vec2::new(platform_width/2.0, platform_height/2.0)
                ),
            };

            // Spawn platform underneath player
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.3, 0.2),
                    custom_size: Some(Vec2::new(platform_width, platform_height)),
                    ..default()
                },
                Transform::from_translation(platform_pos.extend(0.0)),
                collider,
                Platform,
                SpawnedPlatform,
                DespawnTimer::default(),
            ));
            // Mark as used
            jump_controller.ability_available = false;
        }
    }
}

pub fn despawn_platform_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer), With<SpawnedPlatform>>,
) {
    for (entity, mut timer) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}