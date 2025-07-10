//! UI feedback for skillchains and magic bursts.

use bevy::prelude::*;
use super::components::{MagicBurstWindowEvent, SkillchainStart, SkillchainStep};

/// Plugin adding basic combat UI.
pub struct CombatUiPlugin;

impl Plugin for CombatUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkillchainStart>()
            .add_event::<SkillchainStep>()
            .add_event::<MagicBurstWindowEvent>()
            .add_systems(Update, (show_skillchain_notifications,).chain());
    }
}

fn show_skillchain_notifications(
    mut start_ev: EventReader<SkillchainStart>,
    mut step_ev: EventReader<SkillchainStep>,
    mut mb_ev: EventReader<MagicBurstWindowEvent>,
) {
    for ev in start_ev.read() {
        info!("Skillchain started: level {}", ev.level);
    }
    for ev in step_ev.read() {
        info!("Skillchain step -> level {}", ev.level);
    }
    for ev in mb_ev.read() {
        info!("Magic Burst window: {:?}", ev.elements);
    }
}
