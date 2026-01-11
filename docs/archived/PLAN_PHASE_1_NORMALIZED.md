---
status: APPROVED
---

# PLAN_PHASE_1_NORMALIZED
## Deterministic Kernel + Replay Harness

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 1 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.2)  
**EXECUTOR:** Windsurf  
**GATE AUTHORITY:** AMP Principal-Level Auditor

---

## 1. AUTHORITY DECLARATION

This file is the **sole execution authority** for Phase 1.

**Higher Authorities:** KAIZA-MCP > MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md > MARKENZ_EXECUTION_ROADMAPv2.md > all others.

Phase 0 MUST be 100% complete and all gates passing before Phase 1 begins.

---

## 2. PHASE SCOPE (LOCKED)

**Objective:** Prove determinism formally via replay and snapshot equivalence. Implement subsystem RNG streams, canonical world hashing, and deterministic scheduling.

**Deliverables (EXACT):**
- Deterministic scheduler (tick index authoritative, fixed timestep)
- Subsystem RNG streams (Physics, Environment, Biology, Cognition, Genetics, Governance)
- Canonical world hashing (blake3, per-tick checkpoints)
- Snapshot write/read with RNG state preservation
- Replay harness (snapshot + events → identical hash sequence)
- Genesis snapshot (Gem-D, Gem-K, House, Shed, Tools, Vehicles with identity fingerprints)
- Determinism tests (cross-run equality, snapshot equivalence, RNG audit replay)

---

## 3. EXPLICIT NON-SCOPE (FORBIDDEN IN THIS PHASE)

- World representation beyond genesis (terrain, chunks) — Phase 2
- Biology mechanics (metabolism, hydration, circadian, immune, injury, endocrine) — Phase 3
- Cognition (perception, planning, language) — Phase 4
- Genetics (genome, reproduction) — Phase 5
- Social systems — Phase 6
- Governance/economy — Phase 7
- Rendering — Phase 8
- Security hardening — Phase 9

---

## 4. PRESERVATION CLAUSES (BINDING)

Per addendums:
- Gem-D and Gem-K resume with exact identity/memories/skills from Gemini export
- House, Shed, Tools, Vehicles preserved from Gemini export
- Identity fingerprints (blake3 hashes) computed and stored immutably
- No loss, no reset, no modification of imported state

---

## 5. DETERMINISM REQUIREMENTS

### 5.1 Fixed Timestep
- Tick index (u64) is authoritative time source
- Fixed dt (e.g., 50ms per tick) defined in genesis config
- Wall clock never enters state evolution
- No frame skipping, no delta time calculations

### 5.2 RNG Streams (CRITICAL)
- Root seed initialized once at genesis
- 6 deterministic subsystem streams:
  - Physics, Environment, Biology, Cognition, Genetics, Governance
- Algorithm: ChaCha20 (RFC 7539)
- Streams separated by subsystem and stream_id
- Every draw audit-logged with callsite
- Same seed across runs → identical value sequences

### 5.3 Canonical Serialization
- Big-endian byte order
- No floating-point in authority state
- Deterministic container ordering (BTreeMap/BTreeSet)
- Version-aware snapshot format (v1)

### 5.4 Hash Chain
- blake3(PrevHash || CanonicalSerialize(State))
- Computed after every tick
- Checkpoint every N ticks (configurable, default 500)
- Previous hash included in checkpoint record

### 5.5 Replay Equality
- Seed + ordered InputEvents → identical hash sequence (bit-for-bit)
- Snapshot replay from tick T → identical hashes for ticks T+
- Cross-platform identical (Linux x64/arm64, macOS)

---

## 6. IMPLEMENTATION OBLIGATIONS (ANTI-FAKE)

### 6.1 DeterministicRng Implementation

**Causal Input:** Root seed from genesis config  
**State Mutated:** RNG subsystem streams (internal counters)  
**Hash Impact:** RNG values logged as ObservationEvents, not directly hash-affecting  
**Replay Proof:** Same seed → identical sequence across runs

Anti-Fake: ChaCha20 only; no Math.random; state persisted in snapshots; streams separated by subsystem

### 6.2 RNG Audit Logging

**Causal Input:** Every RNG draw in engine code  
**State Mutated:** Global RNG audit log (append-only)  
**Hash Impact:** Audit trail (observable, not hash-affecting)  
**Replay Proof:** Replay produces identical RNG draw audit entries

Anti-Fake: Every draw logged with tick, subsystem, stream_id, callsite, counter, value

### 6.3 Genesis Snapshot

**Causal Input:** Imported Gemini agent/asset data + Markenz genesis seed  
**State Mutated:** Universe at tick 0  
**Hash Impact:** Genesis hash is blake3 of imported state  
**Replay Proof:** Same export data → identical genesis state (bit-for-bit)

