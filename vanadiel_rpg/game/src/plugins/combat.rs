//! Combat systems and battle flow.

use bevy::prelude::*;

use super::core::GameState;

/// Event triggered when a random encounter occurs.
#[derive(Event, Default)]
pub struct EncounterEvent;

/// Marker component for combat entities.
#[derive(Component)]
pub struct Combatant;

/// Plugin for managing combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EncounterEvent>()
            .add_systems(Update, handle_encounter.run_if(in_state(GameState::Exploration)))
            .add_systems(OnEnter(GameState::Battle), start_battle)
            .add_systems(Update, exit_battle.run_if(in_state(GameState::Battle)))
            .add_systems(OnExit(GameState::Battle), cleanup_battle);
    }
}

fn start_battle(mut commands: Commands) {
    // Spawn a placeholder enemy sprite
    commands.spawn((
        Sprite::from_color(Color::RED, Vec2::splat(16.0)),
        Combatant,
    ));
}

fn handle_encounter(
    mut events: EventReader<EncounterEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if events.read().next().is_some() {
        next_state.set(GameState::Battle);
    }
}

fn exit_battle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Exploration);
    }
}

fn cleanup_battle(mut commands: Commands, query: Query<Entity, With<Combatant>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
