# Dwarven Engineering — Risks, Assumptions & Open Questions

Collected from every section of the interview, including a major architecture pivot partway
through (company-hosted World Servers → peer-to-peer with a lightweight Relay/Rendezvous
service). None of these were silently resolved — each either has an explicit decision recorded,
is superseded by the pivot, or is still open and flagged as such.

## Superseded by the peer-to-peer pivot (no longer applicable)

These were resolved decisions in the earlier company-hosted design; the architecture change made
them moot rather than requiring a new resolution:

- **World Server Fleet Manager, bin-packing, host rebalancing policy** — there is no company-run
  World Server to provision, bin-pack, or rebalance. Superseded.
- **Fleet Manager restart RTO (under 2 minutes)** — there is no company-run process to restart.
  The equivalent question now is client-side autosave/reload UX, not an infra RTO — see the new
  open item below.
- **Per-world 99.5% availability SLA** — replaced by: no company SLA on session availability at
  all (it's the host player's own machine); company-run services now target 99.9% instead. See
  `01-requirements.md`.

## Resolved (from the original four open questions, still applicable post-pivot)

1. **WebSocket vs the <50ms latency target** — **RESOLVED**: custom UDP protocol, now used for
   the direct peer-to-peer path (host ↔ connected players) rather than client↔company-server.
   See `02-data-and-api.md`, `03-architecture.md`.

2. **Single-region vs the <50ms target** — **RESOLVED, and simplified by the pivot**: moot for
   gameplay traffic, since that's now a direct peer-to-peer path outside company control; the
   Relay/Rendezvous service (the one latency-sensitive company component left) can run
   single-region at launch since its role is brief connection brokering. See `04-scaling.md`.

3. **Session revalidation gap** — **RESOLVED/ACCEPTED AS-IS**: unchanged by the pivot — sessions
   are validated at connect time only. See `06-fault-tolerance.md`.

4. **Anti-cheat requirement** — **RESOLVED: not needed.** Confirmed non-competitive, cooperative
   game — a cheating player only affects their own world session. See `01-requirements.md`.

## New open questions from the peer-to-peer pivot

5. **No company backup for local save files** (`06-fault-tolerance.md`). If the host's disk
   fails or a save corrupts, that world's progress is lost with no company-side recovery path.
   **Needs a product decision**: accept this as a player-owned-risk model, or add optional cloud
   save backup/sync (which would reintroduce some company-run persistence infrastructure).

6. **Relay fallback for restrictive NAT/firewalls** (`02-data-and-api.md`, `03-architecture.md`).
   The design assumes the Relay/Rendezvous service can broker a direct P2P connection via NAT
   traversal, but some networks (symmetric NAT, restrictive firewalls) can't establish direct
   P2P at all and need a full traffic-relaying fallback (TURN-style), not just a brokering
   handshake. **Needs a decision**: is a fallback relay path in scope, or is "some players can't
   connect to some hosts" an accepted limitation?

7. **Host Client autosave/reload UX** (`06-fault-tolerance.md`). Replaces the old Fleet-Manager-
   restart-RTO question — now a client design question: how often does the Host Client autosave
   locally, and what's the reload flow after a crash? Not specified in this interview.

## Still open — assumptions carried over from before the pivot

8. **Bin-packing rebalancing policy** — superseded, see above (no longer applicable).

9. **Host-level blast radius from bin-packing** — superseded, see above (no longer applicable;
   each session is now isolated to its own host player's machine by construction).

## Non-goals confirmed (not risks, just recorded for traceability)

- No PvP/raiding between colonies (`00-overview.md`).
- No cross-platform play (`00-overview.md`).
- No user-generated mod marketplace (`00-overview.md`).
- No anti-cheat (`01-requirements.md`, confirmed this session).
