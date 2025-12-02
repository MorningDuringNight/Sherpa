use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::{self, PlayerCollider};
use crate::enemy::bundle::{Enemy, EnemyCollider, EnemyMovement};
use bevy::math::bounding::{Aabb2d, AabbCast2d, BoundingVolume, RayCast2d};
use bevy::math::{Dir2, Ray2d};

use crate::config::physics::GRAVITY;
use crate::game_ui::ui::TotalCoin;
use crate::player::Player;
use bevy::math::bounding::IntersectsVolume;
use bevy::{prelude::*, transform};

use crate::config::player::{PLAYER_LENGTH, PLAYER_WIDTH};

#[derive(Event, Debug)]
pub struct PlayerCollisionEvent {
    pub player: Entity,
    pub game_object: Entity,
}

#[derive(Event, Debug)]
pub struct EnemyPlayerCollisionEvent {
    pub player: Entity,
    pub enemy: Entity,
}

#[derive(Event, Debug)]
pub struct EnemyPlatformCollisionEvent {
    pub enemy: Entity,
    pub platform: Entity,
}


pub fn on_collision(
    mut commands: Commands,
    mut events: EventReader<PlayerCollisionEvent>,
    coins: Query<(), With<crate::map::Coin>>,
    mut coinCount: ResMut<TotalCoin>,
) {
    for ev in events.read() {
        if coins.get(ev.game_object).is_ok() {
            println!("🤑🤑🤑");
            commands.entity(ev.game_object).despawn();
            //let mut coinCount = coinCount.unwrap();
            coinCount.amount += 1;
        }
    }
}

use crate::components::motion::{GroundState, JumpController, Momentum, Velocity};
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
    jump_controller: &mut JumpController,
    offset: Vec2,
) {
    if offset.x.abs() > offset.y.abs() {
        // Horizontal collision
        player_pos.x -= offset.x;
        //
        if offset.x > 0.0 {
            // Collided from the left side of a wall → push player to the left
            player_pos.x += PLAYER_WIDTH / 2.;
        } else {
            // Collided from the right side of a wall → push player to the right
            player_pos.x -= PLAYER_WIDTH / 2.;
        }

        // Refresh wall jump timer to use wall jump
        if !ground.is_grounded && jump_controller.can_wall_jump {
            jump_controller.wall_jump_timer.reset();
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
            player_pos.y += PLAYER_LENGTH / 2.;
            ground.is_grounded = true;
            // velocity.x *= PLATFORM_FRICTION;
            momentum.x *= PLATFORM_FRICTION;

            // Restore wall jump when landed
            jump_controller.can_wall_jump = true;
        }
        // colliding with bottom
        else {
            player_pos.y -= PLAYER_LENGTH / 2.;
        }
    }
}

/// Main player–platform collision system
pub fn platform_collider_system(
    mut events: EventWriter<PlayerCollisionEvent>,
    time: Res<Time>,
    mut players: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut Momentum,
            &PlayerCollider,
            &mut GroundState,
            &mut JumpController,
        ),
        With<Player>,
    >,
    colliders: Query<(Entity, &Transform, &Collider), Without<Player>>,
) {
    let dt = time.delta_secs();

    for (
        player,
        mut transform,
        mut velocity,
        mut momentum,
        player_collider,
        mut ground,
        mut jump_controller,
    ) in players.iter_mut()
    {
        let mut player_aabb = predicted_aabb(&transform, &velocity, player_collider, dt);
        ground.is_grounded = false;

        for (game_object, collider_transform, collider) in colliders.iter() {
            let collider_pos = collider_transform.translation.truncate();
            let collider_aabb = collider.aabb.translated_by(collider_pos);

            if player_aabb.intersects(&collider_aabb) {
                let mut player_pos = transform.translation;

                //// CHANGE: compute real platform top and player bottom
                let platform_top = collider_aabb.max.y;
                let player_bottom = player_pos.y - (PLAYER_LENGTH / 2.0);

                //// CHANGE: check if player is ABOVE platform
                let is_above = player_bottom >= platform_top - 1.0;

                //// CHANGE: check if falling
                let is_falling = velocity.0.y <= 0.0;

                //// CHANGE: one-way platform — only collide when falling onto top
                if !(is_above && is_falling) {
                    continue;
                }

                // (Original collision resolution begins)
                let player_center = player_aabb.center();
                let closest = collider_aabb.closest_point(player_center);
                let offset = player_center - closest;

                resolve_collision(
                    &mut player_pos,
                    &mut velocity.0,
                    &mut momentum.0,
                    &mut ground,
                    &mut jump_controller,
                    offset,
                );

                events.write(PlayerCollisionEvent {
                    player,
                    game_object,
                });

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
        &mut JumpController,
        &mut GroundState,
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
                let &mut (
                    id1,
                    ref mut vel1,
                    ref mut trans1,
                    ref mut mom1,
                    collider1,
                    ref mut jump_controller1,
                    ref mut ground_state1,
                ) = obj1;
                let &mut (
                    id2,
                    ref mut vel2,
                    ref mut trans2,
                    ref mut mom2,
                    collider2,
                    ref mut jump_controller2,
                    ref mut ground_state2,
                ) = obj2;

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
                    } else {
                        // Resolve vertically
                        if aabb1.center().y < aabb2.center().y {
                            // Push obj1 down, obj2 up
                            trans1.translation.y -= overlap_y / 2.0;
                            trans2.translation.y += overlap_y / 2.0;

                            // obj2 landed on obj1, reset jumps
                            jump_controller2.can_wall_jump = true;
                            jump_controller2.is_jumping = false;
                            ground_state2.is_grounded = true;
                            ground_state2.coyote_timer.reset();

                            // Apply friction to top of player
                            mom2.0.x *= PLATFORM_FRICTION;
                        } else {
                            // Push obj1 up, obj2 down
                            trans1.translation.y += overlap_y / 2.0;
                            trans2.translation.y -= overlap_y / 2.0;

                            // obj1 landed on obj2, reset jumps
                            jump_controller1.can_wall_jump = true;
                            jump_controller1.is_jumping = false;
                            ground_state1.is_grounded = true;
                            ground_state1.coyote_timer.reset();

                            // Apply friction to top of player
                            mom1.0.x *= PLATFORM_FRICTION;
                        }

                        // Basic vertical momentum resolution
                        let total_momentum = mom1.0.y + mom2.0.y;
                        mom1.0.y = total_momentum * 0.5;
                        mom2.0.y = total_momentum * 0.5;
                        vel1.0.y = 0.0;
                        vel2.0.y = 0.0;
                    }
                }
            }
        }
    }
}

