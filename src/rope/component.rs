// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope component>
use bevy::prelude::*;
use super::config::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};

#[derive(Component, Debug)]
pub(super) struct Rope;

#[derive(Component, Clone, Copy, Debug)]
pub(super) struct SpringJoint {
    pub(super) rest_length: f32,
    pub(super) max_extension: f32,
    pub(super) spring_constant: f32,
}

impl Default for SpringJoint {
    fn default() -> Self {
        SpringJoint {
            rest_length: ROPE_REST_LENGTH,
            max_extension: ROPE_MAX_EXTENSION,
            spring_constant: SPRING_CONSTANT,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum EndPoint {
    Body(Entity),
    Fixed(Vec2),
}

#[derive(Component, Debug)]
pub(super) struct EndPoints {
    pub(super) head: EndPoint,
    pub(super) tail: EndPoint,
}