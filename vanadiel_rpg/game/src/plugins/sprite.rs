use bevy::prelude::*;

use super::movement::Player;
use super::loading::{AppState, GameAssets};

/// Plugin spawning the player sprite once assets are loaded.
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player_sprite);
    }
}

fn spawn_player_sprite(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.characters.clone(),
            TextureAtlas::from(assets.character_layout.clone()),
        ),
        Transform::from_translation(Vec3::new(80.0, 80.0, 1.0)),
        Player,
    ));
}

