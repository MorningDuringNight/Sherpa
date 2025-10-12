//rendering system

use bevy::prelude::*;
use crate::components::rope::{Rope, RopeConstraint, RopeGeometry, RopeSprite};
use crate::components::motion::{RopeForce, NetForce};
use crate::app::FollowedPlayer;          // 用于 query 玩家实体

// ==================== 绳子可视化部分 ====================

pub(super) fn spawn_rope_sprite(
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

pub(super) fn init_ropes(
    mut commands: Commands,
    q_players: Query<Entity, With<FollowedPlayer>>, // 或者用你标记主玩家的组件
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
pub(super) fn apply_rope_geometry(
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

/// 每帧计算 rope sprite 所需的 transform
pub(super) fn compute_rope_geometry(
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



