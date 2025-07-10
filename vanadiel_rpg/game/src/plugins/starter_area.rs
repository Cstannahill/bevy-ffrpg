//! Spawns the Briarwood Village starter area.
use bevy::prelude::*;

use super::interaction::Interactable;
use super::loading::AppState;

/// Plugin for Briarwood Village setup.
pub struct StarterAreaPlugin;

impl Plugin for StarterAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_area);
    }
}

fn spawn_area(mut commands: Commands) {
    // Elder NPC
    commands.spawn((
        Sprite::from_color(Color::rgb(0.3, 0.5, 1.0), Vec2::splat(16.0)),
        Transform::from_xyz(96.0, 96.0, 1.0),
        GlobalTransform::default(),
        Interactable { id: "elder".to_string() },
    ));
    // Guard NPC
    commands.spawn((
        Sprite::from_color(Color::rgb(0.6, 0.6, 0.8), Vec2::splat(16.0)),
        Transform::from_xyz(64.0, 64.0, 1.0),
        GlobalTransform::default(),
        Interactable { id: "guard".to_string() },
    ));
    // Shopkeeper NPC
    commands.spawn((
        Sprite::from_color(Color::rgb(0.8, 0.7, 0.4), Vec2::splat(16.0)),
        Transform::from_xyz(128.0, 64.0, 1.0),
        GlobalTransform::default(),
        Interactable { id: "shopkeeper".to_string() },
    ));
    // Curious child NPC
    commands.spawn((
        Sprite::from_color(Color::rgb(0.9, 0.8, 0.8), Vec2::splat(16.0)),
        Transform::from_xyz(96.0, 32.0, 1.0),
        GlobalTransform::default(),
        Interactable { id: "child".to_string() },
    ));
    // Goblin target in the forest
    commands.spawn((
        Sprite::from_color(Color::rgb(0.7, 0.2, 0.2), Vec2::splat(16.0)),
        Transform::from_xyz(160.0, 32.0, 1.0),
        GlobalTransform::default(),
        Interactable { id: "goblin".to_string() },
    ));
}

