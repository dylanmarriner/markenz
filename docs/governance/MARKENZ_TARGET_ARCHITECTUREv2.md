# MARKENZ TARGET ARCHITECTURE v2
(Offline-first, Deterministic, Fully Transparent)

STATUS: BINDING  
AUTHORITY: Rust engine only (`apps/engine`)

---

## 1. Locked Services and Roles

### apps/engine (Rust — World Authority)
- Single-writer, fixed-timestep deterministic loop.
- Owns **all** world state and mutations.
- Enforces physics, biology, cognition, genetics, economy, and governance.
- Produces:
  - ObservationEvents
  - State diffs
  - Snapshots
  - `world_hash` checkpoints
- Accepts **only** ordered `InputEvents`.

### apps/server (Node.js + TypeScript — Control Plane)
- Identity, authentication, authorization (RBAC).
- InputEvent validation, normalization, ordering.
- Append-only persistence adapter (event log + hash chain).
- WebSocket fanout of ObservationEvents.
- **Never** computes or mutates world state.

### apps/web (React + TypeScript — Observer / Command Centre)
- Read-only by default.
- Visualization, inspection, replay, time-travel.
- Admin actions emit `InputEvents` only.
- WebGPU renderer consumes snapshot-derived render packets only.

### infra/postgres
- Append-only authoritative log storage:
  - `input_events`
  - `observation_events`
  - `snapshots`
  - `hash_checkpoints`
- No UPDATE / DELETE on immutable tables.

### infra/keycloak
- Primary IdP.
- OIDC + WebAuthn/passkeys.
- Offline-capable (local realm + JWKS cache).

### infra/authentik
- Backup IdP (Phase 9).
- Failover only, never authoritative.

### optional/ollama
- Optional cognition assist plugin.
- Never authoritative.
- System must run 100% without it.

---

## 2. Authority Boundaries (Non-Negotiable)

### Engine
- Owns:
  - World state
  - Tick advancement
  - RNG
  - All rule enforcement
  - All commits
- Rejects invalid or illegal InputEvents deterministically.

### Server
- Owns:
  - Auth
  - RBAC
  - Event validation
  - Persistence
- Cannot:
  - Patch state
  - Override outcomes
  - “Fix” determinism

### Web
- Owns:
  - Rendering
  - Inspection
  - Operator tooling
- Cannot:
  - Mutate state
  - Bypass RBAC
  - Influence ordering

---

## 3. Event Pipeline (End-to-End)

1. **Web → Server**
   - User/admin submits InputEvent request.
   - Token attached.

2. **Server**
   - Verify JWT via local JWKS.
   - Enforce RBAC.
   - Validate schema.
   - Normalize into canonical InputEvent.
   - Append to Postgres with hash-chain.

3. **Engine**
   - Reads ordered InputEvents for tick T.
   - Applies authoritative pipeline:
     Perception → Intent → Volition → BioVeto → PhysicsValidate → PolicyValidate → Commit
   - Emits:
     - ObservationEvents
     - State diffs
     - `world_hash`
   - Writes snapshots periodically.

4. **Server → Web**
   - WS fanout of ObservationEvents.

5. **tools/audits**
   - Offline replay from seed + event log.
   - Hash verification.
   - Anomaly detection.
   - PDF audit exports.

---

## 4. Determinism Strategy (Hard Rules)

### Time
- Simulation time is tick-indexed.
- Fixed dt.
- Wall clock never enters state evolution.

### RNG
- Root seed → deterministic subsystem streams.
- Streams separated by concern:
  - Physics
  - Environment
  - Biology
  - Cognition
  - Genetics
  - Governance
- Every draw audit-logged:
  `{ tick, subsystem, stream, callsite, value }`

### Ordering
- Deterministic ordering for:
  - Entities
  - Events
  - Containers
- No unordered maps/sets in authority state.

### Hashing
- Canonical serialization for:
  - World state
  - Render packets
- `world_hash` checkpoint every N ticks.

### Replay
- Seed + ordered InputEvents must reproduce identical hash sequence.
- Snapshot replay must equal full replay.

---

## 5. World Representation

### Chosen Model
- Chunked deterministic grid / heightmap.
- Seeded generation.
- Stable chunk coordinates.

### Navigation
- Grid-based deterministic pathing (early phases).
- Deterministic navmesh generation allowed later.

### Physics
- Authority uses deterministic math only:
  - Fixed-point or constrained integer math.
- Floating-point allowed **only** in renderer.

---

## 6. Biology, Genetics, Cognition Boundaries

### crates/biology
- Physiology
- Endocrine axes
- Immune response
- Nutrition
- Injury / healing
- Reproduction stages
- Emits:
  - BioTelemetry
  - BioVeto(reason)
  - HormoneUpdates
  - PregnancyStageEvents

### crates/genetics
- Double-helix genome
- Recombination
- Mutation (policy bounded)
- Phenotype expression

### crates/cognition
- Perception encoding
- Drives and emotion
- Deterministic planning
- Learning and memory
- Deterministic language (templates + lexicon)
- Emits:
  - ThoughtStream
  - SpeechEvents
  - LearningUpdates
  - MemoryLogs

### crates/world
- Canonical world state
- Tick reducers
- Hashing + snapshots

### crates/physics
- Movement
- Collision
- Constraints

### crates/persistence
- Snapshot format
- Replay harness
- Hash verification

---

## 7. Governance and Policy Enforcement

- Laws and policies are deterministic modules inside engine.
- Policies evaluated during commit pipeline.
- Enforcement produces:
  - Veto reasons
  - Court events
  - Penalties
- Policy changes only via InputEvents.

---

## 8. Security and Identity Integration

### Server-Side Only
- OIDC verification.
- Role mapping.
- Session management.

### Engine
- Does not trust identity directly.
- Trusts only RBAC-validated InputEvents.
- Still enforces legality independently.

### Roles
- Observer: read-only
- Auditor: read-only + export
- Admin: InputEvents only, no puppeting

---

## 9. Observability (Total Transparency)

### Per-Tick
- State diffs
- Event traces
- Hash checkpoints

### Per-Agent
- Perceptions
- Hormones
- Vitals
- Emotions
- Somatic sensations
- Thoughts
- Speech
- Intent queues
- Memory reads/writes
- Learning updates

### World
- Physics
- Resources
- Weather
- Economy
- Governance

### Tooling
- Replay viewer
- Time-travel debugger
- Diff heatmaps
- Causality graph

### Exports
- tools/audits generate PDF audit reports from immutable logs
