---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 8
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_7_GOVERNANCE_AND_ECONOMY
---

# MARKENZ — GOVERNANCE PHASE 8: WEBGPU RENDERER AND VISUALIZATION UPGRADE

## 1. Phase Objective

Implement professional visualization derived from authoritative snapshots. The renderer is never authoritative; it is a pure read-only view of world state computed by engine.

## 2. Governance Domains In Scope

- **None new** (Phase 8 is purely visualization; no new governance domains introduced)

*Sourced from Section 4, PHASE 8, "Governance Domains Affected: None new (purely visualization)."*

## 3. Systems & Modules Touched

- `apps/web` (WebGPU-based renderer)
- `apps/engine` — Render packet generation (derived from snapshots; hashable, stable)

*Sourced from Section 4, PHASE 8, "Engine Modules Touched."*

## 4. Event Types

Events introduced in Phase 8:

- `RenderPacketGenerated` (snapshot_id, packet_hash, entities, terrain, effects)

*Sourced from Section 4, PHASE 8, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 8 completion, the following properties MUST hold:

- **Render Packet Determinism:** Same snapshot → identical render packet hash.
- **Visualization Determinism:** Deterministic layout (no floating-point randomness in positioning/scaling for same snapshot).
- **Replay Consistency:** Rendering same snapshot at different times produces identical visual output.

*Sourced from Section 4, PHASE 8, "Determinism Guarantees."*

## 6. Enforcement Rules

### Renderer Authority Boundary

- **Engine is authoritative:** Renderer reads from snapshots; never computes or mutates world state.
- **Render packets are derived:** Render packets generated from snapshots; not stored as authoritative state.
- **No two-way binding:** Renderer cannot influence engine decisions or outcomes.
- **Read-only operation:** Renderer has zero write access to world state database.

*Sourced from Section 2.1 "Authority Boundaries (Non-Negotiable)," Section 4, PHASE 8, "Objective: Renderer never authoritative."*

### Visualization Constraints

- **Deterministic layout:** Entity positions, terrain chunks, effects placed deterministically from snapshot data.
- **No floating-point nondeterminism:** All positioning uses deterministic fixed-point or integer math (or pre-computed float values from engine).
- **Stable rendering:** Same snapshot always produces identical visual (pixel-perfect, given display resolution).

## 7. Audit & Replay Requirements

### Render Packet Audit

- `tools/audits` verifies render packet hash stability.
- Audit report shows: snapshot_id, render packet hash, entity count, terrain chunk count, effects count.
- Verification: same snapshot (at different points in time) produces same render packet hash.

### Renderer Non-Authoritative Test

- Verify renderer has no write access to world state (Postgres, engine memory).
- Verify no execution path from renderer updates hit engine authority.
- Test: attempt to mutate world via UI → mutation blocked or silently ignored; no effect on game state.

*Sourced from Section 4, PHASE 8, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 9.

### 8.1 Render Packet Hash Stability Test

**Requirement:** Same snapshot produces identical render packet hash across multiple runs.

**Acceptance Criteria:**
- Load snapshot S (world state at tick T).
- Generate render packet P1 from snapshot S.
- Compute hash H1 of render packet P1.
- Later, load same snapshot S again (from database or replay).
- Generate render packet P2 from snapshot S.
- Compute hash H2 of render packet P2.
- H1 == H2 (bit-for-bit equality).
- Multiple snapshots tested (different ticks, different world states).
- Test automated; CI gated.

### 8.2 Renderer Non-Authoritative Test

**Requirement:** Renderer cannot mutate world state; no outcome-affecting mutations via UI.

**Acceptance Criteria:**
- Static analysis: verify no world-state write in renderer code (web/src).
- Runtime test: attempt to modify agent health, inventory, location via UI → mutation fails or is ignored.
- Engine state unchanged after attempted mutation via UI.
- Audit log shows no world-state change events.
- Test automated; CI gated.

### 8.3 Visualization Consistency Test

**Requirement:** UI visualization matches authoritative world state.

**Acceptance Criteria:**
- Agent A at location (100, 50) in engine state.
- Renderer displays agent A at pixel position corresponding to (100, 50).
- Inventory displays items matching engine inventory.
- Health bar shows engine health value.
- Visual matches engine state (within rendering fidelity constraints).
- Test automated or manual visual inspection.

