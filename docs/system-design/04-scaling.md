# Dwarven Engineering — Scaling Strategy

## Per-session simulation scaling (Host Client)

Colonists, factories/belt-based item-flow simulation, and structures all tick on the hosting
player's own client — this is no longer a company scaling problem, since the company doesn't
operate or provision this compute at all (superseded from the earlier company-hosted design; see
`07-risks-and-open-questions.md`). The factory/belt simulation load flagged in
`02-data-and-api.md` is now entirely the host's own hardware budget.

What the company *can* still do: ship the client with the same tick-budget-management approach
previously designed for company-run servers (e.g. degrade non-critical simulation detail — far-
from-player factory tick frequency — before letting tick rate slip), just running on the host's
machine instead of company infrastructure. This is a client engineering concern, not a capacity
planning one.

## Company infrastructure scaling (Auth, Relay/Rendezvous, Patch Delivery, Analytics)

These are the only components the company operates and needs to capacity-plan:

- **Auth/Account Service**: scales with total concurrent player population (login/profile
  requests), not with number of active world sessions — standard stateless service, horizontally
  scalable, backed by the Account/Profile DB.
- **Relay/Rendezvous Service**: scales with *connection setup* rate (new sessions being joined),
  not with ongoing gameplay traffic — it steps out of the path once a P2P connection is
  established (per `03-architecture.md`), so its load profile is much lighter than a service
  that stayed in the gameplay path.
- **Patch/Content Delivery Service**: standard CDN-style scaling, independent of the other
  services.
- **Analytics/Telemetry Worker**: scales with total client population's event rate; async,
  never gameplay-blocking (per `03-architecture.md`).

None of these need the bin-packing/fleet-management scaling strategy from the earlier
company-hosted design — that entire concern (World Server Fleet Manager, host bin-packing,
per-world resource rebalancing) is superseded, not merely simplified.

## Regional placement

Relay/Rendezvous Service can run in one region at launch (or a small number of regions) since
its job is brief connection brokering, not sustained gameplay traffic — its own latency
sensitivity is much lower than the peer-to-peer gameplay path it sets up. Multi-region here is a
cost/reach tradeoff for the company's own infrastructure, not a factor in gameplay latency
(which is governed by the direct peer-to-peer path's own network route, outside company
control).

## Bottleneck watchlist

- **Host Client factory/belt simulation growth**: still the single biggest risk to a session's
  playability as a colony matures — no longer a company-side bottleneck to watch via internal
  dashboards, but a client-side performance ceiling worth surfacing to players (see
  `05-observability.md` for how visibility into this now works without company-run per-session
  telemetry).
- **Relay/Rendezvous connection-setup rate**: the one company-run component whose load actually
  scales with total active sessions (each new join is a brokering event) — worth watching as
  player population grows, even though it's a much lighter load profile than hosting gameplay
  itself would be.
