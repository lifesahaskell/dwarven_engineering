use bevy::prelude::*;
use dwarven_engineering::GamePlugins;

/// Baseline check: the game's plugin group builds and ticks in a headless app
/// (`MinimalPlugins` = no window, no renderer, no GPU) without panicking.
#[test]
fn headless_app_with_game_plugins_updates_without_panicking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(GamePlugins);

    app.update();
}
