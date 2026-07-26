# Dwarven Engineering — Cross-Platform Build (Linux / Windows / macOS)

Target: native builds on all three desktop OSes. Not cross-play with consoles/mobile — that
remains a confirmed non-goal per
[`system-design/00-overview.md`](../system-design/00-overview.md).

## Windowing and rendering

All three platforms go through Bevy's `winit` (windowing) + `wgpu` (rendering) stack — there is no
application-code difference between platforms for windowing or rendering itself:

- **Linux**: enable both the `x11` and `wayland` winit features. Don't assume either alone — some
  distros/desktops default to one, some to the other.
- **Windows**: target `x86_64-pc-windows-msvc`, not the `-gnu` toolchain, for best Bevy/wgpu
  compatibility.
- **macOS**: requires Xcode command-line tools installed to compile (links against system
  frameworks for Metal/windowing).
- **Rendering backend**: wgpu auto-selects Vulkan on Linux/Windows and Metal on macOS. No app code
  branches on this.

## CI without a GPU

CI runners typically have no real GPU. Use Bevy's `MinimalPlugins` (no `WinitPlugin`, no render
plugin) to build a headless `App` for integration tests — the same mechanism that justifies the
single-crate decision in `04-project-skeleton.md`. This means CI never needs GPU access at all,
on any of the three OSes.

**Linux CI gotcha**: even a headless build commonly needs system dev packages installed on the
runner or the build fails at dependency-compilation time (not app-logic time, which makes it look
like a mystery failure): `libasound2-dev`, `libudev-dev`, `libx11-dev`, `libxkbcommon-dev`. Install
these explicitly in the CI job rather than debugging a confusing failure later.

## Asset paths

Bevy's `AssetPlugin` resolves `assets/` relative to the executable by default — this works
unmodified as a folder-next-to-exe on Linux and Windows. On macOS, a proper `.app` bundle needs
assets inside `Contents/Resources` or a custom asset-path configuration — but **defer actual `.app`
bundling until there's a first playable milestone** (packaging infra before there's a game to
package is premature per YAGNI). See `06-roadmap.md` — packaging is explicitly post-M2+.

## Save-file location

Use a platform-path crate (e.g. `dirs`) to resolve the correct save directory per OS
(`%APPDATA%` on Windows, `~/Library/Application Support/` on macOS, XDG `~/.local/share/` on
Linux) rather than hardcoding one path. This lives in the `save_load` plugin alongside the
autosave-interval/reload-UX open question — see `07-open-questions.md`; the *location* is a
straightforward platform-path lookup, the *interval/UX* is not yet decided.

## CI matrix shape

Preserve this user's existing fmt → clippy → test fail-fast order, adding the OS matrix only where
platform bugs actually surface:

1. **Lint job** (`ubuntu-latest` only, cheap, platform-independent): `cargo fmt --all --check`,
   then `cargo clippy --all-targets --all-features -- -D warnings`.
2. **Test job** (matrix: `ubuntu-latest`, `windows-latest`, `macos-latest`, gated on the lint job
   passing first): `cargo test --all-features`, including the headless-`App` smoke test from M0.

## Packaging (deferred)

Linux tarball, Windows zip, macOS `.app` bundle (via `cargo-bundle` or similar) — explicitly
deferred to a post-M2-or-later roadmap item (`06-roadmap.md`), not part of the initial skeleton.

## See also

- [`04-project-skeleton.md`](04-project-skeleton.md) — folder layout and `Cargo.toml` this CI
  config applies to.
- [`06-roadmap.md`](06-roadmap.md) — M0 exit criterion is "boots on all 3 OSes, CI green."
- [`07-open-questions.md`](07-open-questions.md) — autosave interval/reload UX (save-file
  *location* is settled here; the *policy* is not).
