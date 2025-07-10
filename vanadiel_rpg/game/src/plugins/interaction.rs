//! Basic interaction detection and event types.

use bevy::prelude::*;

use super::movement::Player;

/// Component for entities that can be interacted with.
#[derive(Component, Clone)]
pub struct Interactable {
    /// Unique identifier for the interactive object.
    pub id: String,
}

/// Event fired when the player interacts with an [`Interactable`].
#[derive(Event)]
pub struct InteractEvent {
    /// The id of the interacted object.
    pub id: String,
}

/// Plugin handling player interaction logic.
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_systems(Update, player_interactions);
    }
}

fn player_interactions(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    interactables: Query<(&Transform, &Interactable)>,
    mut writer: EventWriter<InteractEvent>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(player_t) = player_query.get_single() else { return; };
    let player_pos = player_t.translation.truncate();

    for (transform, interactable) in &interactables {
        let pos = transform.translation.truncate();
        if player_pos.distance(pos) < 20.0 {
            writer.send(InteractEvent {
                id: interactable.id.clone(),
            });
        }
    }
}
