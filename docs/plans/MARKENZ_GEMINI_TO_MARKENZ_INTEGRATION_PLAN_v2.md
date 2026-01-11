---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
timestamp: 2026-01-10
scope: Gemini Universe → Markenz Integration
fail_mode: FAIL-CLOSED
plan_id: MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_v2
---

# MARKENZ GEMINI → MARKENZ INTEGRATION PLAN v2

**BINDING AUTHORITY:** KAIZA-MCP · AMP (ANTIGRAVITY)  
**MODE:** AUDIT-DRIVEN · PHASE-ALIGNED · DETERMINISTIC · FAIL-CLOSED  
**SCOPE:** Convert Gemini Universe into lawful Markenz Universe under locked governance  
**ENFORCEMENT:** This plan is directly executable by Windsurf without interpretation.

---

## STATUS HEADER

**STATUS:** BINDING · EXECUTION-BLOCKING  
**AUTHORITY:** KAIZA-MCP · AMP  
**SCOPE:** Gemini → Markenz integration  
**FAIL MODE:** FAIL-CLOSED  
**GOVERNING DOCUMENTS:**
- MARKENZ_EXECUTION_ROADMAP_v2.md (Phase law)
- MARKENZ_TARGET_ARCHITECTURE_v2.md (System design)
- MARKENZ_REPO_REFACTOR_MAP_v2.md (Repository structure)
- AMP_DEFINITION_OF_DONE_v2.md (Merge gates)
- MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md (Evolution boundaries)
- PASS_2_REPO_REALITY_AUDIT.md (Current state)
- REUSE_CERTIFICATION_AUDIT_REPORT.md (Reuse constraints)

---

## EXECUTIVE SUMMARY

This plan orchestrates a **lossless, authority-safe, phase-aligned integration** of the existing Gemini Universe into the Markenz Universe under KAIZA-MCP governance.

**Current Reality (PASS 2 Audit):**
- Gemini Universe exists at `/media/linnyux/developing/gemini_universe/` with functional consciousness, biology, genetics, psychology, and world systems
- Markenz repository exists at `/media/linnyux/developing/gemini_universe/markenz/` but **fails Phase 0 legal readiness**
  - Authority boundary violated: simulation logic in `apps/server` (TypeScript) instead of `apps/engine` (Rust)
  - Critical crates missing: `crates/world`, `crates/physics`, `crates/biology`, `crates/genetics`, `crates/cognition`, `crates/persistence`
  - Build broken (edition2024 dependency)
  - Database schema incomplete
  - No replay harness

**Decision:** Execute blocking corrections before asset migration. Phased execution with hard gates.

---

## PART 1: AUDIT-DERIVED GROUND TRUTH

### 1.1 What Exists in Gemini Universe

**Location:** `/media/linnyux/developing/gemini_universe/`

**Functional Subsystems (Reusable):**
| System | Type | Status | Reuse Class |
|--------|------|--------|-------------|
| Metabolism | Biology | Complete, deterministic | REUSE AS-IS |
| Hormones | Biology | Complete, 9 dimensions | REUSE AS-IS |
| Immune System | Biology | Complete, deterministic | REUSE AS-IS |
| Vitals | Biology | Complete, deterministic | REUSE AS-IS |
| Interoception | Sense | Complete, deterministic | REUSE AS-IS |
| Proprioception | Sense | Complete, deterministic | REUSE AS-IS |
| Tactile System | Sense | Complete, deterministic | REUSE AS-IS |
| Granular Emotions | Psychology | Complete, 150+ emotions | REUSE AS-IS |
| Dark Triad System | Psychology | Complete, deterministic | REUSE AS-IS |
| Homestead (House) | World | Complete, deterministic | REUSE AS-IS |
| Shed (Tools) | World | Complete, deterministic | REUSE AS-IS |
| ChaosSys (RNG) | Infrastructure | Complete, deterministic | REUSE AS-IS |
| TimeSourceRegistry | Infrastructure | Complete, replay-capable | REUSE AS-IS |
| Somatic Body | Physiology | Complete, logic sound | REUSE WITH MODS (remove global bus) |
| Free-Will Decision Loop | Cognition | Complete logic, sound | REUSE WITH MODS (inject TimeSource) |
| Event Replay Engine | Infrastructure | Partial | REUSE WITH MODS (make DB optional, fix hash calc) |
| Consciousness Kernel Enhanced | Cognition | Orchestration complete | REUSE WITH MODS (inject TimeSource, EventBus) |
| Full Consciousness Integration | Cognition | Complete, structure sound | REUSE WITH MODS (remove setInterval, tick-driven) |
| World Service | World | Complete, logic sound | REUSE WITH MODS (inject TimeSource, make DB optional) |
| State Container | Infrastructure | Partial | REUSE WITH MODS (implement somatic/brain tick logic) |

**Critical Assets to Preserve:**
1. **House (Homestead)** — structure, location, ownership
2. **Shed** — location, inventory contents
3. **Tools** — types, durability state
4. **Vehicles** — properties, mechanics, state
5. **Gem-D (Agent)** — full state, memories, skills, history, identity
6. **Gem-K (Agent)** — full state, memories, skills, history, identity