fn check_aabb(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool {
    //possible future use for collision top and collision bottom
    let collisioned = (pos1.x - pos2.x).abs() <= width.x + width2.x
        && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    //let collision_top = (pos1.y - pos2.y).abs() <= width.y + width2.y && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    return collisioned;
}

fn check_top(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool {
    return (pos1.x - pos2.x).abs() <= width.x + width2.x
        && (pos1.y > pos2.y || pos2.y > pos1.y)
        && (pos1.y - width.y) <= (pos2.y + width2.y);
}

pub fn update_coyote_timer_system(
    time: Res<Time>,
    mut query: Query<&mut GroundState, With<Player>>,
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

pub fn update_wall_jump_timer_system(
    time: Res<Time>,
    mut query: Query<&mut JumpController, With<Player>>,
) {
    for mut jump_controller in &mut query {
        jump_controller.wall_jump_timer.tick(time.delta());
    }
}

pub fn enemy_player_collision_system(
    mut events: EventWriter<EnemyPlayerCollisionEvent>,
    enemy_query: Query<(Entity, &Transform, &EnemyCollider), With<Enemy>>,
    player_query: Query<(Entity, &Transform, &PlayerCollider), With<Player>>,
) {
    for (enemy_entity, enemy_transform, enemy_collider) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        let enemy_aabb = enemy_collider.aabb.translated_by(enemy_pos);

        for (player_entity, player_transform, player_collider) in player_query.iter() {
            let player_pos = player_transform.translation.truncate();
            let player_aabb = player_collider.aabb.translated_by(player_pos);

            if enemy_aabb.intersects(&player_aabb) {
                events.write(EnemyPlayerCollisionEvent {
                    player: player_entity,
                    enemy: enemy_entity,
                });
            }
        }
    }
}

pub fn enemy_platform_collision_system(
    mut events: EventWriter<EnemyPlatformCollisionEvent>,
    enemy_query: Query<(Entity, &Transform, &EnemyCollider), With<Enemy>>,
    platform_query: Query<(Entity, &Transform, &Collider), Without<Player>>,
) {
    for (enemy_entity, enemy_transform, enemy_collider) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        let enemy_aabb = enemy_collider.aabb.translated_by(enemy_pos);
        
        for (platform_entity, platform_transform, platform_collider) in platform_query.iter() {
            let platform_pos = platform_transform.translation.truncate();
            let platform_aabb = platform_collider.aabb.translated_by(platform_pos);
        
            if enemy_aabb.intersects(&platform_aabb) {
                events.write(EnemyPlatformCollisionEvent {
                    enemy: enemy_entity,
                    platform: platform_entity,
                });
            }
        }
    }
}

pub fn on_enemy_player_collision_system(
    mut commands: Commands,
    mut events: EventReader<EnemyPlayerCollisionEvent>,
) {
    for ev in events.read() {
        println!("Player {:?} hit by enemy {:?}", ev.player, ev.enemy);
        // Eventually transition to next state (game over)
    }
}

pub fn on_enemy_platform_collision_system(
    mut events: EventReader<EnemyPlatformCollisionEvent>,
    mut enemy_query: Query<(&mut EnemyMovement, &mut Transform)>,
) {
    for ev in events.read() {
        if let Ok((mut enemy_movement, mut transform)) = enemy_query.get_mut(ev.enemy) {   
            enemy_movement.down = !enemy_movement.down;

            if enemy_movement.down {
                transform.translation.y -= 5.0;
            }
            else {
                transform.translation.y += 5.0;
            }
        }
    }
}
