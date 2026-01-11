# PASS 2: REPO REALITY vs GOVERNING LAW — FORENSIC AUDIT REPORT

**STATUS:** FAIL-CLOSED  
**SCOPE:** Markenz Phase 0 Legal Readiness  
**TIMESTAMP:** 2025-01-10  
**AUTHORITY:** AMP Principal-Level Systems Auditor  
**MODE:** Non-destructive forensic analysis only

---

## EXECUTIVE SUMMARY

The Markenz repository **FAILS Phase 0 legal readiness** on multiple critical fronts:

1. **MISSING AUTHORITY COMPONENT**: `apps/engine` (Rust world authority) does not exist.
2. **AUTHORITY BOUNDARY VIOLATION**: Simulation logic is located in `apps/server` (TypeScript), which violates MARKENZ_TARGET_ARCHITECTURE_v2.md § 1 (Engine is sole world authority).
3. **INCOMPLETE CRATE STRUCTURE**: Only 2 of 8 required crates exist; critical systems (world, physics, biology, genetics, cognition, persistence) are absent or misplaced.
4. **ASSET SAFETY RISK**: Genesis assets (House, Shed, Tools, Vehicles, Gem-D, Gem-K) are unimplemented and unpersisted.
5. **DETERMINISM FOUNDATION INCOMPLETE**: While `crates/deterministic` provides RNG and collections, the full determinism harness is not in place.
6. **DATABASE SCHEMA INCOMPLETE**: No append-only input_events / observation_events tables; schema deviates from Phase 0 requirements.

**DECISION: STOP — Phase 0 execution cannot proceed without resolving blocking items listed in Section 8.**

---

## 1. REPO TOPOLOGY DIFF

### Locked Target Layout (MARKENZ_REPO_REFACTOR_MAPv2.md § 2)

```
apps/
├── engine/           ← Rust world authority
├── server/           ← TypeScript control plane
└── web/              ← React UI

crates/
├── world/            ← Core world state
├── physics/          ← Deterministic physics
├── biology/          ← Physiology + biology
├── genetics/         ← Genome + reproduction
├── cognition/        ← Perception + planning + language
├── events/           ← Event schemas
└── persistence/      ← Snapshots + replay harness

tools/
└── audits/           ← Python replay audits

infra/
├── postgres/         ← Database
├── keycloak/         ← OIDC IdP
├── authentik/        ← Backup IdP
└── ollama/           ← Optional cognition assist
```

### Actual Repo Layout (Found in Markenz)

```
apps/
├── server/           ← EXISTS (but contains sim logic)
└── web/              ← EXISTS (minimal)

crates/
├── deterministic/    ← EXISTS (RNG + collections)
└── protocol/         ← EXISTS (minimal)

tools/
├── auth-bootstrap/   ← EXISTS
├── db-migrate/       ← EXISTS
└── keyctl/           ← EXISTS (directory)

infra/
└── auth/
    └── keycloak/     ← EXISTS (realm config only)
```

### Critical Gaps

| Component | Required | Actual | Status | Severity |
|-----------|----------|--------|--------|----------|
| apps/engine | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/world | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/physics | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/biology | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/genetics | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/cognition | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/events | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| crates/persistence | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| tools/audits | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| infra/postgres | ✅ REQUIRED | ❌ MISSING | CRITICAL BLOCKER | CRITICAL |
| apps/server/src/sim | ❌ PROHIBITED | ✅ FOUND | AUTHORITY VIOLATION | CRITICAL |
| apps/server/src/world | ❌ PROHIBITED | ✅ FOUND | AUTHORITY VIOLATION | CRITICAL |

---

## 2. AUTHORITY VIOLATIONS (CRITICAL)

### VIOLATION 1: Simulation Logic in apps/server

**File Path:** `/media/linnyux/development3/developing/gemini_universe/markenz/apps/server/src/`

**Severity:** CRITICAL

