---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
timestamp: 2026-01-10
scope: Unified Gemini Universe → Markenz Integration
fail_mode: FAIL-CLOSED
plan_id: MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v1
supersedes: |
  - MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v1
  - MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2 (consolidated herein)
  - MARKENZ_REUSE_MIGRATION_PLAN_v1
  - MARKENZ_REUSE_MIGRATION_PLAN_v2
  - MARKENZ_REUSE_MIGRATION_PLAN_v3 (consolidated herein)
  - MARKENZ_M1_FOUNDATION (explicitly superseded per AMP_WINDSURF_EXECUTABILITY_AUDIT)
---

# MARKENZ UNIFIED INTEGRATION MASTER PLAN v1

**AUTHORITY:** KAIZA-MCP · AMP (ANTIGRAVITY)  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**SCOPE:** Single authoritative integration plan merging all prior plans into mechanically enforceable law  
**AUDIENCE:** Windsurf executor (direct execution authority)  

---

## STATUS & AUTHORITY DECLARATION

**STATUS:** BINDING · MASTER AUTHORITY

**SUPERSEDES:**
- MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v1.md
- MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2.md (all sections consolidated here)
- MARKENZ_REUSE_MIGRATION_PLAN_v3.md (all technical detail absorbed)
- MARKENZ_M1_FOUNDATION.md (explicitly deprecated per AMP_WINDSURF_EXECUTABILITY_AUDIT.md § 2)

**FAIL MODE:** FAIL-CLOSED (any blocker → STOP execution)

**ENFORCEMENT AUTHORITY:** AMP Principal-Level Auditor

---

## SOURCE DOCUMENT INCORPORATION MAP

| Source Document | Authority | Sections Incorporated | Status |
|-----------------|-----------|----------------------|--------|
| MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2.md | AUTHORITATIVE | Part 1 (Audit Ground Truth), Part 2 (Asset & Identity Preservation), Part 3 (Subsystem Mapping), Part 6 (Risk Mitigation), Part 7 (Windsurf Handoff), Part 8 (Go/No-Go Criteria) | ✅ ABSORBED |
| MARKENZ_REUSE_MIGRATION_PLAN_v3.md | SUPPORTING | Section 2 (Deterministic Interfaces), Section 3 (Acceptance Tests), Section 4 (Migration Phases), Section 5 (CI Rule Mapping), Section 6 (Reuse Classification), Section 7 (Windsurf Constraints) | ✅ ABSORBED |
| AMP_WINDSURF_EXECUTABILITY_AUDIT.md | AUDIT VERDICT | Plan Authority Resolution, Conflict Matrix, Executor Ambiguity Scan, Safety Verification, Execution Readiness, Mandatory Actions | ✅ INCORPORATED |
| MARKENZ_EXECUTION_ROADMAP_v2.md | GOVERNING LAW | Global Invariants, Phase 0–9 Objectives & Deliverables, Determinism Foundation | ✅ GOVERNING |
| MARKENZ_TARGET_ARCHITECTURE_v2.md | GOVERNING LAW | Locked Services & Roles (Engine/Server/Web), Authority Boundaries, Event Pipeline, Determinism Strategy, World Representation, Biology/Genetics/Cognition Boundaries | ✅ GOVERNING |
| MARKENZ_REPO_REFACTOR_MAP_v2.md | GOVERNING LAW | Locked Target Repo Layout, Ownership Map, Interface Contracts, Incremental Refactor Steps, Explicit Removals & Prohibitions | ✅ GOVERNING |
| AMP_DEFINITION_OF_DONE_v2.md | GOVERNING LAW | Global Invariants (Authority, Determinism, Transparency, Offline-First, No-Mock/No-Stub, Security), Phase Gate Checklist, Test Suite Requirements, No-Mock Enforcement, Performance Targets, Reproducibility Gates | ✅ GOVERNING |
| MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md | GOVERNING LAW | Evolution Prohibitions, Allowed Growth Mechanisms, Determinism Guarantees, AMP Gate Additions | ✅ GOVERNING |
| PASS_2_REPO_REALITY_AUDIT.md | AUDIT EVIDENCE | Current Repo State, Authority Violations, Blocking Items, Critical Gaps, Stop Conditions | ✅ EVIDENCE |

**CONFIRMATION:** All unique features, constraints, and guarantees from input documents are preserved herein. No features were dropped. All conflicts were resolved per AMP_WINDSURF_EXECUTABILITY_AUDIT.md § 3.

