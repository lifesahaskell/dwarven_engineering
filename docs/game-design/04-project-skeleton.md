# Dwarven Engineering — Project Skeleton

This documents the recommended crate layout. **It does not create the actual `Cargo.toml`/`src/`
files** — scaffolding real code is a follow-up step after this doc set, not part of it.

## Single crate, not a workspace

Recommendation: **single crate**. Reasoning:

- Across this user's existing Rust repos, single-crate is the norm — the only real Cargo workspace
  anywhere (`oxidized-autobots`) is the outlier, not the pattern to match.
- The one plausible justification for splitting — a separate headless-simulation crate for
  deterministic testing — isn't needed. Bevy provides this for free: build the `App` with
  `MinimalPlugins` instead of `DefaultPlugins` (no `WinitPlugin`, no render plugin) inside
  `tests/*.rs` in the *same* crate. No crate boundary required to get headless, GPU-free tests.
- Revisit a workspace split only if a genuinely separate deployable artifact emerges later (e.g.
  a dedicated-host binary distinct from the graphical client) — not on day one, per YAGNI.

## Lib + bin pattern

Mirrors this user's existing `zero2prod` convention: `lib.rs` exposes a `GamePlugins` plugin group
so `tests/` can build a headless `App` against it; `main.rs` is a thin binary that adds
`DefaultPlugins` + `GamePlugins` and calls `.run()`.

## Folder layout

```
dwarven_engineering/
  Cargo.toml               # edition = "2024", [lib] + [[bin]]
  src/
    lib.rs                 # pub struct GamePlugins; re-exports
    main.rs                # thin entrypoint: DefaultPlugins + GamePlugins, .run()
    core/                  # ItemId/RecipeId/TechNodeId newtypes, shared events, typed errors
    world_gen/              mod.rs + Plugin
    survival/                mod.rs + Plugin
    crafting/                mod.rs + Plugin
    factory_sim/             mod.rs + Plugin
    colonist_ai/             mod.rs + Plugin
    structures/              mod.rs + Plugin
    tech_tree/               mod.rs + Plugin
    rendering_camera/        mod.rs + Plugin
    input/                   mod.rs + Plugin
    save_load/               mod.rs + Plugin
    networking/              mod.rs + Plugin (stub, see 02-bevy-architecture.md)
  assets/
    data/                   # items.ron, recipes.ron, factories.ron, tech_tree.ron
  tests/
    crafting_integration.rs
    factory_sim_integration.rs
  scripts/
    run_checks.sh           # local mirror of CI: fmt --check, clippy -D warnings, test
  .github/
    workflows/
      ci.yml                # fmt -> clippy -D warnings -> test (matrix), fail-fast
```

Each plugin subdir under `src/` maps 1:1 to a plugin in `02-bevy-architecture.md` — this keeps the
option open to extract any one of them into its own crate later without a directory reshuffle.

## `Cargo.toml` sketch

```toml
[package]
name = "dwarven_engineering"
edition = "2024"

[lib]
path = "src/lib.rs"

[[bin]]
name = "dwarven_engineering"
path = "src/main.rs"

[dependencies]
bevy = "0"          # PIN THE ACTUAL CURRENT VERSION FROM crates.io — do not trust a guessed number
serde = { version = "1", features = ["derive"] }
ron = "0"           # data-asset format for assets/data/*.ron
thiserror = "1"     # typed errors, per this user's backend-rust-developer quality bar
```

Deliberately minimal — no networking crate yet (the `networking` plugin is a stub, per
`02-bevy-architecture.md`), no pathfinding crate yet (explicit decision point at M5, see
`06-roadmap.md` — don't pre-pick one now).

## CI and scripts convention

`scripts/run_checks.sh` mirrors this user's existing pattern of a local script that runs the same
checks as CI (`cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D
warnings`, `cargo test --all-features`), fail-fast in that order. `.github/workflows/ci.yml` runs
the same, with an OS matrix added at the test stage — see `05-cross-platform-build.md` for the
matrix shape and why lint doesn't need to run per-OS.

## See also

- [`02-bevy-architecture.md`](02-bevy-architecture.md) — the plugin list this folder tree mirrors.
- [`05-cross-platform-build.md`](05-cross-platform-build.md) — CI matrix and per-OS build notes.
- [`06-roadmap.md`](06-roadmap.md) — M0 is exactly "this skeleton boots and CI is green."