### 8.4 WebGPU Integration Test

**Requirement:** WebGPU renderer functional; no crashes or rendering errors.

**Acceptance Criteria:**
- Renderer initializes WebGPU context without errors.
- Terrain chunks render without artifacts.
- Agents render and move smoothly.
- UI overlays (health bars, inventory, etc.) display correctly.
- Performance acceptable (60 FPS target or specified minimum).
- Test automated (integration test harness).

### 8.5 Determinism Test (Renderer Unaffected)

**Requirement:** Adding renderer does not break determinism guarantees from Phase 7.

**Acceptance Criteria:**
- Phase 7 determinism test still passes (engine hashes identical).
- Renderer operates on snapshots; does not affect engine tick rate or outcomes.
- Replay test from Phase 7 still produces identical hashes.
- Test automated; CI gated.

*Sourced from Section 4, PHASE 8, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 8 is considered complete:

1. **WebGPU Renderer Integrated:**
   - Renderer compiles and links without errors.
   - Webpack or build tool successfully bundles web code.

2. **Render Packets Hashable and Reproducible:**
   - Render packet generation deterministic (same snapshot → same hash).
   - Render packet format versioned (snapshots vs. packet versions compatible).

3. **UI Visualization Matches World State:**
   - Visual inspection: terrain, agents, items match engine state.
   - Health bars, inventory, stats match engine values.

4. **No Authority Leakage via Renderer:**
   - Static analysis: no world-state mutations from web code.
   - Runtime test: UI mutations blocked or ineffective.

5. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds (web + renderer).
   - `docker-compose build` succeeds.

*Sourced from Section 4, PHASE 8, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 8:

- **No world mutation via renderer** (Section 4, PHASE 8, "Explicit Prohibition List").
  - Renderer is read-only view only.
  - UI input must flow through server → engine (not bypass engine).

- **No floating-point nondeterminism in authority** (Section 1.1, "Determinism Law").
  - Renderer may use floats for positioning/rendering (client-side).
  - But engine authority path remains deterministic (fixed-point or integer).
  - Render packets derived from engine deterministically.

- **Renderer is client-side only** (Section 4, PHASE 8, "Explicit Prohibition List").
  - WebGPU runs in user's browser.
  - Cannot enforce business logic or governance rules (those are engine-side).
  - Cannot access sensitive data (encryption keys, audit logs, etc.).

- **No rendering shortcuts that compromise world correctness** (Section 4, PHASE 8, "Explicit Prohibition List").
  - If entity is visible in engine, it must be visible in renderer (no culling of important entities).
  - If agent is dead in engine, must appear dead in renderer (no showing dead agents as alive).

*Sourced from Section 4, PHASE 8, "Explicit Prohibition List (Phase 8)."*

## 11. Phase Completion Criteria (Checklist)

Phase 8 is NOT complete until ALL of the following are satisfied:

- [ ] **WebGPU renderer integrated** — Compiles and links; renderer available in UI
- [ ] **Render packets hashable** — Same snapshot → identical render packet hash
- [ ] **Visualization consistent with world state** — UI displays match engine state (agent positions, inventory, health, etc.)
- [ ] **No authority leakage via renderer** — Static analysis + runtime test confirm renderer cannot mutate world
- [ ] **All mandatory tests pass** — Render hash stability, non-authoritative, visualization consistency, WebGPU integration, determinism maintained tests
- [ ] **CI gates pass** — Build, renderer integration, render packet hashing, visualization match, authority leakage gates
- [ ] **Performance acceptable** — Renderer runs at specified FPS target (e.g., 60 FPS)
- [ ] **Determinism maintained from Phase 7** — Replay test from Phase 7 still passes; engine hashes unaffected by renderer

*Sourced from Section 4, PHASE 8, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 2.1, 4.0 (PHASE 8), and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The authority boundary between engine and renderer, and the determinism guarantee that rendering does not affect engine behavior, are inviolable.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 8, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 8, "Governance Domains Affected: None new" |
| 3. Systems & Modules Touched | Section 4, PHASE 8, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 8, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 8, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.1 "Authority Boundaries (Non-Negotiable)" |
| 7. Audit & Replay Requirements | Section 4, PHASE 8, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 8, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 8, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 8, "Explicit Prohibition List (Phase 8)"; Section 1.1 "Determinism Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 8, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 7 (completed)
