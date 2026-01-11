---
status: APPROVED
---

# MARKENZ GEMINI → MARKENZ INTEGRATION PLAN v1

**STATUS:** FOUNDATION PLAN · BINDING · EXECUTION-BLOCKING  
**AUTHORITY:** KAIZA-MCP · ANTIGRAVITY (AMP)  
**MODE:** LOSSLESS MIGRATION · PHASE-ALIGNED · DETERMINISTIC  
**SCOPE:** Convert Gemini Universe into lawful Markenz Universe implementation  
**FAIL MODE:** FAIL_CLOSED  
**TIMESTAMP:** 2025-01-10

---

## 1. PLAN SUMMARY

This plan orchestrates a **lossless, authority-safe, phase-aligned integration** of the existing Gemini Universe into the Markenz Universe under KAIZA-MCP governance.

**Core Objective:**
- Preserve all world assets (house, shed, tools, vehicles, Gem-D, Gem-K)
- Preserve agent identity continuity (Gem-D and Gem-K)
- Migrate authority from ad-hoc TypeScript to locked Rust engine
- Establish determinism as law, not convention
- Make the system immediately inspectable by Windsurf (Markenz target)

**Governing Authority:**
All work is conducted under:
- MARKENZ_EXECUTION_ROADMAP_v2.md (Phase 0-9 definition)
- MARKENZ_TARGET_ARCHITECTURE_v2.md (Rust engine authority)
- MARKENZ_REPO_REFACTOR_MAP_v2.md (Repository structure law)
- AMP_DEFINITION_OF_DONE_v2.md (Merge gates and determinism proofs)
- MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md (World adaptation boundaries)

**Current State (From PASS 2 Audit):**
- Gemini Universe exists as a TypeScript-based simulation in `/media/linnyux/developing/gemini_universe` (outside Markenz)
- Markenz repository exists but is **authority-incomplete**:
  - `apps/engine` (Rust world authority) **MISSING**
  - `crates/world`, `crates/physics`, `crates/biology`, `crates/genetics`, `crates/cognition`, `crates/persistence` **MISSING**
  - Simulation logic located in `apps/server` (TypeScript) — **VIOLATES governance**
  - Database schema incomplete (no input_events/observation_events boundary)
  - Build broken (edition2024 dependency issue)

**Decision:** Execute Phase 0 architecture setup first (Sections 2-4), then asset migration (Section 5).

---

## 2. SOURCE-OF-TRUTH TRANSITION

### 2.1 Current State (Gemini Universe)

**Location:** `/media/linnyux/developing/gemini_universe/`

**Subsystems Present:**
- Consciousness Kernel (planning, perception, emotion)
- Biology (metabolism, hormones, immune, vitals)
- Genetics (phenotype, reproductive pipeline)
- Psychology (dark triad, self-reflection, attachment styles)
- Senses (interoception, proprioception, tactile, vision)
- Language (deterministic English generation)
- World (Homestead, Shed, Tools inventory)
- Somatic Body (physical state, movement)
- Free Will Decision Loop (ethical/volitional processing)
- Event Replay Engine (partial determinism support)
- Twin System (Gem-D and Gem-K initialization and state)

**Assets to Preserve:**
1. **House** (Homestead) — structure, location, ownership
2. **Shed** (Tool storage) — location, inventory
3. **Tools** (implements) — types, durability, functionality
4. **Vehicles** (movement apparatus) — properties, mechanics
5. **Gem-D** (Agent 1) — full state snapshot, history, identity
6. **Gem-K** (Agent 2) — full state snapshot, history, identity

**Critical Data:**
- Agent memories (episodic, semantic, procedural)
- Skill trees and habit formations
- Relationship graphs and trust scores
- Genetic lineage (if reproduction has occurred)
- All event logs from Gemini (for determinism proof)

### 2.2 Target State (Markenz Universe)

**Location:** `/media/linnyux/developing/gemini_universe/markenz/`

**Authority Structure (LOCKED):**
- `apps/engine` (Rust) — sole world state mutator
- `apps/server` (TypeScript) — stateless control plane (validation, ordering, persistence, fanout)
- `apps/web` (React) — read-only observer and command interface
- `crates/world` — canonical world types and state hashing
- `crates/physics` — deterministic movement and constraints
- `crates/biology` — physiology, hormones, immune, reproduction
- `crates/genetics` — genome, recombination, phenotype expression
- `crates/cognition` — perception, planning, memory, language (no LLM)
- `crates/events` — InputEvent and ObservationEvent schemas
- `crates/persistence` — snapshots, replay harness, hash verification
- `infra/postgres` — append-only event log with hash-chain
- `tools/audits` — Python determinism verification and replay audits

**Assets in Markenz:**
Same six assets (House, Shed, Tools, Vehicles, Gem-D, Gem-K), but:
- Stored as serialized state in `crates/world` types (not TypeScript classes)
- Generated from Markenz root seed (deterministic)
- Persisted via `crates/persistence` snapshots
- Fully inspectable via event log and replay audits

### 2.3 Migration Path (Three-Phase)

#### Phase 0: Authority Handoff
**Goal:** Lock Rust as sole world mutator. Establish determinism kernel.  
**Source:** Gemini subsystems (understanding only, no code reuse yet)  
**Target:** `apps/engine` boots and ticks independently  
**Timeline:** 2–3 weeks

