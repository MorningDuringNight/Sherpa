// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use std::time::Duration;
use bevy::prelude::*;
use crate::config::*;
use crate::physics::PhysicsPlugin;
use crate::player::{Player, PlayerPlugin};
use crate::stateMachine::Bot;
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

// change usize to all: single player, single machine config data. 
#[derive(Resource)]
pub enum GameMode {
    LocalCoop, // on one computer
    LocalWithNpc(usize), // main player p1 with ai player 2.
    AiWithAi, // main player p1 with ai player 2.
    NetCoop(usize),
    Simulated,
}
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

// camera components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FollowedPlayer;

const CAMERA_DECAY_RATE: f32 = 3.;

pub fn update_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    followed_q: Query<&Transform, (With<FollowedPlayer>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    let Ok(mut cam) = camera_q.single_mut() else { return };
    let Ok(player_tf) = followed_q.single() else { return };

    let y = player_tf.translation.y.max(SCREEN.1 / 2.0);
    let target = Vec3::new(cam.translation.x, y, cam.translation.z);
    cam.translation.smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
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
    mut players: Query<(Entity, &Transform, &mut Bot), With<Bot>>,
    botActive: Res<BotActive>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut botTimer: ResMut<botTimer>,
    time: Res<Time>,

){  
    if botActive.0 == false{
        return;
    }
    else{
        for (entity, transform, mut Bot,) in players.iter_mut(){
            //put repeating timer
            //if timer has not started: start timer and run function
            //if not start return
            //if started just finished then runfunction
            //
            botTimer.as_deref_mut().tick(time.delta());
            if botTimer.time.finished(){
                Bot.change(&mut keys);
            }
            else {
                return;
            }

            //players.current_state = newState;
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

pub fn run(player_number: Option<usize>) {
    let mut app = App::new();

    #[cfg(all(feature = "client", debug_assertions))]
    app.add_plugins(DevModePlugin);

    #[cfg(feature = "client")]
    {
        app.add_plugins(DefaultPlugins);
        app.add_systems(Update, (bot_update, bot_update_toggle, trigger_bot_input));

        if let Some(player_number) = player_number {
            app.insert_resource(GameMode::NetCoop(player_number));
        }
        else {
            app.insert_resource(GameMode::LocalCoop);
        }

        app.add_plugins(UdpClientPlugin {
            server_addr: "127.0.0.1:5000".to_string(), // localhost
            // server_addr: "home.tailaaef65.ts.net:5000".to_string(), // hostname magic dns.
            // server_addr: "100.110.71.63:5000".to_string(), // tailscaled.
            // server_addr: "3.22.185.76:5000".to_string(),
        });
    }

    #[cfg(feature = "server")]
    {
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GameMode::Simulated);
        app.add_plugins(UdpServerPlugin);
    }

    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })
        .insert_resource(botTimer{time:Timer::new(Duration::from_secs(1),TimerMode::Repeating)})
        .insert_resource(BotActive(false))
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UIPlugin)
        .add_event::<ToggleBotEvent>()
        .add_systems(Startup, init_player_camera)
        .add_systems(Update, update_camera)
        .insert_resource(RopeGeometry::default())
        .add_systems(Startup, init_ropes.after(spawn_players))
        .add_systems(Update, rope_tension_system)
        .add_systems(Update, rope_force_to_system)
        .add_systems(Update, compute_rope_geometry)
        .add_systems(Update, apply_rope_geometry);

    app.run();
}
