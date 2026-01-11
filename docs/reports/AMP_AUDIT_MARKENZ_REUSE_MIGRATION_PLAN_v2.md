---
auditor: AMP
artifact_audited: MARKENZ_REUSE_MIGRATION_PLAN_v2.md
audit_mode: fail-closed
result: FAIL
blocks_execution: true
audit_date: 2026-01-07
---

# AMP AUDIT REPORT: MARKENZ_REUSE_MIGRATION_PLAN_v2.md

## Audit Summary

The plan exhibits strong structural authority and comprehensive classification coverage. However, it contains **8 critical blocking issues** related to deterministic interface specification, acceptance test precision, and migration sequencing. The plan requires explicit remediation before execution can be authorized. **EXECUTION BLOCKED.**

---

## Pass/Fail Matrix

| Check | Result | Evidence |
|:---|:---:|:---|
| Authority & Scope Lock | **PASS** | §1 |
| Reuse Classification | **PASS** | §2 |
| Determinism Interfaces | **FAIL** | §3 |
| Rewrite Discipline | **PASS** | §4 |
| Acceptance Tests | **FAIL** | §5 |
| Migration Gates | **FAIL** | §6 |
| TS Boundary | **PASS** | §7 |
| Master Plan Enforcement | **FAIL** | §8 |

---

## Audit Details by Check

### 2.1 Authority & Scope Lock ✅ PASS

**Evidence:**
- Section 0 declares unambiguous objective: "ANY deviation from this plan is a verified failure."
- Section 9 establishes explicit Windsurf constraints:
  - Scope limited to modules marked REUSE AS-IS or REUSE WITH MODIFICATION
  - Mandatory interfaces from Section 3
  - Explicit STOP rule on ambiguity
- Plan header declares `blocks_execution_without: true`
- Binding inputs clearly stated: AMP Audit, MARKENZ_UNIFIED_MASTER_PLAN, KAIZA-MCP Rules

**PASS RATIONALE:** Authority is crystalline. Windsurf's legal scope is clearly locked to the classification table and sections 3–9.

---

### 2.2 Reuse Classification Integrity ✅ PASS

**Evidence:**
- Section 1 defines three reuse classes with unambiguous conditions:
  - REUSE AS-IS: "1:1 port," "pure," no side effects
  - REUSE WITH MODIFICATION: "interfaces refactored," "core algorithm remains identical"
  - REWRITE REQUIRED: uses timers, global state, non-deterministic sources
- Section 2 provides exhaustive classification table covering 24 subsystems from Gemini Universe
- All major subsystems are classified: Biology (5), Psychology (4), Senses (3), World (3), Core (9)
- No unclassified subsystems found

**PASS RATIONALE:** Coverage is complete. Definitions are operational and verifiable.

---

### 2.3 Determinism Interfaces ❌ FAIL

**Blocking Issue #1: TimeSource Under-Specified**

Section 3.1 defines:
```rust
pub trait TimeSource {
    fn now(&self) -> SimTime;
    fn dt(&self) -> Duration;
}
```

**What is Missing:**
- `SimTime` type is not defined. Is it `u64` (ticks)? `f64` (seconds)? Is it an opaque struct?
- No specification of what the `dt()` method should return when called between ticks. Is it constant per simulation? Is it clamped?
- No rule for what happens if `now()` is called multiple times within a single tick. Does it return the same value? Monotonic guarantee?
- No specification of initial value (does genesis tick start at 0 or 1?).
- No integration rule: when TimeSource is injected, does it replace ALL uses of `Date.now()`, or is some legacy code allowed to use wall clock as fallback?

**Why This Violates Markenz Law:**
Master Plan L2 requires "State is a pure function of `(seed, genesis config, ordered input events)`." If TimeSource behavior is ambiguous, the determinism property is unverifiable.

