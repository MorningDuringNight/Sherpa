use crate::app::FollowedPlayer;
use crate::components::motion::{
    ControlForce, Gravity, GroundState, JumpController, Mass, Momentum, NetForce,
    RopeForce, Velocity,
};
use crate::config::player::*;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct PlayerControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

#[derive(Component, Debug)]
pub struct PlayerCollider {
    pub aabb: Aabb2d,
}

#[derive(Component)]
pub struct Mode {
    pub mode: InputType,
}

#[derive(Clone, PartialEq)]
pub enum InputType {
    Player,   
    AI,
}

impl Default for Mode {
    fn default() -> Self {
        Self { mode: InputType::Player }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub gravity: Gravity,
    pub control_force: ControlForce,
    pub rope_force: RopeForce,
    pub net_force: NetForce,
    pub mass: Mass,
    pub momentum: Momentum,
    pub velocity: Velocity,
    pub transform: Transform,
    pub size: PlayerCollider,
    pub jump_controller: JumpController,
    pub ground_state: GroundState,
}

impl PlayerBundle {
    pub fn new(
        transform: Transform,
        velocity: Velocity,
        mass: Mass,
        jump_controller: JumpController,
        ground_state: GroundState,
    ) -> Self {
        Self {
            gravity: Gravity(true),
            control_force: ControlForce(Vec2::ZERO),
            rope_force: RopeForce(Vec2::ZERO),
            net_force: NetForce(Vec2::ZERO),
            mass,
            momentum: Momentum(Vec2::ZERO),
            velocity,
            transform,
            size: PlayerCollider {
                aabb: Aabb2d::new(Vec2::ZERO, PLAYER_SIZE * 0.5),
            },
            jump_controller,
            ground_state,
        }

    }

}
