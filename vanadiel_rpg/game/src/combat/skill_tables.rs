//! Lookup tables for weapon skills, spells, and skillchain combinations.

use super::components::{Element, ScProp};
use smallvec::smallvec;
use smallvec::SmallVec;

/// Resulting data for a skillchain pair.
#[derive(Clone)]
pub struct ChainResult {
    pub level: u8,
    pub elements: SmallVec<[Element; 4]>,
}

/// Example weapon skill property table.
pub const WEAPON_SKILLS: &[(&str, [ScProp; 2])] = &[
    ("Burning Blade", [ScProp::Scission, ScProp::Liquefaction]),
    ("Flat Blade", [ScProp::Detonation, ScProp::Impaction]),
];

/// Example spell table.
pub const SPELLS: &[(&str, Element, u16)] = &[
    ("Fire", Element::Fire, 3),
    ("Stone", Element::Earth, 3),
];

/// Simple lookup for two properties producing a chain.
pub fn lookup_chain(a: ScProp, b: ScProp) -> Option<ChainResult> {
    use ScProp::*;
    match (a, b) {
        (Scission, Detonation) => Some(ChainResult { level: 1, elements: smallvec![Element::Earth] }),
        (Liquefaction, Impaction) => Some(ChainResult { level: 2, elements: smallvec![Element::Fire, Element::Light] }),
        (Fragmentation, Gravitation) => Some(ChainResult { level: 3, elements: smallvec![Element::Dark] }),
        (Distortion, Fusion) => Some(ChainResult { level: 3, elements: smallvec![Element::Light] }),
        (Light, Light) => Some(ChainResult { level: 4, elements: smallvec![Element::Fire, Element::Wind, Element::Lightning, Element::Light] }),
        (Darkness, Darkness) => Some(ChainResult { level: 4, elements: smallvec![Element::Ice, Element::Earth, Element::Water, Element::Dark] }),
        _ => None,
    }
}
