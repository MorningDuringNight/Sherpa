// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::prelude::*;

use crate::player::bundle::{PlayerBundle, PlayerControls};
use crate::player::config::PlayerSpawnPoint;
use crate::player::config::PlayerSpawnVelocity;
use crate::player::config::PLAYER_SPAWN_MASS;

use crate::physics::component::{Velocity, Mass};
// use crate::rope::component::{Rope};

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
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
    };
    let mass = Mass(PLAYER_SPAWN_MASS * 1.5); // make the first player heavier
    let velocity = Velocity(spawn_velocity.velocity);
    let _p1 = commands.spawn(PlayerBundle::new(controls, texture, transform, velocity, mass)).id();
    // Spawn a second player for testing
    // This is temporary and will be removed later
    // Ideally we would have a better way
    // use load player assets
    let transform = Transform::from_translation(spawn_point.position + Vec3::new(300.0, 0.0, 0.0));
    let texture = asset_server.load("portrait_rainey.png");
    let controls = PlayerControls {
        up: KeyCode::ArrowUp,
        left: KeyCode::ArrowLeft,
        right: KeyCode::ArrowRight,
    };
    let mass = Mass(PLAYER_SPAWN_MASS);
    let _p2 = commands.spawn(PlayerBundle::new(controls, texture, transform, velocity, mass)).id();

    // Add p1 and p2 a rope component
    // commands.spawn(Rope {
    //     constraint: RopeConstraint::default(),
    //     attached_entity_head: p1,
    //     attached_entity_tail: p2,
    // });
}