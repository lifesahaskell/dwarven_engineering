# Dwarven Engineering — Overview

## Problem statement

Dwarven Engineering is a 2.5D factory-builder colony sim with online co-op survival elements.
Players join a persistent world hosted peer-to-peer by one of the players (with a lightweight
cloud relay/rendezvous service for connection setup, not gameplay traffic) and build, craft, and
survive together, with the world and colony state persisting across sessions on the host
player's own machine. The differentiator is gameplay depth (colony sim complexity layered on
survival/crafting/automated factory production), not infrastructure novelty — the P2P
architecture exists to support that gameplay simply and cheaply, not to be the product's hook.
Not a competitive game — no anti-cheat requirement (see `01-requirements.md`).

## Actors

- **Players** — join a shared persistent world in co-op groups, build/craft/manage a colony,
  survive environmental and creature threats. The only actor type in scope; no separate
  server-host/admin or live-ops actor role is modeled here (see non-goals and open questions).

## Why now

The bet is on gameplay mechanics (colony sim depth combined with survival/crafting and
persistence), not a novel infrastructure approach — competitors already do persistent worlds;
this system needs to support that pattern well, not reinvent it.

## Non-goals

- **No PvP/raiding** — purely cooperative; no player-vs-player combat or base destruction by
  other players.
- **No cross-platform play** — single platform at launch, no console/mobile cross-play
  infrastructure.
- **No user-generated mod marketplace** — mod support, if any, has no distribution/marketplace
  infrastructure in this design.
