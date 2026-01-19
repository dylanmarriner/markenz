# MARKENZ EXECUTION ROADMAP (Phase 0 → Phase 9)
STATUS: BINDING
UNIVERSE: Markenz
DETERMINISM: Hardcore (seed + ordered inputs + fixed tick schedule = identical replay)
AUTHORITY: Rust engine only (apps/engine). Everything else is a client.

## Global invariants (apply to every phase)
- Rust owns truth. The single-writer fixed-timestep loop in apps/engine is the ONLY authority that can mutate world state.
- TypeScript control plane (apps/server) may ONLY validate/auth/order inputs and persist immutable logs; it NEVER mutates world state.
- React UI (apps/web) is read-only by default; any action becomes an InputEvent submitted to server and logged.
- No mock data. No stub code. No TODO/FIXME placeholders. No “fake” mechanics. No “manual-only” acceptance.
- All external inputs are InputEvents (admin, chat, policy changes, injections). Tick-indexed and ordered deterministically.
- All randomness uses DeterministicRng streams (engine-side). Every draw is audit-logged (tick, subsystem, stream, callsite, value).
- No wall clock may influence simulation outcomes. Wall clock may schedule ticks but cannot enter state evolution.
- Stable ordering everywhere (entity iteration, event iteration, container iteration). No nondeterministic maps/sets in authority state.
- Cognition cannot mutate world directly. It produces Intent → (Volition) → BioVeto → PhysicsValidate → Commit (all inside engine).
- Replay tests must prove equality of world_hash checkpoints and event-hash-chain integrity.

## Phase 0 — Repo + Offline Stack Baseline Closure
Objective:
- Boot the full local stack offline: Postgres + Keycloak + Rust Engine + TS Server + Web UI.
- Establish append-only event store with tamper-evident hash chain.
- Prove deterministic tick progression, event ingestion, and telemetry fanout.

Deliverables:
- docker compose for offline stack: postgres, keycloak, server, engine, web.
- Keycloak realm import with seeded users + roles: admin (Dylan/Kirsty), observer.
- Postgres migrations: events + snapshots + integrity hash-chain fields.
- Engine boots fixed tick loop, reads events for tick T, emits ObservationEvents + world_hash checkpoints.
- Server:
  - Auth via Keycloak OIDC (JWT verification via JWKS fetched from local Keycloak; cached; no external network).
  - RBAC enforcement (observer cannot submit InputEvents).
  - Append-only event log writer with hash-chain.
  - WS fanout of ObservationEvents (engine → server → web).
- Web UI:
  - Login required.
  - Live tick + world_hash panel.
  - Event timeline viewer (read-only) and admin event sender (admin-only).

Acceptance:
- `docker compose up --build` succeeds on a clean machine with no external network.
- Keycloak login works; admin/observer roles enforced in UI and server.
- Events appended and verified by hash-chain endpoint.
- Engine ticks advance; UI shows tick and world_hash updating.
- A submitted event changes authoritative state (visible via telemetry) and is replayable.

Automated tests:
- Server: schema validation + hash-chain verifier unit tests.
- Engine: determinism harness (two-run + snapshot-run) produces identical world_hash sequence.
- End-to-end: a test script that boots stack, submits a deterministic event sequence, then runs replay verification.

Exit criteria:
- All services compile and start.
- Tests pass.
- Replay verification produces identical world_hash checkpoints for the same seed + same ordered InputEvents.

## Phase 1 — Deterministic Kernel + Event Sourcing + Replay Harness (MVP)
Objective:
- Lock the deterministic kernel: WorldLoop, DeterministicRng, canonical hashing, snapshotting, and replay equality.
- Establish the authoritative pipeline: InputEvent → Validate → Apply → Commit → Observe.

Deliverables:
- Engine:
  - Fixed-timestep scheduler.
  - DeterministicRng with subsystem streams (physics/environment/biology/cognition/reproduction/governance).
  - Canonical world_hash and per-tick checkpoint log.
  - Snapshot write/read with replay-from-snapshot equality proof.
  - Minimal Markenz genesis (seeded terrain chunk(s), house, shed, tools, vehicles, Gem-D, Gem-K).
- Server:
  - Append-only events + hash-chain + integrity endpoint.
  - Ordered event delivery to engine (tick-indexed).
- Web:
  - Live tick + world_hash + integrity status.
  - Event log + per-agent vitals + inner monologue stream.
  - Admin event sender (weather/resource injection/chat) producing InputEvents only.

Acceptance:
- Determinism tests pass:
  - same seed + same ordered InputEvents => identical world_hash sequence
  - snapshot replay equals full replay world_hash sequence
- UI shows live tick, event log, agent vitals, agent inner monologue via WS.
- Hash-chain verification endpoint returns ok for untouched DB.

Automated tests:
- Engine determinism replay test (seed + InputEvents => stable world_hash).
- Snapshot equivalence test.
- Hash-chain verification test (server-side).

Exit criteria:
- Determinism is proven by automated replay tests.
- No authority leakage (server/web cannot mutate state outside InputEvents).

## Phase 2 — Markenz World Model v1 (Terrain + Entities + Inventory + House/Shed/Tools/Vehicles)
Objective:
- Implement scalable deterministic world representation and real mechanics tied to time/energy/tool constraints.

Deliverables:
- Terrain: chunked grid/heightmap (seeded), basic biomes v1.
- Entities: structures, tools, vehicles, resources, inventories, ownership tags.
- Mechanics: gather/mine/build/move with reach, tool requirements, energy/time costs, collision constraints.

