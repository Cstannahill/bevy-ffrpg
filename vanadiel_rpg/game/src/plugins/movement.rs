//! Player and NPC movement systems.

use bevy::prelude::*;
use bevy::input::ButtonInput;
use rand::random;

use super::combat::EncounterEvent;
use super::core::GameState;

/// Plugin handling movement logic.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            keyboard_movement.run_if(in_state(GameState::Exploration)),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn keyboard_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut encounter_writer: EventWriter<EncounterEvent>,
) {
    for mut transform in &mut query {
        let mut delta = Vec3::ZERO;
        if keyboard.just_pressed(KeyCode::KeyA) {
            delta.x -= 1.0;
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            delta.x += 1.0;
        }
        if keyboard.just_pressed(KeyCode::KeyW) {
            delta.y += 1.0;
        }
        if keyboard.just_pressed(KeyCode::KeyS) {
            delta.y -= 1.0;
        }
        if delta != Vec3::ZERO {
            // TODO: add collision checking
            transform.translation += delta;
            // very basic encounter chance
            if random::<f32>() < 0.05 {
                encounter_writer.send(EncounterEvent);
            }
        }
    }
}
