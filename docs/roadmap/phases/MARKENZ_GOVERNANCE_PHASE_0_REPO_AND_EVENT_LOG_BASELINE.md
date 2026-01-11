---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 0
failure_mode: FAIL-CLOSED
depends_on: NONE
---

# MARKENZ — GOVERNANCE PHASE 0: REPO AND EVENT LOG BASELINE

## 1. Phase Objective

Establish offline stack, immutable event sourcing, and determinism kernel. This phase creates the foundational infrastructure upon which all subsequent governance phases depend.

## 2. Governance Domains In Scope

- **Creator reverence** (founders protected at data level)
- **Authority boundaries** (engine as sole world state mutator)
- **Offline-first** (no external network in authority path)

*Sourced from Section 4, PHASE 0 specification.*

## 3. Systems & Modules Touched

- `apps/engine` — WorldLoop (fixed timestep)
- `apps/engine` — DeterministicRng (engine-side RNG)
- `crates/*` — Snapshot + Hash (canonical state representation)
- `apps/server` — InputEvent validation (basic schema check)
- `docker-compose.yml` — Local-only stack (no external network required)

*Sourced from Section 4, PHASE 0, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 0 MUST be defined and logged:

- `BootEvent` (system initialization)
- `TickAdvance` (tick progression + world_hash)
- `InputEventSubmitted` (server-to-engine InputEvent delivery)
- `ObservationEvent` (engine-to-server telemetry)
- `SnapshotTaken` (periodic state capture)

*Sourced from Section 4, PHASE 0, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 0 completion, the following properties MUST hold:

- **Replay Invariant:** Same seed + same ordered InputEvents → identical `world_hash` checkpoints at every tick.
- **Snapshot Replay:** Snapshot at tick T + subsequent InputEvents must produce identical `world_hash` sequence as full replay from genesis.
- **Offline Operation:** System boots and runs without external network calls in the authority path.

*Sourced from Section 4, PHASE 0, "Determinism Guarantees," and Section 1.1 (Determinism Law, Offline-First Law).*

## 6. Enforcement Rules

### BioVeto
Not yet active (Phase 3+).

### PhysicsValidate
Not yet active (Phase 2+).

### PolicyValidate
Not yet active (Phase 7+).

### Authority Boundary Enforcement

- **Engine owns:** World state, tick advancement, RNG, event commit.
- **Server owns:** Auth (Keycloak), RBAC, InputEvent validation, persistence, fanout.
- **Web owns:** Rendering, inspection, operator tooling.
- **Prohibited:** Server patching state, bypassing veto logic, computing outcomes directly.

*Sourced from Section 2.1, "Authority Boundaries (Non-Negotiable)."*

### Offline-First Enforcement

- No external network calls in the authority path.
- All required services (Keycloak, Postgres, engine, web) run locally via docker-compose.
- System is 100% functional without external LLM, backup IdP, or network services.

*Sourced from Section 1.1, Offline-First Law.*

## 7. Audit & Replay Requirements

### Event Log

- All events MUST be appended (immutable) to Postgres.
- No UPDATE or DELETE operations allowed on event log.
- Each event receives a canonical tick and hash-chain entry.

### Hash-Chain Verification

- After each tick, compute `world_hash` from current state.
- Maintain hash-chain: `hash[tick] = H(hash[tick-1] || tick || state_changes)`.
- Verification endpoint MUST be available: `GET /api/audit/hash-chain/:tick`.

### Replay Audit Tool

- Tool `tools/audits/replay_audit.py` (or equivalent) runs determinism tests.
- Generates audit report showing:
  - Seed used
  - Event count and range
  - Hash trajectory
  - Divergence point (if any)
  - Replay verification result (pass/fail)

*Sourced from Section 4, PHASE 0, "Audit & Replay Requirements," and Section 2.4.*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 1.

### 8.1 Determinism Replay Test

**Requirement:** Run the system twice with identical seed and InputEvent sequence; verify identical `world_hash` at every checkpoint.

**Acceptance Criteria:**
- Two full system runs with same seed produce identical hash timeline.
- Tick count and hash values match exactly.
- Test MUST be automated (CI gated).

### 8.2 Snapshot Equivalence Test

**Requirement:** Load a snapshot at tick T; replay events from T onward. Verify that hash sequence from T matches full replay from genesis.

**Acceptance Criteria:**
- Snapshot format is stable and versioned.
- Replay from snapshot produces identical hashes as full replay.
- Test MUST be automated (CI gated).

### 8.3 Hash-Chain Integrity Test

**Requirement:** Verify that event log hash-chain is unbroken from genesis to latest tick.

