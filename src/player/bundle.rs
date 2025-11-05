use bevy::prelude::*;
use crate::player::config::*;
use bevy::math::bounding::Aabb2d;
use crate::collision::PlayerCollider;

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: PlayerCollider,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>, transform: Transform) -> Self {
        let half_size = PLAYER_SIZE / 2.0;
        let aabb = Aabb2d::new(Vec2::ZERO, half_size);
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            transform,
            collider: PlayerCollider {aabb},
        }
    }
}