**Hard Rejections (Cannot Reuse):**
- **RuntimeLoop** (`core/runtime/loop.ts`) — setInterval disabled, no tick mechanism, orphaned
- **SelfReflectionEngine** — entirely stubbed, unimplemented

---

### 1.2 What Exists in Markenz Repository

**Location:** `/media/linnyux/developing/gemini_universe/markenz/`

**Currently Present:**
- `apps/server` (TypeScript) — contains **unauthorized** simulation logic (SimLoop, world state mutations)
- `apps/web` (React) — minimal, read-only observer
- `crates/deterministic` — RNG and collections, partial
- `crates/protocol` — minimal
- `tools/auth-bootstrap`, `tools/db-migrate`, `tools/keyctl`
- `infra/auth/keycloak` — realm config only

**Critical Gaps (Per PASS 2 Audit):**
| Component | Required | Actual | Severity |
|-----------|----------|--------|----------|
| `apps/engine` (Rust authority) | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/world` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/physics` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/biology` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/genetics` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/cognition` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/events` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `crates/persistence` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `tools/audits` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| `infra/postgres` | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER |
| Simulation logic in `apps/server` | ❌ PROHIBITED | ✅ FOUND | CRITICAL VIOLATION |

---

### 1.3 What Is Reusable vs Rewrite-Required

**REUSE AS-IS (Zero modification, direct port to Rust):**
- Metabolism, Hormones, Immune, Vitals, Interoception, Proprioception, Tactile
- Granular Emotions, Dark Triad
- Homestead, Shed, ChaosSys, TimeSourceRegistry
- **Action:** Port 1:1 logic to Rust crates (Tier 1 modules)

**REUSE WITH MODIFICATION (Logic preserved, dependency injection required):**
- Somatic Body (remove global event bus, inject EventEmitter)
- Free-Will Decision Loop (inject TimeSource, verify ChaosSys seeding)
- Event Replay Engine (implement deterministic hash calc, make DB optional)
- Consciousness Kernel Enhanced (inject TimeSource and EventBus, remove Date.now())
- Full Consciousness Integration (remove setInterval, implement tick() method, strengthen world API)
- World Service (inject TimeSource, make DB optional, replace setInterval)
- State Container (implement processSomaticLayer and processBrainLayer)
- **Action:** Apply specified modifications in Phase 2 (Tier 2 modules)

**REWRITE REQUIRED (Cannot reuse as-is, must implement new):**
- RuntimeLoop — critical broken state (setInterval disabled)
- SelfReflectionEngine — entirely stubbed
- **Action:** Implement fresh in Markenz (Tier 3 modules)

---

## PART 2: ASSET & IDENTITY PRESERVATION PLAN

### 2.1 Gem-D and Gem-K Identity Continuity

**Non-Negotiable Law:** No loss of identity, memories, or agency history.

**Preservation Mechanism:**

1. **Export Phase (Pre-Integration)**
   - Extract Gem-D and Gem-K full state snapshots from Gemini event log
   - Serialize to canonical JSON format:
     ```json
     {
       "agent_id": "gem-d",
       "original_name": "Gem-D",
       "consciousness_state": { ... },
       "biology_state": { ... },
       "memory_systems": { ... },
       "relationship_graph": { ... },
       "skill_trees": { ... },
       "genetic_markers": { ... },
       "event_history": [ ... ]
     }
     ```

2. **Schema Mapping**
   - Document exact field-by-field mapping from Gemini agent schema to Markenz `Agent` struct
   - Create bridge document at `apps/engine/assets/gems/GEM_IDENTITY_BRIDGE.md`

3. **Genesis Integration**
   - Store bridge snapshots in `apps/engine/assets/gems/gem_d_bridge.json` and `gem_k_bridge.json`
   - Implement `persistence/bridge.rs:load_agent_bridge()` to hydrate agents at engine startup
   - Hash identity fingerprint: `blake3(agent_name || original_state_hash)` — stored immutably in genesis
   - Both agents load at tick 0 with identical state to Gemini endpoint

4. **Identity Verification**
   - Determinism test: Replay Markenz from genesis → Agent decisions at tick 0-100 **must match** Gemini endpoint decisions under identical perception inputs
   - Failure = STOP (hard gate)

**Determinism Constraint:** Gem-D and Gem-K in Markenz **must produce identical decisions** when fed the same perception + world state as in Gemini. Any deviation is **FAIL-CLOSED**.

---

### 2.2 House, Shed, Tools, Vehicles Preservation

**House (Homestead):**
- Serialize Gemini house location and structure
- Map to Markenz chunked terrain spatial coordinates
- Regenerate in engine using deterministic seed-based world generation
- Guarantee: Same seed in Markenz = same house location as Gemini

**Shed (Tool Storage):**
- Extract tool inventory array from Gemini shed
- Map each tool to Markenz `Tool` type definition
- Initialize as engine genesis state
- Preserve exact durability/wear state

