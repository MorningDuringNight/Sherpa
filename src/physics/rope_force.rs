// SPDX-License-Identifier: MIT
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: Rope force + rendering system

use bevy::prelude::*;
use crate::components::rope::{Rope, RopeConstraint};
use crate::components::motion::{RopeForce, NetForce};
use crate::player::Player;          // 用于 query 玩家实体



/// 每帧先把 RopeForce 清零
pub fn clean_rope_force_system(mut q_rope_force: Query<&mut RopeForce>) {
    for mut rope_force in &mut q_rope_force {
        rope_force.0 = Vec2::ZERO;
    }
}

/// 根据 Hooke 定律计算张力，作用在两个端点
pub fn rope_tension_system(
    q_transforms: Query<&Transform>,
    mut q_rope_force: Query<&mut RopeForce>,
    q_rope: Query<&Rope>,
) {
    for rope in &q_rope {
        let Ok([head_transform, tail_transform]) =
            q_transforms.get_many([rope.attached_entity_head, rope.attached_entity_tail])
        else {
            continue;
        };

        let direction = (tail_transform.translation - head_transform.translation).truncate();
        let current_length = direction.length();

        let force = if current_length > rope.constraint.rest_length {
            let extension = current_length - rope.constraint.rest_length;
            let k = rope.constraint.spring_constant;
            let force_magnitude = k * extension;
            let force_direction = direction.normalize();
            force_direction * force_magnitude
        } else {
            Vec2::ZERO
        };

        if let Ok([mut head_force, mut tail_force]) =
            q_rope_force.get_many_mut([rope.attached_entity_head, rope.attached_entity_tail])
        {
            head_force.0 += force;
            tail_force.0 -= force;
        }
    }
}

/// 把 RopeForce 累加到 NetForce
pub fn rope_force_to_system(mut query: Query<(&RopeForce, &mut NetForce)>) {
    for (rope_force, mut net_force) in &mut query {
        net_force.0 += rope_force.0;
    }
}

// ==================== 绳子可视化部分 ====================

/// 临时存放 rope 的几何信息
#[derive(Resource, Default)]
pub struct RopeGeometry {
    pub updates: Vec<(Entity, Vec3, f32, f32)>, // (rope_entity, 中点, 角度, 长度)
}

#[derive(Component)]
pub struct RopeSprite {
    rope_entity: Entity,
}

pub fn init_ropes(
    mut commands: Commands,
    q_players: Query<Entity, With<Player>>, // 或者用你标记主玩家的组件
) {
    let mut player_entities = Vec::new();
    for entity in q_players.iter() {
    player_entities.push(entity);   
    }
    println!("{:?}", player_entities.len());
    if player_entities.len() < 2 {
        println!("玩家实体不足，无法生成 rope");
        return;
    }

    let p1 = player_entities[0];
    let p2 = player_entities[1];

    // 生成 rope 实体
    let rope_entity = commands
        .spawn(Rope {
            constraint: RopeConstraint {
                rest_length: 200.0,
                spring_constant: 50.0,
                max_extension: 300.0,
            },
            attached_entity_head: p1,
            attached_entity_tail: p2,
        })
        .id();

    // 生成 rope sprite
    spawn_rope_sprite(commands, rope_entity, 200.0);
}

/// 系统 2：应用几何信息（只写 rope sprite 的 transform + sprite）
pub fn apply_rope_geometry(
    rope_geometry: Res<RopeGeometry>,
    mut q_rope_sprites: Query<(&mut Transform, &mut Sprite, &RopeSprite)>,
) {
    for (rope_entity, mid, angle, length) in rope_geometry.updates.iter() {
        for (mut transform, mut sprite, rope_sprite) in &mut q_rope_sprites {
            if rope_sprite.rope_entity == *rope_entity {
                transform.translation = *mid;
                transform.rotation = Quat::from_rotation_z(*angle);
                sprite.custom_size = Some(Vec2::new(*length, 2.0));
                break;
            }
        }
    }
}


pub fn spawn_rope_sprite(
    mut commands: Commands,
    rope_entity: Entity,
    initial_length: f32,
) -> Entity {
    // 先生成 sprite 实体，并保存 ID
    let sprite_entity = commands.spawn((
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.0), // 红色
            custom_size: Some(Vec2::new(initial_length, 2.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        RopeSprite { rope_entity },
    )).id();

    // 再打印 sprite_entity 和 rope_entity
    println!("Spawned rope sprite {:?} for rope {:?}", sprite_entity, rope_entity);

    sprite_entity
}

/// 每帧计算 rope sprite 所需的 transform
pub fn compute_rope_geometry(
    q_ropes: Query<(Entity, &Rope)>,
    q_transforms: Query<&Transform>,
    mut rope_geometry: ResMut<RopeGeometry>,
) {
    rope_geometry.updates.clear();

    for (rope_entity, rope) in &q_ropes {
        if let Ok([head_transform, tail_transform]) = q_transforms.get_many([
            rope.attached_entity_head,
            rope.attached_entity_tail,
        ]) {
            let head = head_transform.translation;
            let tail = tail_transform.translation;

            let mid = (head + tail) / 2.0;
            let diff = tail - head;
            let length = diff.truncate().length();
            let angle = diff.y.atan2(diff.x);

            rope_geometry.updates.push((rope_entity, mid, angle, length));
        }
    }
}



