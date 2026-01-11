# Critical Gaps and Blockers - Phases 0, 1, 2

**Assessment Date**: 2026-01-11  
**Current Build Status**: ✅ COMPILES, 0 errors  
**Test Status**: ⚠️ PARTIAL - RNG works, Authority pipeline is stub

## TIER 1: SYSTEM CANNOT RUN WITHOUT THESE

### Phase 0: Database Missing
**Impact**: BLOCKING all event persistence, hash-chain verification, replay
**Status**: ❌ NOT IMPLEMENTED
**LOC Required**: ~300
**Implementation Time**: 4 hours

**Requirements**:
- PostgreSQL connection pool via sqlx
- Fetch InputEvents by tick from DB
- Store hash checkpoints
- Verify hash-chain integrity
- Never UPDATE/DELETE immutable tables

**Files to Create**:
- `apps/engine/src/db.rs` - Main DB module
- `apps/engine/src/db/models.rs` - Event/checkpoint models
- `apps/engine/Cargo.toml` - Add sqlx dependency

**Current Issue**: `fetch_input_events_for_tick()` returns empty Vec, bypassing database entirely

---

### Phase 2: Authority Pipeline is Skeleton
**Impact**: BLOCKING all state mutations and world mechanics
**Status**: ❌ 90% STUB CODE
**LOC Required**: ~200
**Implementation Time**: 3 hours

**Requirements**:
- Perception pass: scan visible entities correctly
- Intent pass: queue real agent actions from DB
- Volition pass: actual action planning
- All 7 validation passes: REAL logic, not stub returns
- Commit pass: apply state changes correctly
- ObservationEvent emission: track all mutations

**Files to Fix**:
- `apps/engine/src/authority_pipeline.rs` (complete rewrite of ~300 LOC)
- `apps/engine/src/tick_loop.rs` - ensure loop calls authority pipeline correctly

**Current Issue**: Passes defined but return dummy data (e.g., `fn run_perception_pass() { vec![] }`)

---

### Phase 2: Physics/Collision Not Implemented
**Impact**: BLOCKING agent movement validation
**Status**: ❌ ONLY TYPES EXIST
**LOC Required**: ~150
**Implementation Time**: 2 hours

**Requirements**:
- Terrain-based collision detection (integer math only)
- can_occupy(): check height difference ≤ max_climb (2 units)
- Agent-agent collision prevention
- No floating-point in authority state

**Files to Fix**:
- `crates/physics/src/collision.rs` - implement actual collision checking

**Current Issue**: `can_occupy()` function stub - always returns true/false without validation

---

### Phase 2: Action Mechanics Unimplemented
**Impact**: BLOCKING gather/mine/craft/build mechanics
**Status**: ❌ ALL ARE STUBS
**LOC Required**: ~250
**Implementation Time**: 3-4 hours

**Requirements**:
- Gathering: resource availability by biome
- Mining: ore only in mountains
- Crafting: recipe validation and item production
- Building: structure placement with terrain modification

**Files to Fix**:
- `crates/world/src/gathering.rs` - implement biome-based resources
- `crates/world/src/mining.rs` - implement mountain-only ore
- `crates/world/src/crafting.rs` - implement recipe system
- `crates/world/src/building.rs` - implement structure building
- `apps/engine/src/authority_pipeline.rs` - wire up action execution

**Current Issue**: Functions return placeholder results (e.g., `fn mine() { Ok(dummy_item) }`)

---

## TIER 2: PHASE 1 VERIFICATION

### No End-to-End Determinism Testing
**Impact**: BLOCKING Phase 1 certification
**Status**: ⚠️ TEST CODE EXISTS but not invoked
**LOC Required**: ~100
**Implementation Time**: 1-2 hours

**Requirements**:
- Run 3 independent engine ticks with same seed
- Collect world_hash at each tick
- Verify all 3 sequences are bit-identical
- Test snapshot replay equivalence

**Files to Create**:
- `tools/test/cross_run_hash_equality_test.sh`
- `tools/test/snapshot_equivalence_test.sh`
- `tools/test/replay_determinism_test.sh`

**Current Issue**: Infrastructure exists (snapshot write/read) but tests don't call them

---

