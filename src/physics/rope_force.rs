// SPDX-License-Identifier: MIT
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: Rope force + rendering system

use bevy::prelude::*;
use crate::components::rope::{Rope, RopeConstraint};
use crate::components::motion::{RopeForce, NetForce};
use crate::player::Player;          // 用于 query 玩家实体

use crate::config::PlayerSpawnPoint;

use bevy::render::mesh::{Mesh2d, Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MeshMaterial2d, ColorMaterial};

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
    pub updates: Vec<(Entity, Vec2, Vec2)>, // (rope_entity, head, tail)
}

#[derive(Component)]
pub struct RopeSprite {
    rope_entity: Entity,
}

pub fn init_ropes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                rest_length: 300.0,
                spring_constant: 80000.0,
                max_extension: 300.0,
            },
            attached_entity_head: p1,
            attached_entity_tail: p2,
        })
        .id();

    // 生成 rope sprite
    spawn_rope_sprite(commands, rope_entity, 200.0, meshes, materials);
}

/// 系统 2：应用几何信息（只写 rope sprite 的 transform + sprite）
pub fn apply_rope_geometry(
    rope_geometry: Res<RopeGeometry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_rope_sprites: Query<(&mut Transform, &mut Mesh2d, &RopeSprite)>,
) {
    for (rope_entity, head, tail) in rope_geometry.updates.iter() {
        for (mut transform, mut mesh, rope_sprite) in &mut q_rope_sprites {
            if rope_sprite.rope_entity == *rope_entity {
                // println!("Points: head = {:?}, tail = {:?}", head, tail);
                transform.translation = Vec3::new(0.0, 0.0, 0.0);

                let mut pts = Vec::new();
                let steps = 40;
                let L: f32 = 300.0;
                point_curve(&mut pts, &steps, &head, &tail, L);
                let thickness = 2.0;
                // sprite.custom_size = Some(Vec2::new(*length, 2.0));

                let mesh_new = polyline_ribbon_mesh(&pts, thickness);
                let mesh_handle = meshes.add(mesh_new);
                // 更新 mesh2d
                *mesh = Mesh2d(mesh_handle);

                break;
            }
        }
    }
}


