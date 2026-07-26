# Dwarven Engineering — Core Entities & API/Interface Design

## Core entities

- **Player** — account/identity, persists across worlds and sessions.
- **PlayerCharacter** — in-world character state (position, inventory, stats) for a Player
  within a specific World.
- **Session** — an active peer-to-peer connection between a Player's client and a hosting
  Player's client, brokered by the Relay/Rendezvous service.
- **World** — a persistent world instance, owned entirely by the hosting player's client, which
  is the sole authority for that World's state and persists it to the host's local disk. This is
  the consistency boundary (strong within a World session, no cross-World consistency needed);
  it is not company-hosted state.
- **Chunk** — a spatial subdivision of a World's terrain, used for streaming terrain to clients
  and scoping simulation/interest management.
- **Structure** — a placed building/construction within a World, owned by the hosting player's
  client.
- **Colonist** — an AI NPC entity with needs/jobs, simulated on the hosting player's client
  alongside the rest of the World (per the ownership decision below) — not a separate service.
- **Job** — a task assigned to a Colonist (or player-directed), driving Colonist behavior.
- **Need** — a Colonist's internal state (hunger, rest, etc.) that drives Job selection.
- **Item** — an item definition (type, stack size, etc.).
- **Inventory** — a container of Items, attached to a PlayerCharacter, Structure, or Colonist.
- **Recipe** — a crafting/production transformation from input Items to output Items.
- **Factory** (production building) + **TechNode** (tech tree) — automated production chains:
  Factories consume/produce Items via Recipes, connected by belt/pipe-style logistics that move
  Items between Factories automatically. TechNodes unlock new Factories/Recipes as the colony
  progresses. This automated item-flow simulation is a significant per-tick simulation load —
  flagged explicitly for `04-scaling.md`, since it scales with colony complexity, not just
  player count.

## Ownership

- **World/Chunk/Structure**: the hosting player's client owns all state in memory for its
  World, persisting periodically to local disk. There is no company-run server process and no
  shared DB — a DB round-trip per gameplay action would conflict with the <50ms latency target
  in `01-requirements.md` regardless, but here it's moot: there's no company infrastructure in
  the gameplay data path at all once a session is connected.
- **Colonist simulation**: runs on the hosting player's client alongside the rest of the World
  simulation (not a separate service) — avoids a network sync boundary for a tightly-coupled
  simulation, at the cost of colonist AI competing for the same tick budget as everything else
  on that player's hardware (see `04-scaling.md` for the implication of this alongside
  Factory/belt simulation — and note this is now the host's own hardware budget, not a
  company-scaled resource).

## API / interface design

**Real-time gameplay protocol**: **custom UDP protocol** — resolved (previously WebSocket, see
`07-risks-and-open-questions.md`). Switched to actually guarantee the <50ms target in
`01-requirements.md`: TCP head-of-line blocking under packet loss would stall delivery of
everything queued behind a single dropped packet, which a custom UDP protocol avoids by
retransmitting/reordering only what's actually needed (e.g. reliable delivery for
`player.action`, unreliable/latest-wins for `player.move` and `world.state_delta`). This is more
implementation work than WebSocket but removes the latency tension entirely rather than
accepting it as a launch risk.

**Meta/non-realtime API**: REST/JSON for account, auth, and player profile — separate from the
UDP peer-to-peer gameplay connection used once a player joins a hosted world.

**World discovery / connection setup**: peer-to-peer via invite code, brokered by a lightweight
Relay/Rendezvous service (NAT traversal + initial handshake) — not a full world directory
service, since players connect to a specific host's session by invite code, not by browsing
available worlds. Once the Relay/Rendezvous service has brokered the connection, gameplay
traffic flows directly peer-to-peer (host ↔ each connected player), not through company
infrastructure.

**Auth**: standard account auth (e.g. OAuth2/session token) via the REST API issues a token the
client presents to the Relay/Rendezvous service and in the UDP handshake with the host, which
validates it before admitting the player. Sessions are not re-validated after connect — see the
accepted tradeoff in `06-fault-tolerance.md`.

**Versioning**: UDP protocol version exchanged in the connection handshake; REST API versioned
via URL path (`/v1/...`).

### Key operations

| Operation | Style | Payload (conceptual) | Notes |
|---|---|---|---|
| `POST /v1/auth/login` | sync REST | credentials → session token | Issued once, reused for Relay/host handshake |
| `GET /v1/players/{id}/profile` | sync REST | player id → profile, character list | Non-realtime |
| Relay broker request | sync REST/RPC | session token, invite code → host connection info | Relay/Rendezvous service resolves NAT traversal, hands the connecting client what it needs to reach the host directly |
| UDP connect + handshake (direct, peer-to-peer) | connection-oriented | session token, protocol version → accept/reject | Gate for joining the host's session once the Relay has brokered the connection; not re-checked after accept |
| `player.move` | unreliable UDP (peer-to-peer) | position/velocity delta | High-frequency, tick-rate bound; latest-wins, no retransmission needed |
| `player.action` | reliable UDP (peer-to-peer) | action type + target (build, craft, interact) | Host-authoritative — client predicts, host corrects; retransmitted until acked, since a dropped build/craft action can't just be superseded like a position update |
| `world.state_delta` | unreliable UDP (peer-to-peer, host→client) | changed entities since last tick | Sent per-tick to each connected player, scoped to their Chunk interest area; next tick's delta supersedes a lost one |

Idempotency is less relevant here than in a request/response API — the authoritative server
resolves conflicting/duplicate client inputs each tick rather than relying on client-supplied
idempotency keys. The reliable/unreliable split above is the UDP-specific equivalent: pick
per-message-type delivery guarantees instead of get a single TCP-wide guarantee for everything.

