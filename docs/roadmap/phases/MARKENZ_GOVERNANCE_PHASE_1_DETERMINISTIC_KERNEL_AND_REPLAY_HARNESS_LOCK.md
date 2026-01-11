---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 1
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_0_REPO_AND_EVENT_LOG_BASELINE
---

# MARKENZ — GOVERNANCE PHASE 1: DETERMINISTIC KERNEL AND REPLAY HARNESS LOCK

## 1. Phase Objective

Prove determinism formally; lock DeterministicRng and snapshot mechanism. This phase hardens the determinism guarantees established in Phase 0 and prevents future deviations via locked implementation.

## 2. Governance Domains In Scope

- **Determinism** (hardened, replay-equal guarantee)
- **Authority** (server forbidden from computing outcomes)

*Sourced from Section 4, PHASE 1 specification, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/rng` — DeterministicRng subsystem streams (physics, biology, cognition, environment, genetics, governance)
- `crates/world` — Snapshot format + replay-from-snapshot harness
- `crates/core` — Hash canonicalization (deterministic serialization)
- `apps/engine` — Genesis snapshot (Markenz world + Gem-D + Gem-K)

*Sourced from Section 4, PHASE 1, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 1 MUST be defined and logged:

- `RngDraw` (tick, subsystem, stream, callsite, value) — logged RNG consumption
- `SnapshotCreated` (tick, world_hash, snapshot_id) — snapshot checkpoint
- `ReplayStarted` (seed, event_range, snapshot_source) — replay initiation record

*Sourced from Section 4, PHASE 1, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 1 completion, the following properties MUST be proven and MUST hold:

- **Cross-Run Hash Equality:** Seed S + Events E run twice → identical `world_hash` sequence.
- **Snapshot Replay Equivalence:** Snapshot at tick T + events after T → identical hashes as full replay from boot.
- **RNG Reproducibility:** Same tick + subsystem + stream → identical random values (across runs).
- **Hash Chain Continuity:** Every tick produces deterministic hash; no floating-point or nondeterminism leakage.

*Sourced from Section 4, PHASE 1, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### Authority Boundary Enforcement (Expanded)

- **Engine owns:** RNG streams, hash computation, snapshot creation/replay.
- **Server owns:** InputEvent queuing and delivery.
- **Web owns:** Telemetry display (no outcome computation).
- **Prohibited:** Server computing physics/biology/cognition; server replaying snapshots; web influencing RNG.

*Sourced from Section 2.1, "Authority Boundaries (Non-Negotiable)," and Section 2.3 "Enforcement Points."*

### RNG Access Control

- **DeterministicRng is engine-only:** No RNG callable from server or web code.
- **Subsystem streams are separate:** Physics, biology, cognition, environment, genetics, governance each have dedicated RNG streams.
- **Draws are logged:** Every RNG draw recorded: `{ tick, subsystem, stream, callsite, value }`.

*Sourced from Section 2.1, "Determinism Enforcement Layer."*

### Snapshot Lock

- **Snapshots are immutable:** Once created, snapshot data cannot be edited.
- **Snapshot format is versioned:** Changes to format increment version; old versions supported for replay.
- **Replay from snapshot must produce identical hashes:** No exception.

*Sourced from Section 4, PHASE 1, "Determinism Guarantees."*

## 7. Audit & Replay Requirements

### RNG Audit Log

- `tools/audits` consumes RNG logs and generates audit report.
- Report shows: subsystem, stream, tick range, draw count, callsite distribution.
- Every RNG draw MUST appear in log; missing draws → audit failure.

### Snapshot Hash Verification

- `tools/audits` validates replay equality: snapshot + events vs. full replay from boot.
- Report shows: hash match/divergence, first divergent tick (if any), subsystem RNG counters at divergence.
- Test MUST be automated (CI gated).

### Determinism Report

- Generated per test run.
- Includes: seed, hash timeline, replay verification, authority boundary check.
- Report MUST be stored in audit log for inspection.

*Sourced from Section 4, PHASE 1, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 2.

### 8.1 Determinism Replay Test

**Requirement:** Two full runs with identical seed and InputEvent sequence must produce identical hash timeline.

**Acceptance Criteria:**
- Run 1: Seed S, Events E, produce hashes H[0..N]
- Run 2: Seed S, Events E, produce hashes H'[0..N]
- H[i] == H'[i] for all i (bit-for-bit equality)
- Test automated; CI gated.

### 8.2 Snapshot Replay Test

**Requirement:** Snapshot at tick T + events [T+1..N] must produce identical hashes as full replay from boot with same events [0..N].

**Acceptance Criteria:**
- Full replay: Seed S, Events E[0..N], produce hashes H[0..N]
- Snapshot replay: Load snapshot from tick T, replay events E[T+1..N], produce hashes H'[T..N]
- H[T..N] == H'[T..N] (bit-for-bit equality from T onward)
- Test automated; CI gated.

### 8.3 RNG Subsystem Test

**Requirement:** Each RNG subsystem (physics, biology, cognition, environment, genetics, governance) must produce reproducible draws.

**Acceptance Criteria:**
- Draw subsystem stream at tick T with parameters P → value V.
- Draw again with same parameters → identical value V.
- All six subsystems tested independently.
- Test automated; CI gated.

### 8.4 Authority Boundary Test

**Requirement:** Verify server cannot compute physics/biology/cognition outcomes; engine is sole outcome authority.

**Acceptance Criteria:**
- Static analysis: No RNG calls in server code.
- No outcome computation (physics forces, biology states, etc.) in server.
- Runtime test: Submit InputEvent; verify outcome comes from engine, not server.
- Test automated; CI gated.

### 8.5 Snapshot Format Stability Test

**Requirement:** Snapshot format must be stable across multiple snapshots at same tick.

**Acceptance Criteria:**
- Take snapshot at tick T with seed S.
- Take another snapshot at tick T with same seed S.
- Snapshots are byte-for-byte identical.
- Test automated; CI gated.

*Sourced from Section 4, PHASE 1, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 1 is considered complete:

1. **Determinism Regression Test:**
   - Same seed + events → identical hash timeline (automated).
   - Failure blocks merge.

2. **Snapshot Replay Regression Test:**
   - Snapshot + events == full replay (automated).
   - Failure blocks merge.

3. **Authority Leakage Static Analysis:**
   - Scan server code for RNG calls, outcome computation.
   - Scan web code for state mutation.
   - Zero violations required.
   - Failure blocks merge.

4. **RNG Audit Log Generation:**
   - Build must log all RNG draws.
   - `tools/audits` must generate report without errors.
   - Report must be verifiable and complete.
   - Failure blocks merge.

5. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.
   - `docker-compose build` succeeds.

*Sourced from Section 4, PHASE 1, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 1:

- **No server computation of physics/biology/cognition outcomes** (Section 2.1, "Authority Boundaries").
  - Server must NOT predict, calculate, or simulate any state changes.

- **No RNG outside engine** (Section 1.1, "Determinism Law").
  - All RNG calls must originate in engine only.
  - Exception: Test harnesses may use RNG for test setup (not outcome).

- **No snapshot modifications** (Section 4, PHASE 1, "Determinism Guarantees").
  - Snapshots are immutable after creation.
  - Edits to snapshots are prohibited and must fail hard.

- **No replay without hash verification** (Section 4, PHASE 1, "Determinism Guarantees").
  - Every replay must verify output hashes match expected values.
  - Divergence must be detected and reported.

- **No floating-point in authority path** (Section 1.1, "Determinism Law").
  - All state computation must use deterministic fixed-point or integer math.
  - Float values may only appear in non-authoritative visualization/telemetry.

- **No nondeterministic library calls** (Section 1.1, "Determinism Law").
  - Example: `time()`, `rand()`, `HashMap` iteration without stable ordering.
  - All time-dependent logic must use engine tick.
  - All randomness must use seeded DeterministicRng.
  - All containers must iterate in stable order (BTreeMap, sorted Vec, etc.).

*Sourced from Section 4, PHASE 1, "Explicit Prohibition List (Phase 1)," Section 1.1 "Determinism Law."*

## 11. Phase Completion Criteria (Checklist)

Phase 1 is NOT complete until ALL of the following are satisfied:

- [ ] **Determinism proven (three-way test)** — Run 1, Run 2, snapshot replay all produce identical hashes
- [ ] **Snapshot format stable and versioned** — Multiple snapshots at same tick are byte-for-byte identical
- [ ] **RNG audit mechanism working** — `tools/audits` generates complete RNG report; all draws logged
- [ ] **Zero authority leakage** — Static analysis + runtime test confirm server cannot compute outcomes
- [ ] **All RNG subsystems reproduce deterministically** — physics, biology, cognition, environment, genetics, governance
- [ ] **Snapshot replay == full replay** — No divergence in hash sequence from snapshot onward
- [ ] **All mandatory tests pass** — Determinism, snapshot, RNG, authority, format stability tests
- [ ] **CI gates pass** — Build, determinism, snapshot, authority, RNG audit gates all pass
- [ ] **Hash canonicalization complete** — All state serialization is deterministic

*Sourced from Section 4, PHASE 1, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.1, 2.3, 4.0 (PHASE 1), and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The determinism guarantee and authority boundary separation specified herein are constitutional principles that may never be weakened.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 1, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 1, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 1, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 1, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 1, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.1 "Authority Boundaries"; Section 2.3 "Enforcement Points"; Section 4, PHASE 1 "Governance Domains Expanded" |
| 7. Audit & Replay Requirements | Section 4, PHASE 1, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 1, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 1, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 1, "Explicit Prohibition List (Phase 1)"; Section 1.1 "Determinism Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 1, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 0 (completed)
