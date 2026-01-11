---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_0_BOOTSTRAP
phase: 0
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Offline Stack Baseline · Hash-Chain Integrity · Fixed Timestep Determinism
---

# PLAN PHASE 0: BOOTSTRAP
## (Offline Stack · Deterministic Tick Loop · Hash-Chain Authority)

**AUDIENCE:** Windsurf executor (direct execution only)  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Establish a fully offline, deterministically bootable Markenz stack where:
- Simulation advances via fixed timestep (u64 tick index, never wall-clock dependent)
- All world state mutations are performed by Rust engine exclusively
- Events are immutably logged to append-only database with cryptographic hash chain
- Server and Web cannot mutate state directly or indirectly
- RBAC enforcement gates all InputEvent submission
- Two agents (Gem-D, Gem-K) boot from genesis snapshot with preserved identity state
- Every tick produces world_hash checkpoints for determinism verification

---

## 2. ENTRY CONDITIONS (MUST BE TRUE BEFORE STARTING)

- Repository structure exists: `apps/engine`, `apps/server`, `apps/web`, `crates/*`, `infra/*`
- Cargo.toml files present in all crates
- Docker Compose environment file exists
- No cloud service dependencies (no LLMs, no external APIs)
- PostgreSQL and Keycloak configured for local operation only
- All 10 blocking violations from AMP_NO_PLACEHOLDER_AUDIT_PHASE_0_1_2.md have been remediated

---

## 3. ARCHITECTURE LOCKED DECISIONS

### 3.1 Authority Boundary (Immutable)

**apps/engine (Rust):**
- Single-writer authority for world state mutations
- Fixed-timestep deterministic loop: `while accumulator >= dt { tick(); accumulator -= dt; }`
- Tick index (u64) is authoritative time, never wall-clock
- Produces:
  - Per-tick `world_hash` (blake3 canonical state)
  - `InputEventLog` entries (audit-traced mutations)
  - `ObservationEvents` (state diffs, deterministically serialized)

**apps/server (TypeScript/Rust hybrid):**
- JWT verification against local Keycloak JWKS
- RBAC enforcement (Observer, Auditor, Admin roles)
- InputEvent validation, normalization, ordered insertion into append-only log
- WebSocket fanout of ObservationEvents to connected clients
- CANNOT mutate world state
- CANNOT compute authoritative outcomes

**apps/web (React + TypeScript):**
- Read-only visualization
- Submits InputEvents via server only
- WebSocket observer of ObservationEvents
- Admin actions emit InputEvents (never direct mutations)

### 3.2 Time Model (Immutable)

```rust
pub struct SimTime {
    tick: u64,  // Authoritative time
    dt: f64,    // Fixed timestep (milliseconds), e.g., 50ms
}

// NO system time participates in state evolution
```

### 3.3 Event Pipeline (Immutable)

```
InputEvent (Web) 
  ↓ [JWT verify + RBAC check + schema validation]
  → Server (append to input_events table with prev_hash, compute hash)
    ↓ [ordered transmission to engine]
    → Engine (authority pipeline: 10 passes)
      ↓ [state mutation]
      → world_hash checkpoint
        ↓ [append to hash_checkpoints table]
        → ObservationEvent (state diff)
          ↓ [WebSocket fanout]
          → Web (display + log)
```

---

## 4. DETERMINISM GUARANTEES (BINDING)

### 4.1 Fixed Timestep
- Tick index (u64) is simulation time
- dt is fixed (configured in genesis, e.g., 50ms = 20 ticks/sec)
- No variable frame rates
- No accumulator drift (reset to 0 after tick)

### 4.2 Canonical Ordering
- Entity iteration: deterministic (BTreeMap, sorted by ID)
- Event ordering: lexicographical by (Tick, SourceId, Sequence)
- Container traversal: never unordered (no HashSet in authority state)

### 4.3 RNG Isolation (Critical for Phase 0)
- All randomness is deferred to Phase 1
- Phase 0 uses NO randomness in authority decisions
- Genesis state is deterministic (no random initialization)

### 4.4 Hash Checkpoints
- Every N ticks (config-defined, e.g., 100 ticks), emit world_hash
- Hash = blake3(prev_hash || canonical_serialize(world_state))
- Stored immutably in hash_checkpoints table
- NO hash dependency on wall-clock or external state

