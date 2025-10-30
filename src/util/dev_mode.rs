use bevy::{prelude::*};
use bevy::math::bounding::{ Aabb2d, BoundingVolume };
use crate::map::Collider;
use crate::app::MainCamera;

// used for deciding at runtime if debug mode is toggled on or off.
#[derive(Resource)]
pub struct Debug(pub bool);

// toggle on and off if debug mode is true.
pub fn toggle_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<Debug>,
) {
    if keyboard_input.just_pressed(KeyCode::Backslash) {
        debug.0 = !debug.0;
    }
}

pub fn debug_on(debug: Res<Debug>) -> bool {
    debug.0
}

pub fn log_mouse_position(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.get_single() else { return };

    if let Some(cursor) = window.cursor_position() {
        // Cursor is measured from bottom-left corner of the window by default
        let x = cursor.x;
        let y = cursor.y;

        if buttons.just_pressed(MouseButton::Left) {
            info!("🖱 Mouse clicked at SCREEN position: x: {:.2}, y: {:.2}", x, y);
        } else {
            info!("Mouse at SCREEN position: x: {:.2}, y: {:.2}", x, y);
        }
    }
}

pub fn move_camera_with_arrows(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let speed = 500.0;
        let dt = time.delta_secs();

        if keys.pressed(KeyCode::KeyI) && transform.translation.y < ((64.0*32.0)-(720.0/2.0)) {
            transform.translation.y += speed * dt;
        }
        else if keys.pressed(KeyCode::KeyK) && transform.translation.y >= (720.0/2.0) {
            transform.translation.y -= speed * dt;
        }
    } else {
        // (optional) log once if camera isn't found
        // info!("No camera found with CameraController");
    }
}


pub fn draw_colliders(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
    query_player: Query<(&Transform, &crate::player::PlayerCollider)>,
) {
    // Draw regular colliders (red)
    for (transform, collider) in &query {
        let world_aabb = collider.aabb.translated_by(transform.translation.truncate());
        gizmos.rect_2d(
            world_aabb.center(),
            world_aabb.half_size() * 2.0,
            Color::srgba(1.0, 0.0, 0.0, 0.8),
        );
    }

    // Draw player colliders (blue or green)
    for (transform, player_collider) in &query_player {
        let world_aabb = player_collider.aabb.translated_by(transform.translation.truncate());
        gizmos.rect_2d(
            world_aabb.center(),
            world_aabb.half_size() * 2.0,
            Color::srgba(0.0, 0.0, 1.0, 0.8), // bright blue
        );
    }
}