**Key Actions:**
1. Resolve Markenz build (pin base64ct version)
2. Create `apps/engine` (Rust binary)
3. Create `crates/world`, `crates/events`, `crates/persistence`
4. Implement genesis seeding (House, Shed, Tools, Vehicles, Gem-D, Gem-K)
5. Boot engine tick loop with RNG and world hashing
6. Wire server to accept InputEvents only (no state mutations)
7. Establish determinism proofs (replay tests)
8. Migrate database schema (input_events/observation_events tables)

#### Phase 1: Asset Preservation
**Goal:** Migrate Gem-D and Gem-K identity and state losslessly.  
**Source:** Gemini Twin System snapshots  
**Target:** Markenz `crates/world` types with full replay equivalence  
**Timeline:** 1–2 weeks

**Key Actions:**
1. Extract Gem-D and Gem-K full state from Gemini event log
2. Map Gemini state schema to Markenz type schema
3. Create "bridge snapshots" that preserve identity continuity
4. Implement Gem-D and Gem-K genesis in engine (loads bridge state)
5. Prove replay equivalence (same seed + Gemini events → identical agent state in Markenz)

#### Phase 2: Subsystem Reuse
**Goal:** Port Gemini subsystems into Rust crates as locked logic.  
**Source:** Gemini `core/` subsystem code (Metabolism, Hormones, Immune, Vitals, Consciousness, Language, etc.)  
**Target:** `crates/biology`, `crates/cognition`, `crates/genetics`  
**Timeline:** 3–4 weeks

**Key Actions:**
1. Classification (as per REUSE_MIGRATION_PLAN_v3): identify REUSE_AS_IS vs REUSE_WITH_MODS vs REWRITE
2. Port Metabolism, Hormones, Immune, Vitals (1:1 logic port)
3. Port Consciousness Kernel, Free Will Loop (inject TimeSource, ChaosStream, EventBus)
4. Port Language System (wrap in deterministic Rust)
5. Write subsystem-level replay tests
6. Gate Phase 2 completion on all tests passing

---

## 3. ASSET PRESERVATION STRATEGY

### 3.1 Asset Inventory (Six Critical Assets)

| Asset | Gemini Location | Type | Status | Preservation Method |
|-------|-----------------|------|--------|---------------------|
| **House** | `world/homestead.ts` | Structure + Location | Active | Serialize to Markenz spatial type; regenerate from seed |
| **Shed** | `world/shed.ts` | Storage + Inventory | Active | Serialize inventory to Markenz inventory type; preserve contents |
| **Tools** | `world/tools.ts` | Implement array | Active | Map to Markenz tool registry; preserve durability/state |
| **Vehicles** | `world/vehicle.ts` | Movement apparatus | Active | Port mechanics to `crates/physics`; preserve state |
| **Gem-D** | `core/twin-system.ts` | Agent 1 full state | Active | Extract snapshots; bridge to Markenz agent type |
| **Gem-K** | `core/twin-system.ts` | Agent 2 full state | Active | Extract snapshots; bridge to Markenz agent type |

### 3.2 Gem-D and Gem-K Identity Continuity

**Current State (Gemini):**
- Full agent object with:
  - Consciousness state (beliefs, intentions, goals)
  - Biology state (metabolism, hormones, vitals)
  - Memory systems (episodic, semantic, procedural)
  - Relationship graph (with other agents, environment)
  - Skill trees and habit formation data
  - Genetic markers (if applicable)
  - Event history (all decisions and actions)

**Preservation Mechanism:**
1. **Export Phase:** Serialize Gem-D and Gem-K to JSON snapshots (full state tree)
2. **Schema Mapping:** Map Gemini agent schema fields to Markenz `Agent` struct fields
   - Gemini `thoughts[]` → Markenz `consciousness.thoughts`
   - Gemini `bio.metabolism` → Markenz `biology.metabolism`
   - Gemini `memories.episodic` → Markenz `memory.episodic`
   - etc.
3. **Bridge Snapshot:** Create intermediate JSON file documenting exact field mappings
4. **Genesis Integration:** Modify `apps/engine` genesis to load bridge snapshots and hydrate agents
5. **Identity Proof:** Verify that replaying Markenz with Gem-D/Gem-K bridge state produces identical behavior under identical InputEvents

**Determinism Constraint:**
Gem-D and Gem-K in Markenz **must produce identical decisions** when fed the same perception + time state as Gemini did. Any deviation is a **hard failure** (FAIL_CLOSED).

**Implementation:**
- Store bridge snapshot in `apps/engine/assets/gems/gem_d_bridge.json` and `gem_k_bridge.json`
- Load via `persistence/bridge.rs:load_agent_bridge()`
- Hash identity fingerprint: `blake3(agent_name || original_state_hash)` stored immutably in genesis
- Replay audit must verify: "Gem-D in Tick 0 Markenz == Gem-D in final Tick Gemini"

### 3.3 World Assets (House, Shed, Tools, Vehicles)

**Preservation Approach:**

**House (Homestead):**
- Gemini: Fixed coordinates in abstract world space
- Markenz: Chunked deterministic terrain with spatial coordinates
- Mechanism:
  - Map Gemini house location to Markenz chunk + local coordinates
  - Serialize structure as list of spatial objects (walls, doors, furniture)
  - Regenerate in engine using `crates/world/structures.rs`
  - Seed world generation to place house deterministically (same seed = same location)

