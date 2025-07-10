//! Core combat systems including skillchain detection and magic burst logic.

use bevy::prelude::*;
use smallvec::{smallvec, SmallVec};

use super::components::{
    ActionLog, CombatConfig, LoggedSkill, MagicBurstHit, MagicBurstWindow, MagicBurstWindowEvent,
    SkillchainStart, SkillchainStep, SkillchainWindow,
};
use super::components::{Element, ScProp};
use super::skill_tables::lookup_chain_sequence;

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

/// Checks recent weapon skills for known skillchain sequences.
pub fn detect_skillchain(
    mut commands: Commands,
    mut events: EventReader<WeaponSkillEvent>,
    mut log: ResMut<ActionLog>,
    config: Res<CombatConfig>,
    mut sc_start: EventWriter<SkillchainStart>,
    mut sc_step: EventWriter<SkillchainStep>,
    mut mb_event: EventWriter<MagicBurstWindowEvent>,
    time: Res<Time>,
) {
    for ev in events.read() {
        let now = time.elapsed_secs();
        log.skills.push(LoggedSkill {
            time: now,
            properties: ev.properties,
        });
        if log.skills.len() > 6 {
            log.skills.remove(0);
        }

        // Build sequence of first properties from recent skills.
        let prop_seq: Vec<ScProp> = log.skills.iter().map(|s| s.properties[0]).collect();
        let mut matched: Option<(u8, SmallVec<[Element; 4]>)> = None;
        let mut step = 0u8;
        for n in (2..=prop_seq.len()).rev() {
            if let Some((lvl, elements)) = lookup_chain_sequence(&prop_seq[prop_seq.len() - n..]) {
                matched = Some((lvl, elements));
                step = n as u8;
                break;
            }
        }

        if let Some((level, elements)) = matched {
            let base = match step {
                2 => 10.0,
                3 => 8.0,
                4 => 6.0,
                5 => 3.0,
                _ => 2.0,
            } + config.latency_ms as f32 / 1000.0;

            let sc_start_time = now;
            let sc_end_time = now + base;
            let mb_start_time = sc_end_time;
            let mb_end_time = mb_start_time + 5.0;
            let elements_clone = elements.clone();

            commands.spawn((
                SkillchainWindow {
                    start: sc_start_time,
                    end: sc_end_time,
                    elements: elements_clone.clone(),
                    level,
                    step,
                },
                MagicBurstWindow {
                    start: mb_start_time,
                    end: mb_end_time,
                    elements: elements_clone.clone(),
                    level,
                    step,
                },
            ));

            if step == 2 {
                sc_start.send(SkillchainStart { level });
            } else {
                sc_step.send(SkillchainStep { level });
            }
            mb_event.send(MagicBurstWindowEvent {
                elements: elements_clone,
            });
        }
    }
}

/// Applies magic burst bonuses when spells land during a burst window.
pub fn apply_magic_burst(
    mut commands: Commands,
    mut events: EventReader<SpellCastEvent>,
    mut query: Query<(Entity, &SkillchainWindow, &MagicBurstWindow)>,
    mut hit_event: EventWriter<MagicBurstHit>,
    time: Res<Time>,
) {
    for ev in events.read() {
        let now = time.elapsed_secs();
        for (entity, sc, mb) in &mut query {
            if now >= mb.start && now <= mb.end && mb.elements.contains(&ev.element) {
                let bonus = 0.35 + 0.1 * (sc.step.saturating_sub(2)) as f32;
                let final_damage = (ev.damage as f32 * (1.0 + bonus)) as u32;
                info!(
                    "Magic Burst! damage {} (+{:.0}%)",
                    final_damage,
                    bonus * 100.0
                );
                hit_event.send(MagicBurstHit);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
