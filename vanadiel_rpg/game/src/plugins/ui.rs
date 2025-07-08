//! Heads-up display and UI widgets.

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

/// Plugin for user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fps_ui)
            .add_systems(Update, update_fps);
    }
}

#[derive(Component)]
struct FpsText;

fn spawn_fps_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..default()
        },
        FpsText,
    ));
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        for mut text in &mut query {
            if let Some(average) = fps.average() {
                *text = format!("{:.0} fps", average).into();
            }
        }
    }
    // TODO: dialogue windows and battle menus
}
