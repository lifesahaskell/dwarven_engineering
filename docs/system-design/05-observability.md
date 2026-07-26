# Dwarven Engineering — Observability & Telemetry

## What changed from the company-hosted design

The previous design's highest-priority signal — per-world tick time on a company-run World
Server, alerted and paged on — no longer applies. The company doesn't operate the Host Client, so
there's nothing to page an on-call engineer about when a specific session's simulation slows
down; that's the host player's own hardware, not company infrastructure. Observability now
splits cleanly into two categories: RED/USE for the company's own services (still applies, lower
stakes than before), and client-reported telemetry (informational, not actionable via paging).

## RED metrics (Rate, Errors, Duration) — per company-run service

- **Auth/Account Service**: standard RED tracking (request rate, error rate, response duration).
- **Relay/Rendezvous Service**: rate = connection-brokering attempts/sec, errors = failed
  handshakes/NAT-traversal failures, duration = time to broker a connection. This is now the
  closest thing to a "gameplay-adjacent" SLO the company owns, since a slow/broken Relay directly
  delays players joining a session even though it's not in the ongoing gameplay path.
- **Patch/Content Delivery Service**: standard CDN RED metrics.
- **Analytics/Telemetry Worker**: RED tracked but non-critical — never allowed to apply
  backpressure to any client (per `03-architecture.md`).

## USE metrics (Utilization, Saturation, Errors) — per resource

- **Account/Profile DB, Analytics store**: standard connection pool/CPU/disk/replication-lag USE
  metrics — same as any conventional cloud service.
- **Relay/Rendezvous Service**: connection-handling capacity (concurrent brokering sessions),
  since a spike in players joining at once (e.g. after a content update) is its main saturation
  risk.
- **Host Client simulation performance**: no longer a company-operated USE metric — see client
  telemetry below for how the company gets *any* visibility into this without owning the
  infrastructure.

## Client-reported telemetry (replaces per-world dashboards)

Per-world dashboards from the earlier design don't make sense here — there's no company-run
per-session infrastructure to dashboard. Instead: Host Clients can opt in to reporting anonymized
performance telemetry (tick time, factory/colonist counts, frame time) to the
Analytics/Telemetry Worker, giving the company aggregate visibility into "how often do colonies
of size X start struggling" without operating or monitoring any individual session live. This is
informational for game-balance/performance-tuning decisions, not an operational SLO — nothing
here gets paged, since there's no company action to take on an individual struggling session.

## Logging and tracing

Client-side structured logs (tagged with an anonymized session id) for crash reports and
correlation of a specific player action to a reported bug — sent to the Analytics/Telemetry
Worker, not a company-run distributed trace across services (there's no multi-service gameplay
path left to trace; the Host Client is a single process).

## SLOs and alerting

- **Relay/Rendezvous Service**: p95 connection-brokering time under a target (e.g. a few
  seconds), error rate < 1% on handshake attempts — paging on sustained breach, since this is
  the one company-run component that directly affects a player's ability to join a session.
- **Auth/Account Service**: standard p95 latency / error-rate SLOs, paging on sustained breach.
- **Company service availability**: 99.9% for Auth, Relay/Rendezvous, and Patch Delivery
  (matches `01-requirements.md`) — there is no equivalent SLO for world-session availability,
  since that's the host player's own machine, not company infrastructure.
