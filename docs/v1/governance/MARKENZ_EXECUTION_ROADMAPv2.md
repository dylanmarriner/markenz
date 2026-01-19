# MARKENZ EXECUTION ROADMAP (Phase 0 → Phase 9)

STATUS: BINDING  
UNIVERSE: Markenz  
DETERMINISM: Hardcore (seed + ordered inputs + fixed timestep = identical replay)  
AUTHORITY: Rust engine only (`apps/engine`). Everything else is a client.

---

## Global Invariants (Apply to Every Phase)

- **Rust owns truth.**
  - The single-writer, fixed-timestep loop in `apps/engine` is the ONLY component allowed to mutate world state.
- **TypeScript control plane (`apps/server`)**
  - May authenticate, authorize, validate, order, and persist immutable input logs.
  - MUST NEVER mutate world state or compute authoritative outcomes.
- **React UI (`apps/web`)**
  - Read-only by default.
  - Any user or admin action becomes an `InputEvent`, submitted via server, logged, and replayable.
- **PostgreSQL**
  - Append-only event log is the truth source.
  - Periodic snapshots accelerate replay.
  - Replay MUST reproduce identical `world_hash` checkpoints.
- **No mocks. No stubs. No TODO/FIXME.**
  - Every shipped feature must be real, exercised, and verifiable.
- **All external influence is an InputEvent**
  - Admin actions, chat, policy changes, injections.
  - Tick-indexed and deterministically ordered.
- **Deterministic RNG only**
  - Engine-side `DeterministicRng` streams.
  - Every draw audit-logged (tick, subsystem, stream, callsite, value).
- **No wall-clock influence**
  - Wall clock may schedule ticks but never enters state evolution.
- **Stable ordering everywhere**
  - Entity iteration, event iteration, container iteration.
  - No nondeterministic maps/sets in authority state.
- **Cognition cannot mutate the world**
  - Pipeline: Perception → Intent → Volition → BioVeto → PhysicsValidate → Commit (engine-only).
- **Replay is law**
  - Replay tests must prove equality of `world_hash` checkpoints and hash-chain integrity.

---

## Phase 0 — Repo + Offline Stack Baseline Closure

### Objective
- Boot the full stack completely offline.
- Establish immutable event sourcing, hash-chain integrity, and deterministic tick progression.

### Deliverables
- Offline stack boot:
  - Postgres
  - Keycloak
  - Rust engine
  - TypeScript server
  - React web UI
- Keycloak realm import:
  - Roles: `admin`, `observer`, `auditor`
- Postgres schema:
  - Append-only `input_events`
  - Derived `observation_events`
  - `snapshots`
  - Hash-chain fields
- Engine:
  - Fixed-timestep loop starts
  - Genesis snapshot emitted
  - Per-tick `world_hash` checkpoints
- Server:
  - OIDC auth via local Keycloak (JWKS cached locally)
  - RBAC enforcement
  - Append-only event writer with hash-chain
  - WebSocket fanout
- Web:
  - Login required
  - Live tick + `world_hash`
  - Read-only event timeline
  - Admin InputEvent sender (RBAC gated)

### Determinism Introduced
- Fixed timestep
- Canonical event ordering
- Hash-chain enforcement

### Exit Criteria
- `docker compose up --build` works with no internet
- Login + RBAC enforced
- Events logged immutably
- Engine ticks advance
- Submitted events replay identically

---

## Phase 1 — Deterministic Kernel + Replay Harness

### Objective
- Prove determinism formally via replay and snapshot equivalence.

### Deliverables
- Engine:
  - Deterministic scheduler
  - Subsystem RNG streams (physics, biology, cognition, environment, governance)
  - Canonical world hashing
  - Snapshot write/read
  - Replay-from-snapshot equality
  - Genesis world:
    - Markenz seed
    - House, shed, tools, vehicles
    - Agents: Gem-D, Gem-K
- Server:
  - Ordered event delivery guarantees
  - Integrity verification endpoint