---

## 5. CORE MODULES & SPECIFICATIONS

### 5.1 Core Types (crates/world)

**File: crates/world/src/lib.rs**
- Purpose: Root module, re-exports all world types
- Exports: `Agent`, `Asset`, `Terrain`, `Universe`, `StateTransition`, `ObservationEvent`, `InputEvent`
- No mocks, no stubs

**File: crates/world/src/agent.rs**
```rust
pub struct Agent {
    pub id: u64,                    // Unique agent ID
    pub name: String,               // "Gem-D" or "Gem-K"
    pub position: (f32, f32, f32),  // x, y, z coordinates
    pub state_hash: [u8; 32],       // blake3 hash of agent state
    pub inventory: Vec<Asset>,      // Items carried
    pub bio_state: BioState,        // Preserved from genesis
    pub cog_state: CogState,        // Preserved from genesis
}

impl Agent {
    pub fn new(id: u64, name: String, pos: (f32, f32, f32)) -> Self { ... }
    pub fn validate(&self) -> Result<(), String> { ... }
}

pub struct BioState {
    pub energy: f64,
    pub metabolism_rate: f64,
    // All fields preserved from Gemini genesis
}

pub struct CogState {
    pub memory: Vec<MemoryTrace>,
    pub current_goal: Option<Goal>,
    // All fields preserved from Gemini genesis
}
```
- Behavior: Agents are immutable between ticks; mutations only via StateTransition
- No TODO/FIXME
- Validation is complete and enforced

**File: crates/world/src/asset.rs**
```rust
pub struct Asset {
    pub id: u64,
    pub name: String,
    pub location: AssetLocation,  // (AgentId | Position)
    pub state: AssetState,         // Location, durability, ownership
}

impl Asset {
    pub fn validate(&self, world: &Universe) -> Result<(), String> { ... }
}
```
- Behavior: Assets are immutably stored; mutations via StateTransition only
- Assets include: House, Shed, Tools, Vehicles (all from Gemini)

**File: crates/world/src/terrain.rs**
```rust
pub struct Terrain {
    pub chunks: BTreeMap<(i32, i32), Chunk>,
}

pub struct Chunk {
    pub position: (i32, i32),
    pub voxels: Vec<Voxel>,  // Flat array, deterministic iteration
}

impl Terrain {
    pub fn height_at(&self, x: f32, y: f32) -> f32 { ... }
    pub fn biome_at(&self, x: f32, y: f32) -> BiomeType { ... }
}
```
- Behavior: Read-only terrain access; no mutation in Phase 0
- Terrain is static after genesis

**File: crates/world/src/universe.rs**
```rust
pub struct Universe {
    pub seed: u64,                              // Genesis seed
    pub tick: u64,                              // Current tick
    pub agents: BTreeMap<u64, Agent>,           // Agents by ID
    pub assets: BTreeMap<u64, Asset>,           // Assets by ID
    pub terrain: Terrain,                       // World terrain
    pub state_hash: [u8; 32],                   // Current world_hash
    pub prev_state_hash: [u8; 32],              // Previous hash
}

impl Universe {
    pub fn new(seed: u64) -> Self { ... }
    pub fn apply_transition(&mut self, transition: &StateTransition) -> Result<(), String> { ... }
    pub fn compute_hash(&self) -> [u8; 32] { ... }
    pub fn validate_all(&self) -> Result<(), String> { ... }
}
```
- Behavior: Mutable only via apply_transition (authority pipeline)
- Validation called before every state change and snapshot write

---

### 5.2 Events & Messaging (crates/events)

**File: crates/events/src/lib.rs**
- Purpose: Event schemas
- Exports: `InputEvent`, `ObservationEvent`, `StateTransition`, `EventPayload`

**File: crates/events/src/input_event.rs**
```rust
pub struct InputEvent {
    pub tick: u64,
    pub source_agent_id: u64,
    pub sequence: u64,
    pub payload: InputEventPayload,
    pub hash: [u8; 32],
    pub prev_hash: [u8; 32],
}

pub enum InputEventPayload {
    Move { x: f32, y: f32, z: f32 },
    Chat { text: String },
    Gather { resource_type: String },
    Craft { recipe_id: u64 },
    // Phase 0 actions only (no complex cognition)
}

impl InputEvent {
    pub fn validate(&self) -> Result<(), String> { ... }
}
```
- Behavior: Immutable; created by server, consumed by engine
- All fields required; no optionals or defaults

