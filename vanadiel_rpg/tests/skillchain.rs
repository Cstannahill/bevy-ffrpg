use bevy::prelude::*;
use game::combat::components::{ActionLog, CombatConfig, MagicBurstWindowEvent, SkillchainStart, SkillchainStep};
use game::combat::systems::{detect_skillchain, WeaponSkillEvent};
use game::combat::skill_tables::SKILLCHAINS;
use game::combat::components::ScProp;

#[test]
fn fragmentation_distortion_light_chain() {
    let mut app = App::new();
    app.add_event::<WeaponSkillEvent>()
        .add_event::<SkillchainStart>()
        .add_event::<SkillchainStep>()
        .add_event::<MagicBurstWindowEvent>()
        .insert_resource(ActionLog::default())
        .insert_resource(CombatConfig { latency_ms: 0 })
        .add_systems(Update, detect_skillchain);

    let seq = vec![
        WeaponSkillEvent { properties: [ScProp::Fragmentation, ScProp::Gravitation], damage: 100 },
        WeaponSkillEvent { properties: [ScProp::Distortion, ScProp::Fusion], damage: 100 },
        WeaponSkillEvent { properties: [ScProp::Light, ScProp::Light], damage: 100 },
    ];

    for ev in seq { app.world.send_event(ev); }
    app.update();
    app.update();
    app.update();

    let events: Vec<_> = app.world.resource::<Events<SkillchainStart>>().iter_current().collect();
    assert!(!events.is_empty());
}