**Shed (Tool Storage):**
- Gemini: Abstract container with inventory array
- Markenz: Spatial structure with slot-based inventory
- Mechanism:
  - Extract tool list from Gemini shed inventory
  - Map each tool to Markenz `Tool` type
  - Serialize as initial inventory state in genesis
  - Load in `apps/engine` at world creation

**Tools:**
- Gemini: Objects with `{id, name, durability, function}`
- Markenz: Registered in immutable tool registry (crates/world/tools.rs)
- Mechanism:
  - Extract Gemini tool definitions
  - Implement in Markenz tool registry (no changes to logic, only port)
  - Assign stable IDs (must match across replay)
  - Preserve durability as mutable state in agent inventory

**Vehicles:**
- Gemini: Movement apparatus with physics properties
- Markenz: Physics objects managed by `crates/physics`
- Mechanism:
  - Port vehicle mechanics to deterministic Rust (crates/physics/vehicle.rs)
  - Extract Gem-D/K vehicle ownership and state from Gemini
  - Initialize vehicles in engine genesis
  - Verify physics behavior matches Gemini via determinism tests

### 3.4 Risk Mitigation

**Loss Risk:** Data serialization/deserialization bugs → state corruption
- **Mitigation:** Create dual-format validator (Gemini JSON ↔ Markenz binary)
- **Test:** Round-trip 100 snapshots: JSON → Markenz type → JSON → verify bit-identical

**Identity Risk:** Gem-D/Gem-K identity divergence under replay
- **Mitigation:** Store original Gem-D/Gem-K state hash in genesis, replay-verify continuously
- **Test:** Replay audit must report any divergence within first 10 ticks

**Determinism Risk:** Assets behave differently in Markenz due to RNG or logic changes
- **Mitigation:** Freeze asset mechanics (no changes), only syntax port
- **Test:** Determinism replay test with assets active

---

## 4. AUTHORITY MIGRATION STRATEGY

### 4.1 Current Authority Structure (Gemini — Non-Compliant)

**Gemini Topology:**
```
apps/backend
├── src/core/                    ← AUTHORITY (simulation logic)
│   ├── consciousness-kernel.ts
│   ├── biology/
│   ├── psychology/
│   ├── free-will-decision-loop.ts
│   ├── event-replay-engine.ts
│   └── ...
├── src/server.js                ← Node.js HTTP server (stateless or semi-stateful)
└── src/data/
    └── event-log.json           ← Immutable log (some authority)
```

**Problem:**
- Multiple possible sources of truth (core logic, server state, event log)
- No strict single-writer pattern
- Nondeterministic JavaScript semantics
- No replay harness
- Server directly mutates world state (VIOLATES MARKENZ LAW)

### 4.2 Target Authority Structure (Markenz — Compliant)

**Markenz Topology:**
```
apps/engine/                    ← AUTHORITY (deterministic Rust)
├── src/main.rs
├── src/tick_loop.rs            ← Fixed-timestep loop
├── src/genesis.rs              ← Initial world state
└── (calls crates/*)

crates/world/                   ← Canonical state types
├── lib.rs
├── world.rs
├── agent.rs
├── asset.rs
└── hash.rs

crates/physics/                 ← Deterministic mechanics
crates/biology/                 ← Physiology, immune, reproduction
crates/cognition/               ← Perception, planning, language
crates/events/                  ← InputEvent/ObservationEvent schemas

apps/server/                    ← CONTROL PLANE (stateless)
├── src/main.rs
├── src/auth.rs                 ← JWT verification (local JWKS)
├── src/input_handler.rs        ← InputEvent validation, ordering
├── src/persistence.rs          ← Append-only log writer
└── src/websocket.rs            ← ObservationEvent fanout

infra/postgres/                 ← IMMUTABLE EVENT LOG
└── schema/
    ├── input_events            ← Server appends
    ├── observation_events      ← Engine produces
    ├── snapshots               ← Engine writes periodically
    └── hash_checkpoints        ← Engine publishes
```

**Authority Flow:**
```
Web (React, read-only)
  ↓ (InputEvent request + JWT)
Server (stateless, validates)
  ↓ (ordered InputEvent)
Postgres (append-only log)
  ↓ (polled by engine)
Engine (Rust authority, ticks)
  ├ Mutates world state
  ├ Emits ObservationEvents
  ├ Computes world_hash
  └ Writes snapshots
  ↓
Postgres (persists observations)
  ↓
Server (fanout to clients)
  ↓
Web (displays state, read-only)
```

### 4.3 Migration Stages (PHASED, NON-BREAKING)

#### Stage 1: Engine Skeleton (Week 1)
- Create `apps/engine` (Rust binary)
- Implement fixed-timestep loop (100 ticks/sec target, configurable)
- Wire to Postgres (read InputEvents from queue)
- Emit dummy ObservationEvents to verify server fanout
- **Authority transition:** Engine becomes sole tick source
- **No business logic yet** (world state is minimal)

#### Stage 2: World Authority (Week 2)
- Implement `crates/world` types (Agent, Asset, Terrain, Chunk)
- Create genesis snapshot (House, Shed, Tools, Vehicles, Gem-D, Gem-K)
- Implement `crates/persistence` (snapshot format, write/read)
- Implement `crates/events` (InputEvent/ObservationEvent schemas)
- **Authority transition:** Engine owns world state, server cannot mutate
- **Control plane:** Server validates and orders InputEvents only

