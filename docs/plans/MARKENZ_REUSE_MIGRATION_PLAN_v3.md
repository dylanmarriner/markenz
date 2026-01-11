---
status: APPROVED
plan_type: reuse-migration
authority: antigravity
blocks_execution_without: true
---

# MARKENZ REUSE MIGRATION PLAN v3 (MECHANICAL FIX)

MODE: PLANNER · MCP-ENFORCED · FAIL-CLOSED  
AUTHORITY: ANTIGRAVITY (SOLE PLANNING AUTHORITY)  
OUTPUT: SINGLE APPROVED PLAN (.md)  
TARGET: RESOLVE ALL AMP v2 BLOCKERS  
EXECUTION BLOCKING: TRUE  

---

## 0. OBJECTIVE (ABSOLUTE)

You are **Antigravity**.

Your task is to generate **MARKENZ_REUSE_MIGRATION_PLAN_v3.md**, a **mechanically enforceable, CI-verifiable reuse migration plan** that resolves **every blocking issue** identified in:

- `AMP_AUDIT_MARKENZ_REUSE_MIGRATION_PLAN_v2.md`

This plan must PASS AMP without interpretation.

If any requirement is underspecified, subjective, or non-executable → **FAIL CLOSED**.

---

## 1. BINDING INPUTS (READ, DO NOT DEBATE)

You MUST obey:

1. **AMP_AUDIT_MARKENZ_REUSE_MIGRATION_PLAN_v2.md**
   - Every blocker must be explicitly addressed

2. **MARKENZ_UNIFIED_MASTER_PLAN.md**
   - Deterministic lockstep
   - Single-universe authority
   - Offline-only
   - Rust-authoritative simulation

3. **KAIZA-MCP RULES**
   - No executor inference
   - No TODOs / stubs
   - CI-enforced laws only
   - If enforcement cannot be automated → it is invalid

---

## 2. DETERMINISTIC INTERFACES (FULLY SPECIFIED — FIXES BLOCKER GROUP 1)

You MUST define the following as **formal, testable contracts** (NO CODE, NO PSEUDOCODE).

### 2.1 TimeSource Contract

**Contract:**
- **Type Definition**: `type SimTime = u64; // Monotonic tick count starting at 0`
- **Tick Duration**: `fn dt(&self) -> Duration` returns a CONSTANT value defined by the genesis configuration (e.g., 50ms for 20Hz). It NEVER varies.
- **Monotonicity**: `now()` calls within the same tick MUST return the exact same `SimTime` value.
- **Initial State**: Genesis starts at `SimTime = 0`.
- **Serialization**: `SimTime` is serialized as a 64-bit unsigned integer.
- **Replay Invariants**: Replaying `Tick N` must always yield `SimTime == N`.
- **Forbidden**: usage of `std::time::SystemTime`, `chrono::Utc::now`, `Date.now()`, or any OS clock.

### 2.2 ChaosStream / RNG Contract

**Contract:**
- **Algorithm**: ChaCha20 (RFC 7539). NO fallback to PCG or platform RNG.
- **Stream Identity**: Each subsystem receives a unique `ChaosStream` identified by a deterministic `SubsystemId`.
- **Global Seed**: Derived deterministically from the Universe Genesis Hash.
- **Stream Derivation**: `StreamSeed = blake3(GlobalSeed || SubsystemId || ChunkCoord)`.
- **Serialization**: `ChaosStream` state must be fully serializable to allow exact resumption.
- **CI Verification**: `next_float()` series from `Seed X` must be bit-identical across Linux (x64/arm64) and macOS.

### 2.3 EventBus Contract

**Contract:**
- **Structure**: `struct SimEvent { tick: u64, source_id: u32, sequence: u32, payload: Vec<u8> }`.
- **Queue**: A single authoritative `BTreeMap<(Tick, SourceId, Sequence), SimEvent>`.
- **Dispatch**: Events published at `Tick N` are strictly immutable and delivered at the START of `Tick N+1`.
- **Ordering**: Strict lexicographical ordering by `(Tick, SourceId, Sequence)`.
- **Atomicity**: State mutation phase is distinct from Event processing phase. No event sees partial state updates.
- **Replay**: Feeding the same event log must result in identical state transitions.

### 2.4 Persistence Contract

**Contract:**
- **Hash Type**: `LogHash = [u8; 32]` (BLAKE3).
- **Append**: `fn append(&mut self, events: &[SimEvent]) -> Result<LogHash>`.
- **Chain invariant**: `NewHash = blake3(PrevHash || CanonicalSerialize(Events))`.
- **Snapshot Cadence**: Strictly every `N` ticks (e.g., 1000). Defined in Config.
- **Verification**: `fn verify_chain(&self) -> Result<()>` walks the log and recomputes hashes.
- **Failure**: Any hash mismatch results in an immediate `Panic` (Fail-Closed).

---

## 3. ACCEPTANCE TESTS (EXECUTABLE — FIXES BLOCKER GROUP 2)

Rewrite ALL acceptance tests in **machine-executable form**.

