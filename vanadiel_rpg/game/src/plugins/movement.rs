//! Player and NPC movement systems.

use bevy::prelude::*;
use bevy::input::ButtonInput;

/// Plugin handling movement logic.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn keyboard_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
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
            // TODO: integrate random-encounter trigger
        }
    }
}
