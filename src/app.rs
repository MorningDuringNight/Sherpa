// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use bevy::transform;
use crate::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::config::*;
use crate::stateMachine::*;
use bevy::asset::AssetPlugin;
use bevy::sprite::SpritePlugin;
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
// going to implement the replacement for the controls
#[derive(Event)]
struct ToggleBotEvent;

#[derive(Resource)]
struct BotActive(bool);

fn bot_update_toggle(
    mut bot_active: ResMut<BotActive>,
    keyboard: Res<ButtonInput<KeyCode>>,
){
    //toggle logic
    if keyboard.just_pressed(KeyCode::Space) {
        bot_active.0 = !bot_active.0;
    }
}

fn bot_update(
    mut players: Query<(Entity, &Transform,&mut Bot), With<Bot>>,
    botActive: Res<BotActive>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
){
    if botActive.0 == false{
        return;
    }
    else{
        for (entity, transform, mut Bot) in players.iter_mut(){
            let (newState, _) = Bot.change(&mut keys);
        }
        
    }
}
fn trigger_bot_input(
    mut toggle_events: EventWriter<ToggleBotEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        toggle_events.write(ToggleBotEvent);
    }
}

#[derive(Resource)]
pub struct IsMainPlayer(pub bool);

pub fn run(is_main_player: bool) {
    let mut app = App::new();

    #[cfg(all(feature = "client", debug_assertions))]
    app.add_plugins(DevModePlugin);

    #[cfg(feature = "client")]
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "server")]
    app.add_plugins(MinimalPlugins);

    #[cfg(feature = "client")]
    app.add_plugins(UdpClientPlugin {
        server_addr: "3.21.92.34:5000".to_string(),
    });
    #[cfg(feature = "server")]
    app.add_plugins(UdpServerPlugin);

    app.insert_resource(IsMainPlayer(is_main_player))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })
        .insert_resource(BotActive(false))
        .insert_resource(RopeGeometry::default())
    #[cfg(debug_assertions)] // not added in release mode.
    app.add_plugins(DevModePlugin);

   
    
    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })

        .add_systems(OnEnter(MyAppState::InGame), init_player_camera)


        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UIPlugin)

        .add_systems(Update, update_camera
            .run_if(in_state(MyAppState::InGame)))
        .insert_resource(RopeGeometry::default())

        // .add_systems(Startup, init_ropes)
        .add_systems(OnEnter(MyAppState::InGame), init_ropes.after(spawn_player))
        .add_systems(Update, rope_tension_system
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, rope_force_to_system
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, compute_rope_geometry
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, (bot_update, bot_update_toggle, trigger_bot_input)
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, apply_rope_geometry
            .run_if(in_state(MyAppState::InGame)))
        .insert_state(MyAppState::MainMenu)
        .run();
}

    app.run();
}
