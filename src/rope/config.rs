// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope configuration>

const SCALE: f32 = 64.0; // 64 pixels = 1 meter

// Rope parameters
pub(super) const ROPE_REST_LENGTH: f32 = 5.0 * SCALE;  // in pixel
pub(super) const ROPE_MAX_EXTENSION: f32 = 50.0;  // Maximum extension beyond rest length
pub(super) const SPRING_CONSTANT: f32 = 80000.0;  // in Newton/pixel = kg/s^2