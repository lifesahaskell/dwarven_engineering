# Dwarven Engineering — Requirements

## Functional requirements

- **World join/persistence** — players join a persistent world; world state (terrain changes,
  structures, colony state) is saved and reloaded across sessions.
- **Building/crafting** — players place structures and craft items/tools from resources; colony
  infrastructure grows over time.
- **Colonist/NPC simulation** — AI colonists (distinct from players) have needs, jobs, and
  schedules — a colony-sim simulation layer beyond direct player actions.
- **Environmental/creature threats** — weather, hostile creatures, and resource scarcity create
  survival pressure players must respond to.

## Read/write profile

Write-heavy, real-time — constant stream of position/state updates from all players and
simulated entities (colonists, creatures), typical of a real-time multiplayer game server.

## Non-functional requirements

- **Scale**: small co-op scale — 8-16 concurrent players per world session. Peer-to-peer means
  the company doesn't run or capacity-plan world instances at all; the scale question for
  company-owned infrastructure is concurrent Relay/Rendezvous connections and Auth throughput
  (total concurrent player population), not "how many worlds we host" — see `04-scaling.md`.
- **Latency**: <50ms for gameplay-critical updates (movement, combat/interaction sync), over the
  direct peer-to-peer path between host and connected players once the Relay/Rendezvous service
  has brokered the connection. The company doesn't control this path's network quality (it's
  peer-to-peer, not routed through company datacenters) — the target shapes the *protocol*
  design (custom UDP, per `02-data-and-api.md`), not a server placement decision.
- **Availability**: no company SLA on world-session availability — a world session is only as
  available as the host player's own machine and connection, which the company doesn't control
  or operate. Company-owned services (Auth, Relay/Rendezvous, Patch Delivery) do have their own
  availability targets: 99.9% (standard for lightweight, stateless cloud services), since these
  are conventional company-run infrastructure, unlike the P2P gameplay path.
- **Consistency**: strong consistency *within* a world session (the hosting player's client is
  the single authority — structures, inventory, colonist state all serialize through it); no
  cross-world consistency requirement since sessions are fully independent and ephemeral.
  Connected (non-host) players' client state may be momentarily stale/predicted and is
  reconciled against the host (standard client-prediction pattern for real-time action).
- **Security/compliance**: no compliance-driven constraints identified (no PvP, no
  cross-platform, no financial data in scope per `00-overview.md`). **No anti-cheat
  requirement** — confirmed not needed, since this is a cooperative, non-competitive game where
  a cheating player only affects their own world session, not other players' outcomes.
