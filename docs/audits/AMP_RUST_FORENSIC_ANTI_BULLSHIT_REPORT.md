# AMP Forensic Anti-Bullshit Audit Report
## Markenz Rust Engine - Phase 0/N1.3

**Date**: 2026-01-11  
**Auditor**: AMP (FORENSIC MODE)  
**Scope**: `/apps/engine/src/`, `/crates/world/src/`, `/crates/rng/src/`, `/crates/persistence/src/`

---

## OVERALL VERDICT

**🔴 FAIL - CRITICAL DETERMINISM VIOLATIONS**

Phase 0/N1.3 implementation is **INVALID** for determinism claims. The codebase contains:
- **Inert world loop** (tick advancement without state evolution)
- **Dead code passes** (perception/intent/volition that do not affect state)
- **Hash stasis violation** (world_hash constant despite tick advancing)
- **Fake RNG integration** (RNG streams created but never consumed in state mutations)
- **False persistence** (snapshots serialize static state; replay ignores them)

Execution MUST NOT proceed until these are fixed.

---

## CATEGORY 1: WORLD LOOP REALITY CHECK

### Verdict: **FAIL**

**Issue**: The tick loop increments `universe.tick` but **does not cause world state to evolve**.

**Evidence**:

**File**: `apps/engine/src/main.rs` (lines 70-87)

The loop advances tick but calls `authority_pipeline::process_tick()` with empty input_events.

**File**: `apps/engine/src/main.rs` (lines 14-18)

```rust
async fn fetch_input_events_for_tick(_tick: u64) 
    -> Result<Vec<InputEvent>, Box<dyn std::error::Error>> {
    // For Phase N1.3: No database dependency - return empty events
    Ok(Vec::new())  // ALWAYS RETURNS EMPTY
}
```

**File**: `apps/engine/src/authority_pipeline.rs` (lines 14-20)

The `process_tick` function only mutates world inside `for input_event in input_events.iter()`. Since input_events is always empty, the loop never executes.

**Root Cause**: 
- `fetch_input_events_for_tick` hardcoded to return `Vec::new()`
- `authority_pipeline::process_tick` only mutates world inside the input event loop
- Since input_events is always empty, **world state is never modified**
- Only `apply_state_transition()` recomputes hash
- Since `apply_state_transition()` is never called, **hash is never recomputed**

**Impact**: 
- Tick loop is **structurally inert**
- Appears to process ticks but produces no state change

---

## CATEGORY 2: DETERMINISTIC RNG REALITY CHECK

### Verdict: **FAIL**

**Issue**: DeterministicRng is created and streams are defined, but **RNG outputs never affect persisted state**.

**File**: `apps/engine/src/authority_pipeline.rs` (line 5)

The `_rng` parameter is prefixed with underscore (Rust convention for unused variables). It is never called.

**File**: `apps/engine/src/genesis.rs` (lines 66-71)

RNG is used only in genesis terrain generation, not in the tick loop.

**Impact**:
- RNG is "available" but never used during runtime
- RNG subsystems are declared but never exercised in state evolution

---

## CATEGORY 3: HASH BINDING CHECK (CRITICAL)

### Verdict: **FAIL**

**Issue**: World hash is computed once at genesis and **never updated despite tick advancing**.

The hash function serializes the entire Universe struct, which includes `pub tick: u64`. However, in the main loop, tick is incremented but the world_hash is not recomputed.

**Key Finding**: 
- `apply_state_transition()` is called **only** in `authority_pipeline::process_tick()`
- Since `input_events` is always empty, the loop never executes
- Therefore `apply_state_transition()` is never called
- Therefore `world_hash` is never recomputed
- **Hash is constant despite tick advancing**

**Impact**:
- Hash is constant across all ticks
- Promise of deterministic hash sequencing is FALSE

---

## CATEGORY 4: PERSISTENCE ROUND-TRIP CHECK

### Verdict: **FAIL**

**Issue**: Persistence code writes static genesis state. Replay code exists but is never called.

Snapshots are written at fixed cadence, but:
1. No state changes occur, so snapshots contain identical state
2. Replay functionality exists but is never integrated into the engine flow
3. The engine always creates a new universe from seed, never loading from snapshot

**Impact**:
- Persistence is **structurally present but operationally disconnected**

---

## CATEGORY 5: DEAD / DECOY CODE DETECTION

### Verdict: **FAIL**

**Dead Code Identified**:

1. **Authority Pipeline Validation Passes**: Perception/intent/volition passes are instantiated but never consumed
2. **Unused RNG Parameter**: In `authority_pipeline::process_tick`, the `_rng` parameter is unused
3. **Unused Data Structures**: PerceptionData, IntentData, ActionPlan, PlannedAction are instantiated but never affect state
4. **Snapshot Writing**: Snapshots are written but never read back in the engine

---

## CATEGORY 6: PLACEHOLDER / FAKE LOGIC DETECTION

### Verdict: **FAIL**

**Placeholder Logic Found**:

1. **Hardcoded Empty Event Fetching**: `fetch_input_events_for_tick` always returns `Vec::new()`
2. **Always-True Validation**: Functions like `validate_biology_safety()` and `validate_policy()` return true for all inputs
3. **Hardcoded Genesis State**: Agents are always at the same positions with identical vitals

---

## CATEGORY 7: DETERMINISM VIOLATION SCAN

### Verdict: **PASS**

**No traditional determinism violations found**:
- No wall-clock time in state computation
- No system time in state computation  
- Uses BTreeMap (ordered) for agents/assets/chunks
- No parallel execution
- RNG is properly seeded

**However**: Determinism is achieved by **doing nothing**. This is determinism by nullification, not mechanism.

---

## SUMMARY TABLE

| Category | Verdict | Core Issue |
|----------|---------|-----------|
| 1. World Loop | FAIL | No state evolution |
| 2. RNG Reality | FAIL | RNG unused in tick loop |
| 3. Hash Binding | FAIL | Hash never recomputed |
| 4. Persistence | FAIL | Snapshots write-only |
| 5. Dead Code | FAIL | Unused code throughout |
| 6. Fake Logic | FAIL | Placeholder implementations |
| 7. Determinism | PASS | No traditional violations |

---

## EXPLICIT FAILURE STATEMENT

### Phase 0/N1.3 is INVALID

Execution MUST NOT proceed because:

1. The world loop does not mutate state
2. The RNG is not integrated into state evolution
3. The hash is not bound to tick advancement
4. Persistence is unverified round-trip
5. Dead code exists throughout
6. Placeholder logic is disguised as implementation

---

## AUDIT SIGN-OFF

**Verdict**: **FAIL**  
**Critical Violations**: 6 of 7 categories  

Execution is **FORBIDDEN**.
