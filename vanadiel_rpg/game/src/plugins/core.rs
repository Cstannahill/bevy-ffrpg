//! Core game states and resources.

use bevy::prelude::*;

/// Global game state machine.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    /// Exploration mode.
    #[default]
    Exploration,
    /// Turn-based battles.
    Battle,
    /// Dialogue and cutscenes.
    Dialogue,
}

/// Player quest log placeholder.
#[derive(Resource, Default)]
pub struct QuestLog;

/// Player inventory placeholder.
#[derive(Resource, Default)]
pub struct Inventory;

/// World progress flags placeholder.
#[derive(Resource, Default)]
pub struct WorldFlags;

/// Main plugin for setting up global state and resources.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(QuestLog::default())
            .insert_resource(Inventory::default())
            .insert_resource(WorldFlags::default())
            .add_systems(Startup, setup_markers);
    }
}

fn setup_markers() {
    // TODO: scheduling markers for gameplay systems
}
