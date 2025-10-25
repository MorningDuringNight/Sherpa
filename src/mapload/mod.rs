// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Map loading mod>
use bevy::prelude::*;

pub struct MapLoadingPlugin;

impl Plugin for MapLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::platform::PlatformPlugin,
            crate::coin::CoinPlugin,
        ));
    }
}