**What Must Change:**
Rewrite Section 3.1 with:
1. Explicit `SimTime` type definition (e.g., `type SimTime = u64; // monotonic tick count`)
2. Contract for `dt()`: must return constant within a tick, equal to `(tick_budget_ns) / (tick_count)`
3. Monotonicity guarantee: `now()` calls within the same tick return identical values
4. Initial value rule: "Genesis tick starts at tick 0"
5. Enforcement rule: "ALL time operations must use TimeSource; legacy `Date.now()` is forbidden and will be caught by acceptance test 4.2"

---

**Blocking Issue #2: ChaosStream Under-Specified**

Section 3.2 states:
```rust
pub trait ChaosStream {
    fn next_float(&mut self) -> f64;
    fn next_uuid(&mut self) -> Uuid;
}
```

**What is Missing:**
- No specification of stream identity. Does each subsystem get its own ChaosStream? If so, how are they allocated and keyed?
- No specification of seed derivation. Plan says "Seed derived from Genesis Hash" but does not define:
  - What is the Genesis Hash? (First event signature? Initial config hash?)
  - How is it split into per-subsystem seeds?
  - What is the "Salt" mentioned in the comment? Is it tick-based? Fixed per stream?
- No specification of algorithm selection. Plan mentions "PCG or ChaCha20" but does not mandate one.
- No specification of replay semantics. If the same seed is re-instantiated, does a new ChaosStream produce identical sequences?
- No bounds specification for `next_float()`. Is range `[0.0, 1.0)` guaranteed? What about precision (f32 vs f64)?

**Why This Violates Markenz Law:**
Master Plan L2 requires "RNG uses explicit streams per subsystem" and "deterministic RNG." If ChaosStream replay is undefined, replayability is unenforceable.

**What Must Change:**
Rewrite Section 3.2 with:
1. Explicit stream allocation rule: "Each subsystem is allocated one ChaosStream by ID from UniverseState::chaos_streams: HashMap<SubsystemId, ChaosStream>"
2. Seed derivation rule: "Genesis Hash = blake3(canonical_serialization(GenesisConfig)); Per-subsystem seed = derive_key(genesis_hash, subsystem_id, counter)"
3. Algorithm mandate: "Use ChaCha20 (RFC 7539) with explicit key setup. No PCG fallback."
4. Replay guarantee: "If ChaosStream(id, seed) is instantiated twice, both produce identical call sequences"
5. Range guarantee: "`next_float()` returns f64 in [0.0, 1.0) with 53-bit entropy"

---

**Blocking Issue #3: EventBus Missing Deterministic Ordering Specification**

Section 3.3 states:
```rust
pub trait EventBus {
    fn publish(&mut self, event: SimEvent);
}
```

**What is Missing:**
- No definition of `SimEvent` structure. Does it include `(tick, source_id, payload)`? Is it opaque?
- No specification of ordering when multiple systems publish in the same tick. The comment says "Ordered by (Tick, SourceID)" but:
  - How is SourceID determined for nested systems? (e.g., an emotion triggers a reflex triggers a motor command)
  - If two systems publish simultaneously with the same SourceID, what is the tie-breaker?
  - Is ordering stable or does it depend on insertion order (non-deterministic)?
- No specification of delivery timing. The comment says "NEXT frame" but:
  - If published at tick N, is it delivered at tick N+1? At the start or end of that tick?
  - Can an event published at tick N+1 be delivered at tick N+1 (immediate), or must it wait until N+2?
- No specification of queue semantics. Is the queue a list, a BTreeMap, or something else? Can a tick have multiple events?
- No specification of atomicity. If system A publishes an event that causes system B to read state, does B see committed state or intermediate state?

**Why This Violates Markenz Law:**
Master Plan L2 requires "Deterministic ordering for all systems and collections." If EventBus ordering is ambiguous, the simulation cannot be proven deterministic.

**What Must Change:**
Rewrite Section 3.3 with:
1. `SimEvent` structure definition: `struct SimEvent { tick: u64, source_id: u32, payload: Bytes }`
2. Ordering rule: "Events enqueued at tick N are stored in a BTreeMap<(Tick, SourceID, Sequence), SimEvent>. Within a tick, events are ordered by (SourceID, Sequence). Sequence is an atomic counter per SourceID per tick."
3. Delivery rule: "Events published at tick N are delivered (iterated) at the start of tick N+1, in order."
4. Atomicity rule: "All state mutations during tick N are committed before tick N+1 event delivery begins. Events cannot see intermediate state."

