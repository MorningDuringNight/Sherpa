use bevy::prelude::*;
mod dev_mode;
use dev_mode::*;
pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(true))
            .add_systems(Update, toggle_debug)
            .add_systems(
                Update,
                (move_camera_with_arrows, draw_colliders).run_if(debug_on),
            );
    }
}

use bevy::window::PrimaryWindow;

use crate::app::MainCamera;

pub fn print_cursor_pixel(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>, // Filter for 2D camera
) {
    println!("=== print_cursor_pixel called ===");

    // Check window
    match windows.get_single() {
        Ok(window) => {
            println!("✓ Window found");

            match window.cursor_position() {
                Some(cursor_pos) => {
                    println!("✓ Cursor position: {:?}", cursor_pos);

                    // Check camera
                    match camera_q.get_single() {
                        Ok((camera, camera_transform)) => {
                            println!("✓ Camera found");
                            println!("  Camera transform: {:?}", camera_transform.translation());

                            // Try to convert to world coordinates
                            match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                                Ok(world_pos) => {
                                    println!("✓ World position: {:?}", world_pos);

                                    // Convert to tile coordinates
                                    let tile_x = (world_pos.x / 64.0).floor() as i32;
                                    let tile_y = (world_pos.y / 64.0).floor() as i32;
                                    println!("✓ Tile coords: ({}, {})", tile_x, tile_y);
                                }
                                Err(e) => {
                                    println!("✗ Failed to convert to world coords: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("✗ Camera query failed: {:?}", e);
                        }
                    }
                }
                None => {
                    println!("✗ No cursor position");
                }
            }
        }
        Err(e) => {
            println!("✗ Window query failed: {:?}", e);
        }
    }
}