---

## UNIFIED SYSTEM GUARANTEES

### 1. Determinism Guarantee
- **Fixed timestep:** Simulation time is tick-indexed, never wall-clock dependent
- **Canonical ordering:** All events, entities, and containers have stable deterministic iteration order
- **Seeded RNG:** All randomness uses engine-side DeterministicRng streams (ChaCha20, RFC 7539)
- **Audit-logged draws:** Every RNG call logged with { tick, subsystem, stream, callsite, value }
- **Hash invariant:** `world_hash` computed canonically after each tick using blake3(state)
- **Replay equivalence:** Seed + ordered InputEvents → identical world_hash sequence across multiple runs
- **Snapshot equivalence:** Snapshot replay + remaining events = Full replay to same final state
- **Platform independence:** Identical hashes across Linux (x64/arm64) and macOS

### 2. Authority Boundary Guarantee
- **Rust sole authority:** `apps/engine` is the single-writer for all world state mutations
- **Server stateless:** `apps/server` (TypeScript) validates, authorizes, orders, and persists events ONLY
- **Web read-only:** `apps/web` (React) submits InputEvents via server, never mutates state directly
- **Authority boundary enforced:** Code structure prevents server/web from importing world mutation functions
- **No shadow state:** No hidden caches or parallel evolution outside engine

### 3. Identity Continuity Guarantee
- **Gem-D preservation:** Full consciousness state, biology, memories, skills, genetic markers, relationships imported to Markenz genesis
- **Gem-K preservation:** Identical preservation as Gem-D
- **Identity fingerprinting:** `blake3(agent_name || original_state_hash)` computed and stored immutably in genesis
- **Determinism gate:** First 100 ticks of Gem-D/Gem-K in Markenz must produce identical decisions to Gemini endpoint under same perception inputs
- **No loss:** Any divergence from original identity → hard failure (STOP)

### 4. Asset Continuity Guarantee
- **House (Homestead):** Location, structure, and ownership preserved; regenerated in engine using Markenz seed
- **Shed (Tool Storage):** Tool inventory and durability state preserved; initialized as genesis content
- **Tools:** All tool definitions ported to `crates/world`; stable IDs across replay
- **Vehicles:** Mechanics deterministically ported; ownership and state preserved; physics behavior matches Gemini under identical inputs
- **Validation:** `fn validate_all_assets(world: &World) -> Result<()>` called at genesis, before every snapshot write, after every snapshot load

### 5. Offline-First Guarantee
- **No external API calls:** Entire engine runs without network (no cloud APIs, no LLMs)
- **Keycloak local:** OIDC identity provider runs locally with cached JWKS; no network fallback
- **Database local:** PostgreSQL runs via docker compose; append-only event sourcing is local
- **Stack independent:** Full simulation executes on single workstation or closed LAN
- **Docker compose:** `docker compose up --build` with no internet access = operational system

### 6. Self-Evolution Guarantee (per MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md)
- **No code self-modification:** System cannot modify its own source code or engine logic
- **No runtime rule creation:** New behaviors emerge only from bounded parameter drift within fixed rule set
- **Evolution is state:** All adaptation recorded as state transitions, logged as events, hashed
- **Replay reproduces evolution:** Same seed + events = identical evolutionary trajectory
- **Governed evolution:** Governance mechanisms constrain and shape evolutionary paths deterministically
- **Observable evolution:** All trait drift, cultural changes, and learning updates visible in event logs

---

## UNIFIED AUTHORITY & ARCHITECTURE MODEL

### A. Locked Services & Authority Boundaries

**apps/engine (Rust — World Authority)**
- Single-writer, fixed-timestep deterministic loop
- Owns all world state and mutations
- Enforces physics, biology, cognition, genetics, economy, governance
- Accepts ordered InputEvents only
- Emits ObservationEvents and world_hash checkpoints
- No direct UI or server logic
- No wall-clock dependency for state evolution

**apps/server (TypeScript — Control Plane)**
- Identity, authentication, authorization (RBAC)
- InputEvent validation, normalization, ordering
- Append-only persistence adapter (event log + hash chain)
- WebSocket fanout of ObservationEvents
- Never mutates world state
- Never computes authoritative outcomes
- Never "fixes" engine results

**apps/web (React — Observer / Command Centre)**
- Read-only visualization and inspection
- Replay viewer and time-travel debugger
- WebGPU renderer (renderer-only, never authority)
- Admin actions emit InputEvents only
- Cannot bypass RBAC
- Cannot mutate state directly or indirectly

