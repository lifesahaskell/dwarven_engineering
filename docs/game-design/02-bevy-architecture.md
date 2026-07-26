# Dwarven Engineering — Bevy Engine Architecture

## Engine choice

**Bevy**, an ECS-native Rust game engine. Chosen because the game is simulation-heavy (colonists,
items, factories, belts, structures — all naturally entities with data), and Bevy ships
cross-platform windowing/rendering/input/audio for Linux/Windows/macOS out of the box, so none of
that needs to be hand-rolled. This is the user's first game/graphics project — there was no
existing engine precedent to match, so this was a cold-start decision (confirmed directly, not
inferred).

**Verify the current Bevy version against crates.io before generating `Cargo.toml`** — don't
treat any specific version number in this doc set as fixed; Bevy ships new minor versions
periodically and the API shifts release to release.

## Plugin boundaries

One Bevy `Plugin` per gameplay concern, aggregated into a single `GamePlugins` group (see
`04-project-skeleton.md`):

| Plugin | Owns |
|---|---|
| `world_gen` | Chunk generation/streaming, spawning/despawning `Chunk` entities based on player proximity (interest management) |
| `survival` | Player `Needs` decay, day/night/weather cycle, environmental hazard application |
| `crafting` | Hand-craft recipe execution (Tier 0/1): `CraftRequestEvent` → `Inventory` mutation |
| `factory_sim` | `Factory`/`BeltSegment` simulation — the heaviest per-tick load (see `system-design/04-scaling.md`); owns the tick-budget-degradation mechanism (`03-ecs-design.md`) |
| `colonist_ai` | `Colonist`/`Needs`/`Job` systems, job selection, pathfinding |
| `structures` | Structure placement/construction/validation |
| `tech_tree` | `TechNode` unlock-state resource; gates what `crafting`/`factory_sim` allow |
| `rendering_camera` | 2.5D camera controller — isolated so it can change without touching sim code |
| `input` | Translates raw input into intent **events** (see below) |
| `save_load` | Periodic snapshot serialization of World/Chunk/Structure/Colonist/Inventory state |
| `networking` | **Stub only for now** — see below |

### Why `input` emits events instead of calling systems directly

`input` translates raw keyboard/mouse/gamepad input into `PlayerMoveIntent`/`PlayerActionIntent`
events, rather than any plugin reaching into `survival`/`crafting`/`structures` systems directly.
Every other simulation system consumes these same intent events regardless of where they
originated. This is deliberate groundwork for M8 (`06-roadmap.md`): once a connected player's
`player.move`/`player.action` messages (per `system-design/02-data-and-api.md`) arrive over the
network, `networking` just needs to emit the same intent events the local `input` plugin already
produces — wiring multiplayer in becomes additive, not a refactor of every gameplay system.

### The `networking` stub

Today `networking` does nothing — the game is effectively single-player/host-only until M8. It
exists as a named plugin now specifically so the attach point is visible in the codebase from
day one, not invented later. See `system-design/02-data-and-api.md` for the message types it will
eventually handle (`player.move` unreliable, `player.action` reliable, `world.state_delta`
unreliable, host→client) and `system-design/03-architecture.md` for why the Host Client is the
sole simulation authority — this is exactly why solo play and hosting-for-others run identical
systems: a solo player *is* a one-player session of the same host-authoritative model.

## Schedule design

Simulation-affecting plugins (`survival`, `crafting`, `factory_sim`, `colonist_ai`, `structures`)
run in Bevy's `FixedUpdate` at a fixed tick rate. This tick is the same "per-tick" concept
`system-design/02-data-and-api.md` uses when describing `world.state_delta` — the simulation tick
rate and the network state-delta rate are the same clock, not two separate concepts that happen
to share a name.

`rendering_camera` and `input` run in Bevy's `Update` schedule (once per frame, decoupled from the
fixed sim tick). `world_gen`'s streaming logic also runs in `Update` (it reacts to camera/player
position, not simulation state).

A top-level `AppState` resource/state enum gates which systems run, via `run_if(in_state(...))`:

```rust
#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum AppState {
    #[default]
    MainMenu,
    WorldLoading,
    InGame,
    Paused,
}
```

Only `InGame` runs the simulation `SystemSet`s from `03-ecs-design.md`; `Paused` runs rendering
but not `FixedUpdate` simulation; `WorldLoading` runs `save_load`'s load path and nothing else.

## See also

- [`system-design/02-data-and-api.md`](../system-design/02-data-and-api.md) — message types the
  `networking` stub will eventually implement.
- [`system-design/03-architecture.md`](../system-design/03-architecture.md) — Host Client as sole
  authority; explains why solo and hosted play share identical systems.
- [`system-design/04-scaling.md`](../system-design/04-scaling.md) — why `factory_sim` is
  specifically flagged as the heaviest per-tick load.
- [`03-ecs-design.md`](03-ecs-design.md) — concrete Components/Resources/SystemSets for each
  plugin above.