Anti-Fake: Import pure function; identity fingerprints computed and verified; assets validated at genesis

### 6.4 Snapshot Format (v1)

**Causal Input:** Engine tick counter triggers snapshot write  
**State Mutated:** snapshots table (append-only)  
**Hash Impact:** Snapshot includes RNG state and world_hash  
**Replay Proof:** Load snapshot at tick T, apply events → identical to full run

Anti-Fake: Format includes tick, world_state, rng_state, world_hash, prev_hash, checksum

### 6.5 Replay Harness

**Causal Input:** Snapshot blob + remaining InputEvents from DB  
**State Mutated:** Universe during replay (temporary, for verification)  
**Hash Impact:** Replay hashes must match checkpoint hashes exactly  
**Replay Proof:** Snapshot replay produces identical outcomes as full run

Anti-Fake: Load → apply events → hash → compare; abort on mismatch; report first divergence

---

## 7. REQUIRED ARTIFACTS (WINDSURF OUTPUTS)

**Report Filename:** WINDSURF_PHASE_1_EXECUTION_REPORT.md  
**Absolute Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_1_EXECUTION_REPORT.md

**Report Must Include:**
- RNG implementation details (algorithm, streams, seed value)
- Snapshot format specification
- Cross-run determinism proof (3+ runs, hash sequences)
- Snapshot equivalence proof (snapshots at multiple ticks)
- RNG audit log sample (first/last entries, subsystems represented)
- Genesis snapshot metadata (Gem-D/Gem-K identity fingerprints)
- Replay harness test results
- Performance metrics (ticks/sec, no regression from Phase 0)

---

## 8. EXIT CRITERIA (VERIFIABLE)

**ALL REQUIRED. Failure = Phase 1 BLOCKED.**

### Build & Compilation
- [ ] cargo build --release succeeds with zero warnings
- [ ] crates/rng compiles without errors
- [ ] All snapshot/replay code compiles
- [ ] No clippy warnings

### Determinism & Replay
- [ ] TEST-DET-001: 3+ runs with identical seed produce identical hashes (ticks 0–1000)
- [ ] TEST-SNAPSHOT-EQ-001: Snapshots at ticks 250, 500, 750 replay identically to full run
- [ ] RNG bit-identity verified across platforms (if multi-platform available)
- [ ] RNG audit log replay identical across runs

### RNG & Hashing
- [ ] DeterministicRng produces deterministic sequences
- [ ] All 6 subsystem streams functional
- [ ] RNG audit log queryable and correct
- [ ] world_hash checkpoint at every tick
- [ ] Hash-chain integrity verified

### Genesis & Assets
- [ ] Gem-D loaded with identity fingerprint
- [ ] Gem-K loaded with identity fingerprint
- [ ] House/Shed at correct coordinates
- [ ] Tools/vehicles inventory present
- [ ] Genesis reproducible for same seed

### Snapshots
- [ ] Snapshots written every 500 ticks
- [ ] RNG state included in snapshots
- [ ] Snapshot read produces identical Universe
- [ ] Format versioned (v1)

### Integration
- [ ] Engine reads InputEvents from DB in canonical order
- [ ] Emits RNG draws as ObservationEvents
- [ ] Server stores hash checkpoints
- [ ] Web UI shows hash-chain status
- [ ] RNG audit log accessible via /api/rng-audit-log

### Audit Tools
- [ ] tools/audits/determinism_audit.py runs offline
- [ ] Audit verifies hash-chain end-to-end
- [ ] Audit report generated (JSON or PDF)

### AMP Sign-Off
- [ ] AMP Principal-Level Auditor approval in writing BEFORE Phase 2

---

## 9. DETERMINISM & REPLAY GATES

**Gate 1: Cross-Run Hash Equality (TEST-DET-001)**
- Condition: 3 independent runs, identical seed, 1000+ ticks
- Expected: All world_hash sequences identical
- Failure: STOP; escalate with divergence tick

**Gate 2: Snapshot Equivalence (TEST-SNAPSHOT-EQ-001)**
- Condition: Full run vs snapshots at 250, 500, 750
- Expected: Hashes match at all checkpoints
- Failure: STOP; escalate with divergent tick

**Gate 3: RNG Audit Log Replay**
- Condition: Same seed produces identical RNG draws
- Expected: Audit entries bit-identical
- Failure: STOP; escalate with RNG divergence

---

## 10. HARD STOP CONDITIONS

Execution STOPS IMMEDIATELY if:

1. Cross-run hash divergence detected
2. Snapshot equivalence fails
3. RNG seed produces different sequences
4. Snapshot format not deterministically serializable
5. Build fails
6. Identity fingerprints cannot be computed
7. Genesis assets missing
8. Hash-chain broken
9. Any gate test fails
10. AMP auditor directs halt

---

**END OF PHASE 1 NORMALIZED PLAN**