**infra/postgres (PostgreSQL)**
- Append-only event log (immutable tables)
- Tables: input_events, observation_events, snapshots, hash_checkpoints
- No UPDATE / DELETE on immutable tables
- Hash-chain enforced via foreign keys
- Schema frozen per MARKENZ_EXECUTION_ROADMAPv2.md § Phase 0

**infra/keycloak (Keycloak)**
- Primary OIDC IdP
- WebAuthn/passkeys support
- Local realm + cached JWKS for offline operation
- Roles: observer, auditor, admin

### B. Event Pipeline (End-to-End)

```
1. Web → Server (InputEvent request + token)
2. Server (verify JWT, enforce RBAC, validate schema, normalize, append to log with hash-chain)
3. Engine (read InputEvents, apply authority pipeline, emit ObservationEvents + world_hash)
4. Server → Web (fanout ObservationEvents via WebSocket)
5. tools/audits (offline replay, hash verification, anomaly detection)
```

### C. Determinism Strategy

**Time:**
- Simulation time = tick index (u64)
- Fixed dt defined in genesis config (e.g., 50ms for 20Hz)
- Wall clock never enters state evolution

**RNG:**
- Root seed → deterministic subsystem streams
- Streams: Physics, Environment, Biology, Cognition, Genetics, Governance
- Algorithm: ChaCha20 (RFC 7539)
- Every draw audit-logged: { tick, subsystem, stream, callsite, value }

**Ordering:**
- Deterministic iteration for entities, events, containers
- No unordered maps/sets in authority state
- Lexicographical event ordering by (Tick, SourceId, Sequence)

**Hashing:**
- Canonical serialization of world state
- blake3(PrevHash || CanonicalSerialize(State)) per tick
- Checkpoints every N ticks (config-defined, e.g., 1000)

### D. Deterministic Interfaces (Contracts)

**TimeSource Contract:**
- Type: `type SimTime = u64` (monotonic tick starting at 0)
- `dt()` returns constant Duration
- `now()` calls within same tick return identical SimTime
- Serialization: 64-bit unsigned integer
- Forbidden: SystemTime, chrono::Utc::now, Date.now()

**ChaosStream / RNG Contract:**
- Algorithm: ChaCha20 (RFC 7539)
- Stream Identity: SubsystemId-based
- Global Seed: `blake3(GlobalSeed || SubsystemId || ChunkCoord)`
- Fully serializable for resumption
- CI Verification: `next_float()` sequences bit-identical across platforms

**EventBus Contract:**
- Structure: `struct SimEvent { tick: u64, source_id: u32, sequence: u32, payload: Vec<u8> }`
- Queue: Single BTreeMap<(Tick, SourceId, Sequence), SimEvent>
- Dispatch: Events at Tick N applied at START of Tick N+1
- Atomicity: State mutation phase distinct from event processing
- Replay: Same event log = identical state transitions

**Persistence Contract:**
- Hash Type: `[u8; 32]` (blake3)
- Append: `fn append(&mut self, events: &[SimEvent]) -> Result<LogHash>`
- Chain invariant: `NewHash = blake3(PrevHash || CanonicalSerialize(Events))`
- Snapshot cadence: Every N ticks (config-defined)
- Verification: Walk and recompute hashes; any mismatch = Panic (fail-closed)

---

## UNIFIED ASSET & IDENTITY PRESERVATION STRATEGY

### 1. Gem-D and Gem-K Identity Continuity

**Export Phase:**
- Extract Gem-D and Gem-K full state snapshots from Gemini event log
- Serialize to canonical JSON:
  ```json
  {
    "agent_id": "gem-d",
    "original_name": "Gem-D",
    "consciousness_state": { ... },
    "biology_state": { ... },
    "memory_systems": { ... },
    "relationship_graph": { ... },
    "skill_trees": { ... },
    "genetic_markers": { ... },
    "event_history": [ ... ]
  }
  ```

**Schema Mapping:**
- Document field-by-field mapping from Gemini agent schema to Markenz `Agent` struct
- Create bridge document at `apps/engine/assets/gems/GEM_IDENTITY_BRIDGE.md`

**Genesis Integration:**
- Store snapshots in `apps/engine/assets/gems/gem_d_bridge.json` and `gem_k_bridge.json`
- Implement `persistence/bridge.rs:load_agent_bridge()` for engine startup hydration
- Hash fingerprint: `blake3(agent_name || original_state_hash)` stored immutably in genesis
- Both agents load at tick 0 with identical state to Gemini endpoint

