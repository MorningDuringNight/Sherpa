// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Jagger Hershey
// Description: <Collision component>
use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerCollider {
    pub aabb: Aabb2d,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Collider {
    pub aabb: Aabb2d,
}