### Test 4.1: Determinism Replay Test
- **Test ID**: `TEST-DET-001`
- **Command**: `cargo test --package markenz_core test_determinism_replay_100`
- **Inputs**:
    - Genesis Seed: `0xDEADBEEF`
    - Ticks: 100
    - Input Events: None
- **Expected Outputs**:
    - Final State Hash: `0x1234...` (Reference Hash)
- **Exact Failure Signal**: Any divergence in hash between run 1 and run 2, or run 1 and Reference Hash. Output full diff of state JSON.

### Test 4.2: Time Isolation Scan
- **Test ID**: `TEST-ISO-001`
- **Command**: `markenz/tools/audit/scan_time_violations.sh`
- **Rule**: `rg "Date\.now|SystemTime::now|check_time|Instant::now" src/core` must return 0 matches outside of `src/core/time/real_time_source.rs`.
- **Inputs**: `src/core` directory.
- **Expected Outputs**: Exit code 0.
- **Exact Failure Signal**: Non-zero match count. Print offending files and lines.

### Test 4.3: Offline-Only Scan
- **Test ID**: `TEST-OFF-001`
- **Command**: `markenz/tools/audit/scan_network_violations.sh`
- **Rule**: `rg "fetch\(|axios|reqwest|http::|hyper::" src/core` must return 0 matches in simulation logic.
- **Inputs**: `src/core` directory.
- **Expected Outputs**: Exit code 0.
- **Exact Failure Signal**: Non-zero match count. Print offending import.

### Test 4.4: Chaos Stability Test
- **Test ID**: `TEST-RNG-001`
- **Command**: `cargo test --package markenz_core test_chaos_stability`
- **Inputs**:
    - Seed: `0xCAFEBABE`
    - Calls: 1000 `next_float()`
- **Expected Outputs**:
    - Sequence matches `tests/fixtures/chaos_baseline.json` exactly.
- **Exact Failure Signal**: Mismatch at index N. Expected X, Got Y.

### Test 4.5: Single-Universe Enforcement
- **Test ID**: `TEST-UNI-001`
- **Command**: `rg "static mut|lazy_static" src/core`
- **Rule**: No global mutable state.
- **Inputs**: `src/core`
- **Expected Outputs**: 0 matches (except for controlled `Universe` root in `main.rs`).
- **Exact Failure Signal**: Found global state declaration.

### Test 4.6: Zero-Stub Scan
- **Test ID**: `TEST-STUB-001`
- **Command**: `markenz/tools/audit/scan_stubs.sh`
- **Rule**: `rg "TODO|FIXME|HACK|unimplemented!|panic!|@ts-ignore" src/`
- **Inputs**: Entire codebase.
- **Expected Outputs**: 0 matches.
- **Exact Failure Signal**: Found stub or placeholder.

### Test 4.7: Encryption Verification
- **Test ID**: `TEST-ENC-001`
- **Command**: `cargo test --package markenz_persistence test_db_encryption`
- **Inputs**: New DB created with key.
- **Action**: Attempt to read DB file with standard `sqlite3` without key.
- **Expected Outputs**: `sqlite3` returns "file is encrypted or is not a database".
- **Exact Failure Signal**: File is readable as plaintext.

---

## 4. MIGRATION PHASES WITH HARD GATES (FIXES BLOCKER GROUP 3)

### Phase 1: Foundation (TimeSource, ChaosSys, EventBus)
- **Phase ID**: `MIG-PHASE-1`
- **Entry Conditions**:
    - Empty `crates/markenz_core` initialized.
- **Allowed Actions**:
    - Implement `TimeSource` (Section 2.1).
    - Implement `ChaosStream` (Section 2.2).
    - Implement `EventBus` (Section 2.3).
    - Implement `Persistence` (Section 2.4).
- **Exit Conditions (ALL MUST PASS)**:
    - [ ] `TEST-ISO-001` (Time Scan) propagates 0 errors.
    - [ ] `TEST-RNG-001` (Chaos Stability) passes.
    - [ ] `TEST-ENC-001` (Encryption) passes.
- **Explicit STOP**: If `TEST-RNG-001` fails, **HALT**. Do not proceed to Biology.

### Phase 2: Biological Substrate
- **Phase ID**: `MIG-PHASE-2`
- **Entry Conditions**:
    - Phase 1 Complete.
    - `TEST-ISO-001` Passing.
- **Allowed Actions**:
    - Port `Metabolism`, `Hormones`, `Immune`, `Vitals` to Rust.
    - **Prohibited**: any logic change. Only syntax port.
- **Exit Conditions (ALL MUST PASS)**:
    - [ ] `cargo test` on all biology modules.
    - [ ] `TEST-OFF-001` (Offline Scan) passes.
    - [ ] `TEST-STUB-001` (Zero Stubs) passes.
- **Explicit STOP**: If any TODO is found, **HALT**.

### Phase 3: Conscious Loop
- **Phase ID**: `MIG-PHASE-3`
- **Entry Conditions**:
    - Phase 2 Complete.
