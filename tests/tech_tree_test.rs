use dwarven_engineering::core::TechNodeId;
use dwarven_engineering::{TechTree, UnlockedTech};

mod common;
use common::in_game_app;

fn tech_id(id: &str) -> TechNodeId {
    TechNodeId(id.to_string())
}

#[test]
fn tech_tree_loads_from_ron() {
    let app = in_game_app();

    let tree = app.world().resource::<TechTree>();
    let node = tree
        .0
        .get(&tech_id("early_automation"))
        .expect("early_automation node defined");
    assert_eq!(node.prerequisites, vec![]);
}

#[test]
fn nodes_unlock_once_every_prerequisite_is_unlocked_transitively() {
    let app = in_game_app();

    let unlocked = app.world().resource::<UnlockedTech>();
    assert!(
        unlocked.0.contains(&tech_id("early_automation")),
        "no prerequisites: unlocked immediately"
    );
    assert!(
        unlocked.0.contains(&tech_id("powered_production")),
        "its only prerequisite (early_automation) is unlocked, so this unlocks transitively"
    );
}

#[test]
fn a_node_with_an_unsatisfiable_prerequisite_stays_locked() {
    let app = in_game_app();

    let unlocked = app.world().resource::<UnlockedTech>();
    assert!(
        !unlocked.0.contains(&tech_id("integrated_complex")),
        "its prerequisite (multi_input_assembly) is never defined, so it can never unlock"
    );
}