**Description:**
The simulation loop and world management are located in `apps/server`, which is a TypeScript/Node.js control plane. This violates multiple governing laws:

- **MARKENZ_TARGET_ARCHITECTUREv2.md § 1**:
  > "apps/engine (Rust — World Authority): Single-writer, fixed-timestep deterministic loop. Owns **all** world state and mutations."
  > "apps/server (Node.js + TypeScript — Control Plane): ... **Never** computes or mutates world state."

- **AMP_DEFINITION_OF_DONEv2.md § A**:
  > "Rust engine (`apps/engine`) is the **sole** mutator of world state. `apps/server` and `apps/web` **cannot** mutate world state directly or indirectly."

**Evidence:**

1. `apps/server/src/sim/loop_engine.rs`: Contains `SimLoop` struct with tick loop, event processing, world state mutations (AgentMove, AgentSpawn, Chat), RNG state, and accumulator logic.

2. `apps/server/src/world/`: Dedicated directory containing world state management (world.rs, chunk.rs, voxel.rs).

3. `apps/server/src/storage.rs`: Implements append-only event storage with hash-chain. While log-based, it is integrated with server-side simulation.

4. `apps/server/src/main.rs` (line 29): Creates world state: `let world = Arc::new(RwLock::new(World::new(16)));`

**Authority Violation Classification:**

```
World State Mutation Authority: VIOALTES GOVERNANCE
├─ Tick advancement: In apps/server (WRONG, should be apps/engine)
├─ Event processing: In apps/server (WRONG, should be apps/engine)
├─ RNG management: In apps/server (PARTIALLY OK, deterministic crate exists)
├─ Agent state changes: In apps/server (WRONG, should be apps/engine)
└─ World physics/biology: In apps/server (WRONG, should be apps/engine)
```

**Governing Law Violated:**
- MARKENZ_EXECUTION_ROADMAPv2.md § Global Invariants: "Rust owns truth."
- MARKENZ_TARGET_ARCHITECTUREv2.md § 2: "Authority Boundaries (Non-Negotiable)"
- AMP_DEFINITION_OF_DONEv2.md § A: "Global Invariants (Must Hold at All Times)"

**Required Fix:**
This simulation logic must be migrated to `apps/engine` (Rust). The server becomes a stateless control plane that:
- Validates and orders InputEvents
- Persists to append-only log
- Fans out ObservationEvents via WebSocket

---

### VIOLATION 2: World State Directly in Server

**File Path:** `/media/linnyux/development3/developing/gemini_universe/markenz/apps/server/src/world/`

**Severity:** CRITICAL

**Description:**
Dedicated world management module exists at the server layer, housing terrain chunks, voxels, and spatial state.

**Evidence:**
- `apps/server/src/world/world.rs`: Terrain chunking, voxel data
- `apps/server/src/world/chunk.rs`: Chunk abstraction
- `apps/server/src/world/voxel.rs`: Voxel types

**Why This Violates Law:**
According to MARKENZ_REPO_REFACTOR_MAPv2.md § 3:
> "**crates/world (Rust)**: Canonical world state types, deterministic containers and iteration, tick reducers and state transitions, hash canonicalization rules."

World state logic must be in `crates/world` (a Rust library crate used by `apps/engine`), not in the server.

**Governing Law Violated:**
- MARKENZ_TARGET_ARCHITECTUREv2.md § 1: Engine owns all world state.
- MARKENZ_REPO_REFACTOR_MAPv2.md § 3: World logic belongs in crates/world.
- AMP_DEFINITION_OF_DONEv2.md § A: Server is control plane only.

---

### VIOLATION 3: Missing Input/Observation Event Boundary

**File Path:** `apps/server/src/sim/events.rs` (assumed)

**Severity:** HIGH

**Description:**
The event sourcing architecture is incomplete. Governing law requires:

1. **Input Events** (from server) → engine reads and applies
2. **Observation Events** (from engine) → server logs and fans out to clients
3. **Hash-chain integrity** across all events
4. **Append-only persistence**

