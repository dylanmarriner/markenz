---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_1_DETERMINISM
phase: 1
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Deterministic RNG · Replay Harness · Audit-Logged Randomness
requires: PLAN_PHASE_0_BOOTSTRAP (100% complete)
---

# PLAN PHASE 1: DETERMINISM
## (Seeded RNG · Audit-Logged Randomness · Replay Harness · Hash Equivalence)

**AUDIENCE:** Windsurf executor (direct execution only)  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Build deterministic randomness into the engine while maintaining provable reproducibility:
- All randomness uses ChaCha20 (RFC 7539) seeded from genesis
- Every RNG draw is audit-logged with (tick, subsystem, stream, callsite, value)
- RNG streams are isolated per subsystem (Physics, Biology, Cognition, Genetics, Governance, Environment)
- Replay from event log + snapshot produces bit-identical hash sequence
- Snapshot replay at any tick produces identical outcome to full replay
- Platform independence verified (Linux x64/arm64, macOS produce same hashes)

---

## 2. ENTRY CONDITIONS (MUST BE TRUE)

- Phase 0 complete and signed off by AMP auditor
- All Phase 0 tests passing
- No determinism regressions from Phase 0
- All blockers from audit fixed (RNG state preservation, hash updates)

---

## 3. DETERMINISTIC RNG ARCHITECTURE

### 3.1 RNG Types & Hierarchy

**File: crates/rng/src/lib.rs**

```rust
pub mod chacha20;
pub mod rng_stream;
pub mod global_seed;

pub use chacha20::ChaCha20Rng;
pub use rng_stream::RngStream;
pub use global_seed::GlobalSeed;
```

### 3.2 ChaCha20 Implementation (crates/rng/src/chacha20.rs)

**Exact algorithm: RFC 7539**

```rust
/// RFC 7539 ChaCha20 cipher stream generator
pub struct ChaCha20Rng {
    key: [u32; 8],          // 256-bit key
    nonce: [u32; 3],        // 96-bit nonce
    counter: u64,           // 64-bit block counter
    block_index: usize,     // Position within current block (0-63)
}

impl ChaCha20Rng {
    /// Initialize from 32-byte seed (derived from genesis seed)
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        let key = array_chunks::<[u8; 4], 32>(&key)
            .map(|bytes| u32::from_le_bytes(*bytes))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        
        let nonce = array_chunks::<[u8; 4], 12>(&nonce)
            .map(|bytes| u32::from_le_bytes(*bytes))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        
        Self {
            key,
            nonce,
            counter: 0,
            block_index: 64,  // Force generation of first block
        }
    }
    
    /// Generate next 32-bit random value
    pub fn next_u32(&mut self) -> u32 {
        if self.block_index >= 64 {
            self.generate_block();
            self.block_index = 0;
        }
        
        let value = self.current_block[self.block_index / 4];
        self.block_index += 4;
        value
    }
    
    /// Generate next 64-bit random value
    pub fn next_u64(&mut self) -> u64 {
        let lo = self.next_u32() as u64;
        let hi = self.next_u32() as u64;
        (hi << 32) | lo
    }
    
    /// Generate next f64 in [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
    
    fn generate_block(&mut self) {
        // RFC 7539 block function (exact algorithm, not approximated)
        // No deviations allowed
        ...
    }
}
```

- **No deviations from RFC 7539**
- **All 64 bytes per block used sequentially**
- **Counter incremented deterministically**
- **Nonce derived from subsystem + stream**

### 3.3 RNG Stream Management (crates/rng/src/rng_stream.rs)

```rust
/// Subsystem identifier for RNG isolation
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RngSubsystem {
    Physics = 0,
    Biology = 1,
    Cognition = 2,
    Genetics = 3,
    Governance = 4,
    Environment = 5,
}

/// Isolated RNG stream per subsystem
pub struct RngStream {
    subsystem: RngSubsystem,
    stream_id: u64,        // Stream identifier within subsystem
    rng: ChaCha20Rng,
}

impl RngStream {
    /// Create new stream for subsystem
    /// Nonce = blake3(global_seed || subsystem_id || stream_id) first 12 bytes
    pub fn new(
        global_seed: [u8; 32],
        subsystem: RngSubsystem,
        stream_id: u64,
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&global_seed);
        hasher.update(&(subsystem as u64).to_le_bytes());
        hasher.update(&stream_id.to_le_bytes());
        let nonce_bytes = hasher.finalize();
        
        let nonce = [
            nonce_bytes.as_bytes()[0..4].try_into().unwrap(),
            nonce_bytes.as_bytes()[4..8].try_into().unwrap(),
            nonce_bytes.as_bytes()[8..12].try_into().unwrap(),
        ];
        
        Self {
            subsystem,
            stream_id,
            rng: ChaCha20Rng::new(global_seed, nonce),
        }
    }
    
    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    
    pub fn next_f64(&mut self) -> f64 {
        self.rng.next_f64()
    }
}
```

