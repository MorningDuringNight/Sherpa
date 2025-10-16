// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::prelude::*;

use crate::app::FollowedPlayer;
use crate::config::PLAYER_SIZE;
use crate::config::PLAYER_SPAWN_MASS;
use crate::config::PlayerSpawnPoint;
use crate::config::PlayerSpawnVelocity;
use crate::player::bundle::{PlayerBundle, PlayerControls};

use crate::components::motion::{GroundState, JumpController, Mass, Velocity};
use crate::components::rope::{Rope, RopeConstraint};

use crate::app::{GameMode};
use crate::stateMachine::Bot;
use crate::stateMachine::BotState;
use crate::stateMachine::StateMachine;




#[derive(Component)]
// Player with playerNumber
pub enum Player {
    Local(usize), 
    Net(usize),
    Npc(usize),
}

pub fn spawn_players(
    mut commands: Commands, 
    #[cfg(feature = "client")] asset_server: Res<AssetServer>,
    spawn_point: Res<PlayerSpawnPoint>,
    spawn_velocity: Res<PlayerSpawnVelocity>,

    gamemode: Res<GameMode>,
) {

    #[cfg(feature = "server")]
    let (p1_img, p2_img) = (None, None);

    #[cfg(feature = "client")]
    let (p1_img, p2_img) = (
        Some(asset_server.load("spriteguy.png")),
        Some(asset_server.load("portrait_rainey.png")),
    );

    let p1 = single_player(
        &mut commands,
        p1_img,
        Transform::from_translation(spawn_point.position),
        spawn_velocity.velocity,
    );

    let p2 = single_player(
        &mut commands,
        p2_img,
        Transform::from_translation(spawn_point.position + Vec3::new(300.0, 0.0, 0.0)),
        spawn_velocity.velocity,
    );

    let player_list = [&p1, &p2];

    #[cfg(feature = "server")]
    let (wasd_controls, arrow_controls) = (None::<PlayerControls>, None::<PlayerControls>);
    
    #[cfg(feature = "client")]
    let wasd_controls = Some(PlayerControls {
        up: KeyCode::KeyW,
        down: KeyCode::KeyS,
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
    });

    #[cfg(feature = "client")]
    let arrow_controls = Some(PlayerControls {
        up: KeyCode::ArrowUp,
        down: KeyCode::ArrowDown,
        left: KeyCode::ArrowLeft,
        right: KeyCode::ArrowRight
    });

    // player 1 is always the player that the camera is tied to.
    let mut camera_follow_player = 0;
    match *gamemode {
        GameMode::LocalCoop => {
            let bot = Bot::new();
            let state_machine = StateMachine::new(BotState::idel);
            // add FollowCamera to one of these.
            commands.entity(p1).insert((wasd_controls.unwrap(), Player::Local(0), state_machine));
            commands.entity(p2).insert((arrow_controls.unwrap(), Player::Local(1), bot));

            
        }
        GameMode::LocalWithNpc(local_player_number) => {
            camera_follow_player = local_player_number;
            // insert NPC for player that isnt player_number
            commands.entity(*player_list[local_player_number]).insert((wasd_controls.unwrap(), Player::Local(local_player_number)));

            player_list.iter().enumerate().filter(|(i, _)| *i != local_player_number)
                .for_each(|(i, entity)| {
                    commands.entity(**entity).insert(Player::Npc(i));
                }
            );
        }
        GameMode::AiWithAi => {
            player_list.iter().enumerate().for_each(|(i, entity)| {
                    commands.entity(**entity).insert(Player::Npc(i));
                }
            );
        }
        GameMode::NetCoop(local_player_number) => {
            camera_follow_player = local_player_number;
            // insert net player marker for all players that arent LocalPlayer
            // insert localPlayer marker component for this player
            commands.entity(*player_list[local_player_number]).insert((wasd_controls.unwrap(), Player::Local(local_player_number)));
            player_list.iter().enumerate().filter(|(i, _)| *i != local_player_number)
                .for_each(|(i, entity)| {
                    commands.entity(**entity).insert(Player::Net(i));
                }
            );
        }
        GameMode::Simulated => {
            player_list.iter().enumerate().for_each(|(i, entity)| {
                commands.entity(**entity).insert(Player::Local(i));
            });
        }
    }
    commands.entity(*player_list[camera_follow_player]).insert(FollowedPlayer);

    commands.spawn(Rope {
        constraint: RopeConstraint::default(),
        attached_entity_head: *player_list[0],
        attached_entity_tail: *player_list[1],
    });
}

// make player base w or w/o sprite.
// add controls and sprite depending on gamemode.
fn single_player(
    commands: &mut Commands,
    texture: Option<Handle<Image>>,
    transform: Transform,
    velocity: Vec2,
) -> Entity {
    let jump_controller = JumpController::default();
    let ground_state = GroundState::default();
    let mass = Mass(PLAYER_SPAWN_MASS);
    let velocity = Velocity(velocity);

    let mut entity_commands = commands.spawn(PlayerBundle::new(
        transform,
        velocity,
        mass,
        jump_controller,
        ground_state,
    ));

    if let Some(texture) = texture {
        let sprite = Sprite {
            image: texture,
            custom_size: Some(PLAYER_SIZE),
            ..Default::default()
        };
        entity_commands.insert(sprite);
    };

    entity_commands.id()
}
