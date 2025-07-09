//! Entry point for the Vana'diel RPG.

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::texture::ImagePlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_ecs_tilemap::TilemapPlugin;

mod plugins {
    pub mod core;
    pub mod map;
    pub mod movement;
    pub mod combat;
    pub mod ui;
}

use plugins::{combat::CombatPlugin, core::CorePlugin, map::MapPlugin, movement::MovementPlugin, ui::UiPlugin};

/// Launches the game application.
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (640.0, 360.0).into(),
                    title: "Vana'diel RPG".into(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(TilemapPlugin)
        .add_plugins(CorePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, startup_log)
        .run();
}

fn startup_log() {
    info!("Vana'diel Awaits...");
}
