# Dwarven Engineering — Production Milestone Progression

> **PROPOSED DEFAULT — needs the user's confirmation or adjustment.** This tier model has not
> been signed off; it's a concrete starting point so implementation isn't blocked, not a settled
> design decision. Treat any implementation of tier-gated content as provisional until confirmed.

## Why milestones instead of levels

See `00-overview.md` for the rationale — progression is measured by which production chain the
colony has actually built, not by accumulated XP. Each tier below is a checkpoint defined by what
becomes buildable, not by a numeric threshold the player grinds toward.

## Tier model

| Tier | Name | Unlocks | Automation level |
|---|---|---|---|
| 0 | Hand Tools | Axe, pickaxe, campfire, basic shelter | None — manual harvesting only |
| 1 | Workshop | Workbench, furnace/smelter, basic storage, tool upgrades | None — player-operated stations |
| 2 | Early Automation | First `Factory` entities + belts, single-input chains (ore → ingot) | First automation; colonist haulers introduced |
| 3 | Powered Production | Power grid (generator structures), multi-stage recipes, colonist job specialization (miner/farmer/crafter), first defensive structures | Powered, single-line automation |
| 4 | Integrated Complex | Multi-input assemblers, logistics beyond simple belts, housing tiers affecting colonist Needs, escalated creature/environmental threats | Networked, multi-line automation |
| 5 | End-game Automation Goal | Capstone item requiring the full tech tree | Full factory complex — the "you've built a real factory" milestone marker; sandbox play continues after |

### Tier 0 — Hand Tools

Manual harvesting from raw terrain only; no crafting stations exist yet. This is the opening
survival pressure test: the player must gather enough to reach Tier 1 before environmental/
creature threats (per `system-design/01-requirements.md`) become a real risk.

### Tier 1 — Workshop

Player-operated crafting stations exist (workbench, furnace/smelter) but nothing runs without the
player standing there. Smelting unlocks metal tools/parts; basic storage lets the colony
accumulate a surplus for the first time.

### Tier 2 — Early Automation

The pivot point of the whole progression model: the first `Factory` entity and belt segment exist.
Chains are still single-input (e.g. ore in, ingot out). Colonist haulers are introduced here —
the first case of a colonist doing a job a player used to do by hand.

### Tier 3 — Powered Production

Introduces a power grid, so factories have an uptime constraint for the first time (not just an
input-item constraint). Multi-stage recipes appear (chains of more than one Factory). Colonist
job specialization begins (miner/farmer/crafter roles), and the colony can build its first
defensive structures — automation and defense unlock together, since a growing production
footprint is also a growing threat surface.

### Tier 4 — Integrated Complex

Multi-input assemblers (recipes needing more than one input line to converge) and logistics
beyond simple belts (see `07-open-questions.md` — the exact mechanism, e.g. carts/pipes, is not
yet decided). Housing tiers start affecting colonist `Needs` directly, and environmental/creature
threats escalate to match the colony's now-substantial footprint.

### Tier 5 — End-game Automation Goal

A single capstone item whose recipe chain requires the entire tech tree unlocked — the explicit
"you've built a real factory" marker. Reaching it isn't a hard game-over; sandbox play continues,
same as the multiplayer session model has no forced end condition.

## Implementation hook: `milestone_tier` on `TechNode`

Each `TechNode` (see `system-design/02-data-and-api.md` and `03-ecs-design.md`) carries a
`milestone_tier: MilestoneTier` field, so `factory_sim`/`colonist_ai`/`structures` can gate
content by tier as a queryable property rather than hand-checking prose rules per system. This is
the one piece of this tier model that should be treated as fixed regardless of tier-content
adjustments: whatever the final tier list turns out to be, it should be attached to `TechNode`
this way.
