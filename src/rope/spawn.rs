// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope Spawn system>
use bevy::prelude::*;

use crate::event::{Lifetime2RopeSpawn, Rope2PhysicsAttach};

use super::component::{EndPoint, EndPoints, SpringJoint, Rope};
use super::config::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};
use super::bundle::RopeBundle;

pub(super) fn spawn_rope(
    mut commands: Commands,
    mut events: EventReader<Lifetime2RopeSpawn>,
    mut register_events: EventWriter<Rope2PhysicsAttach>,
    tf: Query<&GlobalTransform>,
) {
    for event in events.read() {
        let rest_length = ROPE_REST_LENGTH;
        let max_extension = ROPE_MAX_EXTENSION;
        let spring_constant = SPRING_CONSTANT;
        // let position_head = match tf.iter(event.head_entity) {
        //     Ok(data) => data,
        //     Err(_) => continue,
        // };
        // let position_tail = match tf.iter(event.tail_entity) {
        //     Ok(data) => data,
        //     Err(_) => continue,
        // };
        // let mid = (position_head.translation().truncate() + position_tail.translation().truncate()) / 2.0;
        let rope_entity = commands.spawn((
            RopeBundle {
                spring_joint: SpringJoint {
                    rest_length,
                    max_extension,
                    spring_constant,
                },
                rope_ends: EndPoints {
                    head: EndPoint::Body(event.head_entity),
                    tail: EndPoint::Body(event.tail_entity),
                },
                transform: Transform::default(),
            },
            Rope,
        )).id();

        register_events.write(Rope2PhysicsAttach {
            rope_entity,
            head_entity: event.head_entity,
            tail_entity: event.tail_entity,
            rest_length,
            max_extension,
            spring_constant,
        });
    }
}
