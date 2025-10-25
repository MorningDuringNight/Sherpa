// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope bundle>
use bevy::prelude::*;

use super::component::{SpringJoint, EndPoints};

#[derive(Bundle, Debug)]
pub struct RopeBundle {
    pub spring_joint: SpringJoint,
    pub rope_ends: EndPoints,
    pub transform: Transform,
}