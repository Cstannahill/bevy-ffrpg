//! Components and data types for the combat system.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Elements used by magic and skillchains.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Ice,
    Wind,
    Earth,
    Lightning,
    Water,
    Light,
    Dark,
}

/// Skillchain properties possessed by weapon skills.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ScProp {
    Transfixion,
    Compression,
    Liquefaction,
    Scission,
    Reverberation,
    Detonation,
    Induration,
    Impaction,
    Fragmentation,
    Distortion,
    Fusion,
    Gravitation,
    Light,
    Darkness,
}

/// Weapon skill metadata component.
#[derive(Component)]
pub struct WeaponSkill {
    pub name: String,
    pub tp_cost: u16,
    pub properties: [ScProp; 2],
}

/// Spell metadata component.
#[derive(Component)]
pub struct Spell {
    pub name: String,
    pub element: Element,
    pub cast_time: u16,
}

/// Active skillchain window for bonus damage.
#[derive(Component)]
pub struct SkillchainWindow {
    pub start: f32,
    pub end: f32,
    pub elements: SmallVec<[Element; 4]>,
    pub level: u8,
    pub step: u8,
}

/// Active magic burst window after a skillchain.
#[derive(Component)]
pub struct MagicBurstWindow {
    pub start: f32,
    pub end: f32,
    pub elements: SmallVec<[Element; 4]>,
    pub level: u8,
    pub step: u8,
}

/// Event emitted when a new skillchain begins.
#[derive(Event)]
pub struct SkillchainStart {
    pub level: u8,
}

/// Event emitted for each additional skillchain step.
#[derive(Event)]
pub struct SkillchainStep {
    pub level: u8,
}

/// Event emitted when a magic burst window opens.
#[derive(Event)]
pub struct MagicBurstWindowEvent {
    pub elements: SmallVec<[Element; 4]>,
}

/// Event emitted when a spell successfully magic bursts.
#[derive(Event)]
pub struct MagicBurstHit;

/// Global combat configuration such as latency tolerance.
#[derive(Resource)]
pub struct CombatConfig {
    pub latency_ms: u32,
}

/// Logged weapon skill used for chain detection.
#[derive(Clone)]
pub struct LoggedSkill {
    pub time: f32,
    pub properties: [ScProp; 2],
}

/// Recent action log resource.
#[derive(Resource, Default)]
pub struct ActionLog {
    pub skills: SmallVec<[LoggedSkill; 6]>,
}