- **Allowed Actions**:
    - Implement `FreeWillDecisionLoop` using `TimeSource` and `ChaosStream`.
    - Implement `ConsciousnessKernel` using `EventBus`.
- **Exit Conditions (ALL MUST PASS)**:
    - [ ] `TEST-DET-001` (Replay) passes 100%.
- **Explicit STOP**: If Replay hash diverges, **HALT**.

### Phase 4: World & Persistence Integration
- **Phase ID**: `MIG-PHASE-4`
- **Entry Conditions**:
    - Phase 3 Complete.
- **Allowed Actions**:
    - Implement `WorldService`.
    - Wire up `Persistence` log.
- **Exit Conditions (ALL MUST PASS)**:
    - [ ] Full Integration Test passes (Boot -> Tick 500 -> Snapshot -> Restore -> Tick 600).
- **Explicit STOP**: If Hash Chain invalid, **HALT**.

---

## 5. MASTER PLAN LAW → CI RULE MAPPING (FIXES BLOCKER GROUP 4)

| Markenz Law | CI Test / Command | Failure Action |
|------------|--------|----------------|
| **Offline-Only** | `rg "fetch|axios|http" src/core` => 0 matches | **Build Fail** |
| **Determinism** | `cargo test test_determinism_replay_100` | **Halt Pipeline** |
| **Single Universe** | `rg "static mut" src/core` => 0 matches | **Build Fail** |
| **No Globals** | `rg "lazy_static" src/core` => 0 matches | **Build Fail** |
| **Encryption-at-rest** | `cargo test test_db_encryption` | **Refuse Start** |
| **Zero Stubs** | `rg "TODO|FIXME|HACK" src/` => 0 matches | **Build Fail** |
| **No Implicit Time** | `rg "Date.now" src/core` => 0 matches | **Build Fail** |

---

## 6. COMPLETE REUSE CLASSIFICATION (RESTATE, VERIFIED)

**Scope:** ALL Gemini Universe subsystems found in `apps/backend/src`.

| Module | Gemini Path | Classification | Action |
| :--- | :--- | :--- | :--- |
| **Metabolism** | `core/biology/metabolism.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Hormones** | `core/biology/hormones.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Immune** | `core/biology/immune-system.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Vitals** | `core/biology/vitals.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Chaos** | `chaos/ChaosSys.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Homestead** | `world/homestead.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Shed** | `world/shed.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Interoception** | `core/senses/interoception.ts` | **REUSE WITH CONS** | Sorted Inputs. |
| **Proprioception**| `core/senses/proprioception.ts`| **REUSE AS-IS** | Port 1:1 to Rust. |
| **Tactile** | `core/senses/tactile-system.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Emotions** | `core/psychology/granular-emotions.ts`| **REUSE AS-IS** | Port 1:1 to Rust. |
| **Dark Triad** | `core/psychology/dark-triad.ts` | **REUSE AS-IS** | Port 1:1 to Rust. |
| **Free Will** | `core/free-will-decision-loop.ts` | **REUSE WITH MODIFICATION** | Inject `TimeSource`, `ChaosStream`. |
| **Consciousness**| `core/consciousness-kernel-enhanced.ts`| **REUSE WITH MODIFICATION** | Inject `EventBus`. |
| **Somatic Body** | `core/somatic/SomaticBody.ts` | **REUSE WITH MODIFICATION** | Remove Global Bus. |
| **Event Replay** | `core/event-replay-engine.ts` | **REUSE WITH MODIFICATION** | Use Persistence Trait. |
| **Language** | `core/language-system.js` | **REUSE WITH MODIFICATION** | Wrap in Rust. |
| **Twin Init** | `core/twin-system-initializer.ts` | **REWRITE REQUIRED** | Boot logic. |
| **Boot Manager** | `core/boot-manager.ts` | **REWRITE REQUIRED** | Env/File I/O. |
| **Server** | `core/server.js` | **REWRITE REQUIRED** | Node.js HTTP. |
| **Transport** | `core/server/frontend-server.js`| **REWRITE REQUIRED** | WebSocket State. |
| **Networking** | `core/services/` | **REWRITE REQUIRED** | Async/Promises. |
| **Human Integ** | `core/human-systems-integration.js`| **REWRITE REQUIRED** | External APIs. |
| **Self-Reflect** | `core/psychology/self-reflection.ts`| **REWRITE REQUIRED** | Stubs/Missing. |

---

## 7. WINDSURF EXECUTION CONSTRAINTS

This section MUST state:

- Windsurf may **ONLY** implement what is explicitly authorized in this plan.
- Windsurf **MUST STOP** on any ambiguity or if a requirement conflicts with the codebase state.
- **ANY deviation** (e.g., skipping a test, ignoring a law) requires a **NEW Antigravity plan**.
- **AMP audit is mandatory** before execution of any code.
- **NO INVENTION**: If a function body is missing in the source, it stays missing. Mark as `REWRITE REQUIRED` and stop. Do NOT generate filler logic.

**VERDICT:** PROCEED TO AMP AUDIT.
