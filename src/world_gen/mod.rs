//! `world_gen` — owns "the world exists": the chunk grid.
//!
//! Per `docs/game-design/02-bevy-architecture.md` this plugin will eventually own chunk
//! streaming (spawn/despawn by player proximity). M1 is a static grid only.

use bevy::prelude::*;

use crate::AppState;

/// Half-extent of the spawned grid: coords run `-CHUNK_GRID_RADIUS..=CHUNK_GRID_RADIUS` on both
/// axes, so a radius of 2 gives the 5x5 / 25-chunk grid M1 asks for. Easily tunable.
const CHUNK_GRID_RADIUS: i32 = 2;

/// Marker for a world chunk entity.
#[derive(Component)]
pub struct Chunk;

/// A chunk's position in chunk-space (not world-space).
#[derive(Component)]
pub struct ChunkCoord(pub IVec2);

/// Streaming lifecycle of a chunk. M1 only ever produces `Loaded`; the other variants exist to
/// match `docs/game-design/03-ecs-design.md`.
#[derive(Component)]
pub enum ChunkLoadState {
    Unloaded,
    Loading,
    Loaded,
    Simulating,
}

/// Identity of a player in a session. `PlayerId(0)` is the local/host player.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlayerId(pub u32);

/// The local player is always `PlayerId(0)`; other ids arrive with networking at M8.
pub const LOCAL_PLAYER: PlayerId = PlayerId(0);

/// A player's avatar in the world.
#[derive(Component)]
pub struct PlayerCharacter {
    pub player_id: PlayerId,
}

/// `Update`-schedule set owning world generation, per `docs/game-design/03-ecs-design.md`.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WorldGenSet;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_starting_chunks, spawn_local_player).in_set(WorldGenSet),
        );
    }
}

fn spawn_starting_chunks(mut commands: Commands) {
    for x in -CHUNK_GRID_RADIUS..=CHUNK_GRID_RADIUS {
        for y in -CHUNK_GRID_RADIUS..=CHUNK_GRID_RADIUS {
            commands.spawn((Chunk, ChunkCoord(IVec2::new(x, y)), ChunkLoadState::Loaded));
        }
    }
}

fn spawn_local_player(mut commands: Commands) {
    commands.spawn((
        PlayerCharacter {
            player_id: LOCAL_PLAYER,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