**File: crates/events/src/observation_event.rs**
```rust
pub struct ObservationEvent {
    pub tick: u64,
    pub event_type: String,  // "agent_moved", "state_changed", "action_resolved"
    pub payload: serde_json::Value,  // State diff JSON
    pub hash: [u8; 32],
}

impl ObservationEvent {
    pub fn from_transition(tick: u64, transition: &StateTransition) -> Self { ... }
}
```
- Behavior: Serialized to JSON and sent to Web via WebSocket
- Must contain observable state changes (no mock "before"/"after" strings)

---

### 5.3 Persistence (crates/persistence)

**File: crates/persistence/src/lib.rs**
- Purpose: Snapshot and replay logic
- Exports: `SnapshotManager`, `ReplayManager`

**File: crates/persistence/src/snapshot.rs**
```rust
pub struct Snapshot {
    pub tick: u64,
    pub universe_state: Vec<u8>,  // bincode-serialized Universe
    pub hash: [u8; 32],
    pub input_event_hash: [u8; 32],  // Hash of last input event
}

pub struct SnapshotManager {
    db: Database,
}

impl SnapshotManager {
    pub fn write_snapshot(&mut self, tick: u64, universe: &Universe) -> Result<(), String> { ... }
    pub fn load_snapshot(&self, tick: u64) -> Result<Snapshot, String> { ... }
    pub fn validate_all_snapshots(&self) -> Result<(), String> { ... }
}
```
- Behavior: Snapshots written every N ticks (e.g., 1000 ticks)
- Immutable once written; append-only
- No snapshots may be deleted or modified

**File: crates/persistence/src/replay.rs**
```rust
pub struct ReplayManager {
    db: Database,
}

impl ReplayManager {
    pub fn replay_from_snapshot(
        &self, 
        snapshot_tick: u64, 
        input_events: Vec<InputEvent>
    ) -> Result<(Universe, Vec<[u8; 32]>), String> { ... }
    
    pub fn compare_hashes(
        live_hashes: Vec<[u8; 32]>,
        replay_hashes: Vec<[u8; 32]>
    ) -> Result<(), String> { ... }
}
```
- Behavior: Replays from snapshot + event log; produces hash sequence
- Must match live run hash sequence exactly
- Any divergence is a hard failure (STOP)

---

### 5.4 Authority Pipeline (apps/engine/src/authority_pipeline.rs)

**10-Pass Authority Pipeline (Non-Negotiable Order):**

1. **InputEvent Validation Pass**: Verify schema, agent existence, action legality
2. **RBAC Pass**: Verify JWT/token has permission for action
3. **Biology Veto Pass**: Check agent energy/health for action feasibility
4. **Physics Collision Pass**: Verify no collision with terrain/entities
5. **Policy Pass**: Check governance rules (no violations)
6. **Action Resolution Pass**: Execute action (move, chat, gather, craft)
7. **State Transition Pass**: Commit state changes to Universe
8. **Hash Update Pass**: Compute new world_hash
9. **Observation Emission Pass**: Generate ObservationEvent from state diff
10. **Persistence Pass**: Append event to log, write hash checkpoint if needed

**No passes may be skipped, reordered, or partially executed.**

