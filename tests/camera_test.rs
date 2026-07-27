use bevy::prelude::*;
use dwarven_engineering::{CAMERA_OFFSET, PlayerCharacter};

mod common;
use common::in_game_app;

#[test]
fn exactly_one_camera_spawns_at_the_player_plus_the_fixed_offset() {
    let mut app = in_game_app();

    let mut player_query = app
        .world_mut()
        .query_filtered::<&Transform, With<PlayerCharacter>>();
    let player_pos = player_query
        .single(app.world())
        .expect("world_gen spawns exactly one player")
        .translation;

    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Transform, With<Camera3d>>();
    let cameras: Vec<Vec3> = camera_query
        .iter(app.world())
        .map(|transform| transform.translation)
        .collect();

    assert_eq!(
        cameras,
        vec![player_pos + CAMERA_OFFSET],
        "one 3D camera, offset from the player"
    );
}

#[test]
fn the_camera_follows_the_player_when_the_player_moves() {
    let mut app = in_game_app();

    let moved_to = Vec3::new(3.0, 0.0, -7.0);
    let mut player_query = app
        .world_mut()
        .query_filtered::<&mut Transform, With<PlayerCharacter>>();
    player_query
        .single_mut(app.world_mut())
        .expect("world_gen spawns exactly one player")
        .translation = moved_to;

    app.update();

    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Transform, With<Camera3d>>();
    let camera = camera_query
        .single(app.world())
        .expect("exactly one camera")
        .translation;

    assert_eq!(camera, moved_to + CAMERA_OFFSET);
}
