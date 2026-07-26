# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository state

This repo currently contains only design documentation for Dwarven Engineering — there is no
application code, package manifest, build system, or test suite yet. Don't invent build/lint/test
commands; none exist. Start here before writing any implementation code, since the architecture
decisions below (especially the peer-to-peer pivot, and the Bevy engine/ECS design once
implementation starts) constrain how any future client/service code should be structured.

## Reading the spec

Two doc sets, read `system-design` before `game-design` (the latter assumes the former's
decisions and cross-links into it rather than repeating them):

- [`docs/system-design/README.md`](docs/system-design/README.md) — multiplayer P2P network
  architecture. Read the numbered files in order (`00-overview.md` →
  `07-risks-and-open-questions.md`); later files assume the decisions recorded in earlier ones.
  `03-architecture.md` is the one most future implementation work will need to revisit — it names
  the component inventory that everything else (`05`, `06`) cross-references.
- [`docs/game-design/README.md`](docs/game-design/README.md) — gameplay design (survival/
  crafting/factory-automation pillars, production-milestone progression) and the Rust/Bevy engine
  skeleton (plugin architecture, ECS component design, project layout, cross-platform build,
  milestone roadmap) an implementation agent should follow to start writing the actual game code.

## The big picture

Dwarven Engineering is a 2.5D factory-builder colony sim with co-op survival elements (8-16
players). The core architectural decision: **world sessions are peer-to-peer, hosted by one
player's own client** — not company-run game servers. This replaced an earlier company-hosted
World Server design mid-spec (the pivot and its consequences are recorded throughout, especially
`07-risks-and-open-questions.md`); when in doubt about which design is current, the peer-to-peer
one is, and the superseded company-hosted decisions are kept in `07` only for traceability, not as
a still-valid alternative.

What follows from the pivot:

- **Only four components are company-run**: Auth/Account Service, Relay/Rendezvous Service (NAT
  traversal + initial handshake only — it steps out of the path once a P2P connection is
  established), Patch/Content Delivery, and an async Analytics/Telemetry Worker. None of them sit
  in the ongoing gameplay path.
- **The Host Client is the sole authority for its World** — terrain, structures, colonist
  simulation, and factory/belt production all tick on the hosting player's own machine and persist
  to that machine's local disk. There is no company-run database or process in the gameplay data
  path.
- **The Host Client is an accepted single point of failure, by design, with no mitigation**: if it
  crashes or disconnects, the session ends for every connected player (no host migration). Don't
  propose host-migration/replication solutions without flagging that this is a deliberate,
  recorded non-goal (`06-fault-tolerance.md`), not an oversight.
- **Real-time gameplay traffic uses a custom UDP protocol** (not WebSocket/TCP) between host and
  connected players, chosen specifically to avoid head-of-line blocking under packet loss against
  the <50ms latency target — see `02-data-and-api.md` for the per-message-type reliable/unreliable
  split (`player.move` unreliable, `player.action` reliable, `world.state_delta` unreliable).
  Non-realtime account/profile/auth traffic is separate, REST/JSON.
- **No anti-cheat, no PvP, no cross-platform play, no mod marketplace** — confirmed non-goals, not
  open questions.

Three questions from the pivot are still open (not yet decided) and should be flagged rather than
silently assumed one way or the other if implementation work touches them: no company backup for
local save files, whether a relay-fallback (TURN-style) path is needed for restrictive NAT, and
Host Client autosave/reload UX. See `07-risks-and-open-questions.md` for full context.

## Architecture diagrams: JSON specs are the source of truth

Each `03-architecture-diagram-N.json` is the canonical source for its diagram. The ASCII block and
mermaid block in `03-architecture.md`, and the corresponding `03-architecture-diagram-N.svg`, are
all generated from that JSON via the `system-design-interview` skill's `render_diagram.py` — never
hand-edit any of the three rendered outputs directly. If a diagram needs to change, edit the JSON
spec and re-render all formats that exist for it, so the ASCII, mermaid, and SVG stay in sync with
each other and with the spec. If you find a mismatch between the JSON spec and any rendered output,
that's drift from a hand-edit — fix it by re-rendering, not by patching the output.