**Identity Verification:**
- Determinism test (TEST-IDENTITY-001): Replay Markenz from genesis → Gem-D/Gem-K decisions at ticks 0–100 must match Gemini under identical perception inputs
- Failure = STOP (hard gate)

### 2. House, Shed, Tools, Vehicles Preservation

**House (Homestead):**
- Serialize Gemini house location and structure
- Map to Markenz chunked terrain spatial coordinates
- Regenerate using deterministic seed-based world generation
- Guarantee: Same seed in Markenz = same house location as Gemini

**Shed (Tool Storage):**
- Extract tool inventory array from Gemini shed
- Map each tool to Markenz `Tool` type definition
- Initialize as engine genesis state
- Preserve exact durability/wear state

**Tools:**
- Extract all tool definitions from Gemini tool registry
- Implement in `crates/world/tools.rs` (immutable registry)
- Assign stable IDs (must match across replay)
- Preserve durability as mutable state in agent inventory

**Vehicles:**
- Port vehicle mechanics to deterministic Rust (`crates/physics/vehicle.rs`)
- Extract ownership and state from Gemini agents
- Initialize in engine genesis
- Determinism test: Vehicle physics behavior matches Gemini under identical inputs

### 3. Risk Mitigation

| Risk | Mitigation | Gate |
|------|-----------|------|
| Data serialization/deserialization bugs → state corruption | Create dual-format validator (Gemini JSON ↔ Markenz binary), round-trip test 100 snapshots | TEST-BRIDGE-001 |
| Gem-D/Gem-K identity divergence under replay | Store original state hash in genesis, replay-verify continuously, report any divergence within first 10 ticks | TEST-IDENTITY-001 |
| Assets behave differently in Markenz due to RNG or logic changes | Freeze asset mechanics (no changes), only syntax port, determinism replay test with assets active | TEST-ASSETS-001 |
| Identity loss during integration | Full state snapshot validation, dual comparison (Gemini vs Markenz state at checkpoint) | TEST-PRESERVATION-001 |

---

## UNIFIED SUBSYSTEM MAPPING

### Gemini Subsystem → Markenz Crate Reuse Classification

#### REUSE AS-IS (Zero modification, direct port to Rust)
| System | Gemini Path | Markenz Crate | Action |
|--------|-------------|---------------|--------|
| Metabolism | core/biology/metabolism.ts | crates/biology | Port 1:1 logic to Rust |
| Hormones | core/biology/hormones.ts | crates/biology | Port 1:1 logic to Rust |
| Immune | core/biology/immune-system.ts | crates/biology | Port 1:1 logic to Rust |
| Vitals | core/biology/vitals.ts | crates/biology | Port 1:1 logic to Rust |
| Interoception | core/senses/interoception.ts | crates/biology | Port 1:1 logic to Rust |
| Proprioception | core/senses/proprioception.ts | crates/biology | Port 1:1 logic to Rust |
| Tactile | core/senses/tactile-system.ts | crates/biology | Port 1:1 logic to Rust |
| Emotions (Granular) | core/psychology/granular-emotions.ts | crates/cognition | Port 1:1 logic to Rust |
| Dark Triad | core/psychology/dark-triad.ts | crates/cognition | Port 1:1 logic to Rust |
| Homestead | world/homestead.ts | crates/world | Port 1:1 logic to Rust |
| Shed | world/shed.ts | crates/world | Port 1:1 logic to Rust |
| ChaosSys | chaos/ChaosSys.ts | crates/deterministic | Port 1:1 logic to Rust |
| TimeSourceRegistry | infrastructure/TimeSourceRegistry.ts | crates/deterministic | Port 1:1 logic to Rust |

#### REUSE WITH MODIFICATION (Logic preserved, dependency injection required)
| System | Gemini Path | Markenz Crate | Modifications | Priority |
|--------|-------------|---------------|---------------|----------|
| Somatic Body | core/somatic/SomaticBody.ts | crates/cognition | Remove global event bus; inject EventEmitter | Phase 0 |
| Free-Will Decision Loop | core/free-will-decision-loop.ts | crates/cognition | Inject TimeSource; verify ChaosSys seeding | Phase 0 |
| Event Replay Engine | core/event-replay-engine.ts | crates/persistence | Implement deterministic hash calc; make DB optional | Phase 1 |
| Consciousness Kernel Enhanced | core/consciousness-kernel-enhanced.ts | crates/cognition | Inject TimeSource and EventBus; remove Date.now() | Phase 0 |
| Full Consciousness Integration | core/full-consciousness-integration.ts | crates/cognition | Remove setInterval; implement tick() method; strengthen world API | Phase 0 |
| World Service | core/world/WorldService.ts | crates/world | Inject TimeSource; make DB optional; replace setInterval | Phase 0 |
| State Container | core/StateContainer.ts | crates/world | Implement processSomaticLayer and processBrainLayer | Phase 0 |