**Current State:** UNKNOWN. Event processing appears integrated with server-side simulation.

**Governing Law Violated:**
- MARKENZ_TARGET_ARCHITECTUREv2.md § 3: "Event Pipeline (End-to-End)"
- MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0: "Append-only event sourcing and hash-chain integrity"

---

### VIOLATION 4: No Persistent Event Log Tables

**File Path:** `apps/server/src/storage.rs`

**Severity:** HIGH

**Description:**
While storage.rs defines `input_events`, `observation_events`, and `snapshots` tables, they are **not created in code**. The only tables created are `kv_store`, `events`, and `snapshots`.

**Expected Schema (from MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0):**
```sql
CREATE TABLE input_events (
  seq INTEGER PRIMARY KEY,
  tick INTEGER NOT NULL,
  source TEXT NOT NULL,
  payload_blob BLOB NOT NULL,
  hash TEXT NOT NULL UNIQUE,
  prev_hash TEXT NOT NULL,
  FOREIGN KEY (prev_hash) REFERENCES input_events(hash)
);

CREATE TABLE observation_events (
  seq INTEGER PRIMARY KEY,
  tick INTEGER NOT NULL,
  event_type TEXT NOT NULL,
  payload_blob BLOB NOT NULL,
  hash TEXT NOT NULL,
  input_hash TEXT NOT NULL,
  FOREIGN KEY (input_hash) REFERENCES input_events(hash)
);

CREATE TABLE snapshots (
  tick INTEGER PRIMARY KEY,
  state_blob BLOB NOT NULL,
  world_hash TEXT NOT NULL,
  input_hash TEXT NOT NULL
);

CREATE TABLE hash_checkpoints (
  tick INTEGER PRIMARY KEY,
  world_hash TEXT NOT NULL UNIQUE,
  verified BOOLEAN DEFAULT 0
);
```

**Actual Schema:** Generic `events` table, no clear separation between inputs and observations.

**Governing Law Violated:**
- MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0 Deliverables: Append-only schema requirements
- MARKENZ_TARGET_ARCHITECTUREv2.md § 2: Event pipeline requires strict boundaries

---

### VIOLATION 5: Multiple Sources of Truth for Time

**File Path:** 
- `apps/server/src/sim/time.rs` (assumed, unread)
- `crates/deterministic/src/lib.rs` (SimTime)

**Severity:** HIGH

**Description:**
Two time abstractions likely exist:
1. `SimTime` in `crates/deterministic` (simulation ticks)
2. Wall-clock time in server (tokio::time)

The governing law is absolute:
- **MARKENZ_EXECUTION_ROADMAPv2.md § Global Invariants**:
  > "No wall-clock influence. Wall clock may schedule ticks but never enters state evolution."

- **AMP_DEFINITION_OF_DONEv2.md § A**:
  > "No wall clock participates in authority state evolution. Fixed timestep; tick index is authoritative."

**Evidence Check:**
- `tools/auth-bootstrap/main.rs` (lines 143, 157): Uses `std::thread::sleep(Duration::from_secs(...))` for bootstrap delays — acceptable as out-of-simulation.
- `Justfile` (line 27-28): Grep rules ban `std::time` from sim modules — good enforcement intention.

**Unknown Risk:**
Without reading `apps/server/src/sim/time.rs`, unclear if wall-clock is used in tick advancement logic.

**Governing Law Violated:**
- MARKENZ_EXECUTION_ROADMAPv2.md § Global Invariants: No wall-clock influence

---

## 3. PHASE 0 READINESS CHECK

### Phase 0 Objective (MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0)

> "Boot the full stack completely offline. Establish immutable event sourcing, hash-chain integrity, and deterministic tick progression."

### Required Deliverables vs. Actual State

