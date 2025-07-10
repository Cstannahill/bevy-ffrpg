use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy::window::PresentMode;

mod plugins;
use plugins::{
    CombatPlugin, CorePlugin, InteractionPlugin as GameInteractionPlugin, LoadingPlugin,
    LorePlugin, MapPlugin, MovementPlugin, QuestPlugin, SpritePlugin, StarterAreaPlugin, UiPlugin,
};
mod combat;

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
        .add_plugins((
            CorePlugin,
            LoadingPlugin,
            MapPlugin,
            MovementPlugin,
            GameInteractionPlugin,
            QuestPlugin,
            CombatPlugin,
            SpritePlugin,
            StarterAreaPlugin,
            UiPlugin,
            LorePlugin,
        ))
        .run();
}
