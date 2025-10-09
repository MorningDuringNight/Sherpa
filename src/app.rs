// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use crate::config::*;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::sprite::SpritePlugin;
use bevy::time::Fixed;
use std::env;

use crate::map::{MapPlugin, SCREEN};
use crate::multiplayer::UdpClientPlugin;
use crate::multiplayer::UdpServerPlugin;
use crate::util::DevModePlugin;

use crate::game_ui::UIPlugin;

use crate::physics::rope_force::{
    RopeGeometry, apply_rope_geometry, compute_rope_geometry, init_ropes, rope_force_to_system,
    rope_tension_system,
};
use crate::player::load_players::spawn_players;

// <- compute_rope_geometry 删除了

// move a half screen right and a half screen up.
// so that the origin is in the positive coordinate system
fn init_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0, // draw first (background/world)
            ..default()
        },
        Transform {
            translation: Vec3::new(SCREEN.0 / 2.0, SCREEN.1 / 2.0, 0.0),
            ..Default::default()
        },
        MainCamera,
    ));
}

// Camera Components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FollowedPlayer;

#[derive(Component)]
pub struct MainPlayer;

const CAMERA_DECAY_RATE: f32 = 3.;

// System for the camera movement
fn update_camera(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<MainPlayer>)>,
    player: Single<&Transform, (With<MainPlayer>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { y, .. } = player.translation;

    let min_y = SCREEN.1 / 2.0;
    let clamped_y = y.max(min_y);
    let target = Vec3::new(camera.translation.x, clamped_y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}

#[derive(Resource)]
pub struct IsMainPlayer(pub bool);

pub fn run(is_main_player: bool) {
    let mut app = App::new();
    #[cfg(all(feature = "client", debug_assertions))]
    app.add_plugins(DevModePlugin);


    #[cfg(feature = "client")]
    app.add_plugins(DefaultPlugins);
    app.insert_resource(IsMainPlayer(is_main_player));

    #[cfg(feature = "server")]
    app.add_plugins(MinimalPlugins);

    app.insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint {
            position: PLAYER_INITIAL_POSITION,
        })
        .insert_resource(PlayerSpawnVelocity {
            velocity: PLAYER_INITIAL_VELOCITY,
        })
        .add_systems(Startup, init_player_camera)
        .add_systems(FixedUpdate, update_camera)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UIPlugin)
        .insert_resource(RopeGeometry::default())
        // .add_systems(Startup, init_ropes)
        .add_systems(Startup, init_ropes.after(spawn_players))
        .add_systems(Update, rope_tension_system)
        .add_systems(Update, rope_force_to_system)
        .add_systems(Update, compute_rope_geometry)
        .add_systems(Update, apply_rope_geometry);

    #[cfg(feature = "client")]
        app.add_plugins(UdpClientPlugin {
            // server_addr: "127.0.0.1:5000".to_string(), // localhost
            server_addr: "3.21.92.34:5000".to_string(), // aws relay.
            // server_addr: "100.110.71.63:5000".to_string(), // tailscaled.
        });
    #[cfg(feature = "server")]
        app.add_plugins(UdpServerPlugin);
    app.run();
}