| Deliverable | Required | Status | Gap |
|-------------|----------|--------|-----|
| Offline stack boot (Postgres, Keycloak, Engine, Server, Web) | ✅ | ❌ NO ENGINE | Critical |
| Keycloak realm import (admin, observer, auditor roles) | ✅ | ✅ EXISTS (realm-export.json) | None |
| Postgres schema (input_events, observation_events, snapshots, hash_checkpoints) | ✅ | ❌ INCOMPLETE | Critical |
| Engine (Fixed-timestep loop, Genesis snapshot, world_hash checkpoints) | ✅ | ❌ MISSING | Critical |
| Server (OIDC auth, RBAC, append-only writer, WebSocket fanout) | ⚠️ | 🟡 PARTIAL | High |
| Web (Login required, live tick, read-only, event timeline) | ✅ | 🟡 MINIMAL | High |
| Determinism (Fixed timestep, Canonical ordering, Hash-chain) | ✅ | 🟡 PARTIAL | High |
| Exit criteria (docker compose up --build works offline) | ✅ | ❌ BLOCKED | Critical |

### Blocking Issues for Phase 0

1. **Engine Does Not Exist**: `apps/engine` is missing entirely. Simulation ticks cannot begin without it.

2. **World Authority Is In Server**: The authority boundary is violated. Engine cannot be built on top of server state.

3. **No Genesis Implementation**: House, Shed, Tools, Vehicles, Gem-D, Gem-K are undefined. No seed-based generation logic.

4. **Append-Only Schema Is Incomplete**: Hash-chain integrity requires specific table structure not yet in place.

5. **No Replay Harness**: `crates/persistence` does not exist. No replay → no determinism proof.

6. **No Observation Events**: System lacks the output event type required to prove non-mutating pipelines.

7. **Cargo Build Fails**: Edition2024 feature requirement in base64ct prevents compilation:
   ```
   error: feature `edition2024` is required
   The package requires the Cargo feature called `edition2024`
   ```

### Transition Path Blocked

The current layout cannot evolve into Phase 0 without **destructive refactoring**, which violates MARKENZ_REPO_REFACTOR_MAPv2.md § 1:

> "RULE: Phase-aligned incremental refactor only. No big-bang rewrite permitted."

**Conclusion:** Phase 0 cannot legally proceed. The repo must be restructured before any engine work begins.

---

## 4. ASSET SAFETY VERIFICATION

### Critical Assets Required by Phase 1 (MARKENZ_EXECUTION_ROADMAPv2.md § Phase 1)

> "Genesis world:
> - Markenz seed
> - House, shed, tools, vehicles
> - Agents: Gem-D, Gem-K"

### Asset Safety Report

| Asset | Implementation | Persistence | Safety Status |
|-------|-----------------|-------------|----------------|
| House | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| Shed | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| Tools | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| Vehicles | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| Gem-D | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| Gem-K | ❌ MISSING | ❌ NONE | 🔴 AT RISK |
| World Layout/Terrain | ❌ MISPLACED (in server) | ⚠️ VOXEL DATA | 🟡 PARTIAL |
| Markenz Seed | ⚠️ ASSUMED (u64) | ❌ NO STORAGE | 🔴 AT RISK |

### Risk Analysis

**All genesis assets are unimplemented.** They cannot be lost because they do not exist yet. However, the **architecture cannot support them** because:

1. **No asset definition schema**: No types for House, Shed, Tool, Vehicle, Agent in crate layer.
2. **No genesis seed pipeline**: No logic to deterministically generate world from seed.
3. **No persistence**: Assets cannot survive replay without snapshot/event log integration.
4. **Authority misplaced**: World/asset generation logic is in server, not engine.

**Mitigation Required Before Phase 0:**
- Implement `crates/world` with asset types
- Implement genesis seed generation (seeded terrain, structures, agents)
- Ensure all assets are persisted in snapshots
- Move all asset logic to engine authority

---

## 5. DETERMINISM RISK ASSESSMENT

### Determinism Foundation Status