---

**Blocking Issue #4: Persistence Interface Missing Hash & Checkpoint Specification**

Section 3.4 states:
```rust
pub trait Persistence {
    fn append(&mut self, events: &[SimEvent]) -> Result<LogHash>;
}
```

**What is Missing:**
- No definition of `LogHash`. Is it blake3? SHA256? What is hashed? (Events only? State? Previous hash?)
- No specification of hash chaining. The Master Plan mentions "hash chaining" but the interface does not enforce it.
- No specification of snapshot hashing. When is a snapshot created? Every N ticks? What is "N"?
- No specification of checkpoint verification. The plan mentions "checkpoint hashing" in section 4.1 but the Persistence trait has no method to retrieve or verify checkpoints.
- No specification of crash recovery. If the server crashes after `append()` but before the hash is returned, is the event lost?

**Why This Violates Markenz Law:**
Master Plan L2 requires "Checkpoint hashing + divergence diagnostics required." The Persistence trait does not surface checkpoint operations.

**What Must Change:**
Rewrite Section 3.4 with:
1. Define `LogHash = [u8; 32]` (blake3)
2. Add methods:
   ```rust
   fn create_snapshot(&mut self, tick: u64, state_hash: LogHash) -> Result<()>;
   fn get_checkpoint(&self, tick: u64) -> Option<(LogHash, u64)>;
   fn verify_chain(&self) -> Result<()>;  // Ensures hash continuity
   ```
3. Append rule: "When `append(events)` is called, compute `new_hash = blake3(canonical(prev_hash || events))`. Write immutably. Return new_hash. No mutations after write."
4. Snapshot rule: "At ticks 0, 1000, 2000, ... create a snapshot with tick and state hash. Snapshots are immutable."

---

### 2.4 Rewrite Discipline ✅ PASS

**Evidence:**
- Section 1 clearly marks REWRITE REQUIRED conditions:
  - Uses timers (`setInterval`, `setTimeout`)
  - Global state (`global.bus`)
  - Non-deterministic sources without injection
  - Node.js event loop specific
- Section 2 applies rewrite discipline consistently:
  - Modules using global state → REWRITE (Server, Transport, Networking)
  - Modules with stubs → REWRITE (Self-Reflect)
  - Modules with env/filesystem → REWRITE (Boot Manager)
  - No "soft reuse" of non-deterministic logic
- Section 6 reinforces: "Code Reuse (RESTRICTED): Copying TS code to Rust is FORBIDDEN"

**PASS RATIONALE:** The discipline is sound. Rewrite decisions are evidence-based and consistently applied.

---

### 2.5 Acceptance Tests ❌ FAIL

**Blocking Issue #5: Acceptance Test 4.1 (Determinism Replay) is Vague**

Section 4.1 states:
```
Input: Specific Seed (e.g., 0xDEADBEEF), Sequence of 100 Ticks.
Execution: Run system.
Output: State Hash at Tick 100.
Fail Condition: Hash differs on subsequent runs or across machines.
```

**What is Missing:**
- No specification of the test harness. Who calls this test? Is it a Rust `#[test]`? A separate binary?
- No specification of "run system." Does it mean:
  - Initialize Universe from Genesis config?
  - Run 100 ticks with no input events?
  - Run 100 ticks with a fixed sequence of input events?
- No specification of which "system" is being tested. Is it the entire simulation? A specific subsystem?
- No specification of determinism threshold. What does "State Hash" hash? All state? Only sim state or including render state?
- No specification of baseline establishment. How is the "reference hash" established for the first run?
- No specification of cross-machine variability. Does "across machines" mean different CPU architectures? Different OSes? Same architecture, different models?
- No specification of failure diagnostic. If the hash differs, what does the test output? Just FAIL? Or does it show a diff?