**Tools:**
- Extract all tool definitions from Gemini tool registry
- Implement in Markenz `crates/world/tools.rs` (immutable registry)
- Assign stable IDs (must match across replay)
- Preserve durability as mutable state in agent inventory

**Vehicles:**
- Port vehicle mechanics to deterministic Rust (`crates/physics/vehicle.rs`)
- Extract ownership and state from Gemini agents
- Initialize in engine genesis
- Determinism test: Vehicle physics behavior matches Gemini under identical inputs

---

### 2.3 Risk Mitigation

| Risk | Mitigation | Gate |
|------|-----------|------|
| Data serialization/deserialization bugs → state corruption | Create dual-format validator (Gemini JSON ↔ Markenz binary), round-trip test 100 snapshots | TEST-BRIDGE-001 |
| Gem-D/Gem-K identity divergence under replay | Store original state hash in genesis, replay-verify continuously, report any divergence within first 10 ticks | TEST-IDENTITY-001 |
| Assets behave differently in Markenz due to RNG or logic changes | Freeze asset mechanics (no changes), only syntax port, determinism replay test with assets active | TEST-ASSETS-001 |
| Identity loss during integration | Full state snapshot validation, dual comparison (Gemini vs Markenz state at checkpoint) | TEST-PRESERVATION-001 |

---

## PART 3: SUBSYSTEM MAPPING

### 3.1 Gemini Subsystem → Markenz Crate Mapping

| Gemini Subsystem | Markenz Crate | Status | Authority |
|------------------|---------------|--------|-----------|
| core/biology/metabolism.ts | crates/biology | CREATE | Rust engine only |
| core/biology/hormones.ts | crates/biology | CREATE | Rust engine only |
| core/biology/immune-system.ts | crates/biology | CREATE | Rust engine only |
| core/biology/vitals.ts | crates/biology | CREATE | Rust engine only |
| core/senses/interoception.ts | crates/biology | CREATE | Rust engine only |
| core/senses/proprioception.ts | crates/biology | CREATE | Rust engine only |
| core/senses/tactile-system.ts | crates/biology | CREATE | Rust engine only |
| core/psychology/granular-emotions.ts | crates/cognition | CREATE | Rust engine only |
| core/psychology/dark-triad.ts | crates/cognition | CREATE | Rust engine only |
| core/free-will-decision-loop.ts | crates/cognition | CREATE (with mods) | Rust engine only |
| core/consciousness-kernel-enhanced.ts | crates/cognition | CREATE (with mods) | Rust engine only |
| core/full-consciousness-integration.ts | crates/cognition | CREATE (with mods) | Rust engine only |
| core/language-system.ts | crates/cognition | CREATE | Rust engine only |
| core/somatic/SomaticBody.ts | crates/biology | CREATE (with mods) | Rust engine only |
| world/homestead.ts | crates/world | CREATE | Rust engine only |
| world/shed.ts | crates/world | CREATE | Rust engine only |
| world/vehicle.ts | crates/physics | CREATE | Rust engine only |
| chaos/ChaosSys.ts | crates/deterministic | ENHANCE | Rust engine only |
| core/time-source.ts | crates/deterministic | ENHANCE | Rust engine only |
| core/event-replay-engine.ts | crates/persistence | CREATE (with mods) | Rust engine only |

**All logic lives in Rust authority (`apps/engine`). No simulation logic in `apps/server` or `apps/web`.**

---

### 3.2 Determinism Constraints for Preservation

**All ported subsystems MUST maintain:**
- ✅ Identical state transitions under identical inputs (REPLAY_IDENTITY)
- ✅ Deterministic RNG usage (seeded, logged, auditable)
- ✅ No wall-clock influence (TimeSource injected, testable)
- ✅ Canonical serialization (blake3 hashing stable)
- ✅ Event sourcing (all changes logged)

**Gate:** Determinism test must prove `same_seed + same_events → identical_hash_sequence`

---

## PART 4: AUTHORITY MIGRATION PLAN

### 4.1 How TypeScript Authority Is Eliminated

**Current Violation (PASS 2):**
Simulation logic located in `apps/server` (TypeScript):
- `apps/server/src/sim/loop_engine.rs` (Rust, but in wrong location)
- `apps/server/src/world/` (world state management)
- `apps/server/src/storage.rs` (event processing)

**Elimination:**
1. **Phase 0 Step 1:** Create `apps/engine` (Rust binary)
2. **Phase 0 Step 2:** Move SimLoop logic from server → engine
3. **Phase 0 Step 3:** Create `crates/world`, `crates/physics`, `crates/biology` (pure Rust data & logic)
4. **Phase 0 Step 4:** Server becomes **stateless control plane:**
   - Validates InputEvents (schema only)
   - Orders and persists to append-only log
   - Fans out ObservationEvents via WebSocket
   - **CANNOT** compute outcomes or mutate state

**Proof of Elimination:**
- Static analysis: No `sim/`, `world/`, `physics/`, `biology/` code in `apps/server/src/`
- Authority test: Server cannot call any mutation function in `apps/engine`
- Code review: Verify server imports only validation and persistence crates