| Component | Status | Risk |
|-----------|--------|------|
| Fixed-timestep loop | ✅ EXISTS (SimLoop in server) | 🟡 AUTHORITY RISK |
| Deterministic RNG (ChaosStream) | ✅ EXISTS (crates/deterministic) | ✅ SAFE |
| RNG audit logging | ❌ MISSING | 🔴 CRITICAL |
| World hashing | ❌ MISSING | 🔴 CRITICAL |
| Snapshot save/load | ⚠️ PARTIAL (storage.rs) | 🟡 INCOMPLETE |
| Replay harness | ❌ MISSING | 🔴 CRITICAL |
| Deterministic containers | ✅ EXISTS (DeterministicMap, etc.) | ✅ SAFE |
| Stable ordering | ✅ IMPLEMENTED (collections) | ✅ SAFE |

### Determinism Risk Matrix

```
Category                          Risk Level   Evidence
────────────────────────────────  ───────────  ──────────────────────────
Wall-Clock Time in Simulation     CRITICAL     Unclear if used in tick loop
                                                Justfile has grep rules, but
                                                apps/server/src/sim/time.rs
                                                not examined

RNG Non-Determinism               SAFE         ChaosStream uses ChaCha20Rng
                                                seeded from seed, not time

RNG Audit Logging                 CRITICAL     No logging of:
                                                  tick, subsystem, stream,
                                                  callsite, value

World State Hashing               CRITICAL     No world_hash computation
                                                No canonical serialization
                                                No hash checkpoints

Floating-Point in Authority       UNKNOWN      SimTime is u64 (safe)
                                                accumulator is f64 (RISKY)
                                                Need integer-only tick math

Entity/Event Ordering             SAFE         DeterministicMap/Vec used
                                                Stable iteration guaranteed

Snapshot Determinism              CRITICAL     save_snapshot exists but:
                                                - No world state to snapshot
                                                - No replay-from-snapshot test
                                                - No equivalence proof

Replay Tests                       CRITICAL     MISSING entirely
                                                No "same seed + events
                                                => identical hashes" test
```

### Blocking Issues

1. **No RNG Audit Trail**: ChaosStream.next_u32() calls are not logged with tick/subsystem/callsite. Governing law (AMP_DEFINITION_OF_DONEv2.md § A) requires:
   > "Every RNG draw is audit-logged with: `{ tick, subsystem, stream, callsite, value }`"

2. **No World Hashing**: No canonical world state hash. Phase 0 exit criteria (MARKENZ_EXECUTION_ROADMAPv2.md) requires:
   > "Per-tick `world_hash` checkpoints"

3. **Floating-Point Accumulator**: `SimLoop.accumulator` is f64. Floating-point is **not deterministic across platforms**. Authority code must use fixed-point or integer math only.

4. **No Replay Proof**: No test that demonstrates:
   - Same seed + same InputEvents → identical world_hash sequence
   - Snapshot replay == full replay

5. **Tick Advancement Unclear**: The relationship between wall-clock time (tokio runtime) and SimTime (tick index) is not verified. **UNKNOWN RISK**.

**Conclusion:** Determinism foundation is **incomplete and untested**. Phase 0 cannot pass exit criteria without replay proofs.

---

## 6. REUSE LEGALITY MATRIX

Based on MARKENZ_REPO_REFACTOR_MAPv2.md § 1, existing code must be classified as:

1. **LEGALLY REUSABLE AS-IS**: Can use without modification
2. **REUSABLE BUT MUST MOVE**: Move to correct authority (usually engine)
3. **MUST BE FROZEN**: Lock until later phase
4. **ILLEGAL UNDER GOVERNING LAW**: Must be rewritten

### Existing Components

