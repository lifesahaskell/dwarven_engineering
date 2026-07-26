# Dwarven Engineering — Game Design & Engine Skeleton

This doc set extends [`docs/system-design/`](../system-design/README.md) — it does **not**
re-decide anything already settled there (multiplayer P2P architecture, entities, fault
tolerance). For anything networking/entity/fault-tolerance related, `system-design/00-07` is
already-decided fact; these docs cross-link into it rather than duplicating or re-litigating it.

Where `system-design/` answers "how does the multiplayer architecture work?", this set answers
"what does the player actually do, and how is the Rust/Bevy client built to deliver it?"

Read in this order:

1. [`00-overview.md`](00-overview.md) — core gameplay pillars (survival, hand-crafting, colony
   building, factory automation) and the core loop that ties them together
2. [`01-progression-milestones.md`](01-progression-milestones.md) — the production-milestone tier
   model (Tier 0-5), the game's core progression indicator. **Proposed default, needs
   confirmation.**
3. [`02-bevy-architecture.md`](02-bevy-architecture.md) — engine choice (Bevy), plugin boundaries,
   schedule design, and the networking stub attach point
4. [`03-ecs-design.md`](03-ecs-design.md) — concrete Components/Resources/SystemSets, mapped from
   the `system-design/02-data-and-api.md` entity list
5. [`04-project-skeleton.md`](04-project-skeleton.md) — single-crate recommendation, folder
   layout, `Cargo.toml` sketch
6. [`05-cross-platform-build.md`](05-cross-platform-build.md) — Linux/Windows/macOS build
   specifics, CI matrix, save/asset path handling
7. [`06-roadmap.md`](06-roadmap.md) — milestone-driven implementation backlog (M0-M8) with exit
   criteria
8. [`07-open-questions.md`](07-open-questions.md) — everything still open; nothing above resolves
   these silently

**Status**: documentation only. No `Cargo.toml`/`src/` exists yet — scaffolding the actual crate
described in `04-project-skeleton.md` is a follow-up implementation step, not part of this doc set.
