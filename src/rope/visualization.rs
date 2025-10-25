// //rendering system

// use bevy::prelude::*;
// use super::component::{Rope, RopeConstraint, RopeGeometry, RopeSprite};
// use super::component::{RopeForce, NetForce};
// use crate::app::FollowedPlayer;          // 用于 query 玩家实体

// // ==================== 绳子可视化部分 ====================

// pub(super) fn spawn_rope_sprite(
//     mut commands: Commands,
//     rope_entity: Entity,
//     initial_length: f32,
// ) -> Entity {
//     // 先生成 sprite 实体，并保存 ID
//     let sprite_entity = commands.spawn((
//         Sprite {
//             color: Color::linear_rgb(1.0, 0.0, 0.0), // 红色
//             custom_size: Some(Vec2::new(initial_length, 2.0)),
//             ..default()
//         },
//         Transform {
//             translation: Vec3::new(0.0, 0.0, 1.0),
//             ..default()
//         },
//         RopeSprite { rope_entity },
//     )).id();

//     // 再打印 sprite_entity 和 rope_entity
//     println!("Spawned rope sprite {:?} for rope {:?}", sprite_entity, rope_entity);

//     sprite_entity
// }

// pub(super) fn init_ropes(
//     mut commands: Commands,
//     q_players: Query<Entity, With<FollowedPlayer>>, // 或者用你标记主玩家的组件
// ) {
//     let mut player_entities = Vec::new();
//     for entity in q_players.iter() {
//     player_entities.push(entity);   
//     }
//     println!("{:?}", player_entities.len());
//     if player_entities.len() < 2 {
//         println!("玩家实体不足，无法生成 rope");
//         return;
//     }

// let p1 = player_entities[0];
// let p2 = player_entities[1];

//     // 生成 rope 实体
//     let rope_entity = commands
//         .spawn(Rope {
//             constraint: RopeConstraint {
//                 rest_length: 200.0,
//                 spring_constant: 50.0,
//                 max_extension: 300.0,
//             },
//             attached_entity_head: p1,
//             attached_entity_tail: p2,
//         })
//         .id();

//     // 生成 rope sprite
//     spawn_rope_sprite(commands, rope_entity, 200.0);
// }

// /// 系统 2：应用几何信息（只写 rope sprite 的 transform + sprite）
// pub(super) fn apply_rope_geometry(
//     rope_geometry: Res<RopeGeometry>,
//     mut q_rope_sprites: Query<(&mut Transform, &mut Sprite, &RopeSprite)>,
// ) {
//     for (rope_entity, mid, angle, length) in rope_geometry.updates.iter() {
//         for (mut transform, mut sprite, rope_sprite) in &mut q_rope_sprites {
//             if rope_sprite.rope_entity == *rope_entity {
//                 transform.translation = *mid;
//                 transform.rotation = Quat::from_rotation_z(*angle);
//                 sprite.custom_size = Some(Vec2::new(*length, 2.0));
//                 break;
//             }
//         }
//     }
// }

// /// 每帧计算 rope sprite 所需的 transform
// pub(super) fn compute_rope_geometry(
//     q_ropes: Query<(Entity, &Rope)>,
//     q_transforms: Query<&Transform>,
//     mut rope_geometry: ResMut<RopeGeometry>,
// ) {
//     rope_geometry.updates.clear();

//     for (rope_entity, rope) in &q_ropes {
//         if let Ok([head_transform, tail_transform]) = q_transforms.get_many([
//             rope.attached_entity_head,
//             rope.attached_entity_tail,
//         ]) {
//             let head = head_transform.translation;
//             let tail = tail_transform.translation;

//             let mid = (head + tail) / 2.0;
//             let diff = tail - head;
//             let length = diff.truncate().length();
//             let angle = diff.y.atan2(diff.x);

//             rope_geometry.updates.push((rope_entity, mid, angle, length));
//         }
//     }
// }


use bevy::prelude::*;
use bevy::render::mesh::{Mesh2d, Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages; // 0.16 需要
// 0.16 起，直接从 prelude 导入 Mesh2d / MaterialMesh2dBundle
use bevy::sprite::{MeshMaterial2d, ColorMaterial};


fn point_curve(pts: &mut Vec<Vec2>, steps: &usize, head: Vec2, tail: Vec2, L: f32) {
    // 计算绳子最短距离
    let D = (tail - head).length();
    println!("D = {}, L = {}", D, L);
    if L <= D {
        println!("Warning: Rope length L <= distance D, cannot form catenary curve.");
        // 直接线性插值
        for i in 0..=*steps {
            let t = i as f32 / *steps as f32;
            let x = head.x + (tail.x - head.x) * t;
            let y = head.y + (tail.y - head.y) * t;
            pts.push(Vec2::new(x, y));
        }
        return;
    }
    
    // 计算垂直距离差
    let y_diff = tail.y - head.y;

    // 计算参数 a
    // 二分法求解 a，使得 2a sinh(d/(2a)) = \sqrt(L^2 - y差^2)
    let d = (tail.x - head.x).abs();
    let target = ((L * L) - (y_diff * y_diff)).sqrt();
    let mut a_low = 0.00001;
    let mut a_high = (target.max(1.0) + d.max(1.0)) * 10.0;
    println!("d = {}, target = {}", d, target);
    println!("a_low = {}, a_high = {}", a_low, a_high);
    for j in 0..64 {
        println!("Iteration {}:", j);
        println!("  a_low = {}, a_high = {}", a_low, a_high);
        let a_mid = (a_low + a_high) * 0.5;
        let lhs = 2.0 * a_mid * (d / (2.0 * a_mid)).sinh();
        println!("  a_mid = {}, lhs = {}", a_mid, lhs);
        if lhs < target {
            a_high = a_mid;
        } else {
            a_low = a_mid;
        }
    }
    let a = (a_low + a_high) * 0.5;

    let x0 = -a * ((y_diff / L).atanh());
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 从 prelude 进来后，这里就能识别 Camera2dBundle 了
    commands.spawn((
        Camera2d,
        Camera::default(),
    ));

    // 1) 准备一串曲线采样点（示例：正弦）
    let mut pts = Vec::new();
    let steps = 120;
    // for i in 0..=steps {
    //     let t = i as f32 / steps as f32;
    //     let x = -300.0 + 600.0 * t;
    //     let y = 100.0 * (t * std::f32::consts::TAU).sin();
    //     pts.push(Vec2::new(x, y));
    // }
    let head = Vec2::new(-300.0, 100.0);
    let tail = Vec2::new(300.0, -100.0);
    let L = (tail - head).length() * 1.5; // 绳子长度
    point_curve(&mut pts, &steps, head, tail, L);

    // 2) 生成“带宽丝带网格”
    let thickness = 8.0;
    let mesh = polyline_ribbon_mesh(&pts, thickness);
    let mesh_handle = meshes.add(mesh);

    // 3) 材质（可换成贴图）
    let material = materials.add(ColorMaterial::from(Color::WHITE));
    // let texture = asset_server.load("branding/icon.png"); // 换成你的纹理路径
    // let material = materials.add(ColorMaterial::from(texture));

    // 4) 生成实体并添加组件

    // 0.16: Mesh2d 是包装类型，MaterialMesh2dBundle 还在
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d::<ColorMaterial>(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
    ));
}

/// 把折线 points 变成“带宽丝带”网格（Bevy 0.16 版）
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