### 3.4 Global Seed Management (crates/rng/src/global_seed.rs)

```rust
pub struct GlobalSeed {
    seed: [u8; 32],
    streams: BTreeMap<(RngSubsystem, u64), RngStream>,
}

impl GlobalSeed {
    pub fn from_genesis(seed: u64) -> Self {
        // Derive 32-byte key from genesis seed
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed.to_le_bytes());
        let derived_seed = *hasher.finalize().as_bytes();
        
        Self {
            seed: derived_seed,
            streams: BTreeMap::new(),
        }
    }
    
    /// Get or create stream for subsystem
    /// Stream ID = 0 for primary stream, incrementing for secondary
    pub fn stream(
        &mut self,
        subsystem: RngSubsystem,
        stream_id: u64,
    ) -> &mut RngStream {
        let key = (subsystem, stream_id);
        if !self.streams.contains_key(&key) {
            self.streams.insert(
                key,
                RngStream::new(self.seed, subsystem, stream_id),
            );
        }
        &mut self.streams[&key]
    }
}
```

---

## 4. RNG INTEGRATION WITH UNIVERSE

### 4.1 Universe RNG Ownership (crates/world/src/universe.rs)

```rust
pub struct Universe {
    // ... Phase 0 fields ...
    pub global_seed: GlobalSeed,  // NEW: centralized RNG ownership
}

impl Universe {
    pub fn rng_stream(
        &mut self,
        subsystem: RngSubsystem,
    ) -> &mut RngStream {
        self.global_seed.stream(subsystem, 0)
    }
}
```

**Critical:** Universe owns GlobalSeed exclusively; no temporary RNG creation.

### 4.2 RNG Usage in Authority Pipeline (apps/engine/src/authority_pipeline.rs)

```rust
pub fn process_tick(
    universe: &mut Universe,
    input_events: Vec<InputEvent>,
    db: &mut Database,
) -> Result<Vec<ObservationEvent>, String> {
    
    let mut observations = Vec::new();
    
    for event in input_events {
        // ... Passes 1-5 (unchanged from Phase 0) ...
        
        // Pass 6: Action Resolution
        match &event.payload {
            InputEventPayload::Physics { ... } => {
                let rng = universe.rng_stream(RngSubsystem::Physics);
                // Use rng.next_f64() for physics calculations
                // All draws are deterministic and audit-logged
            },
            InputEventPayload::GatherAttempt { ... } => {
                let rng = universe.rng_stream(RngSubsystem::Environment);
                // Use rng for resource availability checks
            },
            // ...
        }
        
        // ... Passes 7-10 (unchanged from Phase 0) ...
    }
    
    Ok(observations)
}
```

---

## 5. RNG AUDIT LOG

### 5.1 Audit Log Entry (crates/rng/src/audit_log.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RngDrawRecord {
    pub tick: u64,
    pub subsystem: RngSubsystem,
    pub stream_id: u64,
    pub callsite: String,       // Source file:line
    pub value: u64,             // The actual random value drawn
    pub timestamp: u64,         // Tick-relative timestamp
}

pub struct RngAuditLog {
    db: Database,
}

