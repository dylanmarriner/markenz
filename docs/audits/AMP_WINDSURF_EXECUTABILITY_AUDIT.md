# AMP AUDIT REPORT: MARKENZ N1.2 + N1.3

**DATE:** 2026-01-11
**AUDITOR:** ANTIGRAVITY (AMP)
**TARGET:** Normalization Pass N1.2 (Determinism) + N1.3 (WorldLoop)
**VERDICT:** **PASS (GO)**

## 1. Executive Summary

The Constitutional Audit of Markenz Normalization Passes N1.2 and N1.3 confirms that the system adheres to the binding authority of the **MARKENZ_EXECUTION_ROADMAP**, **AMP_DEFINITION_OF_DONE_v2**, and **MARKENZ_TARGET_ARCHITECTURE_v2**.

The Rust-based `apps/engine` has been verified as the sole authority for world state, implementing a deterministic, fixed-timestep loop that is isolated from wall-clock time and OS entropy. The CI guard prevents regressions, and the authority boundaries between Engine (Rust) and Server (Node.js) are respected.

**Decision:** Markenz is **authorized to proceed** to Phase 2.

---

## 2. Findings Table

| Area | Rule | Status | Evidence | Severity |
| :--- | :--- | :--- | :--- | :--- |
| **A1** | **RNG Purity** | **PASS** | `apps/engine/src/main.rs`: Uses `DeterministicRng` seeded from config. `crates/rng` wraps `rand_chacha`. No `thread_rng` in authority. | - |
| **A2** | **Time Purity** | **PASS** | `std::time` used ONLY for scheduling (`interval.tick`). Simulation uses monotonic `tick_index`. No `SystemTime` in logic. | - |
| **A3** | **CI Guard** | **PASS** | `tools/ci/guard.sh` bans `Date.now`, `rand::`, `TODO`, `FIXME` in `apps/`, `crates/`, `infra/`. | - |
| **B1** | **WorldLoop** | **PASS** | `apps/engine/src/tick_loop.rs`: Single-threaded loop owns `universe`, increments `tick` before processing. | - |
| **B2** | **World Hash** | **PASS** | `universe.world_hash` computed/emitted every tick. `crates/world` has `hashing` module. | - |
| **B3** | **Genesis** | **PASS** | `genesis_snapshot` created before loop. `write_snapshot` called periodically. | - |
| **B4** | **No-Input** | **PASS** | Engine ticks successfully with empty InputEvent vector (current state). | - |
| **C1** | **Leakage** | **PASS** | `apps/server` handles auth/persistence only. Checkpoint handler stores matching tick/hash, does not compute it. | - |

---

## 3. Violation Analysis

No blocking violations were found.

* **Minor Note:** `apps/engine/Cargo.toml` and `crates/world/Cargo.toml` import `rand` and `rand_chacha`. While `rand` is imported, code analysis confirms it is not used for global entropy (`thread_rng`) in the identified authority paths. The `DeterministicRng` abstraction strictures usage.

---

## 4. Go / No-Go Decision

**VERDICT: GO**

The System Foundation (N1.2 + N1.3) is **SOLID**.
The codebase proofs match the binding architecture documents.
Windsurf is authorized to begin **Phase 2 (World Model v1)**.

### Required Next Actions

None. The baseline is clean.
