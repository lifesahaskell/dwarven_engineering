//! `factory_sim` — `Factory`/`BeltSegment` simulation and the `SimulationDetailLevel` tick-budget
//! degradation mechanism, per `docs/game-design/06-roadmap.md` M4 and
//! `docs/game-design/03-ecs-design.md`. `factory_sim` owns the heaviest per-tick load
//! (`docs/system-design/04-scaling.md`), so tick-degradation is built in from the start rather
//! than retrofitted.
//!
//! A Factory structure is placed through the existing `structures` plugin — `Factory(FactoryKind)`
//! is just another `StructureKind` variant, so placement/footprint-validation/build-cost is
//! reused as-is. This plugin reactively attaches the Factory-specific instance data
//! (`FactoryState`, its onboard `Inventory`) once that placement lands the entity.

use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;
use serde::Deserialize;

use crate::AppState;
use crate::core::{FactoryKind, ItemId, RecipeId};
use crate::crafting::{Inventory, ItemDatabase, RecipeDatabase};
use crate::structures::{StructureKind, StructurePosition};
use crate::tech_tree::UnlockedTech;
use crate::world_gen::PlayerCharacter;

/// A Factory's onboard item buffer. Small — belts, not bulk storage, are meant to feed it.
/// Easily tunable — not a balance decision.
const FACTORY_INVENTORY_SLOTS: usize = 4;

/// Grid-unit radius (same space as `StructurePosition`) inside which a Factory sims at full rate.
/// Beyond it, `Reduced`; beyond `FACTORY_PAUSED_RADIUS`, `Paused`. Per
/// `docs/system-design/04-scaling.md`'s "degrade non-critical simulation detail" strategy.
/// Easily tunable.
const FACTORY_FULL_SIM_RADIUS: f32 = 5.0;
const FACTORY_PAUSED_RADIUS: f32 = 15.0;

pub struct FactoryDef {
    pub name: String,
    pub power_draw: f32,
    pub recipe_slots: u8,
}

#[derive(Resource)]
pub struct FactoryDatabase(pub HashMap<FactoryKind, FactoryDef>);

/// RON row shape for `assets/data/factories.ron`. No `footprint` field here — that's already
/// covered by `structures::StructureDatabase` (every `StructureKind`, `Factory` included, gets
/// its footprint/build_cost from `structures.ron`); duplicating it in both files would just be
/// two sources of truth for the same number.
#[derive(Deserialize)]
struct FactoryRon {
    kind: FactoryKind,
    name: String,
    power_draw: f32,
    recipe_slots: u8,
}

// ponytail: `include_str!` + eager parse — same rationale as `crafting::load_item_database`.
fn load_factory_database() -> FactoryDatabase {
    let entries: Vec<FactoryRon> =
        ron::de::from_str(include_str!("../../assets/data/factories.ron"))
            .expect("assets/data/factories.ron must parse");
    FactoryDatabase(
        entries
            .into_iter()
            .map(|entry| {
                (
                    entry.kind,
                    FactoryDef {
                        name: entry.name,
                        power_draw: entry.power_draw,
                        recipe_slots: entry.recipe_slots,
                    },
                )
            })
            .collect(),
    )
}

/// Per-Factory runtime state driving automatic crafting.
// ponytail: no power grid exists until Tier 3 (`01-progression-milestones.md`), so `powered` is
// always `true`. Wire it to a generator/grid resource once that milestone lands.
#[derive(Component)]
pub struct FactoryState {
    pub active_recipe: Option<RecipeId>,
    pub progress: f32,
    pub powered: bool,
}

/// Tick-budget degradation level, per `docs/system-design/04-scaling.md`. Bucketed by distance
/// from the local player in `update_simulation_detail_level`.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimulationDetailLevel {
    Full,
    Reduced,
    Paused,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A conveyor segment. Each queued item's `f32` is its progress along the segment, `0.0..=1.0`.
// ponytail: progress caps at 1.0 and waits there — there's no belt-to-structure destination
// hookup yet ("logistics beyond simple belts" is an open question, see
// `docs/game-design/07-open-questions.md` Tier 4). Wire up delivery once that's decided.
#[derive(Component)]
pub struct BeltSegment {
    pub direction: Direction,
    pub speed: f32,
    pub items: VecDeque<(ItemId, f32)>,
}

/// `FixedUpdate`-schedule set owning factory/belt simulation, per
/// `docs/game-design/03-ecs-design.md`.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FactorySimSet;

