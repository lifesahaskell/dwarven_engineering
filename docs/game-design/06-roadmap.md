# Dwarven Engineering — Implementation Roadmap

Milestone-driven backlog. Each milestone has a stated exit criterion that must be true (CI green,
plus any called-out decision points) before the next milestone starts.

## M0 — Skeleton

Empty Bevy app boots on all 3 OSes. Single-crate scaffold per `04-project-skeleton.md`. CI green
(fmt/clippy/test matrix per `05-cross-platform-build.md`). One trivial headless integration test
(`MinimalPlugins` smoke test) as the baseline check.

## M1 — World & Camera

`world_gen` spawns a chunk grid around the origin. 2.5D camera plugin. Single local player moves
via `input`'s intent events. No networking, no other players — this is intentionally solo-only.

## M2 — Inventory & Hand-Crafting

`ItemDatabase`/`RecipeDatabase` loaded from `assets/data/*.ron`. `Inventory` component with
pickup/drop. Tier 0/1 hand-craft only (per `01-progression-milestones.md`) — no stations that run
without the player present.

## M3 — Structures & Placement

`Structure` placement/validation. First buildable structures (storage, workbench, furnace)
consuming `Inventory` per `Recipe`.

## M4 — Factory & Belt Automation

`Factory`/`BeltSegment` simulation. `TechTree` gating begins. The `SimulationDetailLevel`
tick-degradation mechanism (`03-ecs-design.md`) is implemented as a first-class part of this
milestone — not retrofitted after a performance problem appears.

## M5 — Colonist AI

`Colonist`/`Needs`/`Job`, utility-AI job selection, pathfinding.

**Explicit decision point**: this is the first place a new external dependency (a pathfinding
crate) may be justified. Evaluate then, against whatever pathfinding need actually exists at that
point — don't pre-pick one now (see `07-open-questions.md`).

## M6 — Survival Threats

Day/night/weather cycle affecting `Needs` decay. First hostile creature type. Structure
damage/defense.

## M7 — Save/Load

Full state serialization and load-on-launch.

**Exit criterion explicitly includes resolving the autosave-interval/reload-UX open question with
the user first** (see `07-open-questions.md`) — this is not a call the implementation agent makes
alone. Treat that resolution as a literal checkbox on this milestone's exit criteria, not a detail
to fill in silently while implementing.

## M8 — Networking Integration

Wire the stubbed `networking` plugin (`02-bevy-architecture.md`) to the existing custom-UDP spec
in `system-design/02-data-and-api.md`: host-authoritative `world.state_delta` broadcast,
`player.move`/`player.action` ingestion feeding the same intent-event stream `input` already
produces since M1. Scoped explicitly as "hook into the existing spec," not a redesign of the
protocol or the ECS intent-event pattern.

## See also

- [`04-project-skeleton.md`](04-project-skeleton.md) — what M0's skeleton concretely is.
- [`system-design/04-scaling.md`](../system-design/04-scaling.md) — why M4 treats tick-degradation
  as first-class.
- [`system-design/06-fault-tolerance.md`](../system-design/06-fault-tolerance.md) — the autosave
  open question referenced at M7.
- [`07-open-questions.md`](07-open-questions.md) — decision points called out above.
