// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope force system>
use bevy::prelude::*;
use super::component::{SpringJoint, EndPoints, EndPoint};
use crate::event::{ForceEvent};

pub(super) fn rope_tension_system(
    mut events: EventWriter<ForceEvent>,
    q_transforms: Query<&GlobalTransform>,
    q_rope: Query<(&SpringJoint, &EndPoints)> // used to have entity to mark the rope in ForceKind
) {
    for (spring_joint, end_points) in &q_rope {
        let head_pos = match end_points.head {
            EndPoint::Body(e) => {
                if let Ok(t) = q_transforms.get(e) {
                    t.translation().truncate()
                } else {
                    continue; // skip if entity not found
                }
            },
            EndPoint::Fixed(p) => p,
        };

        let tail_pos = match end_points.tail {
            EndPoint::Body(e) => {
                if let Ok(t) = q_transforms.get(e) {
                    t.translation().truncate()
                } else {
                    continue; // skip if entity not found
                }
            },
            EndPoint::Fixed(p) => p,
        };

        let direction = tail_pos - head_pos; // Vec2
        let current_length = direction.length();

        let force = if current_length > spring_joint.rest_length {
            // F = -k * x
            let extension = current_length - spring_joint.rest_length;
            let spring_constant = spring_joint.spring_constant;
            let force_magnitude = spring_constant * extension;
            let force_direction = direction.normalize();
            force_direction * force_magnitude
        } else {
            Vec2::ZERO
        };

        // Write events to apply Rope Force
        if force != Vec2::ZERO {
            match end_points.head {
                EndPoint::Body(e) => {
                    events.write(ForceEvent {
                        target: e,
                        force,
                    });
                }
                EndPoint::Fixed(_) => {}
            }
            match end_points.tail {
                EndPoint::Body(e) => {
                    events.write(ForceEvent {
                        target: e,
                        force: -force,
                    });
                }
                EndPoint::Fixed(_) => {}
            }
        }
    }
}