#### Stage 3: Subsystem Porting (Weeks 3–4)
- Port Metabolism, Hormones, Immune, Vitals to `crates/biology`
- Port Consciousness, Planning, Language to `crates/cognition`
- Port Genetics to `crates/genetics`
- **Authority transition:** All simulation logic moves to Rust
- **Compliance check:** Replay tests pass, determinism proven

#### Stage 4: Finalization (Week 5+)
- Remove TypeScript simulation logic from `apps/server`
- Implement full Phase 0 acceptance tests
- Establish authority immutability guarantees
- Archive Gemini Universe (read-only backup)

### 4.4 Authority Boundaries (LOCKED, ENFORCEABLE)

**Engine MUST:**
- Own all world state mutations
- Enforce all rules (physics, biology, cognition, governance, law)
- Produce deterministic outcomes
- Emit complete ObservationEvents (no hidden state)
- Reject invalid InputEvents deterministically
- Compute world_hash checkpoints
- Write snapshots periodically

**Engine CANNOT:**
- Depend on wall clock for state evolution
- Mutate database directly (only via persistence trait)
- Perform network I/O
- Call external services

**Server MUST:**
- Validate InputEvent schema
- Enforce RBAC (Observer, Auditor, Admin roles)
- Append InputEvents immutably to log
- Persist ObservationEvents from engine
- Maintain hash-chain integrity
- Fanout ObservationEvents to clients via WebSocket

**Server CANNOT:**
- Mutate world state
- Veto or override engine outcomes
- "Fix" determinism divergences
- Cache or buffer authoritative state

**Web MUST:**
- Accept InputEvents as user/admin commands
- Submit InputEvents to server with JWT
- Display snapshots and observations
- Enable inspection and time-travel

**Web CANNOT:**
- Mutate world state directly
- Call engine directly
- Bypass server authentication

**Enforcement:**
- Structural (Rust type system prevents server from accessing world state)
- CI (Static analysis forbids direct world mutation in server code)
- Runtime (Engine rejects InputEvents from unauthorized sources; server rejects mutations from web)

---

## 5. PHASE-ALIGNED EXECUTION STEPS

### Phase 0: Repo + Offline Stack Baseline Closure

**Objective:** Boot the full stack completely offline. Establish immutable event sourcing and deterministic tick progression.

**Entry Criteria:**
- Markenz repository checked out
- Cargo/Rust toolchain available

**Exit Criteria (ALL MUST PASS):**
- `docker compose up --build` works with no internet
- Engine ticks advance at fixed rate
- Server accepts and logs InputEvents
- Web displays live tick counter and world_hash
- Replay tests pass (same seed + events → identical hash sequence)
- Cargo build succeeds

**Deliverables:**

1. **Resolve Build Issues**
   - Pin base64ct version or update MSRV
   - Verify `cargo check` and `cargo build` on all crates
   - **Assignee:** Senior Rust engineer
   - **Effort:** 4 hours

2. **Create apps/engine (Rust Binary)**
   - Depend on: crates/deterministic, crates/world (to be created), crates/events (to be created)
   - Implement:
     - `fn main()` → initialize world, enter tick loop
     - `fn tick(&mut self) → Result<()>` → advance by fixed dt, process InputEvents, emit ObservationEvents
     - `fn world_hash() -> Blake3Hash` → canonical world state hash
   - Start with 100 ticks/sec (configurable)
   - **Assignee:** Senior Rust engineer
   - **Effort:** 40 hours

3. **Create crates/world (Rust Library)**
   - Define core types:
     ```
     pub struct World { ... }
     pub struct Agent { ... }
     pub struct Asset { ... }
     pub struct Chunk { ... }
     pub struct Terrain { ... }
     ```
   - Implement:
     - Canonical serialization (for hashing)
     - World hashing function (blake3 of serialized state)
     - Deterministic iteration (BTreeMap, sorted collections)
     - Genesis snapshot creation
   - No business logic yet (skeleton only)
   - **Assignee:** Senior Rust engineer
   - **Effort:** 30 hours

4. **Create crates/events (Rust Library)**
   - Define schemas:
     ```
     pub enum InputEvent { ... }  // Agent move, spawn, despawn, chat, custom
     pub enum ObservationEvent { ... }  // State change, effect, communication
     ```
   - Implement:
     - Deterministic serialization (for event log)
     - Schema validation
     - Version compatibility
   - **Assignee:** Senior Rust engineer
   - **Effort:** 20 hours

5. **Create crates/persistence (Rust Library)**
   - Implement:
     - Snapshot format (canonical serialization of `World`)
     - `save_snapshot(&world) -> Result<[u8]>`
     - `load_snapshot(&bytes) -> Result<World>`
     - Replay harness: `replay_from_genesis(events) -> Result<World>`
     - Hash verification: `verify_chain(events) -> Result<()>`
   - **Assignee:** Senior Rust engineer
   - **Effort:** 25 hours

6. **Implement Genesis + Initial World**
   - Create Markenz seed (root deterministic RNG source)
   - Implement genesis snapshot containing:
     - House (Homestead) — spatial location, structure
     - Shed (Tool storage) — location, initial inventory
     - Tools — registry and initial state
     - Vehicles — instances, initial state
     - Gem-D — initial agent state (from bridge snapshot)
     - Gem-K — initial agent state (from bridge snapshot)
   - Use deterministic terrain generation (chunked grid)
   - **Assignee:** Senior Rust engineer
   - **Effort:** 35 hours

