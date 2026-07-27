//! `structures` — placement/validation of buildable structures, per
//! `docs/game-design/06-roadmap.md` M3: storage, workbench, furnace, consuming `Inventory`
//! per `StructureDef`'s build cost. Mirrors `crafting`'s `CraftRequestEvent -> Inventory`
//! mutation pattern, just gated by footprint overlap as well as materials.

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use crate::AppState;
use crate::core::{FactoryKind, ItemId};
use crate::crafting::Inventory;
use crate::world_gen::PlayerCharacter;

/// A kind of placeable structure. `Wall` (first defensive structure) arrives at M6
/// (`docs/game-design/06-roadmap.md`) — not added speculatively. `Factory` carries a `FactoryKind`
/// rather than being split into its own top-level entity kind, per `03-ecs-design.md`'s entity
/// mapping notes; `factory_sim` attaches the rest of a Factory's instance data reactively once
/// placement (this module) spawns it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize, Component)]
pub enum StructureKind {
    Storage,
    Workbench,
    Furnace,
    Factory(FactoryKind),
}

pub struct StructureDef {
    pub name: String,
    pub footprint: IVec2,
    pub build_cost: Vec<(ItemId, u32)>,
}

#[derive(Resource)]
pub struct StructureDatabase(pub HashMap<StructureKind, StructureDef>);

/// RON row shape for `assets/data/structures.ron`. `footprint` is a plain tuple (not `IVec2`
/// directly) so this doesn't depend on Bevy's `serialize` feature.
#[derive(Deserialize)]
struct StructureRon {
    kind: StructureKind,
    name: String,
    footprint: (i32, i32),
    build_cost: Vec<(ItemId, u32)>,
}

// ponytail: `include_str!` + eager parse — same rationale as `crafting::load_item_database`.
fn load_structure_database() -> StructureDatabase {
    let entries: Vec<StructureRon> =
        ron::de::from_str(include_str!("../../assets/data/structures.ron"))
            .expect("assets/data/structures.ron must parse");
    StructureDatabase(
        entries
            .into_iter()
            .map(|entry| {
                (
                    entry.kind,
                    StructureDef {
                        name: entry.name,
                        footprint: IVec2::new(entry.footprint.0, entry.footprint.1),
                        build_cost: entry.build_cost,
                    },
                )
            })
            .collect(),
    )
}

/// Marker for a placed structure entity.
#[derive(Component)]
pub struct Structure;

/// Grid-space footprint origin (bottom-left corner), in the same chunk-grid units as
/// `world_gen::ChunkCoord`.
#[derive(Component)]
pub struct StructurePosition(pub IVec2);

/// A request to place `kind` at `position`, per `docs/game-design/03-ecs-design.md`'s
/// `Inventory`-mutation pattern (mirrors `crafting::CraftRequestEvent`).
#[derive(Message)]
pub struct PlaceStructureRequest {
    pub kind: StructureKind,
    pub position: IVec2,
}

/// `FixedUpdate`-schedule set owning structure placement, per
/// `docs/game-design/03-ecs-design.md`.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct StructureSet;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_structure_database())
            .add_message::<PlaceStructureRequest>()
            .add_systems(
                FixedUpdate,
                handle_placement_requests
                    .in_set(StructureSet)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn footprints_overlap(a_pos: IVec2, a_size: IVec2, b_pos: IVec2, b_size: IVec2) -> bool {
    a_pos.x < b_pos.x + b_size.x
        && b_pos.x < a_pos.x + a_size.x
        && a_pos.y < b_pos.y + b_size.y
        && b_pos.y < a_pos.y + a_size.y
}

// ponytail: only the local player can build (solo-only through M7, per
// docs/game-design/06-roadmap.md); revisit once other players' inventories exist at M8.
fn handle_placement_requests(
    mut commands: Commands,
    mut requests: MessageReader<PlaceStructureRequest>,
    structures_db: Res<StructureDatabase>,
    mut inventories: Query<&mut Inventory, With<PlayerCharacter>>,
    existing: Query<(&StructurePosition, &StructureKind)>,
) {
    let Ok(mut inventory) = inventories.single_mut() else {
        return;
    };

    for request in requests.read() {
        let Some(def) = structures_db.0.get(&request.kind) else {
            continue;
        };

        let overlaps = existing.iter().any(|(pos, kind)| {
            let Some(other_def) = structures_db.0.get(kind) else {
                return false;
            };
            footprints_overlap(request.position, def.footprint, pos.0, other_def.footprint)
        });
        if overlaps {
            continue;
        }

        let has_materials = def
            .build_cost
            .iter()
            .all(|(item, qty)| inventory.count(item) >= *qty);
        if !has_materials {
            continue;
        }

        for (item, qty) in &def.build_cost {
            inventory.remove(item, *qty);
        }

        commands.spawn((Structure, request.kind, StructurePosition(request.position)));
    }
}
