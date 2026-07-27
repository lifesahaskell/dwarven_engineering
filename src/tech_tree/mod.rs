//! `tech_tree` — the `TechNode` unlock-state resource that gates `crafting`/`factory_sim`, per
//! `docs/game-design/06-roadmap.md` M4 ("TechTree gating begins") and
//! `docs/game-design/02-bevy-architecture.md`.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use serde::Deserialize;

use crate::core::{FactoryKind, RecipeId, TechNodeId};

/// Tier names from `docs/game-design/01-progression-milestones.md` (marked there as a proposed
/// default, not yet signed off — the tier *names* may still change).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum MilestoneTier {
    HandTools,
    Workshop,
    EarlyAutomation,
    PoweredProduction,
    IntegratedComplex,
    EndgameAutomation,
}

/// What unlocking a `TechNode` makes available.
#[derive(Clone, Debug, Deserialize)]
pub enum UnlockTarget {
    Recipe(RecipeId),
    Factory(FactoryKind),
}

pub struct TechNodeDef {
    pub name: String,
    pub prerequisites: Vec<TechNodeId>,
    pub unlocks: Vec<UnlockTarget>,
    pub milestone_tier: MilestoneTier,
}

#[derive(Resource)]
pub struct TechTree(pub HashMap<TechNodeId, TechNodeDef>);

/// The set of currently-unlocked tech nodes — what `crafting::handle_craft_requests` and
/// `factory_sim::run_factories` actually check.
// ponytail: no research/manual-unlock mechanic is specified anywhere yet (M4 only says "gating
// begins"), so this is computed once at load as the fixpoint of "every prerequisite already
// unlocked" over `TechTree`. Revisit once a player-triggered research system exists.
#[derive(Resource, Default)]
pub struct UnlockedTech(pub HashSet<TechNodeId>);

/// RON row shape for `assets/data/tech_tree.ron`. Same inline-id rationale as `crafting`'s
/// `ItemRon`/`RecipeRon`.
#[derive(Deserialize)]
struct TechNodeRon {
    id: TechNodeId,
    name: String,
    #[serde(default)]
    prerequisites: Vec<TechNodeId>,
    #[serde(default)]
    unlocks: Vec<UnlockTarget>,
    milestone_tier: MilestoneTier,
}

// ponytail: `include_str!` + eager parse — same rationale as `crafting::load_item_database`.
fn load_tech_tree() -> TechTree {
    let entries: Vec<TechNodeRon> =
        ron::de::from_str(include_str!("../../assets/data/tech_tree.ron"))
            .expect("assets/data/tech_tree.ron must parse");
    TechTree(
        entries
            .into_iter()
            .map(|entry| {
                (
                    entry.id,
                    TechNodeDef {
                        name: entry.name,
                        prerequisites: entry.prerequisites,
                        unlocks: entry.unlocks,
                        milestone_tier: entry.milestone_tier,
                    },
                )
            })
            .collect(),
    )
}

fn compute_unlocked(tree: &TechTree) -> UnlockedTech {
    let mut unlocked: HashSet<TechNodeId> = HashSet::new();
    loop {
        let mut changed = false;
        for (id, def) in &tree.0 {
            if !unlocked.contains(id) && def.prerequisites.iter().all(|p| unlocked.contains(p)) {
                unlocked.insert(id.clone());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    UnlockedTech(unlocked)
}

pub struct TechTreePlugin;

impl Plugin for TechTreePlugin {
    fn build(&self, app: &mut App) {
        let tree = load_tech_tree();
        let unlocked = compute_unlocked(&tree);
        app.insert_resource(tree).insert_resource(unlocked);
    }
}
