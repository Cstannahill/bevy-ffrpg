//! Simple quest state management.

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use super::interaction::InteractEvent;
use super::combat::EncounterEvent;

const DIALOG_ELDER: &str = include_str!("../assets/dialogue/elder.txt");
const DIALOG_GUARD: &str = include_str!("../assets/dialogue/guard.txt");
const DIALOG_SHOPKEEP: &str = include_str!("../assets/dialogue/shopkeeper.txt");
const DIALOG_CHILD: &str = include_str!("../assets/dialogue/child.txt");
const DIALOG_GOBLIN: &str = include_str!("../assets/dialogue/goblin.txt");

/// Status of a quest.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum QuestStatus {
    /// Quest not yet accepted.
    NotStarted,
    /// Quest accepted but objectives incomplete.
    InProgress,
    /// Quest fully completed.
    Completed,
}

impl Default for QuestStatus {
    fn default() -> Self {
        QuestStatus::NotStarted
    }
}

/// Persistent quest log storing all quest states.
#[derive(Resource, Default)]
pub struct QuestLog {
    /// Map of quest id to its current status.
    pub quests: HashMap<String, QuestStatus>,
    /// Progress flag for the tutorial herb objective.
    pub herb_collected: bool,
}

#[derive(Deserialize)]
struct QuestDefinition {
    id: String,
    name: String,
    description: String,
}

/// Plugin registering the quest system.
pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestLog>()
            .add_systems(Startup, setup_quest)
            .add_systems(Update, quest_interactions);
    }
}

const TUTORIAL_QUEST: &str = include_str!("../assets/quests/tutorial_herb.ron");

fn setup_quest(mut log: ResMut<QuestLog>) {
    let def: QuestDefinition = ron::from_str(TUTORIAL_QUEST).expect("valid quest");
    log.quests.insert(def.id, QuestStatus::NotStarted);
}

fn quest_interactions(
    mut events: EventReader<InteractEvent>,
    mut log: ResMut<QuestLog>,
    mut encounter_writer: EventWriter<EncounterEvent>,
) {
    for ev in events.read() {
        match ev.id.as_str() {
            "elder" => handle_elder(&mut log),
            "herb" => handle_herb(&mut log),
            "goblin" => {
                info!("{}", DIALOG_GOBLIN);
                encounter_writer.send(EncounterEvent)
            }
            "guard" => info!("{}", DIALOG_GUARD),
            "shopkeeper" => info!("{}", DIALOG_SHOPKEEP),
            "child" => info!("{}", DIALOG_CHILD),
            _ => {}
        }
    }
}

fn handle_elder(log: &mut QuestLog) {
    let status = log
        .quests
        .entry("tutorial_herb".into())
        .or_insert(QuestStatus::NotStarted);
    match *status {
        QuestStatus::NotStarted => {
            info!("{}", DIALOG_ELDER);
            *status = QuestStatus::InProgress;
        }
        QuestStatus::InProgress => {
            if log.herb_collected {
                info!("Elder: Thank you for the herb! Rest at our inn anytime.");
                *status = QuestStatus::Completed;
            } else {
                info!("Elder: The herb grows just outside town.");
            }
        }
        QuestStatus::Completed => {
            info!("Elder: Good to see you again, hero.");
        }
    }
}

fn handle_herb(log: &mut QuestLog) {
    if let Some(status) = log.quests.get(&"tutorial_herb".to_string()) {
        if *status == QuestStatus::InProgress && !log.herb_collected {
            info!("You picked up the healing herb.");
            log.herb_collected = true;
        }
    }
}
