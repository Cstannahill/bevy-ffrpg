//! Combat systems and battle flow.

use bevy::prelude::*;

/// Marker component for combat entities.
#[derive(Component)]
pub struct Combatant;

/// Plugin for managing combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::core::GameState::Battle), start_battle);
    }
}

fn start_battle() {
    // TODO: initialize battle state
}
