---
status: APPROVED
---

# PLAN_PHASE_0_NORMALIZED
## Repo + Offline Stack Baseline Closure

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 0 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.1)  
**EXECUTOR:** Windsurf  
**GATE AUTHORITY:** AMP Principal-Level Auditor

---

## 1. AUTHORITY DECLARATION

This file is the **sole execution authority** for Phase 0.

**Higher Authorities (in order):**
1. KAIZA-MCP (governance system)
2. MARKENZ_EXECUTION_ROADMAPv2.md (global roadmap)
3. MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (master plan, phases 0–9)
4. MARKENZ_TARGET_ARCHITECTUREv2.md (locked architecture)
5. AMP_DEFINITION_OF_DONEv2.md (quality gates)
6. ADDENDUM_WORLD_PRESERVATION_v1.md (asset preservation)
7. ADDENDUM_IDENTITY_CONTINUITY_v1.md (identity continuity)

This file defers to all higher authorities. Windsurf executes this phase as specified.

---

## 2. PHASE SCOPE (LOCKED)

**Objective:** Boot the full stack completely offline. Establish immutable event sourcing, hash-chain integrity, and deterministic tick progression.

**Deliverables (EXACT):**
- Offline stack boot (Postgres, Keycloak, Rust engine, TypeScript server, React web UI)
- Keycloak realm import (roles: admin, observer, auditor)
- PostgreSQL schema (append-only tables: input_events, observation_events, snapshots, hash_checkpoints)
- Rust engine (fixed-timestep loop, genesis snapshot, per-tick world_hash checkpoints)
- TypeScript server (OIDC auth, RBAC, event logging with hash-chain, WebSocket fanout)
- React web UI (login, live tick display, world_hash display, read-only event timeline, admin InputEvent form)

---

## 3. EXPLICIT NON-SCOPE (FORBIDDEN IN THIS PHASE)

Windsurf **MUST NOT** implement:
- Terrain generation or spatial world representation (Phase 2)
- Biology simulation (Phase 3)
- Cognition or planning (Phase 4)
- Genetics or reproduction (Phase 5)
- Social dynamics or scaling (Phase 6)
- Governance or economy (Phase 7)
- WebGPU rendering (Phase 8)
- Security hardening beyond Phase 0 baseline (Phase 9)
- Architectural decisions beyond those specified
- New feature invention
- Scope expansion into future phases

---

## 4. PRESERVATION CLAUSES (BINDING)

### 4.1 World Asset Preservation
Per ADDENDUM_WORLD_PRESERVATION_v1.md:
- House (Homestead) location and structure must be preserved from Gemini export
- Shed (Tool Storage) location and structure must be preserved from Gemini export
- All tools must retain durability states from Gemini export
- All vehicles must retain condition and location from Gemini export
- No regeneration, no replacement, no abstraction loss

### 4.2 Identity Preservation
Per ADDENDUM_IDENTITY_CONTINUITY_v1.md:
- Gem-D (Dylan) must be resumed with exact identity, memories, skills, personality from Gemini export
- Gem-K (Kai) must be resumed with exact identity, memories, skills, personality from Gemini export
- No blank slates, no skill reset, no personality overwrite
- Identity fingerprinting: blake3(Agent_State) must verify provenance

### 4.3 Determinism-Safe Preservation
- Genesis snapshot at tick 0 must include exact imported state
- Import functions are pure (same export JSON → bit-identical genesis state)
- Replay from genesis snapshot must produce identical Agent/Asset structures

---

## 5. DETERMINISM REQUIREMENTS

### 5.1 Time Representation
- Simulation time = tick index (u64, starting at 0)
- Fixed dt defined in genesis config (e.g., 50ms per tick = 20Hz)
- Wall clock never influences state evolution
- Wall clock only schedules tick cadence