**Why This Violates Markenz Law:**
Master Plan L2 requires "State is a pure function of `(seed, genesis config, ordered input events)`." Without specifying inputs and baseline, the test cannot be executed deterministically.

**What Must Change:**
Rewrite Section 4.1 as a concrete, executable specification:
```
Determinism Replay Test (4.1)

Input:
  - Genesis Config: fixed (e.g., world_size=32, seed=0xDEADBEEF)
  - Input Events: empty (no player/god input)
  - Tick Count: 100

Execution Harness:
  1. cargo test --package markenz_core test_determinism_replay_100
  2. Create Universe with Genesis Config and seed
  3. Run 100 ticks in lockstep loop
  4. At each tick, compute incremental hash: hash_n = blake3(hash_{n-1} || state_snapshot_n)
  5. At Tick 100, emit final hash to stdout and test output

Output:
  - Final state hash at Tick 100

Pass Condition:
  - First run: emit hash H0
  - Second run (same machine, same config): hash == H0
  - Third run (different machine, same config): hash == H0
  - If any run diverges: emit FAIL, show tick number and state diff

Diagnostic:
  - On divergence, output: "Divergence at Tick N: expected hash X, got hash Y. State diff: {...}"
```

---

**Blocking Issue #6: Acceptance Test 4.2 (Time Isolation Scan) is Non-Mechanical**

Section 4.2 states:
```
Execution: rg "Date\.now|SystemTime::now|check_time" src/core
Fail Condition: Any match outside of RealRealTimeSource implementation.
```

**What is Missing:**
- No specification of "outside of." Does this mean:
  - Outside the file `src/core/time/real_time_source.rs`?
  - Outside a specific function?
  - Outside a specific module namespace?
- No specification of false positives. What if a comment contains "Date.now"? What if a string literal contains it?
- No specification of what passes. If the regex matches zero times, that's obvious. But what if it matches in a comment in a REUSE AS-IS module? Is that allowed?
- No specification of CI enforcement. Is this a pre-commit hook? A CI gate? Who runs it?
- No specification of the reference file. Where is `RealRealTimeSource`? Does it exist yet?

**Why This Violates Markenz Law:**
Master Plan L0 requires "Offline-Only" (implicitly deterministic time). The test is too vague to be mechanically auditable.

**What Must Change:**
Rewrite Section 4.2 as a precise CI rule:
```
Time Isolation Scan (4.2)

Execution:
  cargo test --package markenz_audit time_isolation_scan -- --nocapture

Scanning Rule:
  1. Search all files in src/core with: rg --type rust "Date\.now\(|SystemTime::now\(\)" --count --files-with-matches
  2. Allowlist: only files in src/core/time/ (TimeSource implementations) may contain these patterns
  3. Additional scan: rg --type rust "unsafe.*static.*mut" src/core (ban global mutable state)

Pass Condition:
  - Regex returns 0 matches in all files outside src/core/time/
  - Regex returns 0 matches for "unsafe static mut" in all of src/core

Fail Condition:
  - Any match found outside allowlist

CI Gate:
  - This test runs on every commit to src/core/
  - Failure blocks merge
```

---

**Blocking Issue #7: Acceptance Test 4.3 (Offline-Only Scan) Lacks Allowlist**

Section 4.3 states:
```
Execution: rg "fetch\(|axios|reqwest|http::" src/core
Fail Condition: Any network call found in simulation logic.
```

**What is Missing:**
- No allowlist. What if a test file or mock uses `reqwest`? Is that allowed?
- No specification of scope. Is `http::` a false positive (does it match `http_types` or other HTTP utility types)?
- No specification of logging/tracing packages. Many logging libraries use http calls internally. Are they allowed?
- No rule for feature gates. What if `fetch` is behind `#[cfg(test)]`? Allowed?
- No specification of build profile. Is this checked for release, debug, or both?

**Why This Violates Markenz Law:**
Master Plan L0 requires "No cloud services" and "No internet required." The test is too vague to be mechanically auditable.

