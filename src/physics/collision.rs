use crate::config::physics::*;
use crate::player::PlayerCollider;
use bevy::math::bounding::{Aabb2d, BoundingVolume};

use bevy::math::bounding::IntersectsVolume;
use crate::player::bundle::Player;
use bevy::prelude::*;
use crate::config::physics::GRAVITY;
use crate::game_ui::ui::TotalCoin;

#[derive(Event, Debug)]
pub struct PlayerCollisionEvent {
    pub player: Entity,
    pub game_object: Entity,
}

pub fn on_collision(
    mut commands: Commands,
    mut events: EventReader<PlayerCollisionEvent>,
    coins: Query<(), With<crate::map::Coin>>,
    mut coin_count: ResMut<TotalCoin>,
) {
    for ev in events.read() {
        if coins.get(ev.game_object).is_ok() {
            println!("🤑🤑🤑");
            commands.entity(ev.game_object).despawn();
            //let mut coinCount = coinCount.unwrap();
            coin_count.amount += 1;
        }
    }
}

use crate::components::motion::{GroundState, Momentum, Velocity};
use crate::map::Collider;

const PLATFORM_FRICTION: f32 = 0.88;

// Predict the player's AABB for the next frame
fn predicted_aabb(
    transform: &Transform,
    velocity: &Velocity,
    player_collider: &PlayerCollider,
    dt: f32,
) -> Aabb2d {
    let current_pos = transform.translation.truncate();
    let future_pos = current_pos + velocity.0 * dt;
    let delta = future_pos - current_pos;

    player_collider.aabb.translated_by(current_pos + delta)
}

fn resolve_collision(
    player_pos: &mut Vec3,
    velocity: &mut Vec2,
    momentum: &mut Vec2,
    ground: &mut GroundState,
    offset: Vec2,
) {
    if offset.x.abs() > offset.y.abs() {
        // Horizontal collision
        player_pos.x -= offset.x;
                                  // 
        if offset.x > 0.0 {
            // Collided from the left side of a wall → push player to the left
            player_pos.x += 32.0;
        } else {
            // Collided from the right side of a wall → push player to the right
            player_pos.x -= 32.0;
        }

        velocity.x = 0.0;
        momentum.x = 0.0;
    } else {
        // Vertical collision
        player_pos.y -= offset.y;
        velocity.y = 0.0;
        momentum.y = 0.0;

        // colliding with top  
        if offset.y > 0.0 {
            player_pos.y += 32.0;
            ground.is_grounded = true;
            // velocity.x *= PLATFORM_FRICTION;
            momentum.x *= PLATFORM_FRICTION;
        }
        // colliding with bottom
        else {
            player_pos.y -= 32.0;
        }
    }
}



/// Main player–platform collision system
pub fn platform_collider_system(
    mut events: EventWriter<PlayerCollisionEvent>,
    time: Res<Time>,
    mut players: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut Momentum,
        &PlayerCollider,
        &mut GroundState,
    ), With<Player>>,
    colliders: Query<(Entity, &Transform, &Collider), Without<Player>>,
    spikes: Query<(&crate::map::SpikeDir,), With<crate::map::Spike>>,
    mut game_over: Option<ResMut<crate::game_ui::ui::GameOver>>,
) {
    let dt = time.delta_secs();

    for (player, mut transform, mut velocity, mut momentum, player_collider, mut ground) in players.iter_mut() {
        let mut player_aabb = predicted_aabb(&transform, &velocity, player_collider, dt);
        ground.is_grounded = false;

        for (game_object, collider_transform, collider) in colliders.iter() {
            let collider_pos = collider_transform.translation.truncate();
            let collider_aabb = collider.aabb.translated_by(collider_pos);

            if player_aabb.intersects(&collider_aabb) {
                let mut player_pos = transform.translation;
                let player_center = player_aabb.center();
                let closest = collider_aabb.closest_point(player_center);
                let offset = player_center - closest;

                // 若为钉子且从下方接触（offset.y <= 0.0 视为玩家在钉子下侧/被钉子刺中）→ 触发游戏结束
                if let Ok((dir,)) = spikes.get(game_object) {
                    // 只判上下方向
                    let deadly = match dir.0 {
                        crate::map::SpikeTip::Down => offset.y <= 0.0,
                        crate::map::SpikeTip::Up => offset.y > 0.0 && velocity.0.y <= 0.0,
                        _ => false,
                    };
                    if deadly {
                        if let Some(mut go) = game_over.as_deref_mut() { go.active = true; }
                    }
                }

                resolve_collision(
                    &mut player_pos,
                    &mut velocity.0,
                    &mut momentum.0,
                    &mut ground,
                    offset,
                );

                events.write(PlayerCollisionEvent {
                    player,
                    game_object,
                });

                // Update the AABB after resolution
                player_aabb = player_collider.aabb.translated_by(player_pos.truncate());
                transform.translation = player_pos;
            }
        }
    }
}


