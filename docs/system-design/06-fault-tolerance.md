# Dwarven Engineering — Fault Tolerance & Resilience

## What changed from the company-hosted design

The previous design's central concern — restarting a crashed company-run World Server from a
snapshot — no longer applies, since there's no company-run World Server. The Host Client is the
host player's own machine; the company can't detect its crash, restart it, or restore its state.
Fault tolerance now splits into: (a) what happens to a session when the Host Client fails, which
is a product/UX design question, not an infra one, and (b) fault tolerance for the company's own
services, which is conventional and lower-stakes than before.

## Per-component failure modes

- **Host Client (single authority per World session)**: a single point of failure by design —
  not mitigated, **accepted**. If the hosting player's client crashes, disconnects, or closes
  the game, **the session ends for everyone** (confirmed — no host migration). Connected players
  are dropped and must wait for the host to relaunch and resume from their local save. This is a
  deliberate simplicity choice: host migration (transferring authority to another peer mid-
  session) would require real-time state handoff and is significant added complexity for a
  cooperative, non-competitive game where "the host left, session's over for now" is an
  acceptable and easily-understood outcome to players.
- **Local save file (on Host Client's disk)**: persisted periodically by the Host Client itself,
  with **no company backup** — if the host's disk fails or the save is corrupted, that world's
  progress is lost unless the player has their own backup. This is a real player-facing risk the
  company doesn't mitigate by default; flagged in `07-risks-and-open-questions.md` as worth a
  product decision (e.g. optional cloud save sync) even though it's out of scope for now.
- **Auth/Account Service / Account/Profile DB**: unchanged in spirit from the earlier design —
  auth is checked at Relay/Rendezvous and host-handshake time, not re-validated mid-session, so
  an Auth outage doesn't disconnect already-playing players, only blocks new logins/joins.
- **Relay/Rendezvous Service**: if it's down, players can't set up *new* P2P connections (can't
  join a session or start one), but **already-connected sessions are entirely unaffected** —
  it's out of the gameplay path once brokering is done (per `03-architecture.md`), which is an
  even cleaner story than the earlier Fleet Manager's "not in the gameplay path" property, since
  there's no restart-detection role for it to lose either.
- **Analytics/Telemetry Worker**: can fail or fall behind with zero impact on gameplay, by
  design (async, non-blocking, per `03-architecture.md`).

## Retries, timeouts, backoff

- **Connecting Client → Relay/Rendezvous**: retried with backoff on a failed brokering attempt
  (e.g. transient NAT-traversal failure) — standard client-side retry, no company-side escalation
  needed since this doesn't affect anyone already in a session.
- **Connected Player → Host Client (P2P)**: client-side reconnect with backoff on disconnect —
  but note a reconnect only succeeds if the Host Client is still up; if the host itself is gone,
  reconnect attempts fail until the host relaunches (per the accepted "session ends" behavior
  above).

## Redundancy / bulkheads

- Each World session is its own bulkhead by construction — one host's crash or overloaded
  simulation cannot affect any other session, since there's no shared process or shared
  infrastructure between sessions at all (stronger isolation than the earlier bin-packed design,
  since sessions don't even share a host machine).
- Company-run services (Auth, Relay/Rendezvous, Patch Delivery, Analytics) are independent of
  each other and of any individual session — standard multi-instance redundant deployment,
  nothing session-specific to bulkhead.

## Degradation paths

- Host Client crashes/disconnects → session ends for everyone; no effect on any other session.
- Auth/Account Service down → already-connected sessions keep playing; new logins/joins blocked
  until it recovers.
- Relay/Rendezvous Service down → already-connected sessions keep playing; new sessions/joins
  blocked until it recovers.
- Analytics Worker down → zero player-facing impact; only telemetry/analytics data is delayed or
  lost.

## Single points of failure

- **Host Client** is an accepted SPOF per session, by design, with no mitigation beyond "the
  session ends and the host can relaunch" — replicating or migrating a peer-hosted authoritative
  simulation is out of scope given the genre (cooperative, non-competitive, small groups who
  understand "the host left").
- **Relay/Rendezvous Service** is a SPOF for new connection setup only, not for already-running
  sessions — standard redundant deployment (multi-instance) is sufficient.

## Disaster recovery (RTO/RPO)

- **RPO/RTO for a World session**: not applicable in the traditional sense — there is no company-
  operated recovery process. The host's own local save frequency determines their personal data-
  loss window on a crash, and "recovery" is simply relaunching the game — this is a client UX
  concern (e.g. autosave frequency, save-file rotation to guard against corruption), not a
  company infrastructure DR plan.
- **RTO/RPO for company-run services** (Auth, Relay/Rendezvous, Patch Delivery, Analytics):
  conventional cloud-service DR — standard managed redundancy is sufficient given these are
  stateless or low-write-volume services; no aggressive target needed since none of them hold
  session-critical state.
