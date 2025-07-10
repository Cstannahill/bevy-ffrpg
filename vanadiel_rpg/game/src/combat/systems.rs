//! Core combat systems including skillchain detection and magic burst logic.

use bevy::prelude::*;
use smallvec::smallvec;

use super::components::{ActionLog, LoggedSkill, MagicBurstWindow, SkillchainWindow};
use super::components::{Element, ScProp};
use super::skill_tables::lookup_chain;

/// Event fired when a weapon skill is used.
#[derive(Event)]
pub struct WeaponSkillEvent {
    pub properties: [ScProp; 2],
    pub damage: u32,
}

/// Event fired when a spell finishes casting.
#[derive(Event)]
pub struct SpellCastEvent {
    pub element: Element,
    pub damage: u32,
}

/// Checks recent weapon skills for valid skillchain pairs.
pub fn detect_skillchain(
    mut commands: Commands,
    mut events: EventReader<WeaponSkillEvent>,
    mut log: ResMut<ActionLog>,
    time: Res<Time>,
) {
    for ev in events.read() {
        let now = time.elapsed_seconds();
        log.skills.push(LoggedSkill { time: now, properties: ev.properties });
        if log.skills.len() > 2 {
            log.skills.remove(0);
        }

        if log.skills.len() < 2 {
            continue;
        }

        let prev = &log.skills[log.skills.len() - 2];
        if now - prev.time > 10.0 {
            continue;
        }

        if let Some(result) = lookup_chain(prev.properties[1], ev.properties[0]) {
            let sc_start = now;
            let sc_end = now + 10.0;
            let mb_start = now + 3.0;
            let mb_end = mb_start + 7.0;
            let elements = result.elements.clone();
            commands.spawn((
                SkillchainWindow { start: sc_start, end: sc_end, elements: elements.clone(), level: result.level },
                MagicBurstWindow { start: mb_start, end: mb_end, elements, level: result.level },
            ));
        }
    }
}

/// Applies magic burst bonuses when spells land during a burst window.
pub fn apply_magic_burst(
    mut commands: Commands,
    mut events: EventReader<SpellCastEvent>,
    mut query: Query<(Entity, &SkillchainWindow, &MagicBurstWindow)>,
    time: Res<Time>,
) {
    for ev in events.read() {
        let now = time.elapsed_seconds();
        for (entity, sc, mb) in &mut query {
            if now >= mb.start && now <= mb.end && mb.elements.contains(&ev.element) {
                let multiplier = 1.3 + (sc.level as f32 * 0.2);
                info!("Magic Burst! Damage x{multiplier:?}");
                // Despawn window after burst.
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Future extensions:
// - Track multiple sequential weapon skills in `ActionLog` to support
//   multi-step chains and shrinking windows.
// - Apply elemental resistance adjustments when a chain or magic burst lands.
// - Compensate for network latency by stretching detection windows based on
//   player equipment or traits like "SkillchainDuration+".
