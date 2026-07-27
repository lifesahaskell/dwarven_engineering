use bevy::prelude::*;
use dwarven_engineering::{Chunk, ChunkCoord, ChunkLoadState, PlayerCharacter};
use std::collections::HashSet;

mod common;
use common::in_game_app;

#[test]
fn world_gen_spawns_a_five_by_five_chunk_grid_around_the_origin() {
    let mut app = in_game_app();

    let mut query = app.world_mut().query_filtered::<&ChunkCoord, With<Chunk>>();
    let spawned: Vec<IVec2> = query.iter(app.world()).map(|coord| coord.0).collect();

    let expected: HashSet<IVec2> = (-2..=2)
        .flat_map(|x| (-2..=2).map(move |y| IVec2::new(x, y)))
        .collect();

    assert_eq!(spawned.len(), 25, "expected exactly 25 chunks");
    assert_eq!(
        spawned.into_iter().collect::<HashSet<_>>(),
        expected,
        "chunk coords must be exactly the -2..=2 x -2..=2 grid"
    );
}

#[test]
fn spawned_chunks_start_loaded() {
    let mut app = in_game_app();

    let mut query = app
        .world_mut()
        .query_filtered::<&ChunkLoadState, With<Chunk>>();
    let states: Vec<&ChunkLoadState> = query.iter(app.world()).collect();

    assert_eq!(states.len(), 25, "every chunk needs a load state");
    assert!(
        states
            .iter()
            .all(|state| matches!(state, ChunkLoadState::Loaded)),
        "M1's static grid is fully loaded, no streaming"
    );
}

#[test]
fn world_gen_spawns_exactly_one_local_player_at_the_origin() {
    let mut app = in_game_app();

    let mut query = app
        .world_mut()
        .query_filtered::<&Transform, With<PlayerCharacter>>();
    let players: Vec<Vec3> = query
        .iter(app.world())
        .map(|transform| transform.translation)
        .collect();

    assert_eq!(players, vec![Vec3::ZERO], "one player, at world origin");
}
