---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 2
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_1_DETERMINISTIC_KERNEL_AND_REPLAY_HARNESS_LOCK
---

# MARKENZ — GOVERNANCE PHASE 2: WORLD REPRESENTATION V1 (TERRAIN + ENTITIES + INVENTORY)

## 1. Phase Objective

Replace abstract world with deterministic spatial reality; introduce real mechanics (terrain chunks, resource depletion, item ownership, inventory).

## 2. Governance Domains In Scope

- **Resource access & scarcity** (terrain chunks, depletable resources)
- **Property & ownership** (items have owners, inventory ownership tracked)

*Sourced from Section 4, PHASE 2, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/world` (chunked terrain, deterministic generation)
- `crates/physics` (collision, movement, reach)
- `crates/inventory` (items, ownership, slots)
- `apps/engine` — Mechanics: gather, mine, move, use-tool

*Sourced from Section 4, PHASE 2, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 2 MUST be defined and logged:

- `TerrainChunkGenerated` (coordinate, seed offset, biome type)
- `EntityCreated` (id, type, location, owner)
- `InventoryUpdated` (agent_id, item_id, location, stack_change)
- `ActionAttempted` (agent, action, target, success/veto_reason)

*Sourced from Section 4, PHASE 2, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 2 completion, the following properties MUST hold:

- **Terrain Determinism:** Same chunk coordinate + root seed → identical biome/resources.
- **Inventory Determinism:** Same action sequence → identical inventory state.
- **Physics Determinism:** Same movement inputs → identical collisions/positions.
- **Resource Depletion:** Same gather actions → identical resource counts and availability.

*Sourced from Section 4, PHASE 2, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### PhysicsValidate (Newly Active)

- **Collision Detection:** Agent cannot move through terrain or entities.
- **Reach Constraint:** Agent can only interact with entities within reach distance.
- **Tool Requirements:** Complex actions (mining, harvesting) require appropriate tools; action vetoed if tool missing.
- **Veto Emission:** Every physics veto must emit reason code + message.

*Sourced from Section 2.3, "Enforcement Points," and Section 7.2 "Runtime Veto Behavior."*

### Property & Ownership Enforcement

- **Ownership Registry:** Every item has immutable owner field (agent ID or "unowned").
- **Transfer Rules:** Ownership changes only via explicit transfer events (gather into inventory, trade, theft-detected, inheritance).
- **Inventory Binding:** Agents cannot use items they do not own (veto at PhysicsValidate).

*Sourced from Section 3.2 "Property & Ownership."*

### Resource Scarcity

- **Finite Resources:** Terrain has fixed resources per chunk; gathering depletes.
- **Growth Mechanics:** Some resources grow over time (crops, animals) based on environment (deterministic spawn rates).
- **Overharvesting Veto:** If resource depleted below threshold, gather action vetoed.

*Sourced from Section 3.5 "Resource Access & Scarcity."*

## 7. Audit & Replay Requirements

### Chunk Generation Log

- `tools/audits` verifies chunk generation reproducibility.
- Audit report shows: coordinate range, biome distributions, resource spawn counts.
- Same seed + coordinates must regenerate identical chunks.

### Inventory Snapshots

- Inventory state captured at intervals.
- Verification: replaying gather actions from snapshot produces consistent state.
- Audit report shows: item counts, ownership validity, no orphaned items.

### Action Causality Trace

- Event log shows: InputEvent → [Perception Gate → Intent → BioVeto] → PhysicsValidate → [Commit or Veto] → Outcome.
- Every action attempts and every veto reason visible in log.
- Tool used in action recorded; reach/collision details captured.

*Sourced from Section 4, PHASE 2, "Audit & Replay Requirements," and Section 2.4 "Audit & Replay Implications."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 3.

### 8.1 Terrain Determinism Test

**Requirement:** Chunk generation at same coordinate with same root seed must produce identical biome and resources.

**Acceptance Criteria:**
- Seed S, Coordinate C → Biome B, Resources R
- Seed S, Coordinate C (replay) → Biome B, Resources R (identical)
- Multiple coordinates tested; all must match.
- Test automated; CI gated.

### 8.2 Inventory Determinism Test

**Requirement:** Gather actions (and transfers) in same sequence must produce identical inventory state.

**Acceptance Criteria:**
- Agent A gathers items [I1, I2, I3] in sequence → Inventory state IV.
- Replay same agent, same items, same sequence → Inventory state IV (identical).
- Ownership records match exactly (agent ID on each item).
- Test automated; CI gated.

### 8.3 Physics Determinism Test

**Requirement:** Movement actions in same sequence must produce identical collision results and positions.

**Acceptance Criteria:**
- Agent moves to [X1, Y1, X2, Y2, ...] with same obstacles → identical position trajectory.
- Collision detection results (hit/no-hit) reproduce exactly.
- Veto reasons (collision blocked, reach insufficient) reproducible.
- Test automated; CI gated.

### 8.4 Causality Trace Test

**Requirement:** Event log must show complete action causality: input → validation → veto/commit → outcome.

**Acceptance Criteria:**
- For every action attempt, log shows: agent, action type, target, veto reason (if vetoed) or success outcome.
- Tools used in actions are recorded.
- Physics validation details (reach distance, collision surface) captured.
- Test automated; verify log completeness.

### 8.5 Resource Scarcity Test

**Requirement:** Resource gathering must deplete resources; overharvesting must trigger veto.

**Acceptance Criteria:**
- Chunk starts with N resource units.
- Gather action removes M units → N-M remaining.
- When N-M < threshold, next gather action vetoed (overharvesting).
- Resource regeneration (if implemented) is deterministic.
- Test automated; CI gated.

*Sourced from Section 4, PHASE 2, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 2 is considered complete:

1. **World Model Complete:**
   - Chunked terrain system implemented.
   - Deterministic generation working (seed → identical chunks).
   - Merge not blocked if incomplete.

2. **Gather Action Determinism:**
   - Same seed + gather sequence → identical inventory state.
   - Test automated; CI gated.

3. **Physics Implementation:**
   - Movement system with collision detection.
   - Reach constraint enforced; tool requirements checked.
   - Test automated; CI gated.

4. **UI Integration:**
   - Web UI displays terrain chunks (graphically).
   - Inventory visible to agent.
   - Item ownership labels shown (if ownership visible in UI spec).

5. **Replay Produces Identical State:**
   - Determinism replay test passes (Phase 1 invariant maintained).
   - World state hash stable across runs.

6. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 2, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 2:

- **No floating-point in authority** (Section 1.1, "Determinism Law").
  - Physics uses deterministic fixed-point or integer math only.
  - Positions, velocities, collision checks must be exact, not approximate.

- **No unlogged resource depletion** (Section 3.5, "Resource Access & Scarcity").
  - Every gather action must emit InventoryUpdated or ActionAttempted event.
  - Resource count changes must be visible in event log.

- **No inventory changes outside events** (Section 4, PHASE 2, "Explicit Prohibition List").
  - All inventory mutations must be triggered by InputEvents (gather, trade, receive).
  - No direct inventory editing by server or admin without event emission.

- **No ownership changes without transfer events** (Section 3.2, "Property & Ownership").
  - Item ownership can only change via explicit transfer (gather, trade, inheritance).
  - No silent ownership reassignment.

- **No arbitrary entity deletion** (Section 3.4, "Violence & Harm Constraints").
  - Entities removed only via explicit death/destruction event.
  - Removal reason logged and auditable.

- **No PhysicsValidate bypass** (Section 7.2, "Runtime Veto Behavior").
  - Server cannot authorize movement through solid terrain.
  - Tool requirements cannot be waived by admin action (must use InputEvent).

*Sourced from Section 4, PHASE 2, "Explicit Prohibition List (Phase 2)," and Section 1.1 "Determinism Law."*

## 11. Phase Completion Criteria (Checklist)

Phase 2 is NOT complete until ALL of the following are satisfied:

- [ ] **Terrain generation deterministic** — Same seed + coordinate → identical chunks, byte-for-byte
- [ ] **Inventory mechanics working** — Agents can gather items; ownership tracked; inventory updates logged
- [ ] **Movement + gather tested** — Movement deterministic; gather deterministic; both tested in CI
- [ ] **Physics validation enforced** — Reach constraints, tool requirements, collision detection working
- [ ] **World state hashing stable** — Terrain + inventory + entity positions hash consistently
- [ ] **Resource depletion tracked** — Overharvesting detected; scarcity enforced; veto emitted
- [ ] **All mandatory tests pass** — Terrain, inventory, physics, causality, scarcity tests
- [ ] **CI gates pass** — Build, determinism, gather, physics, UI integration gates
- [ ] **Ownership registry working** — Every item has owner; transfers logged; no orphaned items
- [ ] **Determinism maintained from Phase 1** — Replay test from Phase 1 still passes

*Sourced from Section 4, PHASE 2, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.3, 3.2, 3.4, 3.5, 4.0 (PHASE 2), and 7.2-7.3. Any implementation deviating from this plan is invalid and must fail closed. The determinism guarantee, authority boundary separation, and property ownership rules specified herein may never be weakened.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 2, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 2, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 2, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 2, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 2, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.3 "Enforcement Points"; Section 3.2 "Property & Ownership"; Section 3.5 "Resource Access & Scarcity" |
| 7. Audit & Replay Requirements | Section 4, PHASE 2, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 2, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 2, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 2, "Explicit Prohibition List (Phase 2)"; Section 1.1 "Determinism Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 2, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 1 (completed)
