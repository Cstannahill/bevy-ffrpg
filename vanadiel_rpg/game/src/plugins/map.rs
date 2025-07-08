//! Map loading and camera setup.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Plugin responsible for map management.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, load_map));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_map() {
    // TODO: load assets/maps/dev_room.tmx via bevy_ecs_tilemap
}