---

### 4.2 How Rust Becomes Sole World Mutator

**Structure (Non-Negotiable):**
```
apps/engine (Rust binary)
├── main.rs (boots, loops, ticks)
├── tick.rs (fixed-timestep loop, tick(dt) → state mutations)
└── calls into crates/
    ├── crates/world (state types, hashing)
    ├── crates/physics (movement, collision)
    ├── crates/biology (metabolism, hormones, immune)
    ├── crates/genetics (reproduction, phenotype)
    ├── crates/cognition (perception, planning, language)
    ├── crates/events (InputEvent validation)
    └── crates/persistence (snapshots, replay)
```

**Authority Law (Enforced):**
1. Only `apps/engine` owns `World` state (exclusive mutable reference)
2. Server passes InputEvents to engine via append-only log
3. Engine validates InputEvents deterministically (reject invalid)
4. Engine computes outcomes and emits ObservationEvents
5. Server persists ObservationEvents immutably, fans out to web

**Test:** `TEST-AUTHORITY-001` — Engine is sole tick source, server cannot create or modify any world type.

---

### 4.3 Event Pipeline Alignment

**End-to-End Flow (Locked):**

1. **Web → Server (InputEvent Request)**
   - User/admin submits action (move, chat, command)
   - Token attached, RBAC enforced

2. **Server (Validation & Persistence)**
   - Verify JWT via local JWKS
   - Enforce RBAC (who can send what event type)
   - Validate InputEvent schema
   - Normalize into canonical form
   - Append to Postgres `input_events` table with hash-chain
   - Return acknowledgment

3. **Engine (Processing & Authority)**
   - Read ordered InputEvents from `input_events` table at tick boundary
   - Apply deterministic pipeline:
     ```
     Perception → Intent → Volition → BioVeto → PhysicsValidate → PolicyValidate → Commit
     ```
   - Emit ObservationEvents (results, state changes, messages)
   - Compute `world_hash` checkpoint
   - Write both to Postgres

4. **Server → Web (Fanout)**
   - WebSocket fanout ObservationEvents to subscribed clients
   - Broadcast tick count and `world_hash` for live display

5. **Audit (Verification)**
   - `tools/audits` reads append-only event log
   - Replays deterministically (seed + InputEvents → identical state)
   - Verifies hash-chain integrity
   - Generates audit report

**Gates:** `TEST-PIPELINE-001` (events flow end-to-end), `TEST-HASH-CHAIN-001` (hashes verify)

---

## PART 5: PHASE-ALIGNED EXECUTION PLAN

### 5.1 Phase 0: Repository Foundation & Authority Lockdown

**Objective:** Lock Rust as sole authority. Establish determinism kernel. Boot offline stack.

**Duration:** 3–4 weeks

**Entry Conditions:**
- Markenz repo readable
- Gemini Universe audited (reuse classification complete)
- Blocking issues documented

**Key Deliverables:**

1. **BUILD RESOLUTION**
   - Fix Cargo edition2024 dependency (pin base64ct version)
   - Verify `cargo check`, `cargo build`, `cargo test` all succeed
   - **Gate:** `TEST-BUILD-001` (clean build, no warnings, release config)

2. **CREATE APPS/ENGINE (Rust Binary)**
   - Skeleton with Cargo.toml
   - Main loop entry point
   - Dependency on crates/ (to be created)
   - **Gate:** `TEST-ENGINE-BOOT-001` (engine starts without panic)

3. **CREATE CRATES/WORLD (Rust Library)**
   - Define `World` struct (authoritative container)
   - Define `Agent` struct with full state (consciousness, biology, memory)
   - Define `Asset` structs (House, Shed, Tool, Vehicle)
   - Implement canonical serialization (for hashing)
   - Implement `blake3` world hashing
   - **Gate:** `TEST-WORLD-HASH-001` (hash stable, deterministic)

4. **CREATE CRATES/EVENTS (Rust Library)**
   - Define `InputEvent` schema (validated by server)
   - Define `ObservationEvent` schema (emitted by engine)
   - Implement deterministic serialization
   - **Gate:** `TEST-EVENT-SCHEMA-001` (events serialize/deserialize identically)

5. **CREATE CRATES/PERSISTENCE (Rust Library)**
   - Define snapshot format (canonical state serialization)
   - Implement snapshot write to Postgres
   - Implement snapshot read from Postgres
   - Implement replay harness (load snapshot + events → identical state)
   - Implement hash verification
   - **Gate:** `TEST-SNAPSHOT-LOAD-001` (snapshot loads, replay equals full replay)

6. **IMPLEMENT GENESIS IN APPS/ENGINE**
   - Seeded terrain generation (deterministic from Markenz seed)
   - House, Shed, Vehicles static content
   - Gem-D and Gem-K agent initialization (from bridge snapshots)
   - Tools registry initialization
   - **Gate:** `TEST-GENESIS-001` (genesis world loads, all assets present)

