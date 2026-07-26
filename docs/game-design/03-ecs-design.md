# Dwarven Engineering — ECS Design

This translates the entity list already established in
[`system-design/02-data-and-api.md`](../system-design/02-data-and-api.md) into Bevy ECS shapes.
Every entity below is **adapted, not reinvented** — where the ECS shape differs from a naive
1:1 mapping, the reason is called out explicitly.

## Definitions vs. instances

Split cleanly into two kinds of data:

- **Definitions** — static, data-driven content loaded once at startup from `assets/data/*.ron`
  into lookup-table `Resource`s (RON is the standard Bevy-ecosystem asset format — see
  `04-project-skeleton.md`).
- **Instances** — per-entity runtime `Component`s that reference definitions by id.

This split doesn't exist as a separate concept in `system-design/02`'s entity list (that doc is
about network/data-model shape, not ECS idiom) — it's the one real adaptation this doc makes.

## Definition resources

```rust
#[derive(Resource)]
struct ItemDatabase(HashMap<ItemId, ItemDef>);
struct ItemDef { name: String, max_stack: u32, category: ItemCategory }
enum ItemCategory { RawResource, Tool, Consumable, Component }

#[derive(Resource)]
struct RecipeDatabase(HashMap<RecipeId, RecipeDef>);
struct RecipeDef {
    inputs: Vec<(ItemId, u32)>,
    outputs: Vec<(ItemId, u32)>,
    craft_time_secs: f32,
    station: StationKind,          // HandCraft, Furnace, Assembler, ...
    requires_tech: Option<TechNodeId>,
}

#[derive(Resource)]
struct FactoryDatabase(HashMap<FactoryKind, FactoryDef>);
struct FactoryDef { name: String, footprint: IVec2, power_draw: f32, recipe_slots: u8 }

#[derive(Resource)]
struct TechTree(HashMap<TechNodeId, TechNodeDef>);
struct TechNodeDef {
    name: String,
    prerequisites: Vec<TechNodeId>,
    unlocks: Vec<UnlockTarget>,     // Recipe(RecipeId) | Factory(FactoryKind)
    milestone_tier: MilestoneTier,  // see 01-progression-milestones.md
}
```

## Instance components

```rust
#[derive(Component)] struct Chunk;
#[derive(Component)] struct ChunkCoord(IVec2);
#[derive(Component)] enum ChunkLoadState { Unloaded, Loading, Loaded, Simulating }

#[derive(Component)] struct Structure;
#[derive(Component)] enum StructureKind { Wall, Storage, Workbench, Furnace, Factory(FactoryKind) }
#[derive(Component)] struct Health { current: f32, max: f32 }

#[derive(Component)] struct FactoryState { active_recipe: Option<RecipeId>, progress: f32, powered: bool }
#[derive(Component)] struct BeltSegment { direction: Direction, speed: f32, items: VecDeque<(ItemId, f32)> }

#[derive(Component)] struct Inventory { slots: Vec<Option<ItemStack>> }
struct ItemStack { item: ItemId, count: u32 }

#[derive(Component)] struct PlayerCharacter { player_id: PlayerId }
#[derive(Component)] struct Colonist;
#[derive(Component)] struct Needs { hunger: f32, rest: f32, safety: f32 }
#[derive(Component)] struct Job { kind: JobKind, target: Option<Entity> }
enum JobKind { Idle, Haul, Mine, Craft, Farm }
```

## Entity mapping notes (adapted, not reinvented)

| `system-design/02` entity | ECS shape | Note |
|---|---|---|
| Player | — | Account/identity concept, not a Bevy entity itself; see `PlayerCharacter` |
| PlayerCharacter | `PlayerCharacter` component on a spawned entity | Direct |
| World | Implicit — the whole `App`'s spawned entity set for a session | Not a single entity/component; a World is "everything currently spawned" |
| Chunk | `Chunk` + `ChunkCoord` + `ChunkLoadState` components | Direct |
| Structure | `Structure` + `StructureKind` components | `Factory` is a `StructureKind` variant, not a separate top-level entity kind |
| Colonist | `Colonist` + `Needs` + `Job` components | Direct |
| Job | `Job` component directly on the Colonist entity | Not a separate joined entity — ECS favors components over relational normalization here |
| Need | Fields on the `Needs` component | Not a separate entity per need |
| Item | Row in `ItemDatabase`, referenced by `ItemId` | Definition, not an instance component |
| Inventory | `Inventory` component (on PlayerCharacter, Structure, or Colonist) | Direct |
| Recipe | Row in `RecipeDatabase` | Definition |
| Factory | `StructureKind::Factory(FactoryKind)` + `FactoryState` component | Definition (`FactoryDatabase`) + instance (`FactoryState`) split |
| TechNode | Row in `TechTree`, carries `milestone_tier` | Definition |

## System groupings

Ordered `SystemSet`s in `FixedUpdate` (see `02-bevy-architecture.md` for schedule rationale):

```
InputSet -> SurvivalSet -> CraftingSet -> FactorySimSet -> ColonistAiSet -> StructureSet
```

Plus, outside `FixedUpdate`: `WorldGenSet` and `RenderCameraSet` in `Update`, and a timer-driven
`SaveLoadSet` independent of the tick schedule (see `04-project-skeleton.md`/`05-cross-platform-
build.md` for save-file specifics).

## Tick-budget degradation (`FactorySimSet`)

`system-design/04-scaling.md` flags factory/belt simulation as the single biggest per-session
scaling risk, with the strategy "degrade non-critical simulation detail (far-from-player factory
tick frequency) before letting tick rate slip." This must be a first-class part of `FactorySimSet`
from the start, not retrofitted once performance becomes a problem (see M4 in `06-roadmap.md`):

```rust
#[derive(Component)]
enum SimulationDetailLevel { Full, Reduced, Paused }
```

`FactorySimSet` buckets `Factory` structures by distance from any connected player/host camera and
assigns `SimulationDetailLevel` accordingly — far factories tick less often (`Reduced`) or not at
all (`Paused`) before the overall tick rate is allowed to slip.

## See also

- [`system-design/02-data-and-api.md`](../system-design/02-data-and-api.md) — source entity list.
- [`system-design/04-scaling.md`](../system-design/04-scaling.md) — tick-degradation rationale.
- [`01-progression-milestones.md`](01-progression-milestones.md) — `MilestoneTier` values.
- [`02-bevy-architecture.md`](02-bevy-architecture.md) — plugin/schedule context for these sets.
