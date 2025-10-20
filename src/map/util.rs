use super::Collider;
use super::mapdata::Boundary;


use super::MapFile;
use super::game_object_builder::GameObject;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::map::game_object_builder::EasedPlatform;
use crate::map::game_object_builder::CubicEasing;
use super::mapdata::Moving;

#[derive(Resource)]
pub struct AtlasLayoutResource {
    pub layout: Handle<TextureAtlasLayout>,
    // map entity names to layout indices.
    pub indices: HashMap<String, usize>,
}

#[derive(Component)]
pub struct FullscreenSprite;

// transform the collider data from the map relative to the game object its defined on.
pub fn collider_from_boundary(
    collision: Option<&Boundary>,
    parent_boundary: &Boundary,
    map_height: u32,
) -> Collider {
    collision
        .map(|c| {
            let half_extents = Vec2::new(c.width, c.height) * 0.5;

            let local_center = Vec2::new(
                c.start_x - parent_boundary.start_x,
                (map_height as f32) - (c.start_y + c.height) - parent_boundary.start_y,
            ) + half_extents;

            Collider {
                aabb: Aabb2d::new(local_center, half_extents),
            }
        })
        .unwrap_or(Collider {
            aabb: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
        })
}

pub fn create_eased(
    moving: &Moving,
) -> EasedPlatform{
    EasedPlatform{
        start: Vec2::new(moving.start_x as f32, moving.start_y as f32),
        end: Vec2::new(moving.end_x as f32, moving.end_y as f32),
        t: 0.0,
        speed: moving.speed as f32,
        forward: true,
        easing: CubicEasing{
            x1: 0.42,
            y1: 0.0,
            x2: 0.58,
            y2: 1.0,
        }
    }    
}

pub fn background_layer(
    map_dimentions: &(u32, u32),
    image_handle: &Handle<Image>,
    z_layer: f32,
) -> impl Bundle {
    (
        Sprite::from_image(image_handle.clone()),
        // transform so that map image is loaded as the visual bottom of the screen / where the camera starts.
        Transform::from_xyz(
            map_dimentions.0 as f32 / 2.0,
            map_dimentions.1 as f32 / 2.0,
            z_layer,
        ),
        FullscreenSprite,
    )
}

pub fn ground() -> GameObject {
    #[cfg(feature = "client")]
    let sprite = Sprite {
        color: Color::srgb(0.3, 0.8, 0.3), // Optional debug color
        custom_size: Some(Vec2::new(1280.0, 5.0)),
        ..Default::default()
    };

    let transform = Transform::from_xyz(1280.0 / 2.0, -1.0, 0.0);

    #[cfg(feature = "client")]
    let visibility = Visibility::default();

    let collider = Collider {
        aabb: Aabb2d::new(Vec2::new(0.0, 0.0), Vec2::new(1280.0, 5.0) * 0.5),
    };

    // 👇 Uses macro to pick correct constructor depending on feature flag
    crate::new_game_object!("Ground", sprite, transform, visibility).with_collider(collider)
}
// game objects -> slice of the entity layer image
// create the AtlasLayoutResource that we defined
pub fn atlas_layout(
    map_data: &MapFile,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> AtlasLayoutResource {
    let texture_size = UVec2::new(
        map_data.metadata.cols * map_data.metadata.tile_size_px,
        map_data.metadata.rows * map_data.metadata.tile_size_px,
    );

    let (layout, indices) = build_layout(map_data, texture_size);
    let layout_handle = atlas_layouts.add(layout);

    AtlasLayoutResource {
        layout: layout_handle,
        indices,
    }
}

// build atlas layout from the boundaries defined in the MapFile
fn build_layout(
    map_data: &MapFile,
    texture_size: UVec2,
) -> (TextureAtlasLayout, HashMap<String, usize>) {
    let mut layout = TextureAtlasLayout::new_empty(texture_size);

    let atlas_indices = map_data
        .entities
        .iter()
        .map(|(entity_id, entity)| {
            let b = &entity.boundary;
            let rect = URect::new(
                // Urect expects left centered origins
                // Translate to left centered origins.
                (b.start_x - b.width / 2.0) as u32,
                (texture_size.y as f32 - (b.start_y + b.height / 2.0)) as u32,
                (b.start_x + b.width / 2.0) as u32,
                (texture_size.y as f32 - (b.start_y - b.height / 2.0)) as u32,
            );
            let index = layout.add_texture(rect);
            (entity_id.clone(), index)
        })
        .collect::<HashMap<_, _>>();

    (layout, atlas_indices)
}