7. **IMPLEMENT SIMLOOP IN APPS/ENGINE**
   - Fixed-timestep tick loop (20Hz default)
   - Tick advancement (monotonic u64 tick count)
   - Event processing from input_events table
   - World state mutation (via crates/world)
   - ObservationEvent emission
   - World hash checkpoint emission every N ticks
   - **Gate:** `TEST-TICK-001` (ticks advance, hashes computed, no panics for 1000 ticks)

8. **FIX DATABASE SCHEMA**
   - Create `input_events` table (append-only)
   - Create `observation_events` table (append-only)
   - Create `snapshots` table
   - Create `hash_checkpoints` table
   - Add constraints (no UPDATE/DELETE on immutable tables)
   - Add hash-chain foreign keys
   - **Gate:** `TEST-SCHEMA-001` (tables created, constraints enforced, migrations succeed)

9. **IMPLEMENT RNG AUDIT LOGGING**
   - Wrap all `ChaosStream.next_*()` calls with audit context
   - Log: `{ tick, subsystem, stream, callsite, value }`
   - Store audit records in append-only event log or separate table
   - **Gate:** `TEST-RNG-AUDIT-001` (audit records created, every RNG call logged)

10. **REMOVE SIMULATION LOGIC FROM APPS/SERVER**
    - Delete `apps/server/src/sim/` directory
    - Delete `apps/server/src/world/` directory
    - Delete `apps/server/src/storage.rs` (move to engine/persistence)
    - Server becomes control plane only (validation, ordering, persistence, fanout)
    - **Gate:** `TEST-SERVER-STATELESS-001` (server cannot mutate state, static analysis)

11. **WRITE DETERMINISM TESTS**
    - `TEST-DET-001`: Determinism replay test (same seed + InputEvents → identical hash sequence)
    - `TEST-SNAPSHOT-EQ-001`: Snapshot equivalence (snapshot replay == full replay)
    - `TEST-HASH-CHAIN-001`: Hash-chain integrity (verify chain unbroken)
    - `TEST-OFFLINE-001`: Offline execution (no network, no cloud APIs)
    - `TEST-IDENTITY-001`: Agent identity preservation (Gem-D/Gem-K decisions match Gemini)
    - **Gate:** All determinism tests pass with zero divergence

**Exit Criteria (ALL MUST PASS):**
- [ ] `cargo build --release` succeeds, no warnings
- [ ] `cargo test --all` passes 100%
- [ ] `docker compose up --build` boots all services offline
- [ ] TEST-BUILD-001 passes
- [ ] TEST-ENGINE-BOOT-001 passes
- [ ] TEST-WORLD-HASH-001 passes
- [ ] TEST-EVENT-SCHEMA-001 passes
- [ ] TEST-SNAPSHOT-LOAD-001 passes
- [ ] TEST-GENESIS-001 passes
- [ ] TEST-TICK-001 passes
- [ ] TEST-SCHEMA-001 passes
- [ ] TEST-RNG-AUDIT-001 passes
- [ ] TEST-SERVER-STATELESS-001 passes
- [ ] TEST-DET-001 passes (100% hash match)
- [ ] TEST-SNAPSHOT-EQ-001 passes
- [ ] TEST-HASH-CHAIN-001 passes
- [ ] TEST-OFFLINE-001 passes
- [ ] TEST-IDENTITY-001 passes (Gem-D/Gem-K match Gemini)
- [ ] No `TODO`, `FIXME`, `stub`, `panic!`, `unimplemented!` in critical paths
- [ ] Authority boundaries validated (code review + static analysis)
- [ ] AMP Principal-Level Auditor approval obtained

**Failure = STOP.** Do not proceed to Phase 1 until all criteria pass.

---

### 5.2 Phase 1: Deterministic Kernel & Replay Harness

**Objective:** Prove determinism formally via replay and snapshot equivalence.

**Duration:** 1–2 weeks (blocked until Phase 0 complete)

**Entry Conditions:**
- Phase 0 complete and approved
- Determinism tests passing
- Engine boots and ticks

**Key Deliverables:**

1. **IMPLEMENT DETERMINISTIC SCHEDULER**
   - Fixed timestep enforcement
   - Subsystem RNG streams (physics, biology, cognition, environment, governance)
   - Tick-indexed subsystem execution
   - **Gate:** `TEST-SCHED-001` (subsystems tick in deterministic order)

2. **IMPLEMENT CANONICAL WORLD HASHING**
   - Canonical serialization for:
     - Agent state (consciousness, biology, memory)
     - World state (assets, terrain chunks)
     - All collections (stable iteration order)
   - blake3 hash computed after each tick
   - Hash stored in `hash_checkpoints` table
   - **Gate:** `TEST-HASH-STABLE-001` (hash identical across multiple runs, same seed)

3. **IMPLEMENT SNAPSHOT WRITE/READ**
   - Snapshot format: full world state binary
   - Write every N ticks (configurable, default 1000)
   - Read and replay from snapshot
   - **Gate:** `TEST-SNAPSHOT-WRITE-001` (snapshots written, readable, valid)

