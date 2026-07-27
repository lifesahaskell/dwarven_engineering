use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

pub mod core;
pub mod crafting;
pub mod factory_sim;
pub mod input;
pub mod rendering_camera;
pub mod structures;
pub mod tech_tree;
pub mod world_gen;

pub use crafting::{
    CraftRequestEvent, CraftingPlugin, Inventory, ItemCategory, ItemDatabase, ItemDef, ItemStack,
    PLAYER_INVENTORY_SLOTS, RecipeDatabase, RecipeDef, StationKind,
};
pub use factory_sim::{
    BeltSegment, Direction, FactoryDatabase, FactoryDef, FactorySimPlugin, FactoryState,
    SimulationDetailLevel,
};
pub use input::{InputPlugin, PLAYER_MOVE_SPEED, PlayerMoveIntent};
pub use rendering_camera::{CAMERA_OFFSET, RenderingCameraPlugin};
pub use structures::{
    PlaceStructureRequest, Structure, StructureDatabase, StructureDef, StructureKind,
    StructurePosition, StructuresPlugin,
};
pub use tech_tree::{
    MilestoneTier, TechNodeDef, TechTree, TechTreePlugin, UnlockTarget, UnlockedTech,
};
pub use world_gen::{
    Chunk, ChunkCoord, ChunkLoadState, LOCAL_PLAYER, PlayerCharacter, PlayerId, WorldGenPlugin,
};

/// Top-level app state gating which systems run, per `docs/game-design/02-bevy-architecture.md`.
///
/// `WorldLoading` and `Paused` exist to match the documented architecture; nothing transitions
/// to or from them yet.
#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    WorldLoading,
    InGame,
    Paused,
}

/// Aggregator for every gameplay plugin, per `docs/game-design/02-bevy-architecture.md`.
///
/// The remaining gameplay plugins (`survival`, `colonist_ai`, ...) land from M5 onward and get
/// `.add(...)`ed here.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AppStatePlugin)
            .add(WorldGenPlugin)
            .add(RenderingCameraPlugin)
            .add(InputPlugin)
            .add(tech_tree::TechTreePlugin)
            .add(CraftingPlugin)
            .add(FactorySimPlugin)
            .add(StructuresPlugin)
            .add(SystemOrderingPlugin)
    }
}

/// Installs `AppState`. Separate from the gameplay plugins only so `StatesPlugin` (absent from
/// `MinimalPlugins`) is guaranteed present before `init_state` runs.
struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<StatesPlugin>() {
            app.add_plugins(StatesPlugin);
        }
        app.init_state::<AppState>()
            .add_systems(Startup, enter_game_immediately);
    }
}

// ponytail: temporary shim — no menu UI exists, so jump straight to `InGame` so the game is
// playable. Delete this and let a menu drive the `MainMenu -> InGame` transition once a menu
// system exists.
fn enter_game_immediately(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

/// Installs the `FixedUpdate` `SystemSet` ordering from `docs/game-design/03-ecs-design.md`:
/// `InputSet -> ... -> CraftingSet -> FactorySimSet -> ... -> StructureSet` (the `SurvivalSet`/
/// `ColonistAiSet` slots in that chain don't exist until M5/M6). A separate plugin only so it can
/// run after every plugin that owns one of these sets has registered its systems.
struct SystemOrderingPlugin;

impl Plugin for SystemOrderingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                input::InputSet,
                crafting::CraftingSet,
                factory_sim::FactorySimSet,
                structures::StructureSet,
            )
                .chain(),
        );
    }
}
