use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Application loading states.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    /// Load game assets.
    #[default]
    AssetLoading,
    /// Gameplay is ready.
    InGame,
}

/// Raw asset handles loaded during the loading state.
#[derive(AssetCollection, Resource)]
pub struct LoadingAssets {
    #[asset(path = "textures/tileset.png")]
    pub tileset: Handle<Image>,
    #[asset(path = "textures/characters.png")]
    pub characters: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 24, columns = 3, rows = 1))]
    pub character_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "audio/overworld_theme.ogg")]
    pub music: Handle<AudioSource>,
}

/// Game assets available after loading completes.
#[derive(Resource)]
pub struct GameAssets {
    pub tileset: Handle<Image>,
    pub characters: Handle<Image>,
    pub character_layout: Handle<TextureAtlasLayout>,
    pub music: Handle<AudioSource>,
}

/// Plugin that manages the asset loading workflow.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .continue_to_state(AppState::InGame)
                    .load_collection::<LoadingAssets>(),
            )
            .add_systems(OnEnter(AppState::AssetLoading), setup_atlas)
            .add_systems(OnEnter(AppState::InGame), on_loaded);
    }
}

fn setup_atlas(
    mut commands: Commands,
    loading: Res<LoadingAssets>,
) {
    commands.insert_resource(GameAssets {
        tileset: loading.tileset.clone(),
        characters: loading.characters.clone(),
        character_layout: loading.character_layout.clone(),
        music: loading.music.clone(),
    });
}

fn on_loaded() {
    info!("Assets ready, entering game!");
}