#### REWRITE REQUIRED (Cannot reuse as-is, must implement new)
| System | Gemini Path | Markenz Component | Reason | Priority |
|--------|-------------|-------------------|--------|----------|
| RuntimeLoop | core/runtime/loop.ts | apps/engine | setInterval disabled, no tick mechanism, orphaned | Phase 0 |
| SelfReflectionEngine | core/psychology/self-reflection.ts | crates/cognition | Entirely stubbed, unimplemented | Phase 4+ |
| Twin System Initializer | core/twin-system-initializer.ts | apps/engine | Boot logic, environment file I/O | Phase 0 |
| Boot Manager | core/boot-manager.ts | apps/engine | Env/File I/O, initialization sequencing | Phase 0 |
| Server | core/server.js | apps/server (enhanced) | Node.js HTTP + WebSocket (already partially exists) | Phase 0+ |
| Transport | core/server/frontend-server.js | apps/server + apps/web | WebSocket state fanout | Phase 1+ |
| Networking | core/services/ | apps/server + crates/events | Async/Promise-based async to structured event model | Phase 1+ |
| Human Integration | core/human-systems-integration.js | tools/audits + future admin APIs | External APIs (keep offline unless explicitly admin-enabled) | Phase 9+ |

---

## UNIFIED PHASE EXECUTION PLAN

### Phase 0 — Repo + Offline Stack Baseline Closure

**Objective:** Boot full stack completely offline; establish immutable event sourcing, hash-chain integrity, deterministic tick progression.

**Entry Conditions:**
- PASS_2_REPO_REALITY_AUDIT blocking items resolved
- Cargo build succeeds
- apps/engine, crates/world, crates/events, crates/persistence created

**Phase 0 Deliverables:**

| Deliverable | Component | Description |
|-------------|-----------|-------------|
| Offline stack boot | Docker compose | Postgres, Keycloak, Rust engine, TypeScript server, React web all operational without network |
| Keycloak realm import | infra/keycloak | Roles: admin, observer, auditor |
| Database schema | infra/postgres | Append-only input_events, observation_events, snapshots, hash_checkpoints tables with hash-chain constraints |
| Engine bootstrap | apps/engine | Fixed-timestep loop, genesis snapshot emission, per-tick world_hash computation |
| Server control plane | apps/server | OIDC JWT verification (local JWKS cache), RBAC enforcement, append-only event persistence, WebSocket fanout |
| Web UI | apps/web | Login required, live tick display, world_hash visibility, read-only event timeline, admin InputEvent sender (RBAC gated) |

**Phase 0 Determinism Foundation:**
- Fixed timestep with tick index as authoritative time
- Canonical event ordering (lexicographical by Tick, SourceId, Sequence)
- Hash-chain enforcement on all immutable tables
- RNG audit logging infrastructure

**Phase 0 Exit Criteria (ALL MUST BE TRUE):**

- [ ] Determinism replay test (TEST-DET-001): 100 ticks, same seed, multiple runs, all hashes identical
- [ ] Snapshot equivalence (TEST-SNAPSHOT-EQ-001): snapshot at tick 50 + 50 events ≡ full run to tick 100
- [ ] Hash-chain integrity (TEST-HASH-CHAIN-001): all prev_hash fields correct, no breaks
- [ ] RNG determinism (TEST-RNG-AUDIT-001): ChaosStream sequences bit-identical across platforms
- [ ] Identity preservation (TEST-IDENTITY-001): Gem-D/Gem-K decisions match Gemini at ticks 0–100
- [ ] Asset validation (TEST-PRESERVATION-001): all 6 assets present, valid, match Gemini state
- [ ] Authority enforcement (TEST-AUTHORITY-001): server cannot mutate state, static analysis confirms
- [ ] Build success: `cargo build --release` and `cargo test --all` both pass with zero warnings
- [ ] Docker compose: Full stack boots offline
- [ ] No critical paths contain TODO, FIXME, stub, panic!, unimplemented!
- [ ] AMP Principal-Level Auditor approval obtained

