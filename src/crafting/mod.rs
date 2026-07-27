//! `crafting` — hand-craft recipe execution (Tier 0/1 only), per
//! `docs/game-design/02-bevy-architecture.md`: `CraftRequestEvent` -> `Inventory` mutation.
//! `ItemDatabase`/`RecipeDatabase` are definitions loaded once at startup; `Inventory` is the
//! per-entity instance data they get read through.

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use crate::AppState;
use crate::core::{ItemId, RecipeId, TechNodeId};
use crate::world_gen::PlayerCharacter;

/// Player's personal inventory capacity. Easily tunable — not a balance decision.
pub const PLAYER_INVENTORY_SLOTS: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum ItemCategory {
    RawResource,
    Tool,
    Consumable,
    Component,
}

pub struct ItemDef {
    pub name: String,
    pub max_stack: u32,
    pub category: ItemCategory,
}

#[derive(Resource)]
pub struct ItemDatabase(pub HashMap<ItemId, ItemDef>);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum StationKind {
    HandCraft,
    Furnace,
    Assembler,
}

pub struct RecipeDef {
    pub inputs: Vec<(ItemId, u32)>,
    pub outputs: Vec<(ItemId, u32)>,
    pub craft_time_secs: f32,
    pub station: StationKind,
    pub requires_tech: Option<TechNodeId>,
}

#[derive(Resource)]
pub struct RecipeDatabase(pub HashMap<RecipeId, RecipeDef>);

/// RON row shape for `assets/data/items.ron` — a `Vec`, not a `HashMap`, so the id sits inline
/// per entry and RON avoids deserializing `ItemId` as a map key.
#[derive(Deserialize)]
struct ItemRon {
    id: ItemId,
    name: String,
    max_stack: u32,
    category: ItemCategory,
}

/// RON row shape for `assets/data/recipes.ron`. Same inline-id rationale as `ItemRon`.
#[derive(Deserialize)]
struct RecipeRon {
    id: RecipeId,
    inputs: Vec<(ItemId, u32)>,
    outputs: Vec<(ItemId, u32)>,
    craft_time_secs: f32,
    station: StationKind,
    #[serde(default)]
    requires_tech: Option<TechNodeId>,
}

// ponytail: `include_str!` + eager parse at plugin build, not Bevy's `AssetServer`. Items/recipes
// never need hot-reload (no mod marketplace, per docs/system-design/00-overview.md), and this
// keeps the data available synchronously in headless tests with no `AssetPlugin`/async load to
// wait on. Revisit if modding or live data tuning ever becomes a real requirement.
fn load_item_database() -> ItemDatabase {
    let entries: Vec<ItemRon> = ron::de::from_str(include_str!("../../assets/data/items.ron"))
        .expect("assets/data/items.ron must parse");
    ItemDatabase(
        entries
            .into_iter()
            .map(|entry| {
                (
                    entry.id,
                    ItemDef {
                        name: entry.name,
                        max_stack: entry.max_stack,
                        category: entry.category,
                    },
                )
            })
            .collect(),
    )
}

fn load_recipe_database() -> RecipeDatabase {
    let entries: Vec<RecipeRon> = ron::de::from_str(include_str!("../../assets/data/recipes.ron"))
        .expect("assets/data/recipes.ron must parse");
    RecipeDatabase(
        entries
            .into_iter()
            .map(|entry| {
                (
                    entry.id,
                    RecipeDef {
                        inputs: entry.inputs,
                        outputs: entry.outputs,
                        craft_time_secs: entry.craft_time_secs,
                        station: entry.station,
                        requires_tech: entry.requires_tech,
                    },
                )
            })
            .collect(),
    )
}

pub struct ItemStack {
    pub item: ItemId,
    pub count: u32,
}

/// Per-entity item storage — attached to `PlayerCharacter` (and later `Structure`/`Colonist`),
/// per `docs/game-design/03-ecs-design.md`.
#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: (0..capacity).map(|_| None).collect(),
        }
    }

    pub fn count(&self, item: &ItemId) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|stack| stack.item == *item)
            .map(|stack| stack.count)
            .sum()
    }

    /// "Pickup" — stacks `count` of `item` into existing slots up to `max_stack` before opening
    /// new slots. Best-effort: if the inventory fills up partway through, returns `false` having
    /// added as much as fit rather than rolling the partial add back.
    pub fn add(&mut self, database: &ItemDatabase, item: &ItemId, mut count: u32) -> bool {
        let Some(def) = database.0.get(item) else {
            return false;
        };

        for slot in self.slots.iter_mut().flatten() {
            if slot.item == *item && slot.count < def.max_stack {
                let take = (def.max_stack - slot.count).min(count);
                slot.count += take;
                count -= take;
                if count == 0 {
                    return true;
                }
            }
        }

        while count > 0 {
            let Some(empty_slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else {
                return false;
            };
            let take = count.min(def.max_stack);
            *empty_slot = Some(ItemStack {
                item: item.clone(),
                count: take,
            });
            count -= take;
        }
        true
    }

    /// "Drop" — removes `count` of `item` across slots. Leaves the inventory unchanged and
    /// returns `false` if it doesn't hold enough.
    pub fn remove(&mut self, item: &ItemId, mut count: u32) -> bool {
        if self.count(item) < count {
            return false;
        }

        for slot in &mut self.slots {
            let Some(stack) = slot else { continue };
            if stack.item != *item {
                continue;
            }
            let take = stack.count.min(count);
            stack.count -= take;
            count -= take;
            if stack.count == 0 {
                *slot = None;
            }
            if count == 0 {
                break;
            }
        }
        true
    }
}

/// A request to craft one batch of `recipe`'s output, per `docs/game-design/03-ecs-design.md`.
#[derive(Message)]
pub struct CraftRequestEvent {
    pub recipe: RecipeId,
}

/// `FixedUpdate`-schedule set owning crafting, per `docs/game-design/03-ecs-design.md`.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CraftingSet;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_item_database())
            .insert_resource(load_recipe_database())
            .add_message::<CraftRequestEvent>()
            .add_systems(
                FixedUpdate,
                handle_craft_requests
                    .in_set(CraftingSet)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

// ponytail: only the local player can craft (solo-only through M7, per
// docs/game-design/06-roadmap.md); revisit once other players' inventories exist at M8.
fn handle_craft_requests(
    mut requests: MessageReader<CraftRequestEvent>,
    recipes: Res<RecipeDatabase>,
    items: Res<ItemDatabase>,
    mut inventories: Query<&mut Inventory, With<PlayerCharacter>>,
) {
    let Ok(mut inventory) = inventories.single_mut() else {
        return;
    };

    for request in requests.read() {
        let Some(recipe) = recipes.0.get(&request.recipe) else {
            continue;
        };
        let has_inputs = recipe
            .inputs
            .iter()
            .all(|(item, qty)| inventory.count(item) >= *qty);
        if !has_inputs {
            continue;
        }
        for (item, qty) in &recipe.inputs {
            inventory.remove(item, *qty);
        }
        for (item, qty) in &recipe.outputs {
            inventory.add(&items, item, *qty);
        }
    }
}