4. **IMPLEMENT REPLAY-FROM-SNAPSHOT EQUALITY**
   - Load snapshot at tick T
   - Replay from T → T+M with identical InputEvents
   - Compare world_hash at each step
   - Ensure identical to full replay T → T+M
   - **Gate:** `TEST-REPLAY-EQ-001` (snapshot replay == full replay)

5. **IMPLEMENT GENESIS WORLD WITH ASSETS**
   - Markenz seed-based world generation
   - House, shed, tools, vehicles deterministic placement
   - Gem-D and Gem-K agents loaded from bridge snapshots
   - All state matches Gemini endpoint state
   - **Gate:** `TEST-GENESIS-ASSETS-001` (all assets present, state matches Gemini)

6. **IMPLEMENT ORDERED EVENT DELIVERY GUARANTEES**
   - Server strictly orders InputEvents by sequence number
   - Engine strictly processes InputEvents in order
   - Any out-of-order event is rejected deterministically
   - **Gate:** `TEST-ORDER-001` (out-of-order events rejected, in-order succeed)

7. **IMPLEMENT INTEGRITY VERIFICATION ENDPOINT**
   - Server provides `/api/integrity/status` endpoint
   - Returns:
     - Current tick
     - World hash
     - Hash chain status (valid/broken)
     - Divergence report (if any)
   - Accessible only to admin
   - **Gate:** `TEST-INTEGRITY-API-001` (endpoint works, returns correct data)

8. **WEB UI ENHANCEMENTS**
   - Hash-chain status panel (shows current hash, prev hash, chain validity)
   - Agent vitals + inner monologue streams (real-time consciousness output)
   - Event log explorer (search, filter, drill-down)
   - Tick counter and tick rate display
   - **Gate:** `TEST-UI-HASH-001` (hash displayed, matches server)

**Exit Criteria (ALL MUST PASS):**
- [ ] Deterministic scheduler implemented
- [ ] World hashing stable and deterministic
- [ ] Snapshots write and read correctly
- [ ] Snapshot replay equals full replay (TEST-REPLAY-EQ-001)
- [ ] Genesis world loads with all assets (TEST-GENESIS-ASSETS-001)
- [ ] Same seed + same events → identical hash timeline (cross-run test)
- [ ] Hash-chain integrity verified end-to-end
- [ ] Integrity verification API works
- [ ] Web UI displays hashes and events
- [ ] No performance regression vs Phase 0 baseline
- [ ] AMP Phase 1 audit report clean

**Failure = STOP.** Do not proceed to Phase 2.

---

### 5.3 Phase 2: World Representation v1 (Terrain + Entities)

**Objective:** Replace abstract world with deterministic spatial reality.

**Duration:** 2–3 weeks (blocked until Phase 1 complete)

**Key Deliverables:**
- Chunked deterministic terrain (32³ or 16³ chunks)
- Biomes v1 (temperature, humidity, biome type)
- Real mechanics (Move, Gather, Build, Mine)
- Constraints (reach, tools, energy, time, collisions)
- All deterministic and replay-identical

**Blocked Until:** Phase 1 complete, hash-chain verified, replay tests passing

---

### 5.4 Phase 3 & Beyond

**Blocked Until:** Phase 2 complete and audited

Phases 3–9 follow locked roadmap (MARKENZ_EXECUTION_ROADMAP_v2.md), each with hard gates and determinism proofs.

---

## PART 6: RISK & FAILURE CONTAINMENT

### 6.1 Identity Loss Risk

**Failure Mode:** Gem-D or Gem-K state corrupted, memories lost, identity unrecognizable.

**Mitigation:**
1. Store original Gemini state hash in genesis immutably
2. Create dual-format validator (Gemini JSON ↔ Markenz binary)
3. Round-trip test: deserialize Gem-D/Gem-K from Gemini JSON → serialize to Markenz binary → deserialize → serialize → verify bit-identical
4. Determinism test: Replay Markenz genesis with Gem-D/Gem-K → decisions at ticks 0–100 must match Gemini decisions under identical inputs
5. If any divergence detected: HALT immediately, produce divergence report, escalate to AMP

**Stop Condition:** If identity hash diverges, STOP Phase 0. Do not proceed without full state recovery.

---

### 6.2 Determinism Divergence Risk

**Failure Mode:** Same seed + same InputEvents → different world hash on rerun. Replay is not replaying.

**Mitigation:**
1. Implement determinism test (TEST-DET-001): run 100 ticks twice, compare hashes at each tick
2. If mismatch: record first divergent tick number
3. Produce divergence report:
   ```
   Divergence Report:
   - First divergent tick: T
   - Expected hash: H1
   - Actual hash: H2
   - Subsystem suspected: [check RNG audit log at T]
   - Root cause: [likely RNG seed, ordering, or wall-clock]
   ```
4. Run with platform-specific strace/ltrace to identify system call causing divergence
5. Fix identified root cause (deterministic RNG, timeout elimination, etc.)
6. Re-run test until passing

