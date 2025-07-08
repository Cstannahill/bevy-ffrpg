//! Map loading and camera setup.

use bevy::prelude::*;
use bevy::color::palettes::basic::YELLOW;

use super::movement::Player;

/// Plugin responsible for map management.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_player, load_map))
            .add_systems(Update, camera_follow);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Sprite::from_color(YELLOW, Vec2::splat(16.0)), Player));
}

fn load_map() {
    // TODO: load assets/maps/dev_room.tmx via bevy_ecs_tilemap
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