7. **Implement RNG Audit Logging**
   - Wrap ChaosStream.next_*() calls with audit context
   - Log: `{ tick, subsystem, stream, callsite, random_value }`
   - Store audit records in event log or separate audit table
   - Required by AMP_DEFINITION_OF_DONE_v2.md § A
   - **Assignee:** Senior Rust engineer
   - **Effort:** 15 hours

8. **Fix Database Schema**
   - Replace generic `events` table with:
     - `input_events` (from server)
     - `observation_events` (from engine)
     - `snapshots` (engine-created)
     - `hash_checkpoints` (engine publishes)
   - Add append-only constraints (no UPDATE/DELETE on immutable tables)
   - Add hash-chain foreign keys
   - Implement schema migration scripts
   - **Assignee:** Database engineer
   - **Effort:** 15 hours

9. **Modify apps/server (TypeScript)**
   - Update InputEvent endpoint to append to new `input_events` table
   - Update ObservationEvent fanout from new `observation_events` table
   - Add hash-chain validation (reject if prev_hash broken)
   - **Preserve:** Auth, RBAC, WebSocket fanout
   - **Assignee:** TypeScript engineer
   - **Effort:** 20 hours

10. **Implement Determinism Replay Tests**
    - Test 1: `test_determinism_replay_100`
      - Inputs: Markenz seed, 100 ticks, no InputEvents
      - Expected: Same world_hash sequence on rerun
    - Test 2: `test_snapshot_equivalence`
      - Snapshot at tick 50, replay from snapshot to tick 100
      - Expected: Same world_hash at tick 100 as full run
    - Test 3: `test_hash_chain_integrity`
      - Verify prev_hash chain unbroken
      - Expected: All hashes form valid chain
    - Test 4: `test_rng_determinism`
      - Same subsystem RNG seed → identical float sequences
      - Expected: Bit-identical across runs
    - **Assignee:** QA/Rust engineer
    - **Effort:** 20 hours

**Total Phase 0 Effort:** ~220 hours (4–5 weeks, 1 senior Rust engineer + 1 DB engineer)

**Phase 0 Gate (AMP Definition of Done v2 § B):**
- [ ] Build passes: `cargo build --release`
- [ ] Unit tests pass: `cargo test --all`
- [ ] Integration tests pass: `test_determinism_replay_100`, `test_snapshot_equivalence`
- [ ] Docker compose boots: `docker compose up --build`
- [ ] No panics in first 1000 ticks
- [ ] WebSocket fanout working
- [ ] Replay audit report generated

**Sign-Off:** AMP Principal-Level Auditor verifies compliance and approves Phase 1 entry.

---

### Phase 1: Deterministic Kernel + Replay Harness

**Objective:** Prove determinism formally via replay and snapshot equivalence.

**Entry Criteria:**
- Phase 0 complete and gated
- Engine ticks stably
- Determinism tests passing

**Exit Criteria:**
- Same seed + same events → identical hash timeline
- Snapshot replay == full replay
- No authority leakage detected

**Key Activities:**
- Enhance RNG audit logging (add callsite stack traces)
- Implement replay time-travel viewer in web UI
- Port basic biology (metabolism) to engine (minimal, skeleton)
- Create baseline performance metrics
- Implement hash-chain status panel in web

**Effort:** ~40 hours (1 week)

**Gate:** All determinism tests passing, audit report clean.

---

### Phase 2: World Representation v1 (Terrain + Entities)

**Objective:** Replace abstract world with deterministic spatial reality.

**Key Activities:**
- Implement chunked deterministic terrain
- Biomes v1 (grassland, forest, mountain, water)
- Asset mechanics (move, gather, build, mine)
- Collision detection
- Inventory mechanics

**Effort:** ~60 hours (2 weeks)

**Gate:** Replay tests still pass; actions deterministic.

---

### Phase 3: Embodied Biology v1

**Objective:** Enforce biological reality and veto unsafe actions.

**Key Activities:**
- Port Gemini Metabolism, Hormones, Immune, Vitals
- Implement BioVeto (logged rejection reasons)
- Agents must eat, sleep, breathe
- Biology deterministic under replay

**Effort:** ~50 hours (1.5 weeks)

**Gate:** Biological constraints active; agents survive/starve/heal realistically.

---

### Phase 4: Cognition Engine (No LLM)

**Objective:** Deterministic minds and language, fully offline.

**Key Activities:**
- Port Consciousness Kernel, Free Will Loop
- Implement Planning (GOAP/HTN style)
- Port Language System (deterministic NLG)
- Inner monologue streaming
- Skill trees and habit formation

**Effort:** ~80 hours (2–3 weeks)

**Gate:** Identical thoughts/speech for identical state; replay identical.

---

### Phase 5: Social Dynamics + Scaling

**Objective:** Emergent society without determinism drift.

**Key Activities:**
- Relationship graph
- Trust/conflict/bonding
- Gossip and reputation
- Culture metrics
- Scale to dozens of agents

**Effort:** ~60 hours (2 weeks)

**Gate:** Social state replay-identical; tick rate stable.

---