pub fn player_collider_system(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut Transform,
        &mut Momentum,
        &PlayerCollider,
    )>,
) {
    let dt = time.delta_secs();
    let mut players: Vec<_> = query.iter_mut().collect();

    for i in 0..players.len() {
        if i + 1 >= players.len() {
            break;
        }

        let (one, two) = players.split_at_mut(i + 1);

        if let Some(obj1) = one.last_mut() {
            for obj2 in two.iter_mut() {
                let &mut (_id1, ref mut vel1, ref mut trans1, ref mut mom1, collider1) = obj1;
                let &mut (_id2, ref mut vel2, ref mut trans2, ref mut mom2, collider2) = obj2;

                // Predict future positions
                let future_pos1 = trans1.translation.truncate() + vel1.0 * dt;
                let future_pos2 = trans2.translation.truncate() + vel2.0 * dt;

                let aabb1 = collider1.aabb.translated_by(future_pos1);
                let aabb2 = collider2.aabb.translated_by(future_pos2);

                if aabb1.intersects(&aabb2) {
                    // --- Compute overlap rectangle ---
                    let overlap_x = (aabb1.max.x.min(aabb2.max.x)) - (aabb1.min.x.max(aabb2.min.x));
                    let overlap_y = (aabb1.max.y.min(aabb2.max.y)) - (aabb1.min.y.max(aabb2.min.y));

                    if overlap_x < overlap_y {
                        // 🧭 Resolve horizontally
                        if aabb1.center().x < aabb2.center().x {
                            // push obj1 left, obj2 right
                            trans1.translation.x -= overlap_x / 2.0;
                            trans2.translation.x += overlap_x / 2.0;
                        } else {
                            trans1.translation.x += overlap_x / 2.0;
                            trans2.translation.x -= overlap_x / 2.0;
                        }

                        // Basic horizontal momentum resolution
                        let total_momentum = mom1.0.x + mom2.0.x;
                        mom1.0.x = total_momentum * 0.5;
                        mom2.0.x = total_momentum * 0.5;
                        vel1.0.x = 0.0;
                        vel2.0.x = 0.0;
                    } 
                }
            }
        }
    }
}

fn check_aabb(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool{
    //possible future use for collision top and collision bottom
    let collisioned = (pos1.x - pos2.x).abs() <= width.x + width2.x && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    //let collision_top = (pos1.y - pos2.y).abs() <= width.y + width2.y && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    return collisioned;
}

fn check_top(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool{
    return (pos1.x - pos2.x).abs() <= width.x + width2.x && (pos1.y > pos2.y || pos2.y > pos1.y) && (pos1.y - width.y) <= (pos2.y + width2.y);
}

pub fn update_coyote_timer_system(
    time: Res<Time>,
    mut query:Query<&mut GroundState, With<Player>>,
) {
    for mut ground_state in &mut query {
        // If in air tick the coyote timer
        if !ground_state.is_grounded {
            ground_state.coyote_timer.tick(time.delta());
        }
        // Grounded reset timer
        else {
            ground_state.coyote_timer.reset();
        }
    }
}
