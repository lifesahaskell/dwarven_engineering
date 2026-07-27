# CLAUDE.md

Dwarven Engineering is a 2.5D factory-builder colony sim with co-op survival elements (8-16
players), built as a single-crate Bevy app.

## Before committing

Run `scripts/run_checks.sh`; it mirrors CI exactly, and lint has to pass, not just tests. To run
the checks individually, copy the commands out of that script rather than approximating them —
the bare `rustfmt` binary on hand-picked files won't match `cargo fmt --all --check`, and a bare
`cargo clippy` misses test/bench targets and won't fail on warnings.

## Reading the spec

Read `docs/system-design/` before `docs/game-design/` — the latter assumes the former's decisions
and cross-links into it rather than repeating them. Within each set, start at `README.md` and read
the numbered files in order; later files assume the decisions recorded in earlier ones.
`03-architecture.md` names the component inventory that `05` and `06` cross-reference, and is what
most future implementation work needs to revisit.

Read the spec before writing implementation code — the constraints below determine how any
client/service code has to be structured.

## Networking: spec'd, not built

There is no netcode yet — networking is M8, after colonist AI, survival, and save/load. Before
writing any, read `docs/system-design/` in full: the custom-UDP protocol and its per-message
reliable/unreliable split (`02-data-and-api.md`) and the four company-run components
(`03-architecture.md`) are already decided, not open for redesign.

Three of those decisions constrain work happening now:

- **Sessions are peer-to-peer, hosted by a player's own client**, not company-run servers. This
  replaced a company-hosted World Server design mid-spec; when docs seem to disagree, peer-to-peer
  is current, and the superseded decisions survive in `07-risks-and-open-questions.md` for
  traceability only, not as a live alternative.
- **The Host Client is the sole authority and persists its World to local disk**, with no
  company-run database or process in the gameplay data path. M7 save/load has to assume that.
- **Host Client failure ending the session for everyone is a deliberate, recorded non-goal**
  (`06-fault-tolerance.md`), not an oversight. Don't propose host migration or replication without
  flagging that first.

Confirmed non-goals, not open questions: no anti-cheat, no PvP, no cross-platform play, no mod
marketplace. Still genuinely open, and worth flagging rather than assuming either way: company
backup for local save files, a TURN-style relay fallback for restrictive NAT, and Host Client
autosave/reload UX — all in `07-risks-and-open-questions.md`.

## Architecture diagrams: the JSON spec is the source of truth

Each `03-architecture-diagram-N.json` is canonical for its diagram. The ASCII block and mermaid
block in `03-architecture.md`, and the matching `03-architecture-diagram-N.svg`, are all generated
from that JSON by the `system-design-interview` skill's `render_diagram.py` — never hand-edit a
rendered output. To change a diagram, edit the JSON and re-render every format that exists for it
so all of them stay in sync. A mismatch between the JSON and a rendered output is drift from a
hand-edit: fix it by re-rendering, not by patching the output.