pub fn spawn_rope_sprite(
    mut commands: Commands,
    rope_entity: Entity,
    initial_length: f32,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Entity {
    // 先生成 sprite 实体，并保存 ID
    
    // 1) 计算绳子曲线采样点
    // 将spawn_point.position作为head，spawn_point.position + Vec3::new(300.0, -100.0, 0.0)作为tail
    // 但是要转换成 Vec2
    let mut pts = Vec::new();
    let steps = 40;
    let head = Vec2::new(50.0, 0.0);
    let tail = Vec2::new(350.0, 0.0);
    let L = 300.0; // 绳子长度
    point_curve(&mut pts, &steps, &head, &tail, L);

    // 2) 生成 Mesh
    let thickness = 2.0;
    let mesh = polyline_ribbon_mesh(&pts, thickness);
    let mesh_handle = meshes.add(mesh);

    // 3) 材质（可换成贴图）
    let material = materials.add(ColorMaterial::from(Color::WHITE));

    let sprite_entity = commands.spawn((
        // Sprite {
        //     color: Color::linear_rgb(1.0, 0.0, 0.0), // 红色
        //     custom_size: Some(Vec2::new(initial_length, 2.0)),
        //     ..default()
        // },
        
        Mesh2d(mesh_handle),
        MeshMaterial2d::<ColorMaterial>(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        // Transform {
        //     translation: Vec3::new(0.0, 0.0, 1.0),
        //     ..default()
        // },
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
            let head2 = Vec2::new(head.x, head.y);
            let tail2 = Vec2::new(tail.x, tail.y);

            rope_geometry.updates.push((rope_entity, head2, tail2));
        }
    }
}

fn point_curve(pts: &mut Vec<Vec2>, steps: &usize, head: &Vec2, tail: &Vec2, L: f32) {
    println!("head = {:?}, tail = {:?}", head, tail);
    // 计算绳子最短距离
    let D = (tail - head).length();
    // println!("D = {}, L = {}", D, L);
    if L <= D {
        // println!("Warning: Rope length L <= distance D, cannot form catenary curve.");
        // 直接线性插值
        for i in 0..=*steps {
            let t = i as f32 / *steps as f32;
            let x = head.x + (tail.x - head.x) * t;
            let y = head.y + (tail.y - head.y) * t;
            pts.push(Vec2::new(x, y));
        }
        return;
    }

    // 计算水平距离差
    let x_diff = tail.x - head.x;
    
    // 计算垂直距离差
    let y_diff = tail.y - head.y;

    if x_diff.abs() < 0.0001 {
        // 特殊情况：垂直线段
        // 计算最低点
        let y_min = head.y.min(tail.y);
        let y_max = head.y.max(tail.y);
        // 计算最低点对应的 y 坐标
        // y_max - y0 + y_min - y0 = L
        // 2y0 = y_min + y_max - L
        // y0 = (y_min + y_max - L) / 2
        let y0 = (y_min + y_max - L) / 2.0;
        // 生成线段
        // 直接从 y_max 到 y0。
        for i in 0..=*steps {
            let t = i as f32 / *steps as f32;
            let y = y_max + (y0 - y_max) * t;
            pts.push(Vec2::new(head.x, y));
        }
        return;
    }

    // 计算参数 a
    // 二分法求解 a，使得 2a sinh(d/(2a)) = \sqrt(L^2 - y差^2)
    let d = (tail.x - head.x).abs();
    let target = ((L * L) - (y_diff * y_diff)).sqrt();
    let mut a_low = 0.00001;
    let mut a_high = (target.max(1.0) + d.max(1.0)) * 10.0;
    // println!("d = {}, target = {}", d, target);
    // println!("a_low = {}, a_high = {}", a_low, a_high);
    for j in 0..48 {
        // println!("Iteration {}:", j);
        // println!("  a_low = {}, a_high = {}", a_low, a_high);
        let a_mid = (a_low + a_high) * 0.5;
        let lhs = 2.0 * a_mid * (d / (2.0 * a_mid)).sinh();
        // println!("  a_mid = {}, lhs = {}", a_mid, lhs);
        if lhs < target {
            a_high = a_mid;
        } else {
            a_low = a_mid;
        }
    }
    let a = (a_low + a_high) * 0.5;
    
    let r = (y_diff / L).clamp(-0.999_999, 0.999_999);
    let mut x0: f32;
    if head.x < tail.x {
        x0 = -a * (r.atanh()) + (head.x + tail.x) * 0.5;
    } else {
        x0 = a * (r.atanh()) + (head.x + tail.x) * 0.5;
    }
    let c = head.y - a * ((head.x - x0) / a).cosh();

    println!("a = {}", a);
    println!("x0 = {}", x0);
    println!("c = {}", c);

    for i in 0..=*steps {
        let t = i as f32 / *steps as f32;
        // 垂链线方程 y = a cosh(x/a)
        let x = head.x + (tail.x - head.x) * t;
        let y = a * ((x - x0) / a).cosh() + c;
        // println!("Point {}: ({}, {})", i, x, y);
        pts.push(Vec2::new(x, y));
    }
}

fn polyline_ribbon_mesh(points: &[Vec2], width: f32) -> Mesh {
    assert!(points.len() >= 2, "需要至少两个点");
    let half = width * 0.5;

    // 1) 平滑切线 -> 法线
    let mut normals: Vec<Vec2> = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let dir = if i == 0 {
            (points[1] - points[0]).normalize_or_zero()
        } else if i == points.len() - 1 {
            (points[i] - points[i - 1]).normalize_or_zero()
        } else {
            ((points[i + 1] - points[i]).normalize_or_zero()
                + (points[i] - points[i - 1]).normalize_or_zero())
            .normalize_or_zero()
        };
        normals.push(Vec2::new(-dir.y, dir.x));
    }

    // 2) 顶点 + UV
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(points.len() * 2);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(points.len() * 2);
    for (i, p) in points.iter().enumerate() {
        let n = normals[i] * half;
        let left = *p + n;
        let right = *p - n;

        positions.push([left.x, left.y, 0.0]);
        positions.push([right.x, right.y, 0.0]);

        let v = i as f32 / (points.len() - 1) as f32;
        uvs.push([0.0, v]);
        uvs.push([1.0, v]);
    }

    // 3) 索引
    let mut indices: Vec<u32> = Vec::with_capacity((points.len() - 1) * 6);
    for i in 0..points.len() - 1 {
        let base = (i * 2) as u32;
        indices.extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
    }

    // 4) 组装 Mesh —— 0.16 需要 RenderAssetUsages
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices)); // 0.16 用 insert_indices
    mesh
}