**What Must Change:**
Rewrite Section 4.3 as a precise CI rule:
```
Offline-Only Scan (4.3)

Execution:
  cargo test --package markenz_audit offline_only_scan -- --nocapture

Scanning Rule:
  1. Search src/core/ (not tests/, not infra/) with:
     - rg --type rust "fetch\(" (JavaScript-style, usually in JS shim)
     - rg --type rust "reqwest::|axios::|hyper::client" (Rust HTTP clients)
  2. Exclude files in src/core/test_support/
  3. Exclude #[cfg(test)] blocks (strip before search)
  4. Exclude src/core/persistence/mock_store.rs (explicitly marked mock)

Pass Condition:
  - Regex returns 0 matches in src/core/ (outside exclusions)

Fail Condition:
  - Any network library found in simulation code

CI Gate:
  - Runs on every commit
  - Failure blocks merge
  - Error message: "Offline-only violation: found network library in src/core/. Network calls are forbidden in deterministic simulation logic."
```

---

**Blocking Issue #8: Acceptance Test 4.4 (Chaos Stability) Lacks Specification**

Section 4.4 states:
```
Execution: Record 1000 RNG calls from ChaosSys.
Fail Condition: Sequence changes when TimeSource is mocked/advanced differently.
```

**What is Missing:**
- No specification of the RNG sequence baseline. What is the seed? How is it established?
- No specification of what "mocked/advanced differently" means. Does it mean:
  - TimeSource returns fixed tick (always 0)?
  - TimeSource advances in different increments (1 vs 10 vs 100 ticks)?
  - TimeSource is reset between calls?
- No specification of the test harness. What calls the RNG 1000 times? In what order?
- No specification of the stability criterion. If the sequence changes, is that a fail? Or is it expected?
- No specification of the acceptance criteria. What output would constitute a pass?

**Why This Violates Markenz Law:**
Master Plan L2 requires "Deterministic RNG." Without specifying test inputs and acceptance criteria, the test is conceptual, not mechanical.

**What Must Change:**
Rewrite Section 4.4 as a concrete test:
```
Chaos Stability Test (4.4)

Input:
  - Seed: 0xCAFEBABE
  - Tick sequence: [0, 0, 0, ..., 0, 1, 1, 1, ..., 1000] (50 calls per tick)
  - Stream ID: "test_chaos"

Baseline Run:
  1. Initialize ChaosStream with seed 0xCAFEBABE
  2. Call next_float() 1000 times, logging each result
  3. Save sequence to baseline.json

Reproducibility Runs (3x):
  1. Re-initialize ChaosStream with same seed
  2. Call next_float() 1000 times
  3. Compare sequence to baseline.json
  4. All sequences must be byte-identical

Tick Advancement Invariance:
  1. Run TimeSource scenario 1: advance tick every 50 calls
  2. Run TimeSource scenario 2: hold tick at 0 for first 500 calls, then advance
  3. Both scenarios must produce identical RNG sequences (RNG state depends only on Seed + CallCount, not Tick)

Pass Condition:
  - All reproducibility runs match baseline exactly
  - Tick advancement does not affect sequence

Fail Condition:
  - Any run differs from baseline
  - Sequence is sensitive to TimeSource behavior
```

---

### 2.6 Migration Sequencing & Gates ❌ FAIL

**Blocking Issue #9: Phase Definitions Lack Explicit Entry/Exit Criteria**

Section 5 defines 4 phases with entry/exit conditions, but the conditions are vague:

