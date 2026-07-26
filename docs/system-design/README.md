# Dwarven Engineering — System Design Spec

A 2.5D factory-builder colony sim with online co-op survival elements: small groups (8-16
players) build, craft, and manage a colony together — including automated factory/tech-tree
production — hosted peer-to-peer by one of the players, with a lightweight company-run
Relay/Rendezvous service only for connection setup (not gameplay traffic). Non-competitive, no
anti-cheat requirement.

Read in this order:

1. [`00-overview.md`](00-overview.md) — problem statement, actors, why now, non-goals
2. [`01-requirements.md`](01-requirements.md) — functional + non-functional requirements
3. [`02-data-and-api.md`](02-data-and-api.md) — core entities, data model, API/interface design
4. [`03-architecture.md`](03-architecture.md) — components, ASCII architecture diagrams, CAP
   tradeoffs
5. [`04-scaling.md`](04-scaling.md) — scaling strategy and capacity estimates
6. [`05-observability.md`](05-observability.md) — RED/USE metrics, SLOs, per-world dashboards
7. [`06-fault-tolerance.md`](06-fault-tolerance.md) — failure modes, resilience, DR
8. [`07-risks-and-open-questions.md`](07-risks-and-open-questions.md) — assumptions and open
   questions requiring sign-off before implementation

**Status**: the architecture pivoted from company-hosted World Servers to peer-to-peer hosting
partway through this spec — `03-architecture.md`, `04-scaling.md`, `05-observability.md`, and
`06-fault-tolerance.md` reflect the current P2P design; superseded company-hosted decisions are
logged (not deleted) in `07-risks-and-open-questions.md` for traceability. Three new open
questions from the pivot remain: no company backup for local save files, whether a relay
fallback is needed for restrictive NAT, and Host Client autosave/reload UX — worth resolving
before implementation.