**Stop Condition:** If determinism divergence cannot be eliminated, STOP immediately. Escalate with full diagnostics.

---

### 6.3 Asset Corruption Risk

**Failure Mode:** House location wrong, Shed inventory missing, Tools durability uninitialized, Vehicles state broken.

**Mitigation:**
1. Create asset validation function: `fn validate_all_assets(world: &World) -> Result<()>`
   - Verifies house exists and has valid location
   - Verifies shed exists with non-empty inventory
   - Verifies all tools registered and durability in valid range [0, 1]
   - Verifies all vehicles present with valid state
2. Call in genesis immediately after world creation
3. Call before every snapshot write
4. Call after every snapshot load
5. If validation fails: panic with asset report

**Stop Condition:** If asset validation fails at any gate, STOP. Do not proceed without full asset recovery.

---

### 6.4 Authority Boundary Violation Risk

**Failure Mode:** Server or web UI mutates world state. Code paths from server call engine mutation functions.

**Mitigation:**
1. Structural enforcement: World state types **only in** `crates/world`, never in server
2. Static analysis: Scan for server imports of world mutation functions
   - If found: build fails immediately
3. Code review: All server changes reviewed for state mutation
4. Test: `TEST-AUTHORITY-001` — server cannot create or call any World mutation function
5. Authority graph: Document all inter-crate dependencies, verify no circular authority

**Stop Condition:** If authority boundary violated, STOP. Do not merge without full separation.

---

### 6.5 Determinism Gates (Go/No-Go Criteria)

**Phase 0 Must Pass:**
- [ ] Determinism replay test: 100 ticks, same seed, multiple runs, all hashes identical
- [ ] Snapshot equivalence: snapshot at tick 50 + 50 events == full run tick 100
- [ ] Hash-chain integrity: all prev_hash fields correct, no breaks
- [ ] RNG determinism: ChaosStream sequences bit-identical across platforms (Linux x64/arm64, macOS)
- [ ] Identity preservation: Gem-D/Gem-K decisions match Gemini at ticks 0–100
- [ ] Asset validation: all 6 assets present, valid, match Gemini state
- [ ] Authority: server cannot mutate state, static analysis confirms

**If ANY gate fails: STOP Phase 0. Do not proceed to Phase 1 without fixing all failures.**

---

## PART 7: WINDSURF HANDOFF CONTRACT

### 7.1 What Windsurf May Do

**Explicitly Authorized:**
1. Create all specified crates and binaries
2. Implement signatures specified in this plan
3. Write tests as specified
4. Commit code following this plan exactly
5. Run verification scripts
6. Report results to AMP auditor
7. Escalate blockers to AMP with evidence
8. Propose workarounds **only if** documented in escalation

**Judgment Authority:**
- Windsurf may choose implementation details (algorithms, library selection) **provided they maintain:**
  - Determinism (no wall-clock, no Math.random, seeded RNG only)
  - Authority boundaries (Rust only, no server mutation)
  - Replay equivalence (same input sequence → identical state)

---

### 7.2 What Windsurf May NOT Do

**Explicitly Forbidden:**
1. Reuse Gemini code without applying specified modifications (Tier 2 mods, Tier 3 rewrites)
2. Modify existing `apps/server`, `apps/web`, `infra/` code without explicit plan instruction
3. Implement TODO/FIXME/stub/mock code
4. Merge without passing all specified tests
5. Skip any phase gate criterion
6. Invent missing Gemini data (if something is unclear, escalate)
7. Change repo layout from locked structure
8. Add external dependencies without explicit approval
9. Merge code if AMP audit identifies critical violations
10. Proceed to next phase if current phase gates fail

---

### 7.3 Escalation Rules

**Windsurf must escalate to AMP auditor (HALT execution) if:**

1. **Ambiguity in Plan:** If any requirement is unclear or conflicts with existing code, **do not guess** — escalate with evidence
2. **Blocker Not Documented:** If a blocker (error, missing info) arises that is not covered in this plan, escalate with full context
3. **Determinism Divergence:** If TEST-DET-001 fails (hash mismatch), escalate with divergence report
4. **Build Failure:** If `cargo build` fails due to issue not covered in plan, escalate with build log
5. **Asset Corruption:** If asset validation fails, escalate with asset report
6. **Authority Violation:** If code review detects state mutation outside engine, escalate
7. **Test Failure:** If any specified test fails, produce test output and escalate
8. **Timeline Overrun:** If phase takes >2x estimated time, escalate for re-planning

**Escalation Format:**
```
FROM: Windsurf
TO: AMP Auditor
SUBJECT: Escalation — [Category]
CONTEXT: [What you were trying to do]
BLOCKER: [The specific issue]
EVIDENCE: [Build logs, code snippets, test output, etc.]
PROPOSAL: [Your suggested fix, if any]
```

---

### 7.4 Stop Conditions

**Execution STOPS if:**

