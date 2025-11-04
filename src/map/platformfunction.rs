use bevy::prelude::*;
use crate::map::game_object_builder::CubicEasing;
use crate::map::game_object_builder::EasedPlatform;
use crate::map::loader::MovingPlatform;

impl CubicEasing {
    fn ease(&self, t: f32) -> f32 {
        // standard cubic-bezier time easing: find y for given t
        // here we solve for x(t) = input, y(t) = output
        cubic_bezier_y(t, self.x1, self.y1, self.x2, self.y2)
    }
}

/// Cubic-bezier evaluation for easing curves
pub fn cubic_bezier_y(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // De Casteljau algorithm for y(t)
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    // cubic bezier along y
    uuu * 0.0 +
    3.0 * uu * t * y1 +
    3.0 * u * tt * y2 +
    ttt * 1.0
}


/// Animate t linearly over time, but apply cubic-bezier easing to movement
pub fn linear_move_with_easing(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut EasedPlatform)>,
) {
    for (mut transform, mut platform) in &mut q {
        let dt = time.delta_secs() / platform.speed;

        // Linear t progression with ping-pong
        if platform.forward {
            platform.t += dt;
            if platform.t >= 1.0 {
                platform.t = 1.0;
                platform.forward = false;
            }
        } else {
            platform.t -= dt;
            if platform.t <= 0.0 {
                platform.t = 0.0;
                platform.forward = true;
            }
        }

        // Apply easing curve to t
        let eased_t = platform.easing.ease(platform.t);

        // Interpolate linearly between start and end using eased_t
        let pos = platform.start.lerp(platform.end, eased_t);
        transform.translation = pos.extend(0.0);
    }
}