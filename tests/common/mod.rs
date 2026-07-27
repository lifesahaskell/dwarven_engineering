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

/// Runs one `FixedUpdate` tick deterministically, independent of wall-clock time.
///
/// `Time<Fixed>`'s accumulator is fed from real elapsed time, so `app.update()` may run
/// `FixedUpdate` zero or many times. Advance the fixed clock by exactly one timestep and run the
/// schedule directly instead.
// ponytail: `mod common;` is recompiled per integration-test binary, so a binary that doesn't
// call this (camera_test, world_gen_test, input_test) would otherwise get a dead_code warning.
#[allow(dead_code)]
pub fn run_one_fixed_tick(app: &mut App) {
    let timestep = app.world().resource::<Time<Fixed>>().timestep();
    app.world_mut()
        .resource_mut::<Time<Fixed>>()
        .advance_by(timestep);
    app.world_mut().run_schedule(FixedUpdate);
}