pub struct FactorySimPlugin;

impl Plugin for FactorySimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_factory_database()).add_systems(
            FixedUpdate,
            (
                attach_factory_state,
                update_simulation_detail_level,
                run_factories,
                advance_belts,
            )
                .chain()
                .in_set(FactorySimSet)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// Runs before `structures::StructureSet` each tick (see the `InputSet -> ... -> FactorySimSet ->
/// ... -> StructureSet` ordering in `docs/game-design/03-ecs-design.md`), so a Factory placed
/// this tick is only visible here starting next tick — that's one extra `FixedUpdate` step
/// between placing a Factory and it having `FactoryState`, not a bug.
type NewlyPlacedStructure = (Added<StructureKind>, Without<FactoryState>);

fn attach_factory_state(
    mut commands: Commands,
    factories: Query<(Entity, &StructureKind), NewlyPlacedStructure>,
) {
    for (entity, kind) in &factories {
        if matches!(kind, StructureKind::Factory(_)) {
            commands.entity(entity).insert((
                FactoryState {
                    active_recipe: None,
                    progress: 0.0,
                    powered: true,
                },
                SimulationDetailLevel::Full,
                Inventory::new(FACTORY_INVENTORY_SLOTS),
            ));
        }
    }
}

fn update_simulation_detail_level(
    player: Query<&Transform, With<PlayerCharacter>>,
    mut factories: Query<(&StructurePosition, &mut SimulationDetailLevel)>,
) {
    let Ok(player_pos) = player.single().map(|transform| transform.translation) else {
        return;
    };

    for (pos, mut detail) in &mut factories {
        let factory_pos = Vec3::new(pos.0.x as f32, 0.0, pos.0.y as f32);
        let distance = player_pos.distance(factory_pos);
        *detail = if distance <= FACTORY_FULL_SIM_RADIUS {
            SimulationDetailLevel::Full
        } else if distance <= FACTORY_PAUSED_RADIUS {
            SimulationDetailLevel::Reduced
        } else {
            SimulationDetailLevel::Paused
        };
    }
}

// ponytail: a Factory runs the first recipe it finds whose tech is unlocked and whose inputs are
// on hand; `FactoryDef::recipe_slots` (multiple concurrent recipes) isn't enforced yet — one
// active recipe at a time is all M4 needs. Revisit if multi-slot factories become a real
// requirement.
fn run_factories(
    time: Res<Time<Fixed>>,
    recipes: Res<RecipeDatabase>,
    items: Res<ItemDatabase>,
    unlocked: Res<UnlockedTech>,
    mut factories: Query<(&SimulationDetailLevel, &mut FactoryState, &mut Inventory)>,
) {
    for (detail, mut state, mut inventory) in &mut factories {
        let tick_scale = match detail {
            SimulationDetailLevel::Full => 1.0,
            SimulationDetailLevel::Reduced => 0.25,
            SimulationDetailLevel::Paused => continue,
        };

        if state.active_recipe.is_none() {
            let ready = recipes.0.iter().find(|(_, def)| {
                let tech_ok = match &def.requires_tech {
                    Some(tech) => unlocked.0.contains(tech),
                    None => true,
                };
                tech_ok
                    && def
                        .inputs
                        .iter()
                        .all(|(item, qty)| inventory.count(item) >= *qty)
            });
            if let Some((id, _)) = ready {
                state.active_recipe = Some(id.clone());
                state.progress = 0.0;
            }
        }

        let Some(recipe_id) = state.active_recipe.clone() else {
            continue;
        };
        let Some(recipe) = recipes.0.get(&recipe_id) else {
            state.active_recipe = None;
            continue;
        };

        state.progress += time.delta_secs() * tick_scale;
        if state.progress >= recipe.craft_time_secs {
            for (item, qty) in &recipe.inputs {
                inventory.remove(item, *qty);
            }
            for (item, qty) in &recipe.outputs {
                inventory.add(&items, item, *qty);
            }
            state.active_recipe = None;
            state.progress = 0.0;
        }
    }
}

fn advance_belts(time: Res<Time<Fixed>>, mut belts: Query<&mut BeltSegment>) {
    for mut belt in &mut belts {
        let step = belt.speed * time.delta_secs();
        for (_, progress) in belt.items.iter_mut() {
            *progress = (*progress + step).min(1.0);
        }
    }
}