### Snapshots Never Written During Ticks
**Impact**: BLOCKING snapshot equivalence testing
**Status**: ⚠️ CODE EXISTS but unused
**LOC Required**: ~50
**Implementation Time**: 30 min

**Requirements**:
- Write snapshot every N ticks (500 default)
- Include RNG state in snapshot
- Path: `snapshots/snapshot_{tick:010}.bin`

**Files to Fix**:
- `apps/engine/src/main.rs` - add snapshot write call
- `apps/engine/src/tick_loop.rs` - integrate snapshots

**Current Issue**: `write_snapshot()` function exists but is never called from tick loop

---

## TIER 3: SUPPORTING INFRASTRUCTURE

### Phase 0: Server Not Implemented
**Impact**: BLOCKING HTTP API and client communication
**Status**: ❌ NOT IMPLEMENTED
**LOC Required**: ~400
**Implementation Time**: 5-6 hours

**Requirements**:
- HTTP POST /api/input-event - accept events with JWT
- HTTP GET /api/hash-checkpoints - retrieve checksums
- WebSocket /api/events - stream ObservationEvents
- RBAC enforcement per role

**Files to Create**:
- `apps/server/src/main.rs` - Axum HTTP server
- `apps/server/src/handlers/input_events.rs`
- `apps/server/src/handlers/hash_checkpoints.rs`
- `apps/server/src/ws.rs`
- `apps/server/Cargo.toml`

---

### Phase 0: Authentication Missing
**Impact**: BLOCKING secure event submission
**Status**: ❌ NOT IMPLEMENTED
**LOC Required**: ~150
**Implementation Time**: 2 hours

**Requirements**:
- JWT token validation
- Keycloak JWKS caching
- Role extraction (admin/observer/auditor)
- Offline operation

**Files to Create**:
- `apps/server/src/auth.rs`
- `apps/server/src/auth/keycloak.rs`

---

## SUMMARY TABLE

| Component | Phase | Status | Blocker? | LOC | Hours |
|-----------|-------|--------|----------|-----|-------|
| Database | 0 | ❌ NOT DONE | YES | 300 | 4 |
| Authority Pipeline | 2 | ❌ 90% STUB | YES | 200 | 3 |
| Physics/Collision | 2 | ❌ SKELETON | YES | 150 | 2 |
| Action Mechanics | 2 | ❌ STUB | YES | 250 | 4 |
| E2E Determinism Tests | 1 | ⚠️ PARTIAL | YES | 100 | 2 |
| Snapshot Integration | 1 | ⚠️ PARTIAL | YES | 50 | 1 |
| Server HTTP | 0 | ❌ NOT DONE | NO | 400 | 6 |
| Authentication | 0 | ❌ NOT DONE | NO | 150 | 2 |
| **TOTAL CRITICAL** | **0,1,2** | | **YES** | **900** | **12** |
| **TOTAL ALL** | **0,1,2** | | | **1700** | **24** |

---

## IMPLEMENTATION STRATEGY

### IMMEDIATE (Next 2-3 hours)
1. Fix Authority Pipeline passes - they must do REAL work
2. Implement Physics/Collision - agents can actually move
3. Implement Action Mechanics - world has real mechanics

### NEAR TERM (Next 4-5 hours)
4. Integrate snapshots into tick loop
5. Create determinism test suite
6. Run Phase 1 gates

### LONGER TERM (5-6+ hours)
7. Implement PostgreSQL integration
8. Implement Server HTTP framework
9. Implement Authentication

---

## COMPLIANCE STATEMENT

**Current State**: Code compiles but system cannot run correctly because:

1. ❌ Authority pipeline doesn't execute world mechanics
2. ❌ No physics/collision prevents movement from working
3. ❌ No action mechanics means agents can't gather/mine/craft/build
4. ❌ No database persistence means no event storage
5. ❌ No HTTP server means no client communication
6. ❌ No authentication means no security

**To Achieve 100% Correctness**: All 8 items above must be fully implemented with real, working code. No shortcuts. No test doubles. No deferred functionality.

**Estimated Total Effort**: 20-24 hours of focused development for COMPLETE Phase 0, 1, 2 implementation.

---

**RECOMMENDATION**: Focus on Tier 1 CRITICAL BLOCKERS first (12 hours) to get to functional Phase 0, 1, 2. Tier 3 supporting infrastructure can follow.