**Phase 0 No-Go Criteria (ANY OF THESE = STOP):**
- Determinism test fails (hash mismatch on rerun)
- Build fails in any CI environment
- Authority boundary violated (server mutates state)
- Asset data loss detected (Gem-D/Gem-K/House/Shed incomplete)
- RNG diverges across platforms
- Database migration fails or corrupts data
- Hash-chain broken
- Panics occur in first 1000 ticks

**Estimated Duration:** 2–4 weeks (senior Rust engineer, full-time)

---

### Phases 1–9 (Reference from MARKENZ_EXECUTION_ROADMAPv2.md)

| Phase | Objective | Key Deliverables | Determinism Gate |
|-------|-----------|------------------|------------------|
| Phase 1 | Deterministic Kernel + Replay | Deterministic scheduler, subsystem RNG streams, canonical hashing, snapshot replay | Snapshot replay ≡ full replay |
| Phase 2 | World Representation v1 | Chunked terrain, biomes, structures, real mechanics (move, gather, build, mine) | Actions deterministic, replay identical |
| Phase 3 | Embodied Biology v1 | Metabolism, hydration, thermoregulation, circadian, immune, injury/healing, BioVeto | Agents starve/fatigue/heal deterministically |
| Phase 4 | Cognition Engine (No LLM) | Deterministic planner, skill trees, deterministic English (grammar templates + lexicon) | Identical thoughts/speech for identical state |
| Phase 5 | Social Dynamics + Scaling | Relationship graph, attachment, trust/conflict, gossip, culture, multi-agent scaling | Social state replay-identical |
| Phase 6 | Genetics + Reproduction | Double-helix genome, recombination, mutation, phenotype, reproduction pipeline, lineage | Same parents + seed → same child genome |
| Phase 7 | Economy + Governance | Property, markets, farming, elections, laws, courts, enforcement | Governance outcomes deterministic |
| Phase 8 | WebGPU Renderer + Transparency | WebGPU rendering, render packets, diff heatmaps, causality graph, time-travel UI | Renderer hash-stable, UI never mutates |
| Phase 9 | Security + Integrity Hardening | Keycloak primary (WebAuthn), encryption at rest, tamper-evident audit logs, immutable auth trail | Tampering detected, passkeys work offline |

---

## UNIFIED RISK & CONTAINMENT MODEL

### Cross-Document Risk Mapping

**Risk Category 1: Determinism Divergence**

**Failure Mode:** Same seed + same InputEvents → different world_hash on rerun.

**Mitigation:**
1. Implement TEST-DET-001: Run 100 ticks twice, compare hashes at each tick
2. If mismatch: Record first divergent tick
3. Produce divergence report with RNG audit log at divergent tick
4. Run with strace/ltrace to identify system call causing divergence
5. Fix identified root cause
6. Re-run test until passing

**Stop Condition:** If determinism divergence cannot be eliminated, STOP immediately and escalate with full diagnostics.

---

**Risk Category 2: Asset Corruption**

**Failure Mode:** House location wrong, Shed inventory missing, Tools durability uninitialized, Vehicles state broken.

**Mitigation:**
1. Create `fn validate_all_assets(world: &World) -> Result<()>`
2. Call in genesis immediately after world creation
3. Call before every snapshot write
4. Call after every snapshot load
5. If validation fails: Panic with asset report

**Stop Condition:** If asset validation fails at any gate, STOP. Do not proceed without full asset recovery.

---

**Risk Category 3: Authority Boundary Violation**

**Failure Mode:** Server or web UI mutates world state.

**Mitigation:**
1. Structural enforcement: World state types **only** in `crates/world`, never in server
2. Static analysis: Scan for server imports of world mutation functions
3. Code review: All server changes reviewed for state mutation
4. Test: TEST-AUTHORITY-001 — server cannot create or call any World mutation function
5. Authority graph: Document all inter-crate dependencies, verify no circular authority

**Stop Condition:** If authority boundary violated, STOP. Do not merge without full separation.

---

**Risk Category 4: Identity Loss**

**Failure Mode:** Gem-D/Gem-K state corrupted, memories lost, relationships broken, identity fingerprint diverges.

