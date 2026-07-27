//! `input` — translates raw device input into intent **messages**, never into movement directly.
//!
//! Per `docs/game-design/02-bevy-architecture.md` this indirection is the seam M8's `networking`
//! plugin plugs into: a remote player's `player.move` becomes the same `PlayerMoveIntent` the
//! local keyboard produces, so no gameplay system needs to change. Don't collapse it.
//!
//! Note: Bevy 0.19 calls buffered events `Message` (`Event` is observer-only now), so the design
//! doc's "intent events" are `Message`s here.

use bevy::input::InputPlugin as BevyInputPlugin;
use bevy::prelude::*;

use crate::AppState;
use crate::world_gen::PlayerCharacter;

/// Player movement speed in world units per second. Easily tunable — not a balance decision.
pub const PLAYER_MOVE_SPEED: f32 = 5.0;

/// A request to move the player on the ground plane. `direction.x` is world X, `direction.y` is
/// world Z (world Y is up and unaffected).
#[derive(Message)]
pub struct PlayerMoveIntent {
    pub direction: Vec2,
}

/// First `FixedUpdate` set, per `docs/game-design/03-ecs-design.md`'s
/// `InputSet -> SurvivalSet -> ...` ordering.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InputSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // `MinimalPlugins` omits Bevy's own `InputPlugin`, so `ButtonInput<KeyCode>` wouldn't
        // exist in headless tests. `DefaultPlugins` already has it — hence the guard.
        if !app.is_plugin_added::<BevyInputPlugin>() {
            app.add_plugins(BevyInputPlugin);
        }
        app.add_message::<PlayerMoveIntent>()
            .add_systems(Update, write_move_intent.run_if(in_state(AppState::InGame)))
            .add_systems(
                FixedUpdate,
                apply_move_intent
                    .in_set(InputSet)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

/// Raw keys in, one intent message out. Deliberately contains no movement math.
// ponytail: WASD only. Arrow keys / gamepad / rebinding when someone actually asks.
fn write_move_intent(
    keys: Res<ButtonInput<KeyCode>>,
    mut intents: MessageWriter<PlayerMoveIntent>,
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.y -= 1.0; // forward = -Z
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        intents.write(PlayerMoveIntent {
            direction: direction.normalize(),
        });
    }
}

fn apply_move_intent(
    mut intents: MessageReader<PlayerMoveIntent>,
    time: Res<Time<Fixed>>,
    mut player: Query<&mut Transform, With<PlayerCharacter>>,
) {
    let Ok(mut transform) = player.single_mut() else {
        return;
    };

    for intent in intents.read() {
        let step = Vec3::new(intent.direction.x, 0.0, intent.direction.y);
        transform.translation += step * PLAYER_MOVE_SPEED * time.delta_secs();
    }
}
