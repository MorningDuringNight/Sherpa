// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Event>
use bevy::prelude::*;

// Event plugins
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Lifetime2PlayerSpawn>()
           .add_event::<Player2LifetimeSpawned>()
           .add_event::<Lifetime2RopeSpawn>()
           .add_event::<Control2PhysicsIntent>()
           .add_event::<Player2ControlAttach>()
           .add_event::<Player2PhysicsAttach>()
           .add_event::<Rope2PhysicsAttach>();
    }
}

// Lifetime calls Player to Spawn
#[derive(Event, Debug)]
pub struct Lifetime2PlayerSpawn {
    pub node: u32,
    pub texture: Handle<Image>,
    pub position: Vec2,
    pub controls: ControlType,
    pub mass: Option<f32>,
}

// Player sends Lifetime Spawn feedback
#[derive(Event, Debug)]
pub struct Player2LifetimeSpawned {
    pub entity: Entity,
    pub node: u32,
}

// Lifetime calls Rope to Spawn
#[derive(Event, Debug)]
pub struct Lifetime2RopeSpawn {
    pub head_entity: Entity,
    pub tail_entity: Entity,
}

// Control sends player Intent to Physics
#[derive(Event, Debug)]
pub struct Control2PhysicsIntent {
    pub player: Entity,
    pub intent: IntentType,
}
#[derive(Debug, Clone)]
pub enum IntentType {
    Move{ axis_x: f32 },
    JumpStart,
    JumpHold{ dt: f32 },
    JumpEnd,
}

// Player requests Control to Attach a control component
#[derive(Event, Debug)]
pub struct Player2ControlAttach {
    pub entity: Entity,
    pub spec: ControlType
}
#[derive(Debug, Clone)]
pub enum ControlType {
    Keyboard {
        up: KeyCode,
        left: KeyCode,
        right: KeyCode,
    },
    Aibot,
}

// Player requests Physics to Attach a player physics component
#[derive(Event, Debug)]
pub struct Player2PhysicsAttach {
    pub entity: Entity,
    pub mass: f32,
}

// Rope requests Physics to Attach a rope physics component
#[derive(Event, Clone, Copy, Debug)]
pub struct Rope2PhysicsAttach {
    pub rope_entity: Entity,
    pub head_entity: Entity,
    pub tail_entity: Entity,
    pub rest_length: f32,
    pub max_extension: f32,
    pub spring_constant: f32,
}

// Collision sends Physics Information
// TODO-Collision: Write an event to send collision graphic information.
//                 Entity, Overlap Info, ect.

// TODO-Map Loading: Write platform and coin spawn events.