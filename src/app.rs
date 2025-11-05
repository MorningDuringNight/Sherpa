// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::event::EventPlugin;
use crate::camera::CameraPlugin;
use crate::lifetime::LifetimePlugin;
use crate::physics::PhysicsPlugin;
use crate::collision::CollisionPlugin;

pub fn run() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "katsuo_ropebound=debug,bevy=warn,wgpu=warn,naga=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(EventPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LifetimePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}