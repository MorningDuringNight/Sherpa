// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::math::VectorSpace;
use bevy::prelude::*;
use rand::Rng;
use crate::player::bundle::{PlayerBundle, PlayerControls};
use crate::config::PlayerSpawnPoint;
use crate::config::PlayerSpawnVelocity;
use crate::config::PLAYER_SPAWN_MASS;

use crate::components::motion::{GroundState, JumpController, Mass, Velocity};
use crate::components::rope::{Rope, RopeConstraint};

use crate::map::Collider;
use crate::app::FollowedPlayer;
use crate::stateMachine::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, spawn_point: Res<PlayerSpawnPoint>, spawn_velocity: Res<PlayerSpawnVelocity>) {
    let transform = Transform::from_translation(spawn_point.position);
    let texture = asset_server.load("spriteguy.png");
    let controls = PlayerControls {
        up: KeyCode::KeyW,
        down: KeyCode::KeyS,
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
    };
    let jump_controller = JumpController::default();
    let ground_state = GroundState::default();
    let mass = Mass(PLAYER_SPAWN_MASS); // make the first player heavier (deleted for now but multiply mass)
    let velocity = Velocity(spawn_velocity.velocity);
    let p1 = commands.spawn(PlayerBundle::new(controls, texture, transform, velocity, mass, jump_controller, ground_state, position)).insert(FollowedPlayer).insert(StateMachine::new(BotState::idel)).id(); // insert component
    // Spawn a second player for testing
    // This is temporary and will be removed later
    // Ideally we would have a better way
    // use load player assets
    let transform = Transform::from_translation(spawn_point.position + Vec3::new(300.0, 0.0, 0.0));
    let texture = asset_server.load("portrait_rainey.png");
    let controls = PlayerControls {
        up: KeyCode::ArrowUp,
        down: KeyCode::ArrowDown,
        left: KeyCode::ArrowLeft,
        right: KeyCode::ArrowRight,
    };
    let jump_controller = JumpController::default();
    let ground_state = GroundState::default();
    let mass = Mass(PLAYER_SPAWN_MASS);
    let position= Position((spawn_point.position + Vec3::new(300.0, 0.0, 0.0)).truncate());
    let p2 = commands.spawn(PlayerBundle::new(controls, texture, transform, velocity, mass, jump_controller, ground_state, position)).insert(Bot::new()).id();

    // Add p1 and p2 a rope component
    commands.spawn(Rope {
        constraint: RopeConstraint::default(),
        attached_entity_head: p1,
        attached_entity_tail: p2,
    });
    
    // Ground platform setup
}