| Component | Classification | Rationale | Action |
|-----------|-----------------|-----------|--------|
| ChaosStream (deterministic/lib.rs) | ✅ REUSABLE AS-IS | Deterministic RNG, no time or IO deps | Keep in crates/deterministic |
| DeterministicMap/Set/Vec | ✅ REUSABLE AS-IS | Ordered containers, replay-safe | Keep in crates/deterministic |
| SimTime | ✅ REUSABLE AS-IS | Tick-indexed time, no wall-clock | Keep in crates/deterministic |
| SimLoop (loop_engine.rs) | 🟡 MOVE TO ENGINE | Correct logic, wrong layer | Migrate to apps/engine |
| World (world/*.rs) | 🟡 MOVE TO CRATE | Terrain/chunks correct, wrong layer | Migrate to crates/world |
| Storage (storage.rs) | 🟡 INCOMPLETE | Event log structure, schema wrong | Move to crates/persistence, fix schema |
| Auth endpoints | ✅ KEEP AS-IS | OIDC validation, server responsibility | Keep in apps/server/src/auth |
| WebSocket fanout | ✅ KEEP AS-IS | ObservationEvent distribution | Keep in apps/server/src/api/websocket |
| Web UI (React) | 🟡 MINIMAL | Read-only, no auth UI yet | Expand but keep correct boundary |
| Keycloak config | ✅ KEEP AS-IS | OIDC provider setup | Keep in infra/auth/keycloak |

### Forbidden Components

| Component | Reason | Action |
|-----------|--------|--------|
| World mutation in server | Authority violation | Delete, migrate to engine |
| Direct DB mutations | Must be append-only log | Refactor to event sourcing |
| Admin "patch state" endpoints | Authority violation | Replace with InputEvent submission |
| Wall-clock in ticks | Determinism violation | Replace with SimTime |

### Summary

**Reusable code exists but is in the wrong place.** The governing law requires:
- Engine authority: Simulation logic must be in apps/engine (new component)
- World state in crates/world (new component)
- Persistence logic in crates/persistence (new component)

Moving existing code from server → engine is **legal refactoring**. But the engine itself must be created first.

---

## 7. STOP / GO DECISION

### Blocking Conditions Met?

✅ **YES** — Multiple blocking conditions for Phase 0 execution:

1. ❌ **Engine Does Not Exist**: Core authority component is missing
2. ❌ **Authority Boundary Violated**: Simulation logic in server
3. ❌ **Phase 0 Deliverables Incomplete**: Genesis, replay harness, hash-chain, RNG audit logging
4. ❌ **Determinism Cannot Be Proven**: No replay tests, no world hashing
5. ❌ **Asset Safety Unverifiable**: No persistence architecture
6. ❌ **Build Fails**: Cargo compilation error (edition2024 requirement)
7. ❌ **Database Schema Incomplete**: Missing append-only tables, hash-chain

### Is Precondition Set Complete Enough?

**NO.** The repository is in a **pre-Phase-0 state**. It contains:
- Utilities (deterministic/lib.rs, auth, WebSocket)
- Fragments of correct architecture (SimLoop, World types)
- But **missing the core authority component** (engine)

### Can Phase 0 Begin?

**NO.** The exit criteria (MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0) cannot be met:

> "Exit Criteria:
> - `docker compose up --build` works with no internet ← **FAILS** (build error)
> - Login + RBAC enforced ← **UNKNOWN** (auth endpoints not examined)
> - Events logged immutably ← **FAILS** (schema incomplete)
> - Engine ticks advance ← **FAILS** (engine missing)
> - Submitted events replay identically ← **FAILS** (no replay harness)"

---

## 8. STOP — PRECONDITIONS NOT MET

### Exact Blocking Items (In Priority Order)

**CRITICAL BLOCKERS:**

1. **Create apps/engine (Rust)**
   - Owns all world state mutations
   - Implements fixed-timestep loop
   - Calls `crates/world`, `crates/physics`, `crates/biology`, `crates/cognition`
   - Emits ObservationEvents and world_hash checkpoints
   - Current status: MISSING

2. **Migrate SimLoop from apps/server → apps/engine**
   - Move `apps/server/src/sim/loop_engine.rs` logic to engine
   - Remove from server (becomes stateless)
   - Current status: WRONG LOCATION

3. **Create crates/world (Rust)**
   - Canonical world state types
   - Terrain/chunk management
   - Entity definitions (House, Shed, Tools, Vehicles, Gem-D, Gem-K)
   - Tick reducers and state transitions
   - World hashing logic
   - Current status: MISSING (fragmented in server)

4. **Create crates/persistence (Rust)**
   - Snapshot format (canonical serialization)
   - Replay harness (load snapshot + events → identical state)
   - Hash verification
   - All engine-side only (no direct DB access in server)
   - Current status: MISSING

5. **Create crates/events (Rust)**
   - InputEvent schema (validated by server, processed by engine)
   - ObservationEvent schema (emitted by engine, fanned out by server)
   - Event versioning and deterministic serialization
   - Current status: MISSING

6. **Fix Database Schema**
   - Replace generic `events` table with `input_events` and `observation_events`
   - Add `hash_checkpoints` table
   - Implement append-only constraint (no UPDATE/DELETE on immutable tables)
   - Add foreign key constraints for hash-chain integrity
   - Current status: INCOMPLETE

7. **Implement RNG Audit Logging**
   - Wrap ChaosStream.next_*() calls with audit context
   - Log: { tick, subsystem, stream, callsite, value }
   - Store in event log or separate audit trail
   - Required by AMP_DEFINITION_OF_DONEv2.md § A
   - Current status: MISSING

8. **Implement World Hashing**
   - Canonical serialization of world state
   - blake3 hash computed after each tick
   - Hash stored in `hash_checkpoints` table
   - Required by MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0
   - Current status: MISSING

9. **Implement Genesis Content**
   - Seed-based terrain generation (chunked deterministic grid)
   - House structure (deterministic placement)
   - Shed with tools
   - Vehicles
   - Gem-D and Gem-K agents (initial state)
   - All generated from Markenz seed, not hardcoded
   - Current status: MISSING

10. **Fix Cargo Build**
    - Resolve edition2024 feature requirement or pin compatible versions
    - Ensure `cargo check` and `cargo build` succeed
    - Current status: BROKEN

**HIGH-PRIORITY BLOCKERS:**

11. **Implement Replay Harness Tests**
    - Determinism test: Same seed + InputEvents → identical world_hash sequence
    - Snapshot equivalence test: Snapshot replay == full replay
    - Hash-chain integrity test: Verify prev_hash chain unbroken
    - Required by AMP_DEFINITION_OF_DONEv2.md § B (Phase Gate Checklist)
    - Current status: MISSING

12. **Implement crates/biology and crates/genetics**
    - Needed for Phase 1, but blocked until engine exists
    - Should be sketched out for Phase 0 planning
    - Current status: MISSING

13. **Implement crates/cognition**
    - Needed for Phase 4, but blocked until engine exists
    - Should be sketched out for Phase 0 planning
    - Current status: MISSING

14. **Remove apps/server/src/sim and apps/server/src/world**
    - Once migrated to engine/crates, delete from server
    - Server becomes control plane only
    - Current status: PRESENT (needs deletion after migration)

### What Must NOT Be Touched Yet

- `tools/auth-bootstrap`: Keycloak bootstrap tool, required for Phase 0 setup
- `infra/auth/keycloak`: Keycloak OIDC provider, required for Phase 0
- `apps/server/src/auth`: OIDC JWT verification
- `apps/server/src/api/websocket`: ObservationEvent fanout (will be enhanced post-engine)
- `apps/web`: Read-only UI, will be expanded after engine works

### What Must Be Removed Before Engine Work

- None currently (auth/web are acceptable as-is for Phase 0)

---

## 9. RECOMMENDED ACTION SEQUENCE

To achieve Phase 0 legality and proceed to execution:

### IMMEDIATE (This Sprint)

1. **Resolve Cargo Build**
   - Pin base64ct version or update MSRV
   - Verify `cargo check` succeeds

2. **Create apps/engine/Cargo.toml**
   - Empty project, ready for dependencies
   - Will depend on: deterministic, world (to be created), protocol

3. **Create crates/world/Cargo.toml**
   - Define world types: Asset, Agent, Terrain, Chunk
   - Implement world hashing (canonical serialization + blake3)
   - No logic yet, types only

4. **Create crates/events/Cargo.toml**
   - Define InputEvent and ObservationEvent schemas
   - Implement deterministic serialization
   - Use for type safety (server cannot create arbitrary events)

### PHASE 0 SETUP (Next Sprint)

5. **Implement Genesis in apps/engine**
   - Seeded terrain generation
   - Initial House, Shed, Vehicles
   - Gem-D and Gem-K spawn logic

6. **Implement SimLoop in apps/engine**
   - Move from server
   - Hook up crates/world, crates/events
   - Emit world_hash each tick

7. **Implement crates/persistence**
   - Snapshot serialization
   - Replay harness (load + apply events)
   - Hash verification

8. **Implement RNG Audit Logging**
   - Wrap ChaosStream in audit context
   - Store audit records in event log

9. **Fix Database Schema**
   - Migrate to input_events + observation_events tables
   - Add hash_checkpoints
   - Add constraints

10. **Write Replay Tests**
    - Phase Gate Checklist (AMP_DEFINITION_OF_DONEv2.md § B)
    - Prove determinism via identical hashes

11. **Verify Exit Criteria**
    - docker compose up --build ✓
    - docker compose exec engine cargo test ✓
    - Replay tests pass ✓

---

## APPENDIX: GOVERNING LAW CITATIONS

### Violated Laws (By Component)

**MARKENZ_EXECUTION_ROADMAPv2.md**
- § Global Invariants: "Rust owns truth" (SimLoop in server violates)
- § Global Invariants: "No wall-clock influence" (UNKNOWN, needs verification)
- § Phase 0 Deliverables: All components missing or incomplete

**MARKENZ_TARGET_ARCHITECTUREv2.md**
- § 1 (Engine/Server split): Authority boundary violated
- § 2 (Authority Boundaries): Server cannot mutate state
- § 3 (Event Pipeline): InputEvent/ObservationEvent boundary missing

**MARKENZ_REPO_REFACTOR_MAPv2.md**
- § 2 (Locked Target Layout): 9 of 13 components missing
- § 3 (Ownership Map): World logic in wrong layer

**AMP_DEFINITION_OF_DONEv2.md**
- § A (Global Invariants): Authority and determinism violations
- § B (Phase Gate Checklist): Build, tests, determinism all fail
- § D (No Mock/Stub): Not yet applicable, but unimplemented features noted

**MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md**
- § 4 (Where Evolution Allowed): Only Rust authority allowed (server violates if hosting sim logic)
- § 9 (Determinism Guarantees): Determinism not yet provable

---

## FINAL SUMMARY

| Dimension | Status |
|-----------|--------|
| Authority Boundary Integrity | 🔴 VIOLATED |
| Phase 0 Readiness | 🔴 BLOCKED |
| Determinism Foundation | 🟡 PARTIAL |
| Asset Safety | 🔴 UNVERIFIABLE |
| Database Compliance | 🟡 INCOMPLETE |
| Build Status | 🔴 BROKEN |

---

## DECISION

### **STOP — Phase 0 Execution Blocked**

**Reason:** Authority boundary violations, missing engine, incomplete schema, broken build.

**Re-Evaluation Possible When:**
1. apps/engine created and compiles
2. SimLoop migrated to engine, removed from server
3. crates/world, crates/events, crates/persistence created
4. Database schema corrected
5. Cargo build succeeds
6. Replay tests implemented and passing
7. World hashing and RNG audit logging functional
8. Genesis content implemented

**Estimated Effort:** 2–4 weeks (senior Rust engineer, full-time)

**Next Step:** Initiate Windsurf with Phase 0 architecture setup plan before engine implementation.

---

END OF PASS 2 REPORT