**Acceptance Criteria:**
- Hash-chain computation produces consistent verification report.
- No gaps or corrupted hashes detected.
- Test MUST be automated (CI gated).

### 8.4 Boot Validation Test

**Requirement:** System boots offline; no external network dependency in authority path.

**Acceptance Criteria:**
- `docker compose up --build` succeeds without external network.
- Keycloak login works (local instance).
- Engine ticks advance deterministically.
- UI displays tick and current hash.

### 8.5 Authority Leakage Test

**Requirement:** Verify that server cannot compute or override world state.

**Acceptance Criteria:**
- Static analysis scans for server-side RNG, state computation, outcome calculation.
- Runtime test confirms engine owns all state mutations.
- Test MUST be automated (CI gated).

*Sourced from Section 4, PHASE 0, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 0 is considered complete:

1. **Build Gate:**
   - `cargo build --release` succeeds (engine).
   - `npm run build` succeeds (web + server).
   - `docker-compose build` succeeds.

2. **Offline Functionality Gate:**
   - `docker compose up --build` starts without external network.
   - Keycloak login works; RBAC enforced.
   - Events appended with hash-chain; no persistence errors.

3. **Determinism Gate:**
   - Engine ticks advance; UI shows tick + current hash.
   - Replay produces identical hashes (regression test).
   - Snapshot replay == full replay (regression test).

4. **Authority Boundary Gate:**
   - Static analysis scans for prohibited patterns (server-side RNG, outcome computation).
   - Zero authority leakage detected.

5. **Event Log Gate:**
   - Events are immutably logged.
   - Hash-chain verification endpoint responds correctly.

*Sourced from Section 4, PHASE 0, "CI Gates."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 0:

- **No external network in authority path** (Section 1.1, Offline-First Law).
  - Exception: Optional external services (LLM, backup IdP) allowed only as plugins after core infrastructure boots.

- **No wall-clock time in state evolution** (Section 1.1, Determinism Law).
  - State progression MUST be tick-based only.

- **No nondeterministic RNG** (Section 1.1, Determinism Law).
  - All RNG MUST be engine-side, seeded, logged.

- **No unlogged mutations** (Section 1.1, Transparency Law).
  - Every world state change MUST correspond to an event in the log.

- **No admin bypass of event log** (Section 6.3, "How Admin Actions Are Logged").
  - Even admin InputEvents MUST flow through standard pipeline.

- **No server-side outcome computation** (Section 2.1, "Authority Boundaries").
  - Server MUST NOT compute physics, biology, cognition, or RNG outcomes.

- **No TODO/FIXME/stub/mock/fake implementations** (Section 1.1, No-Mock / No-Stub Law).
  - All features exercised via automated tests; no placeholders.

*Sourced from Section 4, PHASE 0, "Explicit Prohibition List (Phase 0)," and Section 1.1.*

## 11. Phase Completion Criteria (Checklist)

Phase 0 is NOT complete until ALL of the following are satisfied:

- [ ] **Offline stack boots** — `docker compose up --build` succeeds without external network
- [ ] **Events immutably logged with hash-chain** — Event log append-only in Postgres; hash-chain verified
- [ ] **Determinism proven via replay test** — Same seed + InputEvents → identical world_hash sequence
- [ ] **No authority leakage detected** — Static analysis + runtime test confirm server cannot mutate state
- [ ] **Keycloak login works** — RBAC enforced; no external auth required (offline-capable)
- [ ] **Engine ticks advance** — UI shows tick counter and current world_hash
- [ ] **Snapshot mechanism functional** — Snapshots taken; format stable and versioned
- [ ] **CI gates pass** — Build, offline boot, determinism, authority tests all pass
- [ ] **All mandatory tests pass** — Determinism, snapshot, hash-chain, boot, authority leakage tests
- [ ] **Zero TODO/FIXME/stub/mock in gated source** — Static analysis confirms

*Sourced from Section 4, PHASE 0, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.1, 4.0 (PHASE 0), 5.0, and 7.1-7.4. Any implementation deviating from this plan is invalid and must fail closed. The determinism invariant, offline-first requirement, and authority boundary enforcement specified herein are constitutional principles that may never be weakened or suspended.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 0, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 0, "Governance Domains Introduced" |
| 3. Systems & Modules Touched | Section 4, PHASE 0, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 0, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 0, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.1 "Authority Boundaries"; Section 2.3 "Enforcement Points"; Section 1.1 "Offline-First Law" |
| 7. Audit & Replay Requirements | Section 4, PHASE 0, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 0, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 0, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 0, "Explicit Prohibition List (Phase 0)"; Section 1.1 "No-Mock / No-Stub Law", "Offline-First Law", "Determinism Law", "Transparency Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 0, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 0 only (bootstrap phase)
