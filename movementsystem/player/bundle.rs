use bevy::prelude::*;
use crate::config::player::*;

#[derive(Component, Clone)]
pub struct Player {
    pub controls: PlayerControls,
}

#[derive(Clone)]
pub struct PlayerControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub player: Player,
    pub velocity: Velocity,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(controls: PlayerControls, texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            player: Player { controls },
            velocity: Velocity::default(),
            transform: transform,
        }
    }
}