impl RngAuditLog {
    pub fn record_draw(
        &mut self,
        tick: u64,
        subsystem: RngSubsystem,
        stream_id: u64,
        callsite: &str,
        value: u64,
    ) -> Result<(), String> {
        let record = RngDrawRecord {
            tick,
            subsystem,
            stream_id,
            callsite: callsite.to_string(),
            value,
            timestamp: tick,
        };
        
        self.db.insert_rng_draw(&record)?;
        Ok(())
    }
}
```

### 5.2 Database Table for RNG Audit

```sql
CREATE TABLE rng_audit_log (
    id BIGSERIAL PRIMARY KEY,
    tick BIGINT NOT NULL,
    subsystem TEXT NOT NULL,       -- "Physics", "Biology", etc.
    stream_id BIGINT NOT NULL,
    callsite TEXT NOT NULL,        -- "physics.rs:42"
    value BIGINT NOT NULL,         -- The actual u64 value
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rng_tick ON rng_audit_log(tick);
CREATE INDEX idx_rng_subsystem ON rng_audit_log(subsystem);
```

---

## 6. REPLAY HARNESS

### 6.1 Replay Test Suite (tests/determinism.rs)

**TEST-DET-001: Fixed Seed Reproducibility**

```rust
#[test]
fn test_determinism_fixed_seed() -> Result<(), String> {
    let seed = 12345u64;
    let num_ticks = 100;
    
    // Run 1
    let (_, hashes1) = simulate(seed, num_ticks)?;
    
    // Run 2
    let (_, hashes2) = simulate(seed, num_ticks)?;
    
    // Run 3
    let (_, hashes3) = simulate(seed, num_ticks)?;
    
    // All three runs must produce identical hashes
    assert_eq!(hashes1, hashes2, "Run 1 and 2 diverged");
    assert_eq!(hashes2, hashes3, "Run 2 and 3 diverged");
    
    Ok(())
}

fn simulate(seed: u64, num_ticks: u64) -> Result<(Universe, Vec<[u8; 32]>), String> {
    let mut universe = Universe::from_genesis(seed)?;
    let mut hashes = Vec::new();
    
    for tick in 0..num_ticks {
        // Simulate one tick with no external input (deterministic)
        let events = vec![];  // No input = fully deterministic
        process_tick(&mut universe, events, &mut db)?;
        
        hashes.push(universe.state_hash);
    }
    
    Ok((universe, hashes))
}
```

**TEST-SNAPSHOT-EQ-001: Snapshot Replay Equivalence**

```rust
#[test]
fn test_snapshot_replay_equivalence() -> Result<(), String> {
    let seed = 12345u64;
    
    // Full run: ticks 0-1000
    let (universe_full, hashes_full) = full_run(seed, 1000)?;
    
    // Snapshot run: save at tick 500, replay to 1000
    let (universe_snap, hashes_snap) = snapshot_run(seed, 500, 1000)?;
    
    // Hashes from tick 500-1000 must match exactly
    let full_tail = &hashes_full[500..1000];
    let snap_tail = &hashes_snap[0..500];  // Relative to snapshot
    
    assert_eq!(full_tail, snap_tail, "Snapshot replay diverged from full run");
    assert_eq!(universe_full.state_hash, universe_snap.state_hash, "Final state differs");
    
    Ok(())
}
```

**TEST-HASH-CHAIN-001: Hash Chain Integrity**

```rust
#[test]
fn test_hash_chain_integrity() -> Result<(), String> {
    let db = Database::connect()?;
    let records = db.fetch_all_hash_checkpoints()?;
    
    let mut prev_hash = [0u8; 32];  // Genesis hash
    
    for record in records {
        assert_eq!(record.prev_hash, prev_hash, "Hash chain broken at tick {}", record.tick);
        prev_hash = record.hash;
    }
    
    Ok(())
}
```

**TEST-RNG-001: RNG Chaos Stability**

```rust
#[test]
fn test_rng_chaos_stability() -> Result<(), String> {
    // Generate 1000 random values with known seed
    let mut seed = GlobalSeed::from_genesis(42);
    let mut stream = seed.stream(RngSubsystem::Physics, 0);
    
    let mut values = Vec::new();
    for _ in 0..1000 {
        values.push(stream.next_u64());
    }
    
    // Compare to fixture (pre-computed on reference platform)
    let fixture = REFERENCE_1000_VALUES;  // Const array
    assert_eq!(values, fixture, "RNG values diverged from reference");
    
    Ok(())
}
```

**TEST-RNG-AUDIT-001: RNG Sequence Bit-Identical Across Platforms**

```rust
#[test]
fn test_rng_platform_independence() -> Result<(), String> {
    // Run on x64 Linux, arm64 Linux, macOS
    // All must produce identical RNG sequences
    
    let seed = 999u64;
    let mut rng = GlobalSeed::from_genesis(seed);
    let stream = rng.stream(RngSubsystem::Biology, 0);
    
    let mut values = Vec::new();
    for _ in 0..100 {
        values.push(stream.next_u64());
    }
    
    // Load reference from platform fixture
    let reference = load_fixture("rng_ref_platform_independent.bin")?;
    assert_eq!(values, reference, "Platform-dependent RNG behavior detected");
    
    Ok(())
}
```

---

## 7. DETERMINISM GUARANTEES (BINDING)

### 7.1 Identical Seed → Identical Sequence
Same seed + same input events = identical state progression and hash sequence across all runs.

### 7.2 Snapshot Equivalence
Loading a snapshot at tick N and replaying from that point = continuing full simulation from tick 0 to same final state and hash.

### 7.3 Platform Independence
Same seed and events on Linux x64, arm64, macOS must produce bit-identical hashes and RNG values.

### 7.4 No Wall-Clock Participation
No system time, elapsed time, or date/time functions participate in state evolution. Tick index only.

### 7.5 Fully Audit-Logged Randomness
Every RNG draw is recorded in rng_audit_log with tick, subsystem, callsite, and value.

---

## 8. SUCCESS CRITERIA (ALL REQUIRED)

### Build & Compilation
- [ ] `cargo build --release` succeeds, zero warnings in rng crates
- [ ] `cargo test --lib crates/rng` passes all unit tests
- [ ] No clippy warnings in crates/rng

### RNG Tests
- [ ] **TEST-DET-001** passing: 100 ticks, fixed seed, 3 runs identical
- [ ] **TEST-SNAPSHOT-EQ-001** passing: snapshot replay ≡ full run
- [ ] **TEST-HASH-CHAIN-001** passing: no hash chain breaks
- [ ] **TEST-RNG-001** passing: ChaCha20 produces fixture values
- [ ] **TEST-RNG-AUDIT-001** passing: platform-independent RNG

### Integration Tests
- [ ] **TEST-DETERMINISM-INTEGRATION-001**: End-to-end determinism with all subsystems
- [ ] **TEST-REPLAY-E2E-001**: Full replay pipeline (event log → replay) matches live

### Regression Testing
- [ ] All Phase 0 tests still passing (no determinism regressions)
- [ ] Phase 0 exit criteria still met

### RNG Audit Log
- [ ] [ ] Audit log table created and populated
- [ ] [ ] All RNG draws logged (sampled verification: 10 random ticks checked)
- [ ] [ ] Audit log immutable (no UPDATE/DELETE allowed)

---

## 9. FORBIDDEN ACTIONS (HARD FAILS)

Windsurf MUST NOT:

1. Deviate from RFC 7539 ChaCha20 algorithm
2. Use any RNG other than ChaCha20 in authority state (no std::rand, no JavaScript Math.random)
3. Skip RNG audit logging on any draw
4. Recreate RNG instance mid-tick (preserve state across all draws)
5. Use wall-clock time in RNG seeding
6. Add non-deterministic floating point operations
7. Use unordered collections (HashSet, HashMap) in authority state
8. Implement TODO/FIXME/stub in RNG code
9. Add external randomness sources
10. Parallelize RNG stream generation (must be sequential)

---

## 10. HARD STOP CONDITIONS

Execution STOPS immediately if:

1. **RNG diverges on replay** (TEST-DET-001 fails)
2. **Snapshot replay differs from live** (TEST-SNAPSHOT-EQ-001 fails)
3. **Hash chain broken** (TEST-HASH-CHAIN-001 fails)
4. **Platform-dependent RNG detected** (TEST-RNG-AUDIT-001 fails)
5. **Regression in Phase 0 tests** (any Phase 0 test fails)
6. **Audit log missing entries** (RNG draw not logged)
7. **Build fails** in any environment
8. **Critical performance drop** (tick rate >20% slower than Phase 0)

---

## 11. PHASE 1 EXIT CHECKLIST

Phase 1 is DONE only when ALL are TRUE:

**Build:**
- [ ] All rng crates compile
- [ ] All unit tests pass
- [ ] Zero clippy warnings
- [ ] No regression in Phase 0 build

**RNG Determinism:**
- [ ] TEST-DET-001 passing
- [ ] TEST-SNAPSHOT-EQ-001 passing
- [ ] TEST-RNG-001 passing
- [ ] TEST-RNG-AUDIT-001 passing

**Authority Integration:**
- [ ] TEST-HASH-CHAIN-001 passing
- [ ] RNG draws in every subsystem audit-logged
- [ ] Universe owns GlobalSeed (no temporary instances)

**Regression:**
- [ ] All Phase 0 tests still passing
- [ ] Phase 0 determinism maintained

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval in writing

---

## END OF PLAN

**Authority:** KAIZA-MCP · AMP Principal Planner  
**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_1_DETERMINISM  
**Timestamp:** 2026-01-11