### Phase 6: Genetics + Reproduction

**Objective:** True population growth with lineage.

**Key Activities:**
- Port Genetics module (double-helix genome)
- Recombination and mutation
- Reproduction pipeline (consent → intercourse → conception → gestation → birth)
- Lineage trees

**Effort:** ~50 hours (1.5 weeks)

**Gate:** Lineage deterministic and inspectable.

---

### Phase 7: Economy + Governance

**Objective:** Deterministic rules governing society and resources.

**Key Activities:**
- Property and ownership
- Markets and trade
- Farming and animals
- Elections and voting
- Laws and penalties

**Effort:** ~70 hours (2 weeks)

**Gate:** Laws constrain actions deterministically.

---

### Phase 8: WebGPU Renderer + Transparency UI

**Objective:** Professional visualization without authority leakage.

**Key Activities:**
- WebGPU renderer (async, non-blocking)
- Render packets (deterministic derivation from snapshots)
- Diff heatmaps (what changed each tick)
- Causality graph (why did X happen)
- Time-travel debugger UI

**Effort:** ~100 hours (3 weeks)

**Gate:** Renderer hash-stable; UI never mutates state.

---

### Phase 9: Security + Integrity Hardening

**Objective:** Lock security without breaking determinism.

**Key Activities:**
- Keycloak WebAuthn/passkeys
- Authentik backup IdP
- Encryption at rest (postgres + file storage)
- Tamper-evident audit logs
- Immutable auth/admin audit trail

**Effort:** ~60 hours (2 weeks)

**Gate:** Tampering detected deterministically.

---

## 6. RISK & CONTAINMENT PLAN

### 6.1 Critical Risks

| Risk | Severity | Containment |
|------|----------|-------------|
| **Gem-D/Gem-K identity loss** | CRITICAL | Export full snapshots, dual-format validator, replay equivalence proof |
| **Asset state corruption** | CRITICAL | Round-trip testing, checksum verification, backup snapshots |
| **Determinism divergence** | CRITICAL | Atomic hash mismatch → HALT (fail-closed), generate divergence report |
| **Build failure** | HIGH | Pin all dependency versions before migration, CI verification |
| **Performance regression** | HIGH | Establish Phase 0 baseline, gate on `≥80% of baseline` |
| **Authority leakage** (server mutates state) | HIGH | Structural type system enforcement, CI static analysis |
| **Database schema corruption** | HIGH | Append-only constraints, foreign key enforcement, migration rollback script |
| **RNG non-determinism** | HIGH | Per-call audit logging, cross-platform testing (Linux/macOS/Windows) |

### 6.2 Failure Modes & Recovery

**Failure Mode 1: Determinism Divergence**
- **Symptom:** Replay hash mismatch at tick N
- **Trigger:** Different random sequences, latent floating-point bug, container iteration order
- **Recovery:**
  1. Generate divergence report (RNG state, containers, last-matching hash)
  2. Halt pipeline
  3. Developer investigates callsite of first divergence
  4. Fix (usually RNG seeding or container ordering)
  5. Rerun determinism tests until passing
  6. Escalate to AMP auditor if unresolved after 48 hours

**Failure Mode 2: Gem-D/Gem-K Identity Mismatch**
- **Symptom:** Agent behavior diverges from Gemini after first decision
- **Trigger:** State serialization bug, schema mismatch, RNG seed divergence
- **Recovery:**
  1. Compare bridge snapshot with loaded state (JSON diff)
  2. Verify RNG seed derivation (same subsystem ID → same stream)
  3. Trace first decision (perception → planning → action)
  4. Identify state field divergence
  5. Fix mapping or RNG seed
  6. Rerun agent determinism test until passing

**Failure Mode 3: Database Integrity Violation**
- **Symptom:** Corrupted hash chain (prev_hash mismatch)
- **Trigger:** Schema bug, migration failure, concurrent write
- **Recovery:**
  1. Restore from backup
  2. Reapply migrations with lock
  3. Verify hash chain
  4. Replay from beginning
  5. Confirm determinism

**Failure Mode 4: Build Failure**
- **Symptom:** `cargo build` fails during development
- **Trigger:** Dependency version conflict, Rust incompatibility, missing feature
- **Recovery:**
  1. Freeze dependency versions
  2. Create minimal reproducible build (MRB) script
  3. Test on target platform (Linux x64)
  4. If unfixable, mark as blocker and escalate
  5. No merge until build passes

### 6.3 Containment Boundaries

**Gemini Universe (Archive):**
- Frozen after asset extraction
- Maintained as read-only reference
- No further mutation allowed
- Used only for audit and verification

**Markenz Universe (Active):**
- Determinism and authority are non-negotiable
- Any violation triggers FAIL_CLOSED
- Rolling backups (snapshots every 1000 ticks)
- Audit log immutable (append-only)

**Phase Gates:**
- No phase entry without previous phase PASS
- No merge without all tests passing
- No production deployment without auditor sign-off

---

## 7. WINDSURF HANDOFF CONTRACT

### 7.1 Constraints & Responsibilities

**Windsurf Authority:**
- Execute Phase 0 exactly as specified in Section 5
- Create all listed crates and binaries
- Implement all specified functions and traits
- Write all acceptance tests
- Verify all gates pass before declaring phase complete