**Mitigation:**
1. Full state snapshot validation (TEST-BRIDGE-001): Round-trip 100 snapshots (Gemini JSON ↔ Markenz binary)
2. Dual comparison (TEST-PRESERVATION-001): Gem-D/Gem-K state at checkpoint tick 0 identical to Gemini
3. Determinism gate (TEST-IDENTITY-001): First 100 ticks produce identical decisions
4. Immutable fingerprinting: Identity hash stored in genesis, never mutated

**Stop Condition:** If TEST-IDENTITY-001 fails, STOP Phase 0. Do not proceed to Phase 1 without full identity recovery.

---

### Escalation Rules (Windsurf to AMP Auditor)

**Windsurf must escalate to AMP auditor (HALT execution) if:**

1. **Ambiguity in Plan:** Any requirement unclear or conflicts with existing code → escalate with evidence
2. **Blocker Not Documented:** If blocker not covered in plan → escalate with full context
3. **Determinism Divergence:** TEST-DET-001 fails → escalate with divergence report
4. **Build Failure:** Cargo fails due to undocumented issue → escalate with build log
5. **Asset Corruption:** Asset validation fails → escalate with asset report
6. **Authority Violation:** Code review detects state mutation outside engine → escalate
7. **Test Failure:** Any specified test fails → escalate with test output
8. **Timeline Overrun:** Phase takes >2x estimated time → escalate for re-planning

**Escalation Format:**
```
FROM: Windsurf
TO: AMP Auditor
SUBJECT: Escalation — [Category]
CONTEXT: [What you were trying to do]
BLOCKER: [The specific issue]
EVIDENCE: [Build logs, code snippets, test output, etc.]
PROPOSAL: [Suggested fix, if any]
```

---

## WINDSURF EXECUTION CONTRACT (FINAL)

### 7.1 What Windsurf May Do

**Explicitly Authorized:**

1. Create all specified crates and binaries
2. Implement signatures specified in this plan
3. Write tests as specified
4. Commit code following this plan exactly
5. Run verification scripts
6. Report results to AMP auditor
7. Escalate blockers to AMP with evidence
8. Propose workarounds only if documented in escalation

**Judgment Authority:**

Windsurf may choose implementation details (algorithms, library selection) **provided they maintain:**
- **Determinism** (no wall-clock, no Math.random, seeded RNG only)
- **Authority boundaries** (Rust only, no server mutation)
- **Replay equivalence** (same input sequence → identical state)

---

### 7.2 What Windsurf May NOT Do

**Explicitly Forbidden:**

1. Reuse Gemini code without applying specified modifications
2. Modify existing `apps/server`, `apps/web`, `infra/` code without explicit plan instruction
3. Implement TODO/FIXME/stub/mock code
4. Merge without passing all specified tests
5. Skip any phase gate criterion
6. Invent missing Gemini data (if unclear, escalate)
7. Change repo layout from locked structure
8. Add external dependencies without explicit approval
9. Merge code if AMP audit identifies critical violations
10. Proceed to next phase if current phase gates fail

---

### 7.3 Stop Conditions

**Execution STOPS if:**

1. Any Phase Gate Fails: If test or criterion fails → STOP and escalate
2. Determinism Divergence: If replay produces different hashes → STOP immediately
3. Authority Violation Detected: If server or web mutates state → STOP
4. Asset Loss Detected: If Gem-D/Gem-K/House/Shed incomplete → STOP
5. Build Broken: If cargo build fails → STOP until fixed
6. AMP Auditor Directs: AMP can halt at any time with written directive

**Upon STOP:**
- Do not proceed to next phase
- Do not commit incomplete work
- Produce full diagnostic report
- Escalate to AMP with evidence
- Wait for re-planning or approval to proceed

---

## FINAL GO / NO-GO CHECKLIST

### Phase 0 Must-Pass Criteria (ALL REQUIRED)

**Build & Compilation:**
- [ ] Cargo build succeeds (release + test builds, zero warnings)
- [ ] All unit tests passing (cargo test --all)
- [ ] All integration tests passing
- [ ] Docker compose boots all services, no errors

**Determinism & Replay:**
- [ ] TEST-DET-001 passing: 100 ticks, same seed, identical hashes across multiple runs
- [ ] TEST-SNAPSHOT-EQ-001 passing: Snapshot replay ≡ full replay
- [ ] TEST-HASH-CHAIN-001 passing: Hash-chain integrity verified (no breaks)
- [ ] TEST-RNG-AUDIT-001 passing: RNG sequences bit-identical across platforms (Linux x64/arm64, macOS)
- [ ] TEST-RNG-001 passing: Chaos stability test (1000 calls match fixture)

