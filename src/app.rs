// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use std::time::Duration;
use bevy::prelude::*;
use crate::config::*;
use crate::observer::state::{self, ObservationState};
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
use crate::enemy::EnemyPlugin;

use crate::game_ui::UIPlugin;

use crate::physics::rope_force::{
    RopeGeometry, apply_rope_geometry, compute_rope_geometry, init_ropes, rope_force_to_system,
    rope_tension_system,
};
use crate::player::load_players::spawn_players;

use crate::observer::plugin::ObserverPlugin;

fn dummy<T: bevy::asset::Asset>() -> Handle<T> {
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(1);

    // Bevy supports weak_from_u128, so just expand the u64 into u128
    let id = COUNTER.fetch_add(1, Ordering::Relaxed) as u128;

    Handle::<T>::weak_from_u128(id)
}

// change usize to all: single player, single machine config data. 
#[derive(Resource)]
pub enum GameMode {
    LocalCoop, // on one computer
    LocalWithNpc(usize), // main player p1 with ai player 2.
    AiWithAi, // main player p1 with ai player 2.
    NetCoop(usize),
    Simulated,
}
#[derive(Resource, Deref, DerefMut)]
struct botTimer {time:Timer}
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

#[derive(Resource)]
pub struct GameAssets {
    pub fish: Handle<Image>,
    pub background: Handle<Image>,
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
    pub main_menu: Handle<Image>,
}



// camera components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct Background;

#[derive(Component)]
pub struct FollowedPlayer;

const CAMERA_DECAY_RATE: f32 = 3.;

pub fn update_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    followed_q: Query<&Transform, (With<FollowedPlayer>, Without<MainCamera>)>,
    time: Res<Time>,
    mut background: Query<&mut Transform, (With<Background>, Without<MainCamera>, Without<FollowedPlayer>)>,
) {
    let Ok(mut cam) = camera_q.single_mut() else { return };
    let Ok(player_tf) = followed_q.single() else { return };
    let Ok(mut bg) = background.single_mut() else { return };

    let y = player_tf.translation.y.max(SCREEN.1 / 2.0);
    let target = Vec3::new(cam.translation.x, y, cam.translation.z);
    cam.translation.smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
    let bgTarget = Vec3::new(bg.translation.x, y, bg.translation.z);
    bg.translation.smooth_nudge(&bgTarget, CAMERA_DECAY_RATE, time.delta_secs());
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
    mut players: Query<(Entity, &GlobalTransform, &mut Bot), With<Bot>>,
    botActive: Res<BotActive>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut botTimer: ResMut<botTimer>,
    time: Res<Time>,
    obs: ResMut<ObservationState>
){  
    if botActive.0 == false{
        return;
    }
    else{
        for (entity, transform, mut Bot) in players.iter_mut(){
            //put repeating timer
            //if timer has not started: start timer and run function
            //if not start return
            //if started just finished then runfunction
            //
            botTimer.as_deref_mut().tick(time.delta());
            if botTimer.time.finished(){
                Bot.change(
                    &time,
                    transform,
                    &mut keys,
                    &obs,
                );
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
        app.add_systems(Update, (bot_update, bot_update_toggle, trigger_bot_input)
            .run_if(in_state(MyAppState::InGame)));

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

        let asset_server = app.world().get_resource::<AssetServer>().unwrap().clone();
        let game_assets = GameAssets {
            fish: asset_server.load("fish.PNG"),
            background: asset_server.load("sherpa_background.png"),
            tile_fg: asset_server.load("level1/tile_fg.png"),
            entity: asset_server.load("level1/entity.png"),
            main_menu: asset_server.load("mainMenu.png"),
        };
        app.insert_resource(game_assets);
    }

    #[cfg(feature = "server")]
    {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.add_plugins(bevy::input::InputPlugin);

        app.insert_resource(GameMode::Simulated);
        app.add_plugins(UdpServerPlugin);

        let game_assets = GameAssets {
            fish: dummy(),
            background: dummy(),
            tile_fg: dummy(),
            entity: dummy(),
            main_menu: dummy(),
        };
        app.insert_resource(game_assets);
    }


    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })
        .insert_resource(botTimer{time:Timer::new(Duration::from_secs(1),TimerMode::Repeating)})
        .insert_resource(BotActive(false))
        .insert_resource(RopeGeometry::default());
    
    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })

        .add_systems(Startup, init_player_camera)


        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(ObserverPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(EnemyPlugin)

        .add_systems(Update, update_camera
            .run_if(in_state(MyAppState::InGame)))
        .insert_resource(RopeGeometry::default())

        .add_systems(Startup, init_ropes.after(spawn_players))
        .add_systems(Update, rope_tension_system
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, rope_force_to_system
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, compute_rope_geometry
            .run_if(in_state(MyAppState::InGame)))
        .add_event::<ToggleBotEvent>()
        .add_systems(Update, (bot_update, bot_update_toggle, trigger_bot_input)
            .run_if(in_state(MyAppState::InGame)))
        .add_systems(Update, apply_rope_geometry
            .run_if(in_state(MyAppState::InGame)));

    #[cfg(feature = "server")]
        app.insert_state(MyAppState::InGame);
    #[cfg(feature = "client")]
        app.insert_state(MyAppState::MainMenu);
    app.run();
}

