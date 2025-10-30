use bevy::prelude::*;
mod dev_mode;
use dev_mode::*;
pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Debug(false))
            .add_systems(Update, toggle_debug)
            .add_systems(
                Update,
                (
                    move_camera_with_arrows,
                    draw_colliders,
                ).run_if(debug_on)
            );
    }
}
