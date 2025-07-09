//! Map loading and camera setup.

use bevy::prelude::*;
use bevy::color::palettes::basic::BLUE;
use bevy_ecs_tilemap::prelude::*;

use super::movement::Player;
use super::loading::{AppState, GameAssets};

/// Plugin responsible for map management.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_npc))
            .add_systems(OnEnter(AppState::InGame), load_map)
            .add_systems(Update, camera_follow);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


#[derive(Component)]
struct Npc;

fn spawn_npc(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(BLUE, Vec2::splat(16.0)),
        Transform::from_translation(Vec3::new(96.0, 96.0, 1.0)),
        GlobalTransform::default(),
        Npc,
    ));
}

fn load_map(mut commands: Commands, assets: Res<GameAssets>) {
    // Spawn a simple 10x10 tilemap with border walls
    let map_size = TilemapSize { x: 10, y: 10 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let index = if x == 0 || y == 0 || x == map_size.x - 1 || y == map_size.y - 1 {
                1
            } else {
                0
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(index),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let texture = assets.tileset.clone();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture),
        tile_size,
        map_type: TilemapType::Square,
        ..Default::default()
    });
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in &mut camera_query {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
// TODO: pathfinding and collision layers
