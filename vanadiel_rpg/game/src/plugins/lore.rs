//! Loads world lore data from docs.

use bevy::prelude::*;
use serde::Deserialize;

const LORE_STR: &str = include_str!("../../docs/world_lore.toml");

#[derive(Debug, Deserialize)]
pub struct WorldLore {
    pub regions: Vec<Region>,
    #[serde(rename = "intro_quest")]
    pub intro_quest: IntroQuest,
    pub characters: Option<Vec<Character>>, 
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub name: String,
    pub locations: Option<Vec<String>>, 
    pub factions: Option<Vec<String>>, 
}

#[derive(Debug, Deserialize)]
pub struct IntroQuest {
    pub start: String,
    pub summary: String,
    pub allies: Option<Vec<String>>, 
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
}

#[derive(Resource)]
pub struct LoreResource(pub WorldLore);

/// Plugin that parses `world_lore.toml` into [`LoreResource`].
pub struct LorePlugin;

impl Plugin for LorePlugin {
    fn build(&self, app: &mut App) {
        let parsed: WorldLore = toml::from_str(LORE_STR).expect("world_lore parsed");
        app.insert_resource(LoreResource(parsed));
    }
}