1. **Any Phase Gate Fails:** If test or criterion fails, STOP and escalate
2. **Determinism Divergence:** If replay produces different hashes, STOP immediately
3. **Authority Violation Detected:** If server or web mutates state, STOP
4. **Asset Loss Detected:** If Gem-D/Gem-K/House/Shed incomplete, STOP
5. **Build Broken:** If cargo build fails, STOP until fixed
6. **AMP Auditor Directs:** AMP can halt at any time with written directive

**Upon STOP:**
- Do not proceed to next phase
- Do not commit incomplete work
- Produce full diagnostic report
- Escalate to AMP with evidence
- Wait for re-planning or approval to proceed

---

## PART 8: GO / NO-GO CRITERIA

### 8.1 Phase 0 Go Criteria (ALL MUST BE TRUE)

- [ ] Cargo build succeeds (release + test builds, zero warnings)
- [ ] All unit tests passing (cargo test --all)
- [ ] All integration tests passing (determinism, snapshot, hash-chain, RNG, identity)
- [ ] Docker compose boots all services, no errors
- [ ] Offline execution verified (no network access, tcpdump confirms)
- [ ] TEST-BUILD-001 passing (clean build)
- [ ] TEST-ENGINE-BOOT-001 passing (engine starts)
- [ ] TEST-WORLD-HASH-001 passing (hash stable)
- [ ] TEST-EVENT-SCHEMA-001 passing (events serialize correctly)
- [ ] TEST-SNAPSHOT-LOAD-001 passing (snapshots load, replay equals full)
- [ ] TEST-GENESIS-001 passing (genesis world loads with all assets)
- [ ] TEST-TICK-001 passing (ticks advance, hashes computed, no panics)
- [ ] TEST-SCHEMA-001 passing (database schema correct)
- [ ] TEST-RNG-AUDIT-001 passing (RNG calls logged)
- [ ] TEST-SERVER-STATELESS-001 passing (server cannot mutate state)
- [ ] TEST-DET-001 passing (determinism replay identical hashes)
- [ ] TEST-SNAPSHOT-EQ-001 passing (snapshot replay == full replay)
- [ ] TEST-HASH-CHAIN-001 passing (hash-chain integrity verified)
- [ ] TEST-OFFLINE-001 passing (no network, no cloud APIs)
- [ ] TEST-IDENTITY-001 passing (Gem-D/Gem-K decisions match Gemini)
- [ ] TEST-BRIDGE-001 passing (round-trip asset validation)
- [ ] TEST-PRESERVATION-001 passing (no state loss during integration)
- [ ] No `TODO`, `FIXME`, `stub`, `panic!`, `unimplemented!` in critical paths (static scan)
- [ ] Authority boundaries validated (code review: Rust only, no server mutation)
- [ ] Asset validation: House, Shed, Tools, Vehicles, Gem-D, Gem-K all present and valid
- [ ] Performance: No regression vs baseline (if baseline established)
- [ ] Security: No hardcoded secrets, no auth bypass, RBAC enforced
- [ ] AMP Principal-Level Auditor approval obtained

**Failure of ANY criterion = Phase 0 NO-GO. Do not proceed to Phase 1.**

---

### 8.2 Phase 0 No-Go Criteria (ANY of THESE = STOP)

- [ ] Determinism test fails (hash mismatch on rerun)
- [ ] Snapshot equivalence test fails (replay != full run)
- [ ] Build fails in any CI environment
- [ ] Authority boundary violated (server mutates state, code analysis confirms)
- [ ] Asset data loss detected (Gem-D/Gem-K/House/Shed incomplete or invalid)
- [ ] RNG diverges across platforms (determinism broken)
- [ ] Database migration fails or corrupts data
- [ ] Hash-chain broken (prev_hash mismatch detected)
- [ ] Panics occur in first 1000 ticks
- [ ] Performance regression >50% vs baseline
- [ ] AMP audit report identifies critical violations (authority, determinism, or asset loss)
- [ ] Build time >1 hour (performance issue, investigate)

**If ANY no-go criterion is true: STOP Phase 0 immediately. Escalate with evidence.**

---

## PART 9: FINAL DECISION

**STATUS: BINDING · EXECUTION-READY**

This plan is:
- ✅ **Binding:** All sections are law, not suggestions
- ✅ **Mechanically Enforceable:** Every step has success/failure criteria
- ✅ **Phased & Contained:** Risk is bounded per phase
- ✅ **Auditable:** Complete traceability to governing law
- ✅ **Fail-Closed:** Any violation halts execution
- ✅ **Directly Executable:** Windsurf can execute without interpretation

**Authority:**
- **Plan Owner:** ANTIGRAVITY (AMP)
- **Execution Authority:** Windsurf
- **Approval Authority:** AMP Principal-Level Auditor
- **Escalation Path:** KAIZA-MCP governing law

**Next Step:**
Execute Phase 0 as specified using this plan as the sole authority.

---

**END OF PLAN**

---

**SIGNATURE & AUTHORITY**

**Approved By:** ANTIGRAVITY (AMP / Planner)  
**Timestamp:** 2026-01-10  
**Authority:** KAIZA-MCP v2  
**Status:** BINDING & EXECUTION-BLOCKING  
**Plan ID:** MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_v2
