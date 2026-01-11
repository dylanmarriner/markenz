# Phase 0, 1, 2 Completion Plan - Priority Order

## Current Status
- ✅ Builds successfully
- ✅ Basic types and RNG working
- ✅ Event schemas defined
- ❌ Database NOT implemented
- ❌ Authority pipeline is skeleton
- ❌ Physics/collision NOT implemented
- ❌ No end-to-end testing

## CRITICAL PATH TO 100% COMPLETION

### TIER 1: ABSOLUTE BLOCKERS (Phases 0, 1, 2 cannot work without these)

#### 1.1 Authority Pipeline Real Implementation (Phase 2 - 100 LOC)
**Current**: All passes return dummy data
**Required**: Actual state mutations for Move, Gather, Mine, Build, Craft

Files to fix:
- `apps/engine/src/authority_pipeline.rs`: implement all 10 passes with REAL logic
  - Perception: scan visible entities
  - Intent: queue agent actions
  - Volition: validate actions against world state
  - BioVeto: check agent health/energy
  - PhysicsValidate: check terrain collision
  - PolicyValidate: check governance rules
  - Commit: apply state changes to Universe
  - ObservationEvents: emit actual changes

#### 1.2 Physics/Collision System (Phase 2 - 150 LOC)
**Current**: Position struct only, no collision detection
**Required**: Real heightmap-based collision checking

Files to fix:
- `crates/physics/src/collision.rs`:
  - Implement `can_occupy()`: check terrain height and agents
  - Implement `move_to()`: validate movement and update position
  - Integer-only math (no floating point)

#### 1.3 Action Mechanics Real Implementation (Phase 2 - 200 LOC)
**Current**: Stubs returning placeholder data
**Required**: Real action validation and execution

Files to fix:
- `crates/world/src/gathering.rs`: gather resources by biome
- `crates/world/src/mining.rs`: mine ores in mountains only
- `crates/world/src/crafting.rs`: craft items from recipes
- `crates/world/src/building.rs`: build structures on terrain

#### 1.4 Fix Engine Loop to Call Authority Pipeline (Phase 0 - 50 LOC)
**Current**: Authority pipeline called but results ignored
**Required**: Actually apply ObservationEvents from authority

Files to fix:
- `apps/engine/src/main.rs`: capture and log observation events
- `apps/engine/src/tick_loop.rs`: integrate results into universe

### TIER 2: PHASE 1 COMPLETION (Determinism verification)

#### 2.1 Snapshot Integration (Phase 1 - 50 LOC)
**Current**: write_snapshot/read_snapshot exist but never called
**Required**: Call them every N ticks from engine loop

Files to fix:
- `apps/engine/src/main.rs`: add snapshot writes every 500 ticks
- `apps/engine/src/tick_loop.rs`: integrate snapshots

#### 2.2 End-to-End Determinism Tests (Phase 1 - 100 LOC)
**Current**: test_determinism.rs only tests snapshot equality
**Required**: Cross-run hash equality, snapshot replay equivalence

Files to create:
- `tools/test/cross_run_hash_equality_test.sh`
- `tools/test/snapshot_equivalence_test.sh`
- `tools/test/replay_determinism_test.sh`

### TIER 3: PHASE 0 REAL INFRASTRUCTURE (Database)

#### 3.1 PostgreSQL Integration (Phase 0 - 300 LOC)
**Current**: No database at all
**Required**: Real PostgreSQL connection and event persistence

Dependency: `sqlx` 0.7

Files to create:
- `apps/engine/src/db.rs`: PostgreSQL client
- `apps/engine/src/db/input_events.rs`: fetch events from DB
- `apps/engine/src/db/hash_checkpoints.rs`: store hashes
- Update `apps/engine/Cargo.toml`: add sqlx

#### 3.2 Server HTTP Framework (Phase 0 - 400 LOC)
**Current**: No server implementation
**Required**: Axum-based HTTP server with endpoints

Dependency: `axum`, `tokio`, `tower`

Files to create:
- `apps/server/src/main.rs`: HTTP server
- `apps/server/src/handlers/input_events.rs`: POST /api/input-event
- `apps/server/src/handlers/hash_checkpoints.rs`: GET /api/hash-checkpoints
- `apps/server/src/ws.rs`: WebSocket /api/events
- Create `apps/server/Cargo.toml`

#### 3.3 Auth/Keycloak (Phase 0 - 150 LOC)
**Current**: No authentication
**Required**: JWT token validation

Files to create:
- `apps/server/src/auth.rs`: JWT verification
- `apps/server/src/auth/keycloak.rs`: JWKS caching
- Configure test Keycloak realm

## IMPLEMENTATION ORDER (from blocking to supporting)

1. **Authority Pipeline** (Tier 1.1) - Makes engine tick meaningfully
2. **Physics/Collision** (Tier 1.2) - Enables movement validation
3. **Action Mechanics** (Tier 1.3) - Enables gathering/mining/crafting
4. **Engine Loop Integration** (Tier 1.4) - Makes results observable
5. **Snapshot Integration** (Tier 2.1) - Makes phase 1 testable
6. **Determinism Tests** (Tier 2.2) - Verifies phase 1 gates
7. **PostgreSQL** (Tier 3.1) - Real persistence for Phase 0
8. **Server HTTP** (Tier 3.2) - Real API for Phase 0
9. **Auth/Keycloak** (Tier 3.3) - Real security for Phase 0

## ESTIMATED EFFORT

- Tier 1 (Critical path): ~500 LOC, 3-4 hours
- Tier 2 (Determinism): ~150 LOC, 1-2 hours
- Tier 3 (Infrastructure): ~850 LOC, 5-6 hours
- **Total**: ~1500 LOC, 9-12 hours for 100% complete Phase 0, 1, 2

## SUCCESS CRITERIA

### Phase 0 ✅
- [ ] cargo build --release succeeds
- [ ] docker compose up starts all services
- [ ] PostgreSQL stores InputEvents
- [ ] Server accepts HTTP POST /api/input-event
- [ ] WebSocket streams ObservationEvents
- [ ] Keycloak JWT validation works

### Phase 1 ✅
- [ ] 3 independent runs produce identical world_hash sequences
- [ ] Snapshots taken every 500 ticks
- [ ] Snapshot replay produces identical hashes as full run
- [ ] RNG audit log is deterministic

### Phase 2 ✅
- [ ] Terrain generation is deterministic (same seed = same terrain)
- [ ] Action validation is deterministic (same inputs = same results)
- [ ] Agents move, gather, mine, build, craft with real mechanics
- [ ] No floating-point in authority state

## NO STUBS. NO MOCKS. ALL REAL CODE.

Every function implemented will:
- Have real logic, not placeholder returns
- Use deterministic algorithms (integer math, ChaCha20, no randomness except RNG)
- Be tested with actual data
- Follow the plan specifications exactly

---

**Next Action**: Start with Authority Pipeline (Tier 1.1) - this unblocks everything else.
