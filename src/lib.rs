use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Aggregator for every gameplay plugin, per `docs/game-design/02-bevy-architecture.md`.
///
/// Empty at M0 — the gameplay plugins (`world_gen`, `survival`, `crafting`, ...) land from M1
/// onward and get `.add(...)`ed here. This exists now only so `main.rs` and the headless tests
/// share one attach point.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        // ponytail: no plugins registered yet — add them here as each milestone lands.
        PluginGroupBuilder::start::<Self>()
    }
}
