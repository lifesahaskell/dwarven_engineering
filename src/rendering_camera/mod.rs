//! `rendering_camera` — the 2.5D camera controller, isolated from sim code per
//! `docs/game-design/02-bevy-architecture.md`.

use bevy::prelude::*;

use crate::AppState;
use crate::world_gen::PlayerCharacter;

/// Fixed camera offset from the player, giving the angled-overhead "2.5D" look.
/// Easily tunable — not a considered composition decision.
pub const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 12.0, 8.0);

/// `Update`-schedule set owning the camera, per `docs/game-design/03-ecs-design.md`.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RenderCameraSet;

pub struct RenderingCameraPlugin;

impl Plugin for RenderingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_camera)
            .add_systems(
                Update,
                follow_player
                    .in_set(RenderCameraSet)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

/// Spawns the camera with a placeholder transform; `follow_player` positions it, and runs before
/// anything can render because `StateTransition` precedes `Update` in the same frame.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

// ponytail: hard snap, no lerp. Smoothing is a polish item — add it only if the snap actually
// looks bad in motion.
fn follow_player(
    player: Query<&Transform, With<PlayerCharacter>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<PlayerCharacter>)>,
) {
    let Ok(player_pos) = player.single().map(|transform| transform.translation) else {
        return;
    };

    for mut camera_transform in &mut camera {
        *camera_transform =
            Transform::from_translation(player_pos + CAMERA_OFFSET).looking_at(player_pos, Vec3::Y);
    }
}