**Windsurf Prohibitions:**
- **NO** code reuse from Gemini yet (Phase 0 is infrastructure only)
- **NO** skipping tests or gate criteria
- **NO** TODO/FIXME/stub implementations
- **NO** deviation from locked repo layout
- **NO** architecture redesign or "optimizations"
- **NO** external dependencies without explicit approval
- **NO** wall-clock logic in authority state

**Windsurf Escalation Triggers:**
1. Any ambiguity in this plan → stop and escalate to AMP auditor
2. Any blocker not listed in this plan → escalate with evidence
3. Determinism divergence → halt and escalate with divergence report
4. Any phase gate failing → halt and escalate with failure details
5. Build failures lasting >4 hours → escalate

**AMP Auditor Responsibilities:**
- Review Windsurf code commits for authority violations
- Run acceptance tests independently
- Generate audit reports per phase
- Gate phase progression
- Approve final deployment

### 7.2 Interface Contract (Engine ↔ Server)

**Server sends to Engine (via Postgres input_events):**
```
InputEvent {
  tick: u64,           // When should this occur?
  source: String,      // Web/admin/simulation?
  event_type: String,  // "AgentMove", "AgentChat", "Inject", etc.
  payload: Vec<u8>,    // Binary serialized event data
  signature: [u8; 64], // ECDSA or HMAC (for integrity, not auth)
}
```

**Engine returns (via Postgres observation_events):**
```
ObservationEvent {
  tick: u64,
  event_type: String,
  payload: Vec<u8>,
  caused_by: Option<InputEvent::id>,  // Trace back to input
}

WorldHash {
  tick: u64,
  hash: [u8; 32],       // BLAKE3
  prev_hash: [u8; 32],  // For chain verification
}
```

**Error Handling:**
- Engine rejects malformed InputEvents → ObservationEvent with "InvalidInput" reason
- Engine vetoes illegal actions → ObservationEvent with "Vetoed: reason"
- Engine halts on hash mismatch → Server logs critical, pauses new ticks

### 7.3 Success Criteria (Phase 0 Completion)

**ALL of the following must be true:**

1. ✅ **Build:** `cargo build --release` succeeds with no warnings
2. ✅ **Tests:** `cargo test --all` passes 100%
3. ✅ **Docker:** `docker compose up --build` boots all services
4. ✅ **Offline:** No network access required (confirmed via `tcpdump`)
5. ✅ **Determinism:** `test_determinism_replay_100` passes (same seed → identical hash sequence)
6. ✅ **Snapshot:** `test_snapshot_equivalence` passes
7. ✅ **Hash Chain:** `test_hash_chain_integrity` passes
8. ✅ **Authority:** Engine is sole tick source; server cannot mutate state (verified via code review)
9. ✅ **RNG:** `test_rng_determinism` passes (cross-platform identical sequences)
10. ✅ **World:** Genesis contains House, Shed, Tools, Vehicles (verified in snapshot)
11. ✅ **Web:** Live tick display and world_hash visible in UI
12. ✅ **Audit:** Phase 0 audit report clean (no violations detected)

**Failure = STOP.** Do not proceed to Phase 1 until all criteria pass.

---

## 8. FINAL GO / NO-GO CRITERIA

### 8.1 Go Criteria (ALL must be true for Phase 0 → Phase 1)

- [ ] Cargo build succeeds (release + test builds)
- [ ] All unit tests passing
- [ ] All integration tests passing (determinism, snapshot, hash-chain, RNG)
- [ ] Docker compose boots with no errors
- [ ] No unsafe code in critical paths (audit performed)
- [ ] Authority boundaries validated (code review + static analysis)
- [ ] Database schema migrated (input_events/observation_events tables created)
- [ ] Genesis snapshot creates and loads correctly
- [ ] Gem-D and Gem-K assets present and loadable
- [ ] RNG audit logging functional
- [ ] World hashing implemented and computing correctly
- [ ] No panics in 1000-tick baseline run
- [ ] Replay audit report generated with no anomalies
- [ ] AMP Principal Auditor approval obtained

### 8.2 No-Go Criteria (ANY of these = STOP Phase 0)

- [ ] Determinism test fails (hash mismatch on rerun)
- [ ] Snapshot equivalence test fails
- [ ] Build fails in CI (any platform)
- [ ] Authority boundary violated (server mutates state)
- [ ] Asset data loss detected (Gem-D/Gem-K/House/Shed incomplete)
- [ ] RNG diverges across platforms
- [ ] Database migration fails or corrupts data
- [ ] Performance regresses >50% vs baseline
- [ ] Hash chain broken (prev_hash mismatch)
- [ ] Panics occur in first 1000 ticks
- [ ] AMP audit report identifies critical violations

### 8.3 Phase 0 Timeline

| Week | Deliverable | Owner | Status |
|------|-------------|-------|--------|
| W1 | Cargo build fixed, apps/engine skeleton | Rust Eng | TODO |
| W2 | crates/world, crates/events, genesis snapshot | Rust Eng | TODO |
| W3 | crates/persistence, replay tests, DB migration | Rust Eng + DB | TODO |
| W4 | RNG audit logging, full integration, UI updates | Rust Eng + TS | TODO |
| W5 | Final testing, audit report, auditor approval | QA + Auditor | TODO |

**Critical Path:** Build fix → Engine skeleton → Persistence → Tests → Audit

**Estimated Completion:** 5 weeks (Jan 10 → Feb 14, 2025)