```rust
pub fn process_tick(
    universe: &mut Universe,
    input_events: Vec<InputEvent>,
    db: &mut Database,
) -> Result<Vec<ObservationEvent>, String> {
    
    let mut observations = Vec::new();
    
    for event in input_events {
        // Pass 1: InputEvent Validation
        event.validate()?;
        
        // Pass 2: RBAC
        verify_rbac(&event)?;
        
        // Pass 3: Biology Veto
        check_bio_feasibility(universe, &event)?;
        
        // Pass 4: Physics Collision
        check_collision(universe, &event)?;
        
        // Pass 5: Policy
        check_governance(universe, &event)?;
        
        // Pass 6: Action Resolution
        let state_before = serialize_agent(universe, event.source_agent_id)?;
        execute_action(universe, &event)?;
        let state_after = serialize_agent(universe, event.source_agent_id)?;
        
        // Pass 7: State Transition
        universe.apply_transition(&StateTransition {
            event: event.clone(),
            before_state: state_before,
            after_state: state_after,
        })?;
        
        // Pass 8: Hash Update
        let new_hash = universe.compute_hash();
        universe.state_hash = new_hash;
        
        // Pass 9: Observation Emission
        let obs = ObservationEvent::from_transition(universe.tick, &StateTransition { ... });
        observations.push(obs);
        
        // Pass 10: Persistence
        db.append_input_event(&event)?;
        if universe.tick % 100 == 0 {
            db.write_hash_checkpoint(universe.tick, new_hash)?;
        }
    }
    
    Ok(observations)
}
```

- **No TODOs, no FIXMEs, no stub implementations**
- **Every pass must complete or reject the entire tick**
- **No partial executions; no rollback logic**

---

### 5.5 Tick Loop (apps/engine/src/tick_loop.rs)

```rust
pub fn run_fixed_timestep_loop(
    mut universe: Universe,
    db: Database,
    dt: f64,  // milliseconds per tick
) -> Result<(), String> {
    
    let mut accumulator: f64 = 0.0;
    let start_time = std::time::Instant::now();
    
    loop {
        // Fetch real wall time for accumulator only (never enters state)
        let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;
        accumulator = elapsed_ms % (dt * 1000.0);  // Prevent drift
        
        while accumulator >= dt {
            // Read InputEvents from DB for this tick
            let events = db.fetch_input_events_for_tick(universe.tick)?;
            
            // Authority pipeline
            let observations = process_tick(&mut universe, events, &mut db)?;
            
            // Advance tick
            universe.tick += 1;
            accumulator -= dt;
            
            // Broadcast observations
            broadcast_to_ws(&observations)?;
        }
        
        // Small sleep to avoid busy-waiting
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
```

- **No randomness in Phase 0**
- **Tick index is only source of time**
- **All elapsed time tracking is read-only (never mutates state)**

---

## 6. DATABASE SCHEMA (IMMUTABLE)

**PostgreSQL (NOT SQLite for distributed replay)**

### Table: input_events
```sql
CREATE TABLE input_events (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    sequence BIGINT NOT NULL,
    source_agent_id BIGINT NOT NULL,
    payload BYTEA NOT NULL,  -- bincode-serialized InputEventPayload
    hash BYTEA NOT NULL UNIQUE,  -- blake3([u8; 32])
    prev_hash BYTEA NOT NULL,  -- blake3([u8; 32])
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT hash_chain CHECK (
        (id = 1 AND prev_hash = x'0000000000000000...') OR
        (id > 1 AND prev_hash IN (SELECT hash FROM input_events))
    )
);
```