### 5.2 RNG Initialization
- Root seed: Markenz genesis seed (constant, derived from Gemini export)
- SubSystem streams: 6 streams initialized
  - Physics (stream_id=0), Environment (stream_id=1), Biology (stream_id=2)
  - Cognition (stream_id=3), Genetics (stream_id=4), Governance (stream_id=5)
- Algorithm: ChaCha20 (RFC 7539)
- RNG draws audit-logged: { tick, subsystem, stream, callsite, value }

### 5.3 Ordering Rules
- Event ordering: canonical lexicographical by (Tick, SourceId, Sequence)
- Entity iteration: BTreeMap (deterministic sorted order)
- No HashMap/HashSet in authority state

### 5.4 Hash Participation
- world_hash computed after each tick using blake3(PrevHash || CanonicalSerialize(State))
- Checkpoints stored at fixed cadence (default every 100 ticks)
- Hash-chain: each event includes hash of previous event
- Genesis hash computed from imported state (deterministic)

### 5.5 Replay Guarantees
- Replay requirement: Seed + ordered InputEvents → identical world_hash sequence
- Snapshot equivalence: Snapshot at tick T + remaining events = Full run from tick 0
- Platform independence: Identical hashes across Linux (x64/arm64) and macOS

---

## 6. IMPLEMENTATION OBLIGATIONS (ANTI-FAKE)

### 6.1 InputEvent Schema & Persistence

**Causal Input:** User/admin submits InputEvent via /api/input-event (POST with JWT)  
**State Mutated:** input_events table (append-only)  
**Hash Impact:** Hash-chain links to previous event  
**Replay Proof:** Replay from DB must apply same InputEvents in canonical order

Anti-Fake: No UPDATE/DELETE on input_events; foreign key constraint enforces hash-chain; validation before append

### 6.2 ObservationEvent Schema & Emission

**Causal Input:** Engine tick processes InputEvents  
**State Mutated:** observation_events table (append-only)  
**Hash Impact:** Audit trail (not hash-affecting)  
**Replay Proof:** Same InputEvents → identical ObservationEvents

Anti-Fake: Payload includes (type, old_val, new_val) for state diffs; well-formed schema validation

### 6.3 Snapshot Creation & Verification

**Causal Input:** Engine tick counter triggers snapshot (every N ticks)  
**State Mutated:** snapshots table (append-only)  
**Hash Impact:** Includes world_hash checkpoints  
**Replay Proof:** Load snapshot at tick T + apply events = Full run hash

Anti-Fake: Format (tick, state_blob, world_hash, input_hash, checksum); bincode serialization; versioning

### 6.4 World Hash Checkpoints

**Causal Input:** Engine tick completes  
**State Mutated:** hash_checkpoints table (append-only)  
**Hash Impact:** Computed from current state  
**Replay Proof:** Checkpoint sequence must match replay exactly

Anti-Fake: blake3(PrevHash || CanonicalSerialize(World)); big-endian; frequency every 100 ticks

### 6.5 Authority Pipeline (Bootstrap Phase 0)

**Causal Input:** Ordered InputEvents from input_events table  
**State Mutated:** Universe (agents, chunks, assets) and state hash  
**Hash Impact:** Each mutation updates world_hash  
**Replay Proof:** Same events replay identically

Anti-Fake: Pipeline: Fetch → Validate → Apply → Diff → Emit → Hash → Log; no wall-clock

---

## 7. REQUIRED ARTIFACTS (WINDSURF OUTPUTS)

**Report Filename:** WINDSURF_PHASE_0_EXECUTION_REPORT.md  
**Absolute Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_0_EXECUTION_REPORT.md

**Report Must Include:**
- Build status (cargo build --release result)
- Test results (all unit + integration tests)
- Determinism test results (TEST-DET-001)
- Snapshot equivalence (TEST-SNAPSHOT-EQ-001)
- Hash-chain integrity (TEST-HASH-CHAIN-001)
- Docker compose boot log
- Schema verification
- RBAC verification
- Performance baseline
- Authority boundary verification
- AMP sign-off status

