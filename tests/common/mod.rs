use bevy::prelude::*;
use dwarven_engineering::GamePlugins;

/// A headless app (`MinimalPlugins` — no window, no renderer, no GPU) ticked far enough that the
/// `Startup` shim's `AppState::InGame` transition has been applied and every
/// `OnEnter(AppState::InGame)` system has run.
///
/// One `update()` suffices: `StatesPlugin` schedules `StateTransition` after `PreUpdate`, so the
/// state set during `Startup` is applied within the same first frame.
pub fn in_game_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(GamePlugins);
    app.update();
    app
}