- Web:
  - Hash-chain status panel
  - Agent vitals + inner monologue streams
  - Event log explorer

### Determinism Introduced
- Snapshot equivalence proof
- Cross-run hash equality

### Exit Criteria
- Same seed + same events → identical hash timeline
- Snapshot replay == full replay
- No authority leakage detected

---

## Phase 2 — World Representation v1 (Terrain + Entities)

### Objective
- Replace abstract world with deterministic spatial reality.

### Deliverables
- Chunked deterministic terrain
- Biomes v1
- Structures, tools, vehicles, inventories
- Real mechanics:
  - Move
  - Gather
  - Build
  - Mine
- Constraints:
  - Reach
  - Tools
  - Energy
  - Time
  - Collisions

### Exit Criteria
- Actions succeed/fail deterministically
- Full causality trace visible
- Replay identical

---

## Phase 3 — Embodied Biology v1

### Objective
- Enforce biological reality and veto unsafe actions.

### Deliverables
- Metabolism (energy, macros)
- Hydration + electrolytes
- Thermoregulation
- Circadian rhythm + sleep
- Vitamins/minerals (deficiency/toxicity)
- Injury/healing
- Immune response
- Endocrine axes (HPA/HPG)
- BioVeto with logged reasons

### Exit Criteria
- Agents starve, fatigue, heal, sleep
- Unsafe actions vetoed with reasons
- Biology deterministic under replay

---

## Phase 4 — Cognition Engine (No LLM)

### Objective
- Deterministic minds and language, fully offline.

### Deliverables
- Perception → Drives → Intent → Action queue
- Deterministic planner (GOAP/HTN)
- Skill trees + habit formation
- Deterministic English:
  - Grammar templates
  - Lexicon tables
  - Pragmatics rules
- Continuous inner monologue
- Learning and memory growth

### Exit Criteria
- Identical thoughts/speech for identical state
- No LLM dependency
- Replay identical

---

## Phase 5 — Social Dynamics + Scaling

### Objective
- Emergent society without determinism drift.

### Deliverables
- Relationship graph
- Attachment styles
- Trust/conflict/bonding
- Gossip + reputation propagation
- Culture metrics
- Multi-agent scaling (dozens of agents)

### Exit Criteria
- Social state replay-identical
- Stable tick rate under load

---

## Phase 6 — Genetics + Reproduction

### Objective
- True population growth with lineage.

### Deliverables
- Double-helix genome
- Recombination + mutation (policy bounded)
- Phenotype expression
- Reproduction pipeline:
  - Consent
  - Intercourse
  - Probabilistic conception
  - Gestation stages
  - Birth
- Lineage trees
- Genetic disorder toggles

### Exit Criteria
- Same parents + seed → same child genome
- Lineage deterministic and inspectable

---

## Phase 7 — Economy + Governance

### Objective
- Deterministic rules governing society and resources.

### Deliverables
- Property and ownership
- Resource markets
- Farming and animals
- Elections
- Laws and policies
- Courts and penalties
- Enforcement via authority

### Exit Criteria
- Laws constrain actions deterministically
- Governance outcomes replay-identical

---

## Phase 8 — WebGPU Renderer + Transparency UI

### Objective
- Professional visualization without authority leakage.

### Deliverables
- WebGPU renderer
- Render packets derived from snapshots
- Multi-monitor layouts
- Diff heatmaps
- Causality graph
- Time-travel debugger

### Exit Criteria
- Renderer hash-stable for snapshots
- UI never mutates state

---

## Phase 9 — Security + Integrity Hardening

### Objective
- Lock security without breaking determinism or offline mode.

### Deliverables
- Keycloak primary (WebAuthn/passkeys)
- Authentik backup
- Encryption at rest
- Tamper-evident audit logs
- Immutable auth/admin audit trail
- Integrity explorer UI

### Exit Criteria
- Tampering detected deterministically
- Passkeys work offline
- Replay and hashes still pass