### Table: observation_events
```sql
CREATE TABLE observation_events (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    input_event_id BIGINT NOT NULL REFERENCES input_events(id),
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,  -- State diff JSON
    hash BYTEA NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Table: snapshots
```sql
CREATE TABLE snapshots (
    tick BIGINT PRIMARY KEY,
    universe_state BYTEA NOT NULL,  -- bincode serialized
    hash BYTEA NOT NULL UNIQUE,
    input_event_hash BYTEA NOT NULL,  -- Hash of last input event
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Table: hash_checkpoints
```sql
CREATE TABLE hash_checkpoints (
    tick BIGINT PRIMARY KEY,
    world_hash BYTEA NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**Immutability Enforcement:**
- No UPDATE, DELETE, TRUNCATE on any table
- All writes are append-only
- Foreign keys enforce hash-chain integrity
- Database file encryption (PostgreSQL with encryption at rest)

---

## 7. KEYCLOAK + OIDC SETUP

### 7.1 Realm Configuration
- **Realm Name:** markenz-local
- **Roles:** admin, observer, auditor
- **Client ID:** markenz-web
- **Client Secret:** (gitignored, loaded from env)
- **JWKS Endpoint:** http://localhost:8080/auth/realms/markenz-local/protocol/openid-connect/certs
- **Token Endpoint:** http://localhost:8080/auth/realms/markenz-local/protocol/openid-connect/token

### 7.2 Server Auth Middleware (apps/server/src/auth/jwt_middleware.rs)

```rust
pub struct JwtMiddleware {
    jwks: JwksCache,
    required_role: MarkenzRole,
}

impl JwtMiddleware {
    pub fn verify_token(&self, auth_header: &str) -> Result<UserClaims, String> {
        let token = extract_bearer_token(auth_header)?;
        let claims = self.jwks.verify_and_decode(token)?;  // No "decode without verify"
        
        if claims.realm_access.roles.contains(&self.required_role) {
            Ok(claims)
        } else {
            Err("Insufficient role".to_string())
        }
    }
}
```

- **No mock user data**
- **No hardcoded tokens**
- **JWT verification is mandatory; cannot be bypassed**

### 7.3 Web Login (apps/web/src/auth/oidc_provider.tsx)

```typescript
export const OIDCProvider: React.FC = ({ children }) => {
    const config = {
        authority: process.env.REACT_APP_OIDC_ISSUER,
        client_id: process.env.REACT_APP_OIDC_CLIENT_ID,
        redirect_uri: `${window.location.origin}/callback`,
        response_type: "code",
        scope: "openid profile roles",
    };
    
    return (
        <AuthProvider config={config}>
            {children}
        </AuthProvider>
    );
};
```

- **All OIDC credentials in environment variables**
- **No secrets in code**
- **Offline operation: local Keycloak with cached JWKS**

---

## 8. WEB SOCKET PROTOCOL

### 8.1 Event Stream (JSON)

**Server → Web:**
```json
{
    "type": "tick_update",
    "tick": 42,
    "world_hash": "abc123...",
    "observations": [
        {
            "type": "agent_moved",
            "agent_id": 1,
            "from": [0, 0, 0],
            "to": [1, 0, 0]
        }
    ]
}
```

### 8.2 Input Command (JSON)

**Web → Server:**
```json
{
    "type": "input_event",
    "action": "move",
    "target_x": 5.0,
    "target_y": 0.0,
    "target_z": 0.0
}
```

Server adds auth context, serializes to InputEvent, appends to DB.

---

## 9. SUCCESS CRITERIA (ALL REQUIRED TO PASS)

### Build & Compilation
- [ ] `cargo build --release` succeeds, zero warnings
- [ ] `cargo test --all` passes (all unit tests)
- [ ] `cargo clippy` reports no warnings in critical paths
- [ ] Docker Compose builds all images without errors

### Determinism
- [ ] **TEST-DET-001:** Run 100 ticks with fixed seed 3 times; all produce identical world_hash sequence
- [ ] **TEST-SNAPSHOT-EQ-001:** Load snapshot at tick 50, replay remaining ticks; match live run hash exactly
- [ ] **TEST-HASH-CHAIN-001:** Verify hash_checkpoints table; no breaks in chain

### Authority & Isolation
- [ ] **TEST-AUTHORITY-001:** Static code analysis confirms server CANNOT import world mutation functions
- [ ] **TEST-STUB-001:** No TODO, FIXME, mock, stub, panic!, unimplemented! in critical paths (authority_pipeline.rs, tick_loop.rs, universe.rs)
- [ ] **TEST-UNI-001:** Universe is the only mutable global; controlled via engine

### Identity & Assets
- [ ] **TEST-IDENTITY-001:** Gem-D and Gem-K boot with preserved state from genesis
- [ ] **TEST-PRESERVATION-001:** No state loss during first 100 ticks
- [ ] **TEST-ASSETS-001:** House, Shed, Tools present and valid after genesis

### Database & Schema
- [ ] **TEST-SCHEMA-001:** All 4 tables created (input_events, observation_events, snapshots, hash_checkpoints)
- [ ] **TEST-HASH-CHAIN-CONSTRAINT-001:** Append-only constraint enforced; DELETE/UPDATE rejected on immutable tables
- [ ] **TEST-EVENT-ENCODING-001:** InputEvents serialize/deserialize correctly (round-trip validation)

### Auth & RBAC
- [ ] **TEST-RBAC-001:** Observer role denied InputEvent submission (token rejected by server)
- [ ] **TEST-RBAC-002:** Admin role allowed InputEvent submission (token accepted)
- [ ] **TEST-JWT-001:** Invalid JWT tokens rejected by middleware

### Observability
- [ ] **TEST-OBSERVATION-001:** Every state change produces an ObservationEvent (no silent mutations)
- [ ] **TEST-TIMELINE-001:** Event timeline visible in Web UI with correct tick/hash

### Infrastructure
- [ ] **TEST-OFFLINE-001:** `docker compose up` works with zero internet access (tcpdump confirms)
- [ ] **TEST-ENGINE-BOOT-001:** Engine boots cleanly, emits genesis snapshot
- [ ] **TEST-WS-001:** WebSocket fanout working; Web receives tick updates

---

## 10. FORBIDDEN ACTIONS (HARD FAILS)

Windsurf MUST NOT:

1. Use wall-clock time in authority state evolution
2. Implement TODO/FIXME/mock/stub code in critical paths
3. Reorder, skip, or partially execute authority pipeline passes
4. Add randomness to Phase 0 (randomness deferred to Phase 1)
5. Mutate Universe outside authority pipeline
6. Bypass RBAC checks in server
7. Hard-code user data or tokens
8. Use UPDATE/DELETE on immutable tables
9. Implement snapshot mocking (all snapshots must be real)
10. Add external dependencies without explicit approval

---

## 11. HARD STOP CONDITIONS

Execution STOPS immediately if:

1. **Determinism test fails** (hash mismatch on rerun)
2. **Authority violation detected** (server/web mutates state)
3. **Build fails** in any environment
4. **Asset loss detected** (Gem-D, Gem-K, House, or Shed incomplete)
5. **Hash-chain broken** (prev_hash mismatch detected)
6. **RBAC bypass discovered** (unauthorized role submits events)
7. **Database corruption** (immutability constraint violated)
8. **Snapshot divergence** (replay ≠ live run)
9. **Critical panic** in first 1000 ticks
10. **AMP auditor directs halt** (explicit written directive)

Upon STOP:
- Do not commit code
- Do not proceed to Phase 1
- Escalate with full diagnostic evidence
- Wait for AMP re-planning

---

## 12. PHASE 0 EXIT CHECKLIST

Phase 0 is DONE only when ALL of the following are TRUE:

**Build:**
- [ ] All crates compile (release + test)
- [ ] All unit tests pass
- [ ] No clippy warnings
- [ ] Docker Compose successful

**Determinism:**
- [ ] TEST-DET-001 passing (100 ticks, 3 runs, identical hashes)
- [ ] TEST-SNAPSHOT-EQ-001 passing (replay ≡ live)
- [ ] TEST-HASH-CHAIN-001 passing (no breaks)

**Authority:**
- [ ] TEST-AUTHORITY-001 passing (server isolation confirmed)
- [ ] TEST-STUB-001 passing (no placeholders)
- [ ] TEST-UNI-001 passing (Universe is only mutable)

**Identity:**
- [ ] TEST-IDENTITY-001 passing (Gem-D, Gem-K present)
- [ ] TEST-PRESERVATION-001 passing (no state loss)
- [ ] TEST-ASSETS-001 passing (all assets valid)

**Schema:**
- [ ] TEST-SCHEMA-001 passing (tables created)
- [ ] TEST-HASH-CHAIN-CONSTRAINT-001 passing (immutability enforced)
- [ ] TEST-EVENT-ENCODING-001 passing (serialization works)

**Auth:**
- [ ] TEST-RBAC-001 passing (observer denied)
- [ ] TEST-RBAC-002 passing (admin allowed)
- [ ] TEST-JWT-001 passing (invalid tokens rejected)

**Observability:**
- [ ] TEST-OBSERVATION-001 passing (observable state)
- [ ] TEST-TIMELINE-001 passing (timeline works)

**Infrastructure:**
- [ ] TEST-OFFLINE-001 passing (no internet needed)
- [ ] TEST-ENGINE-BOOT-001 passing (clean boot)
- [ ] TEST-WS-001 passing (WebSocket working)

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval in writing

**NO EXCEPTIONS. Failure of ANY criterion = PHASE 0 NO-GO. Do not proceed to Phase 1.**

---

## END OF PLAN

**Authority:** KAIZA-MCP · AMP Principal Planner  
**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_0_BOOTSTRAP  
**Timestamp:** 2026-01-11