**Identity & Asset Preservation:**
- [ ] TEST-IDENTITY-001 passing: Gem-D/Gem-K decisions match Gemini at ticks 0–100
- [ ] TEST-BRIDGE-001 passing: Round-trip asset validation (100 snapshots Gemini JSON ↔ Markenz binary)
- [ ] TEST-PRESERVATION-001 passing: No state loss during integration
- [ ] TEST-ASSETS-001 passing: Assets present, valid, determinism preserved

**Authority & Boundaries:**
- [ ] TEST-AUTHORITY-001 passing: Server cannot mutate state, static analysis confirms
- [ ] TEST-STUB-001 passing: No TODO, FIXME, stub, panic!, unimplemented! in critical paths
- [ ] TEST-UNI-001 passing: No global mutable state (except controlled `Universe` root)

**Schema & Persistence:**
- [ ] TEST-SCHEMA-001 passing: Database schema correct
- [ ] Database migration succeeds (no data corruption)
- [ ] Hash-checkpoints table created and functional
- [ ] Append-only constraint enforced

**Infrastructure:**
- [ ] TEST-EVENT-SCHEMA-001 passing: Events serialize correctly
- [ ] TEST-ENGINE-BOOT-001 passing: Engine starts cleanly
- [ ] TEST-WORLD-HASH-001 passing: Hash stable across runs
- [ ] TEST-OFFLINE-001 passing: No network access detected (tcpdump confirms)
- [ ] TEST-ISO-001 passing: Time isolation scan (no Date.now outside real_time_source.rs)
- [ ] TEST-OFF-001 passing: Offline-only scan (no fetch/axios/http in core)
- [ ] TEST-ENC-001 passing: Encryption verification (DB file unreadable without key)

**Security & RBAC:**
- [ ] RBAC enforcement verified
- [ ] Observer role denied InputEvents
- [ ] Admin role allowed InputEvents
- [ ] No hardcoded secrets
- [ ] No auth bypass

**Observability & Auditing:**
- [ ] Genesis snapshot emitted
- [ ] Per-tick world_hash checkpoints logged
- [ ] RNG audit log created and queryable
- [ ] Event timeline visible in UI

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval obtained in writing

**Failure of ANY criterion = Phase 0 NO-GO. Do not proceed to Phase 1.**

---

### Phase 0 No-Go Criteria (STOP IF ANY ARE TRUE)

- [ ] Determinism test fails (hash mismatch on rerun)
- [ ] Snapshot equivalence test fails (replay ≠ full run)
- [ ] Build fails in any CI environment
- [ ] Authority boundary violated (server mutates state, code analysis confirms)
- [ ] Asset data loss detected (Gem-D/Gem-K/House/Shed incomplete or invalid)
- [ ] RNG diverges across platforms (determinism broken)
- [ ] Database migration fails or corrupts data
- [ ] Hash-chain broken (prev_hash mismatch detected)
- [ ] Panics occur in first 1000 ticks
- [ ] Performance regression >50% vs baseline
- [ ] AMP audit report identifies critical violations
- [ ] Build time >1 hour

**If ANY no-go criterion is true: STOP Phase 0 immediately. Escalate with evidence.**

---

## FINAL DECISION & BINDING AUTHORITY

**STATUS: BINDING · EXECUTION-READY**

This plan is:
- ✅ **Binding:** All sections are law, not suggestions
- ✅ **Mechanically Enforceable:** Every step has success/failure criteria
- ✅ **Phased & Contained:** Risk bounded per phase
- ✅ **Auditable:** Complete traceability to governing law
- ✅ **Fail-Closed:** Any violation halts execution
- ✅ **Directly Executable:** Windsurf can execute without interpretation
- ✅ **Lossless Synthesis:** All unique features/constraints/guarantees from input documents preserved

**Authority:**
- **Plan Owner:** ANTIGRAVITY (AMP)
- **Execution Authority:** Windsurf
- **Approval Authority:** AMP Principal-Level Auditor
- **Escalation Path:** KAIZA-MCP governing law

**Next Step:** Execute Phase 0 as specified using this plan as the sole authority.

---

**END OF PLAN**

---

**SIGNATURE & AUTHORITY**

**Synthesized By:** ANTIGRAVITY (AMP / Planner)  
**Timestamp:** 2026-01-10  
**Authority:** KAIZA-MCP v2  
**Status:** BINDING & EXECUTION-READY  
**Plan ID:** MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v1