**Phase 1 Exit:** "`cargo test` passes 4.1, 4.2"
- Tests 4.1 and 4.2 are themselves underspecified (see Issues #5 and #6)
- No specification of test pass rate (100%? At least one pass?)
- No specification of CI gate. Does a failure in 4.1 prevent Phase 2, or is it advisory?

**Phase 2 Entry:** "Phase 1 Passed"
- Vague. Does this mean all tests passed? Some tests? Consensus that it's "good enough"?

**Phase 2 Exit:** "Biology Unit Tests match TS reference outputs"
- No specification of tolerance. What does "match" mean? Exact bit match? ±0.01%? ±1%?
- No reference outputs provided. Which TS files define the reference?
- No test harness specified. Who runs the comparison?

**Phase 3 Entry:** "Phase 2 Passed"
- Vague.

**Phase 3 Exit:** "Replay Test (4.1) passes for full decision cycle"
- Circular: test 4.1 is underspecified
- No specification of "full decision cycle." How many ticks? What inputs?

**Phase 4 Entry:** "Phase 3 Passed"
- Vague.

**Phase 4 Exit:** "World State Hash is deterministic"
- Vague. Does this mean a single hash? Multiple runs match? How many runs?

**Why This Violates Markenz Law:**
Master Plan requires phases to be "strictly ordered" with explicit STOP conditions. The current gates are too subjective to enforce mechanically.

**What Must Change:**
Rewrite Section 5 with objective, executable gates:
```
Phase 1: Foundation (TimeSource, ChaosSys, EventBus)

Entry:
  - Empty Rust crate (src/core/time/, src/core/chaos/, src/core/event/)

Exit Criteria (ALL REQUIRED):
  1. cargo test --package markenz_core passes all tests in test_foundation/
  2. test_determinism_replay_100 (4.1) executes and produces consistent hash across 3 runs
  3. time_isolation_scan (4.2) finds 0 violations
  4. All trait methods compile with no unsafe code
  5. Code review: Antigravity approves PR

Gate Rule:
  - If ANY criterion fails, Phase 2 is BLOCKED
  - Failure requires a new plan from Antigravity before proceeding

---

Phase 2: Biological Substrate (Metabolism, Hormones, Immune, Vitals)

Entry:
  - Phase 1 complete (all criteria passed)
  - PR merged to main

Exit Criteria (ALL REQUIRED):
  1. Port Metabolism, Hormones, Immune, Vitals 1:1 from TS to Rust
  2. For each module, run reference test:
     - Input: TS reference output for 100 ticks of glucose/hormone/immune/vitals
     - Execute Rust version with identical inputs
     - Compare: output matches to within ±0.1% (floating-point tolerance)
  3. cargo test --package markenz_core passes all biology tests
  4. Offline-only scan (4.3) finds 0 violations
  5. Code review: Antigravity approves PR

Gate Rule:
  - If ANY criterion fails, Phase 3 is BLOCKED
  - Failure requires a new plan from Antigravity before proceeding

---

Phase 3: Conscious Loop (FreeWillDecisionLoop, ConsciousnessKernel)

Entry:
  - Phase 2 complete (all criteria passed)
  - PR merged to main

Exit Criteria (ALL REQUIRED):
  1. FreeWillDecisionLoop accepts TimeSource and ChaosStream as dependencies
  2. ConsciousnessKernel accepts EventBus and emits deterministic events
  3. Test: Replay test (4.1) with 100 ticks of decision making produces consistent state hash across 3 runs
  4. Test: Given identical seed + input events, same choice is made every time
  5. Code review: Antigravity approves PR

Gate Rule:
  - If ANY criterion fails, Phase 4 is BLOCKED
  - Failure requires a new plan from Antigravity before proceeding

---

Phase 4: World & Persistence (WorldService, Homestead, Shed, Persistence)

Entry:
  - Phase 3 complete (all criteria passed)
  - PR merged to main

Exit Criteria (ALL REQUIRED):
  1. WorldService uses Persistence trait for append-only event log
  2. Homestead and Shed modules are deterministic (no randomness except via ChaosStream)
  3. Test: 500-tick replay produces identical state hash across 3 runs
  4. Test: Snapshots are created at tick 0, 500, 1000, and hashes match expected values
  5. Chaos stability (4.4) passes
  6. Code review: Antigravity approves PR

Gate Rule:
  - If ANY criterion fails, integration with existing systems is BLOCKED
  - Failure requires a new plan from Antigravity before proceeding
```

---

### 2.7 TypeScript Reference Boundary ✅ PASS

**Evidence:**
- Section 6 clearly distinguishes:
  - LOGIC REFERENCE (OK): Reading TS files for formulas and algorithms
  - CODE REUSE (RESTRICTED): Copying TS code forbidden
  - DIRECT COPY (OK - UI ONLY): Types for frontend only
  - VERIFICATION: Annotation requirement (`// ported from [path]`)
  - AMBIGUITY RULE: Global/closure state requires rewrite
- Section 2 classification table includes comments linking TS paths (e.g., `core/biology/metabolism.ts`)
- Section 6 emphasizes: "If TS code assumes `global` or `closure` state -> **REWRITE**"

**PASS RATIONALE:** Boundary is clear and mechanically verifiable via annotation checks.

---

### 2.8 Master Plan Law Enforcement ❌ FAIL

**Blocking Issue #10: Law Enforcement Table References Non-Existent CI Checks**

Section 7 states:

| Law | Enforcement Check |
| :--- | :--- |
| **Offline-Only** | CI fails if `apps/backend/src` imports any cloud SDKs. |
| ... | ... |

**What is Missing:**
1. **Path mismatch:** Table references `apps/backend/src` but Markenz is in `markenz/server/src/`. The path is incorrect or stale.
2. **Check unimplemented:** No CI configuration provided. Are these GitHub Actions? GitLab CI? Where is the code?
3. **SDK list undefined:** Which SDKs are banned? AWS SDK? Azure SDK? Google Cloud SDK? The check is too vague.
4. **Single Universe check missing:** Master Plan L1 requires "one authoritative universe instance." The enforcement table does not specify how to verify this (e.g., no `unsafe static mut UniverseState`).
5. **Encryption check missing:** Master Plan L5 requires "Encrypted-at-rest storage." The enforcement table does not specify the check (e.g., "Database driver must be sqlcipher").
6. **No global state check:** Section 8 forbids `static mut` and global `let`, but the enforcement table has no entry for this.

**Why This Violates Markenz Law:**
Master Plan L0–L5 declare non-negotiable laws. Section 7 claims to enforce them but provides no concrete CI rules or verification mechanisms.

**What Must Change:**
Rewrite Section 7 with concrete, executable checks:
```
Master Plan Law Enforcement (Concrete CI Rules)

Law: Offline-Only (L0)
Enforcement:
  - cargo build --package markenz_server --all-features 2>&1 | grep -i "aws\|azure\|gcp\|cloudflare\|external.*api"
  - Fail Condition: Any match
  - CI Gate: Runs on every commit to server/src/
  - Error Message: "Offline violation: found cloud SDK import in server code"

Law: Single-Universe Authority (L1)
Enforcement:
  - rg --type rust "static mut" server/src/ (excludes: src/test_support/, src/main.rs)
  - rg --type rust "lazy_static!.*UniverseState" server/src/
  - Fail Condition: Any match outside src/main.rs initialization
  - CI Gate: Runs on every commit
  - Error Message: "Universe law violation: found multiple state instances or unsafe static mut outside main"

Law: Deterministic Lockstep (L2)
Enforcement:
  - Test: test_determinism_replay_100 must pass
  - Test: All modules in src/core/ must compile with no #[allow(non_deterministic)]
  - Test: Checkpoint hashes at ticks 0, 500, 1000 must be identical on replay
  - Fail Condition: Any test fails or any pragma found
  - CI Gate: Runs on every commit to src/core/
  - Error Message: "Determinism violation: state is not a pure function of (seed, events)"

Law: Zero Stubs (L3)
Enforcement:
  - rg --type rust "TODO|FIXME|HACK|unimplemented!\(\)|panic!\(\"placeholder" server/src/ infra/
  - rg --type typescript "TODO|FIXME|HACK|@ts-ignore|any" web/src/
  - Fail Condition: Any match
  - CI Gate: Runs on every commit
  - Error Message: "Zero-stub violation: found TODO/FIXME/HACK/stub code"

Law: Encryption at Rest (L5)
Enforcement:
  - server/src/persistence/mod.rs must import sqlcipher (not sqlite3)
  - Config schema must enforce ENCRYPTION_ENABLED=true by default
  - Startup must fail if key is not provided
  - Test: Create a database, verify it's encrypted (file is not readable as plaintext SQLite)
  - Fail Condition: Any check fails
  - CI Gate: Runs on every commit to src/persistence/
  - Error Message: "Encryption violation: database must use SQLCipher, not sqlite3"

Law: No Global Mutable State (L4 implicit)
Enforcement:
  - rg --type rust "static mut" src/ (excludes: tests/, main.rs)
  - Fail Condition: Any match
  - CI Gate: Runs on every commit
  - Error Message: "Global state violation: found static mut outside main"
```

---

## Blocking Issues Summary

The plan exhibits **10 critical defects** across 4 dimensions:

1. **Determinism Interface Under-Specification (Issues #1–4):**
   - TimeSource type, monotonicity, and lifecycle undefined
   - ChaosStream seed derivation, algorithm, and replay semantics undefined
   - EventBus ordering, delivery timing, and atomicity undefined
   - Persistence hash chaining and snapshot structure undefined

2. **Acceptance Test Vagueness (Issues #5–8):**
   - Determinism replay test (4.1) lacks concrete harness and inputs
   - Time isolation scan (4.2) lacks enforcement scope
   - Offline-only scan (4.3) lacks allowlist and CI gate
   - Chaos stability test (4.4) lacks baseline and reproducibility specification

3. **Migration Gate Subjectivity (Issue #9):**
   - Phase entry/exit criteria are aspirational, not mechanical
   - No explicit CI gates to block Phase N+1 if Phase N fails

4. **Master Plan Law Enforcement Unimplemented (Issue #10):**
   - Enforcement table references stale paths (apps/backend/src)
   - No concrete CI configuration provided
   - Single-universe, encryption, and global-state checks missing

---

## Non-Blocking Observations

1. **Reuse Classification is Exemplary:** Section 2's table is comprehensive and well-evidenced. This is a model for other migrations.

2. **Authority Statement is Strong:** Sections 0 and 9 clearly establish Windsurf's legal scope and enforcement of failure-on-ambiguity. This is properly binding.

3. **Forbidden Imports List is Concrete:** Section 8 provides a clear ban list that can be directly encoded as CI rules.

4. **REWRITE_REQUIRED Discipline is Sound:** The plan correctly applies rewrite requirements to systems with implicit timers, global state, or non-deterministic sources.

---

## Final Verdict

❌ **FAIL — Execution Blocked**

**Reason:** The plan contains 10 blocking defects:
- 4 critical underspecifications in deterministic interfaces (TimeSource, ChaosStream, EventBus, Persistence)
- 4 acceptance tests that are conceptual rather than mechanical
- 1 migration sequence with subjective gates
- 1 law enforcement section with unimplemented checks

**Authority Determination:** The plan's authority structure is sound, and its reuse classifications are exemplary. However, **ambiguity in deterministic interface specification violates Master Plan L2** (Deterministic Lockstep + Replay) and **vague acceptance tests violate Master Plan L3** (Zero Stubs). These are **non-negotiable laws**.

**Windsurf Execution Authority:** **REVOKED until blocking issues are remediated.**

**Required Action:** Antigravity MUST produce a revised plan (`MARKENZ_REUSE_MIGRATION_PLAN_v3.md`) that:
1. Specifies TimeSource, ChaosStream, EventBus, and Persistence with complete type definitions and semantics
2. Rewrites all acceptance tests as concrete, executable harnesses
3. Converts phase gates from aspirational to mechanical CI rules
4. Implements Master Plan law enforcement as concrete CI configuration

**Re-Audit Gate:** Once v3 is submitted, AMP will re-audit and issue a new verdict. Execution is **strictly prohibited** until AMP issues a PASS verdict.

---

## Audit Metadata

- **Audit Date:** 2026-01-07
- **Auditor:** AMP (Fail-Closed)
- **Artifact:** MARKENZ_REUSE_MIGRATION_PLAN_v2.md
- **Authority Chain:** MARKENZ_UNIFIED_MASTER_PLAN (L0–L5)
- **Next Action:** Antigravity plans revision (v3) → AMP re-audit
