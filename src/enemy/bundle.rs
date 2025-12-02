use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct EnemyCollider {
    pub aabb: Aabb2d,
}

#[derive(Component, Debug)]
pub struct EnemySpeed {
    pub speed: f32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub transform: Transform,
    pub collider: EnemyCollider,
    pub speed: EnemySpeed,
    pub sprite: Sprite,
}

impl EnemyBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            enemy: Enemy,
            collider: EnemyCollider {
                aabb: Aabb2d::new(Vec2::ZERO, Vec2::new(32.0, 32.0)),
            },
            speed: EnemySpeed{ speed: 150.0 },
            transform: Transform::from_xyz(x, y, 0.0),
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            }
        }
    }
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}