Acceptance:
- Real mechanics: a gather action changes world + inventory and consumes energy/time; blocked if constraints fail.
- Logs show full causality chain (per tick):
  perception → intent → veto reason (if any) → physics validation → commit → updated world_hash.

Automated tests:
- Deterministic action tests (same seed + same InputEvents => identical inventories and world_hash).

Exit criteria:
- World model v1 complete and deterministic under replay.

## Phase 3 — Embodied Biology v1 (Metabolism + Hydration + Sleep + Hormones + Somatics)
Objective:
- Implement causal physiology and veto logic that prevents impossible actions.

Deliverables:
- Metabolism/macros, hydration/electrolytes, thermoregulation, circadian rhythm/sleep.
- Nutrition depth: vitamins/minerals with deficiency/toxicity thresholds.
- Injury/healing, infection risk, immune response.
- Endocrine axes (HPA/HPG) and neurochemistry proxies (dopamine/serotonin/norepinephrine).
- BioVeto emits stable, logged reasons.

Acceptance:
- Agents starve/dehydrate/sleep; hormones respond; action veto blocks unsafe actions with reasons.
- All physiology variables observable and evented.

Automated tests:
- Deterministic physiology progression under fixed tick.
- Veto reasons stable and traceable.

Exit criteria:
- Biology fidelity v1 complete and replay-identical.

## Phase 4 — Offline Cognition Engine (No LLM)
Objective:
- Deterministic cognition, planning, and English language without LLM dependency.

Deliverables:
- Perception gating → drives → intent selection → action queue.
- Deterministic planner (GOAP/HTN) with skill learning.
- Deterministic NLG/NLU: grammar/templates + lexicon tables + grounding to entities/memories.
- Inner monologue continuous, always-on, fully logged.

Acceptance:
- Agents produce coherent English utterances and plans without any LLM.
- Same inputs => same utterances, plans, and learning updates.

Automated tests:
- Plan selection deterministic.
- NLG deterministic for same belief/memory/emotion state.

Exit criteria:
- Cognition and language are fully functional offline and replay-identical.

## Phase 5 — Social Dynamics + Multi-Agent Scaling
Objective:
- Emergent social systems + performance scaling without determinism drift.

Deliverables:
- Relationship graph, attachment styles, trust/bonding/conflict.
- Gossip/reputation system; culture metrics tracking.
- Deterministic throttling for telemetry (must not affect outcomes).

Acceptance:
- Dozens of agents run on one machine; no nondeterministic drift.
- Social dynamics observable and replayable.

Automated tests:
- Multi-agent replay equality (world_hash across runs).
- Performance regression tests (tick stability).

Exit criteria:
- Social + scaling stable, deterministic, auditable.

## Phase 6 — Reproduction + Genetics (Double Helix)
Objective:
- Population growth via deterministic genetics and reproductive biology.

Deliverables:
- Genome: double helix encoding, recombination mapping, bounded mutation (policy-controlled).
- Reproduction pipeline: consent + readiness + intercourse → probabilistic conception (seeded) → gestation stages → birth.
- Offspring inherits phenotype priors; lineage/family trees; heritability reports.
- Genetic disorder toggles as policy modules.

Acceptance:
- Birth deterministically produces the same child genome/phenotype given same parents + seed + event stream.
- Lineage viewer shows correct inheritance.

Automated tests:
- Known seed yields expected child genome hash.
- Recombination determinism test.

Exit criteria:
- Genetics + reproduction deterministic, policy-constrained, fully observable.

## Phase 7 — Governance + Economy Tools
Objective:
- Deterministic governance and economy constraints enforced by authority.

Deliverables:
- Property/ownership, markets/quotas, dispute resolution.
- Elections, policy proposals, law modules, courts/penalties.
- All governance decisions logged and replayable.

Acceptance:
- Governance rules constrain actions and appear as veto causes.
- Economy metrics stable on replay.

Automated tests:
- Law enforcement determinism.
- Governance event replay equality.

Exit criteria:
- Governance + economy deterministic and auditable.

## Phase 8 — WebGPU Renderer + Visualization Upgrade
Objective:
- Professional visualization derived from authoritative snapshots (renderer never authoritative).

Deliverables:
- WebGPU renderer reading render packets derived from snapshots.
- Deterministic render packet generation (hashable, stable for a given snapshot).
- Multi-monitor layouts, filters, search, deep inspection panels.

Acceptance:
- Visual matches authoritative state.
- Render packets stable and hashable for given snapshot.

Automated tests:
- Render packet hash stable for a known snapshot.

Exit criteria:
- Renderer integrated, safe, non-authoritative.

## Phase 9 — Security + Integrity Hardening
Objective:
- Harden identity, encryption, and audit integrity without compromising offline-first operation.

Deliverables:
- Keycloak primary with WebAuthn/passkeys fully enabled offline.
- Authentik backup IdP wired (failover plan) without authority leakage.
- Encryption at rest:
  - DB encryption (disk-level) + application envelope encryption for sensitive server-side metadata/secrets.
- Tamper-evident log verification UI explorer + API.
- Immutable audit logs for auth/admin activity.

Acceptance:
- Tamper detection test fails when any event row is altered.
- Passkeys work offline on LAN.
- Secrets never in repo; secure defaults.

Automated tests:
- Tamper detection.
- RBAC enforcement.
- Crypto envelope tests.
- Replay + integrity still pass.

Exit criteria:
- Security hardened, determinism intact, audits complete.
