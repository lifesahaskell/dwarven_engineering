# Dwarven Engineering — Game Design Overview

## Scope of this doc set

This doc extends [`docs/system-design/`](../system-design/README.md), which already covers the
multiplayer peer-to-peer architecture (host-authoritative sessions, custom UDP protocol, company-
run Auth/Relay/Patch/Analytics services). This doc set does not revisit any of that — see
[`system-design/00-overview.md`](../system-design/00-overview.md) for genre, actors, and non-goals,
and [`system-design/01-requirements.md`](../system-design/01-requirements.md) for functional/non-
functional requirements. What follows here is *game design and engine architecture*: what the
player actually does, and how the Rust/Bevy client is structured to deliver it.

## Core gameplay pillars

Dwarven Engineering combines four pillars into one loop, not four separate modes:

1. **Open-world survival** — terrain, weather, day/night, and hostile creatures create ongoing
   pressure. The player must manage their own needs (hunger, rest) and their colony's exposure to
   environmental/creature threats.
2. **Hand-crafting** — the starting-tier gameplay verb: gather raw resources, craft tools and
   basic structures by hand, no automation yet.
3. **Colony building** — place structures, assign/observe AI colonists (distinct from players)
   with their own needs, jobs, and schedules — a simulation layer beyond direct player action.
4. **Factory automation** — the mid-to-late game verb: recipes chain through placed `Factory`
   structures connected by belt-style logistics, gated by a tech tree. This is the pillar that
   turns manual survival into a scaling production economy, and it's also the game's stated core
   progression indicator (see `01-progression-milestones.md`) — not levels or XP.

## The core loop

```
survive  -->  hand-craft  -->  build structures  -->  automate production
  ^                                                          |
  |                                                          v
  `------------------ unlock tech, survive harder threats <--'
```

Concretely: a new player starts with nothing but hand tools and must survive (pillar 1) long
enough to hand-craft (pillar 2) their first tools and shelter. Surviving buys time to place
structures and grow a colony (pillar 3). Once basic structures exist, factory automation (pillar
4) becomes available, and progressing the tech tree both unlocks new automation *and* raises the
threats the player must survive against — closing the loop rather than trivializing survival once
automation kicks in.

## Why production milestones, not XP

A traditional level/XP system rewards time spent, not systems built. This project's progression
indicator is instead **what production chain currently exists** — a hand-crafted axe, a smelting
line, a powered assembler network — because that's the actual skill/complexity the player has
built, and it's legible at a glance (walk through the colony, see the tier). See
`01-progression-milestones.md` for the concrete tier model.

## Multiplayer framing

Per `system-design/03-architecture.md`, the hosting player's client is the sole simulation
authority for a World session — every pillar above (terrain, colonists, factories) runs on that
one machine. This doc set's Bevy architecture (`02-bevy-architecture.md`) is written so the same
systems run identically whether the client is hosting solo or hosting for connected players — see
that file for why.
