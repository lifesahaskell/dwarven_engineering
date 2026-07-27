use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

pub mod input;
pub mod rendering_camera;
pub mod world_gen;

pub use input::{InputPlugin, PlayerMoveIntent, PLAYER_MOVE_SPEED};
pub use rendering_camera::{RenderingCameraPlugin, CAMERA_OFFSET};
pub use world_gen::{
    Chunk, ChunkCoord, ChunkLoadState, PlayerCharacter, PlayerId, WorldGenPlugin, LOCAL_PLAYER,
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
/// The remaining gameplay plugins (`survival`, `crafting`, ...) land from M2 onward and get
/// `.add(...)`ed here.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AppStatePlugin)
            .add(WorldGenPlugin)
            .add(RenderingCameraPlugin)
            .add(InputPlugin)
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