---

## 8. EXIT CRITERIA (VERIFIABLE)

**ALL REQUIRED. Failure = Phase 0 BLOCKED.**

### Build & Compilation
- [ ] cargo build --release succeeds with zero warnings
- [ ] cargo test --all passes
- [ ] All Rust crates compile individually
- [ ] No clippy warnings in critical paths
- [ ] Docker images build without errors

### Determinism & Replay
- [ ] TEST-DET-001: 3+ identical runs produce identical hashes (100+ ticks each)
- [ ] TEST-SNAPSHOT-EQ-001: Snapshot replay matches full run
- [ ] TEST-HASH-CHAIN-001: All input_events have valid hash-chain
- [ ] Hashes bit-identical across runs

### Authority & Boundaries
- [ ] Server makes NO world state mutations
- [ ] Web makes NO world state mutations
- [ ] Authority pipeline is Rust-only
- [ ] No shared mutable state

### Schema & Persistence
- [ ] PostgreSQL schema exists: input_events, observation_events, snapshots, hash_checkpoints
- [ ] Append-only constraint enforced
- [ ] Database migration succeeds
- [ ] Hash-chain foreign keys functional

### Infrastructure (Offline-First)
- [ ] docker compose up --build succeeds without internet
- [ ] All containers start successfully
- [ ] Keycloak realm imported; test users authenticate
- [ ] PostgreSQL initialized with schema
- [ ] Engine boots and logs first 10 ticks

### Security & RBAC
- [ ] Observer role denied InputEvent submission
- [ ] Admin role allowed InputEvent submission
- [ ] JWT verification via local JWKS
- [ ] No hardcoded secrets
- [ ] Auth required for /api/input-event

### Observability
- [ ] Genesis snapshot emitted at boot
- [ ] Per-tick world_hash checkpoints logged
- [ ] All InputEvents logged immutably
- [ ] WebSocket /api/events streams ObservationEvents
- [ ] Event timeline visible in UI

### AMP Sign-Off
- [ ] AMP Principal-Level Auditor approval in writing BEFORE Phase 1

---

## 9. DETERMINISM & REPLAY GATES

**Gate 1: Identical Hash Sequences (TEST-DET-001)**
- Condition: Run engine 3 times with identical seed + InputEvents
- Expected: All produce identical hashes (ticks 0–100)
- Failure: STOP; escalate with divergence report

**Gate 2: Snapshot Equivalence (TEST-SNAPSHOT-EQ-001)**
- Condition: Full run vs snapshots at ticks 250, 500, 750
- Expected: Hashes match exactly for resumed ticks
- Failure: STOP; escalate with divergent tick

**Gate 3: Hash-Chain Integrity (TEST-HASH-CHAIN-001)**
- Condition: Verify all input_events hash-chain
- Expected: hash[n] = blake3(payload[n] || hash[n-1])
- Failure: STOP; escalate with broken link

---

## 10. HARD STOP CONDITIONS

Execution STOPS IMMEDIATELY if:

1. Determinism test fails (hash divergence)
2. Snapshot equivalence fails
3. Build fails
4. Authority boundary violated
5. Asset data loss detected
6. RNG diverges across runs
7. Database migration fails
8. Hash-chain broken
9. Panic in first 100 ticks
10. Performance regression >50%
11. AMP auditor directs halt

**Upon HARD STOP:** Do NOT proceed to Phase 1; escalate to AMP with evidence.

---

## 11. NO-MOCK / NO-STUB ENFORCEMENT

Per AMP_DEFINITION_OF_DONEv2.md:

- Reject: TODO, FIXME, stub, mock, fake, placeholder
- Reject: unimplemented!(), todo!()
- Reject: type bypass pragmas in gated code
- Every feature must emit observable events
- If feature not in logs, it does not exist

---

**END OF PHASE 0 NORMALIZED PLAN**
