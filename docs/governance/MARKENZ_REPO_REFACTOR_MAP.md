# Refactor Map: Gemini Universe → Markenz (Locked Stack, Rust Authority)
STATUS: BINDING
RULE: No big-bang rewrite. Phase-aligned incremental refactor with compilation gates.

## Guiding principle
We do NOT “convert TypeScript into authority.” Rust engine owns truth from Phase 0 onward.
Existing conceptual subsystems (Physics→Biology→Cognition, veto logic, transparency UI) are preserved in semantics but relocated to the locked stack and authority boundaries.

## Target repo layout (locked)
apps/
  engine/   Rust world authority (binary)
  server/   Node.js/TypeScript control plane (auth, RBAC, events, persistence adapters, ws)
  web/      React/TypeScript UI + WebGPU (observer/admin)
crates/
  world/       core state + tick loop + hashing
  physics/     deterministic physics, collision, movement
  biology/     physiology, hormones, nutrition, reproduction
  cognition/   perception, drives, learning, language (no LLM required)
  events/      event types, schema, serialization, validation
  persistence/ snapshotting, replay, postgres adapters (engine-side logic; server writes events)
tools/
  audits/   Python replay audits, anomaly detection, metrics, PDF report generation

## What gets moved/created (surgical ownership map)

### apps/engine (Rust)
- BootChain:
  - fixed timestep WorldLoop
  - deterministic scheduler (tick index authoritative)
- Authority pipeline:
  Perception/Context → Intent/Volition → BioVeto → PhysicsValidate → PolicyValidate → Commit
- Deterministic RNG + audit logs (engine-side)
- Hash checkpoints + snapshot write
- Markenz genesis content:
  - terrain seed, house, shed, tools, vehicles
  - agents Gem-D and Gem-K with reverence constraints

### crates/world (Rust)
- Canonical world state types (deterministic containers and ordering)
- Tick application / reducers
- State hashing canonicalization + checkpoint strategy

### crates/events (Rust + shared schema artifacts)
- Canonical InputEvent/ObservationEvent schema versioning
- Serialization rules for hashing and transport stability

### crates/persistence (Rust)
- Snapshot format, replay harness, hash equivalence verifier
- DB readers for replay (read-only path), while server remains DB writer for InputEvents

### apps/server (TypeScript)
- Auth:
  - Keycloak OIDC JWT verification via local JWKS
  - Authentik backup integration (Phase 9)
- RBAC enforcement:
  - observer/auditor/admin gating for InputEvents
- InputEvent ingestion:
  - schema validation, normalization, tick assignment rules (server never invents outcomes)
- Postgres writer:
  - append-only events with hash-chain
  - snapshot metadata storage (optional)
- Fanout:
  - WS broadcast of ObservationEvents received from engine

### apps/web (React)
- Observer mode:
  - deep inspection of all telemetry
  - replay viewer + time-travel debugger
- Command Centre:
  - admin-only input forms creating InputEvents
- WebGPU renderer:
  - renderer-only, consuming render packets derived from snapshots/telemetry

### tools/audits (Python)
- Replay audits: seed + event list => reproduce world_hash checkpoints
- Anomaly detection and determinism drift detection
- PDF audit exports

## Interface contracts (engine/server/web)

### Engine ↔ Server (authoritative boundary)
- Server sends ordered InputEvents only.
- Engine returns ObservationEvents + checkpoints only.
- Contract versioning:
  - event schema version in every payload
  - engine rejects mismatched versions deterministically
- Transport:
  - local/LAN only
  - WS or TCP for engine→server stream; HTTP/WS for server→engine event delivery
- Ordering:
  - server must provide a strict, canonical order per tick
  - engine also enforces order deterministically (e.g., by event id)

### Server ↔ Web
- Web never receives raw mutable state; only ObservationEvents + snapshot-derived payloads.
- Web submits InputEvents only through server with tokens.

## Data model migration plan (event log, snapshots, hashes)
- Phase 0:
  - Introduce new tables: events (append-only, hash-chain) + snapshots (tick, world_hash, payload pointer)
- Phase 1:
  - Engine produces world_hash checkpoints and snapshots; tools/audits validate replay
- Phase 2+:
  - Expand snapshot payload structure (world chunks, entities, inventories, bio/cog telemetry)
  - Maintain backward-compat schema versioning

## Incremental refactor steps aligned to phases
- Phase 0:
  - Stand up locked stack skeleton and persistence
  - Engine ticks + minimal genesis + telemetry
- Phase 1:
  - Deterministic kernel + replay harness
  - UI panels for tick/hash/event log/agent monologue
- Phase 2:
  - Replace rooms/zones with chunked terrain model
- Phase 3:
  - Move BioSys to crates/biology; wire BioVeto into authority pipeline
- Phase 4:
  - Move cognition stack to crates/cognition; deterministic NLG/NLU
- Phase 5–9:
  - Expand social/economy/governance/renderer/security with gates and audits

## Explicit removals / prohibitions
- No authoritative simulation in apps/server.
- No physics/biology/cognition state mutation in UI.
- No bypass channels (no “admin patch state” endpoints). Admin uses InputEvents only.
