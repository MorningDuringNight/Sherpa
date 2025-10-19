// src/util/map/loader.rs
use bevy::prelude::*;
use std::path::Path;

use super::game_object_builder::GameObject;
use super::mapdata::{EntityKind};
use super::util::*;
use super::{MAP_NAME, MapFile};

#[derive(Resource)]
pub struct MapDimensions {
    pub w: u32,
    pub h: u32,
}

#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

#[derive(Component, Default)]
pub struct Platform;

#[derive(Component, Default)]
pub struct Coin;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TrampolineBounce(pub f32);

#[derive(Component, Default)]
pub struct Spike;

// entrypoint for spawning different types of objects.
// this should probably be its own file or folder even but we can keep it for now.
pub fn game_objects(
    image: &Handle<Image>,
    atlas: &Res<AtlasLayoutResource>,
    map_data: &Res<MapFile>,
    map_height: u32,
) -> Vec<GameObject> {
    let mut bundles = Vec::new();

    for (id, entity) in &map_data.entities {
        // match for entity.kind, Platform or Coin enum
        println!("id: {}", id);
        let index = atlas.indices[id];

        let sprite = Sprite {
            image: image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index,
            }),
            ..Default::default()
        };
        let transform = Transform::from_xyz(entity.boundary.start_x, entity.boundary.start_y, 0.0);
        let mut bundle = match entity.kind {
            EntityKind::Platform => {
                let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);
                GameObject::new(id, sprite, transform, Visibility::default())
                    .with_collider(collider)
                    .with_marker::<Platform>()
            }
            EntityKind::Coin => {
                let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);
                GameObject::new(id, sprite, transform, Visibility::default())
                    .with_collider(collider)
                    .with_marker::<Coin>()
            }
            EntityKind::Spike => {
                let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);
                GameObject::new(id, sprite, transform, Visibility::default())
                    .with_collider(collider)
                    .with_marker::<Spike>()
            }
            EntityKind::Trampoline => {
                let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);
                let mut bundle = GameObject::new(id, sprite, transform, Visibility::default())
                    .with_collider(collider);
                // 添加弹力强度组件
                let bounce_strength = entity.attributes.bounce_strength.unwrap_or(1.0);
                bundle = bundle.with_component(TrampolineBounce(bounce_strength));
                bundle
            }
        };

        bundles.push(bundle);
    }

    bundles
}

pub fn load_map(
    mut commands: Commands,
    map: Res<MapFile>,
    images: Res<MapTextureHandles>,
    atlas: Res<AtlasLayoutResource>,
    map_dimensions: Res<MapDimensions>,
) {
    let map_entities = game_objects(&images.entity, &atlas, &map, map_dimensions.h);

    for game_entity in map_entities {
        game_entity.spawn(&mut commands);
    }
    let ground = ground();
    ground.spawn(&mut commands);
}

pub fn load_background_layers(
    mut commands: Commands,
    images: Res<MapTextureHandles>,
    map_dimensions: Res<MapDimensions>,
) {
    commands.spawn(background_layer(
        &(map_dimensions.w, map_dimensions.h),
        &(images.tile_fg),
        -1.0,
    ));
}


pub fn load_map_resouces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(MAP_NAME)
        .join(format!("{MAP_NAME}.json"));

    let json_str = std::fs::read_to_string(&json_path).expect("Failed to read JSON file");

    let map: MapFile = serde_json::from_str(&json_str).expect("Failed to parse JSON into MapFile");

    let tile_fg_handle = asset_server.load(&map.layer_images.tile_fg);
    let entity_handle = asset_server.load(&map.layer_images.entity);

    let texture_atlas = atlas_layout(&map, &mut atlas_layouts);

    let map_width = map.metadata.cols * map.metadata.tile_size_px;
    let map_height = map.metadata.rows * map.metadata.tile_size_px;
    commands.insert_resource(MapDimensions {
        w: map_width,
        h: map_height,
    });

    commands.insert_resource(texture_atlas);
    commands.insert_resource(map);

    // I dont see why this is necessary if every game object should be able to point to its asset,
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
}
