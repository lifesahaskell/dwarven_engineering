use bevy::prelude::*;
use dwarven_engineering::{PLAYER_MOVE_SPEED, PlayerCharacter, PlayerMoveIntent};

mod common;
use common::in_game_app;

#[test]
fn a_move_intent_moves_the_player_along_world_x_for_one_fixed_tick() {
    let mut app = in_game_app();

    // Bypass the keyboard-read system: it reads real device state, which isn't testable headlessly.
    app.world_mut()
        .write_message(PlayerMoveIntent { direction: Vec2::X });

    // `Time<Fixed>`'s accumulator is fed from real elapsed time, so `app.update()` may run
    // `FixedUpdate` zero or many times. Advance the fixed clock by exactly one timestep and run
    // the schedule directly instead — deterministic, no wall-clock dependency.
    let timestep = app.world().resource::<Time<Fixed>>().timestep();
    let step = timestep.as_secs_f32();
    app.world_mut()
        .resource_mut::<Time<Fixed>>()
        .advance_by(timestep);
    app.world_mut().run_schedule(FixedUpdate);

    let mut query = app
        .world_mut()
        .query_filtered::<&Transform, With<PlayerCharacter>>();
    let translation = query
        .single(app.world())
        .expect("world_gen spawns exactly one player")
        .translation;

    assert_eq!(
        translation,
        Vec3::new(PLAYER_MOVE_SPEED * step, 0.0, 0.0),
        "intent X maps to world X, scaled by speed * fixed timestep"
    );
}
