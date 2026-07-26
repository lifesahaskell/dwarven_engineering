# Dwarven Engineering — Game Design Open Questions

Nothing in `docs/game-design/` resolves any of these silently. They're carried forward as open
until the user decides them.

## Carried forward from `system-design/07-risks-and-open-questions.md`

1. **No company backup for local save files.** If the host's disk fails or a save corrupts,
   that world's progress is lost with no company-side recovery path. Needs a product decision:
   accept this as a player-owned-risk model, or add optional cloud save backup/sync. Relevant to
   `save_load` (see `04-project-skeleton.md`) but not resolved by it.

2. **Relay fallback for restrictive NAT/firewalls.** The design assumes the Relay/Rendezvous
   service can broker a direct P2P connection via NAT traversal, but some networks (symmetric NAT,
   restrictive firewalls) can't establish direct P2P at all. Needs a decision: is a full
   traffic-relaying fallback (TURN-style) in scope, or is "some players can't connect to some
   hosts" an accepted limitation? Relevant to M8 (`06-roadmap.md`) but not resolved by it.

3. **Host Client autosave/reload UX.** How often does the Host Client autosave locally, and what's
   the reload flow after a crash? **This blocks M7's exit criteria** (`06-roadmap.md`) — it must be
   resolved with the user before M7 is considered done, not decided unilaterally during
   implementation.

## New, game-design-local

4. **Milestone-tier default needs confirmation.** The Tier 0-5 production-milestone model in
   `01-progression-milestones.md` is a proposed default, not a signed-off design. Specific tier
   content (which items/recipes/structures belong at which tier) may need adjustment once actual
   gameplay balancing starts.

5. **Pathfinding-dependency decision point.** `colonist_ai` (M5, `06-roadmap.md`) will need some
   pathfinding approach for colonist movement. No crate or algorithm is chosen yet — this is an
   explicit decision point at M5, evaluated against the actual need at that time, not pre-picked
   during initial skeleton design.

## See also

- [`system-design/07-risks-and-open-questions.md`](../system-design/07-risks-and-open-questions.md)
  — full context for items 1-3, including the architecture pivot they originated from.
- [`06-roadmap.md`](06-roadmap.md) — where items 2, 3, and 5 gate specific milestones.
- [`01-progression-milestones.md`](01-progression-milestones.md) — item 4's source doc.