---

## 9. WINDSURF EXECUTION CONSTRAINTS

This plan is **directly executable by Windsurf without interpretation.**

**Windsurf's Charter:**
1. Follow this plan exactly.
2. Create all specified files and functions.
3. Run all specified tests.
4. Escalate any ambiguities to AMP auditor.
5. Halt on any failure gate.

**Windsurf's Prohibitions:**
1. No code generation from Gemini (Phase 0 is new, not port)
2. No skipping tests
3. No TODO/FIXME implementations
4. No creative problem-solving (if stuck, escalate)
5. No merging without audit approval

**Authority:**
- AMP auditor has final authority over all decisions
- Windsurf proposes code, auditor approves/rejects
- No ambiguous merge requests allowed

---

## APPENDIX A: REFERENCE DOCUMENTS

**Governing Laws (BINDING):**
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/governance/MARKENZ_EXECUTION_ROADMAP_v2.md`
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/governance/MARKENZ_TARGET_ARCHITECTURE_v2.md`
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/governance/MARKENZ_REPO_REFACTOR_MAP_v2.md`
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/governance/AMP_DEFINITION_OF_DONE_v2.md`
- `/media/linnyux/developing/gemini_universe/markenz/docs/governance/MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md`

**Audit Reports (FACTUAL CONSTRAINTS):**
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/audit/PASS_2_REPO_REALITY_AUDIT.md`

**Reuse & Migration:**
- `/media/linnyux/development3/developing/gemini_universe/markenz/docs/plans/MARKENZ_REUSE_MIGRATION_PLAN_v3.md`

**Assets (TO PRESERVE):**
- Gem-D state: `/media/linnyux/development3/developing/gemini_universe/agents/` (extract snapshots)
- Gem-K state: `/media/linnyux/development3/developing/gemini_universe/agents/` (extract snapshots)
- World: `/media/linnyux/development3/developing/gemini_universe/apps/backend/src/world/`
- Consciousness: `/media/linnyux/development3/developing/gemini_universe/apps/backend/src/core/`
- Biology: `/media/linnyux/development3/developing/gemini_universe/apps/backend/src/core/biology/`

---

## APPENDIX B: CRITICAL UNKNOWNS & ASSUMPTIONS

### Known Unknowns

**UNKNOWN 1: Exact Gem-D/Gem-K state format in Gemini**
- **Issue:** Source code not yet examined for full schema
- **Assumption:** Standard TypeScript agent object with nested properties
- **Risk:** Schema mismatch during bridge
- **Mitigation:** Extract and validate snapshots before Phase 0 complete

**UNKNOWN 2: Gemini event log format and completeness**
- **Issue:** Not yet verified that all events are persisted
- **Assumption:** JSON log exists and is audit-trail grade
- **Risk:** Missing events → incomplete replay
- **Mitigation:** Inspect event log, verify continuity

**UNKNOWN 3: Performance baseline (Gemini vs Markenz)**
- **Issue:** No baseline tick rate established
- **Assumption:** Markenz will be similar or faster
- **Risk:** Tick rate regression
- **Mitigation:** Measure Gemini baseline, gate Phase 0 at ≥80% baseline

**UNKNOWN 4: Exact RNG behavior in Gemini**
- **Issue:** Random seed sources, streams, distribution functions unknown
- **Assumption:** Can port 1:1 to Rust ChaCha20
- **Risk:** Behavior divergence
- **Mitigation:** Detailed RNG audit, test bit-identical sequences

**UNKNOWN 5: Geometry/physics mapping (Gemini abstract ↔ Markenz spatial)**
- **Issue:** Gemini world is abstract; Markenz is chunked grid
- **Assumption:** Can map linearly
- **Risk:** Asset placement divergence
- **Mitigation:** Create mapping spec before Phase 1

### Stated Assumptions

1. **Markenz governance documents are authoritative.** If they conflict with this plan, governance documents win.
2. **Determinism is non-negotiable.** Any violation is a STOP.
3. **Rust is sole authority.** TypeScript/JavaScript cannot mutate world state.
4. **Event sourcing is immutable.** No retroactive event mutation allowed.
5. **Tests are executable, not manual.** No visual-confirmation-only tests.
6. **Authority is structurally enforced.** Not just convention or code review.
7. **Offline-first is required.** No external service dependencies at runtime.

---

## FINAL DECISION

### Status: APPROVED FOR WINDSURF EXECUTION

**This plan is:**
- ✅ **Binding:** All sections are law, not suggestions
- ✅ **Mechanically Enforceable:** Every step has success/failure criteria
- ✅ **Phased & Contained:** Risk is bounded per phase
- ✅ **Auditable:** Complete traceability to governing law
- ✅ **Fail-Closed:** Any violation halts execution

**Next Step:**
Execute Phase 0 as specified in Section 5 using this plan as the sole authority.

**Authority:**
- **Plan Owner:** ANTIGRAVITY (AMP)
- **Execution Authority:** Windsurf
- **Approval Authority:** AMP Principal-Level Auditor
- **Escalation:** KAIZA-MCP governing law

---

**END OF PLAN**

**Signature:** ANTIGRAVITY (AMP)  
**Timestamp:** 2025-01-10  
**Authority:** KAIZA-MCP v2  
**Status:** BINDING & EXECUTION-BLOCKING

