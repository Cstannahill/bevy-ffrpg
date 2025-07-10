//! Lookup tables for skills and skillchains loaded from TOML.

use once_cell::sync::Lazy;
use serde::Deserialize;
use smallvec::{smallvec, SmallVec};

use super::components::{Element, ScProp};

/// Weapon skill definition loaded from `weapon_skills.toml`.
#[derive(Clone, Deserialize)]
pub struct WeaponSkillInfo {
    pub name: String,
    pub tp_cost: u16,
    pub properties: [ScProp; 2],
}

/// Spell definition loaded from `spells.toml`.
#[derive(Clone, Deserialize)]
pub struct SpellInfo {
    pub name: String,
    pub element: Element,
    pub cast_time: u16,
}

/// Skillchain entry mapping a sequence of properties to a level and elements.
#[derive(Clone, Deserialize)]
pub struct ChainEntry {
    pub properties: Vec<ScProp>,
    pub level: u8,
    pub elements: Vec<Element>,
}

/// All weapon skills defined for the game.
pub static WEAPON_SKILLS: Lazy<Vec<WeaponSkillInfo>> = Lazy::new(|| {
    toml::from_str(include_str!("../../assets/data/weapon_skills.toml"))
        .expect("valid weapon skill table")
});

/// All spells available to players.
pub static SPELLS: Lazy<Vec<SpellInfo>> = Lazy::new(|| {
    toml::from_str(include_str!("../../assets/data/spells.toml"))
        .expect("valid spell table")
});

/// Complete skillchain lookup table.
pub static SKILLCHAINS: Lazy<Vec<ChainEntry>> = Lazy::new(|| {
    toml::from_str(include_str!("../../assets/data/skillchain_table.toml"))
        .expect("valid skillchain table")
});

/// Checks if the provided property sequence forms a known skillchain.
pub fn lookup_chain_sequence(seq: &[ScProp]) -> Option<(u8, SmallVec<[Element; 4]>)> {
    for entry in SKILLCHAINS.iter() {
        if entry.properties.as_slice() == seq {
            let mut elems: SmallVec<[Element; 4]> = smallvec![];
            elems.extend_from_slice(&entry.elements);
            return Some((entry.level, elems));
        }
    }
    None
}
