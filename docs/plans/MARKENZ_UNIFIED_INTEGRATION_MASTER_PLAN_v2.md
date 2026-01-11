---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
timestamp: 2026-01-10
scope: Unified Markenz Execution — Roadmap v2 Complete Integration
fail_mode: FAIL-CLOSED
plan_id: MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2
supersedes: |
  - MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v1 (incomplete roadmap integration)
  - MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2 (consolidated herein)
  - MARKENZ_REUSE_MIGRATION_PLAN_v3 (consolidated herein)
---

# MARKENZ UNIFIED INTEGRATION MASTER PLAN v2
# (Roadmap-Complete · Phase 0 → Phase 9 · Fully Explicit)

**AUTHORITY:** KAIZA-MCP · AMP (ANTIGRAVITY)  
**MODE:** BINDING · DETERMINISTIC · ROADMAP-INCLUSIVE · FAIL-CLOSED  
**SCOPE:** Single authoritative execution plan merging all governing laws and audit findings; explicitly covers Phase 0 through Phase 9 with hard gates.  
**AUDIENCE:** Windsurf executor (direct execution authority)

---

## 1. STATUS & SUPERSESSION DECLARATION

**STATUS:** BINDING · MASTER AUTHORITY · ROADMAP-INTEGRATED

**SUPERSEDES:**
- MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v1.md (incomplete; Phases 2–9 missing)
- MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2.md (all sections consolidated here)
- MARKENZ_REUSE_MIGRATION_PLAN_v3.md (all technical detail absorbed)
- All prior integration/reuse/audit plans (consolidated into single source)

**FAIL MODE:** FAIL-CLOSED (any blocker → STOP execution)

**ENFORCEMENT AUTHORITY:** AMP Principal-Level Auditor

**AUDIT RESOLUTION:** This plan resolves all 5 MANDATORY ACTIONS and 5 CRITICAL AMBIGUITIES identified in AMP_ROADMAP_INTEGRATION_AUDIT.md. See § 6 (Resolved Ambiguity Specifications) for explicit resolution.

---

## 2. ROADMAP INCORPORATION INDEX

**Confirmation:** MARKENZ_EXECUTION_ROADMAP_v2.md is fully, explicitly, and losslessly integrated herein. Every phase (0 → 9) has dedicated sections below with objectives, deliverables, determinism gates, exit criteria, and escalation rules.

| Phase | Roadmap Title | Unified Plan Section | Status |
|-------|---|---|---|
| Phase 0 | Repo + Offline Stack Baseline Closure | § 5.1 | ✅ EXPLICIT |
| Phase 1 | Deterministic Kernel + Replay Harness | § 5.2 | ✅ EXPLICIT |
| Phase 2 | World Representation v1 (Terrain + Entities) | § 5.3 | ✅ EXPLICIT |
| Phase 3 | Embodied Biology v1 | § 5.4 | ✅ EXPLICIT |
| Phase 4 | Cognition Engine (No LLM) | § 5.5 | ✅ EXPLICIT |
| Phase 5 | Social Dynamics + Scaling | § 5.6 | ✅ EXPLICIT |
| Phase 6 | Genetics + Reproduction | § 5.7 | ✅ EXPLICIT |
| Phase 7 | Economy + Governance | § 5.8 | ✅ EXPLICIT |
| Phase 8 | WebGPU Renderer + Transparency UI | § 5.9 | ✅ EXPLICIT |
| Phase 9 | Security + Integrity Hardening | § 5.10 | ✅ EXPLICIT |

**NO PHASE OMITTED.** Phase ordering is strictly enforced with hard gates.

---

## 3. UNIFIED SYSTEM GUARANTEES

### 1. Determinism Guarantee
- **Fixed timestep:** Simulation time is tick-indexed (u64), never wall-clock dependent
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

### 3. Identity Continuity Guarantee (Gem-D, Gem-K)
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

### 6. Self-Evolution Guarantee
Per MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md:
- **No code self-modification:** System cannot modify its own source code or engine logic
- **No runtime rule creation:** New behaviors emerge only from bounded parameter drift within fixed rule set
- **Evolution is state:** All adaptation recorded as state transitions, logged as events, hashed
- **Replay reproduces evolution:** Same seed + events = identical evolutionary trajectory
- **Governed evolution:** Governance mechanisms constrain and shape evolutionary paths deterministically
- **Observable evolution:** All trait drift, cultural changes, and learning updates visible in event logs

---

## 4. UNIFIED AUTHORITY & ARCHITECTURE MODEL

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
- Schema frozen per MARKENZ_EXECUTION_ROADMAP_v2.md § Phase 0

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

---

## 5. PHASE 0 → PHASE 9 EXECUTION PLAN
### (All Phases Explicitly Detailed)

---

### 5.1 PHASE 0: Repo + Offline Stack Baseline Closure

**Objective:**
Boot the full stack completely offline. Establish immutable event sourcing, hash-chain integrity, and deterministic tick progression.

**Deliverables:**
- Offline stack boot:
  - Postgres (append-only tables)
  - Keycloak (local realm, JWKS cache)
  - Rust engine (`apps/engine`)
  - TypeScript server (`apps/server`)
  - React web UI (`apps/web`)
- Keycloak realm import:
  - Roles: `admin`, `observer`, `auditor`
- Postgres schema:
  - Append-only `input_events` (tick, source, payload, hash, prev_hash)
  - Derived `observation_events` (tick, type, payload, hash, input_hash)
  - `snapshots` (tick, state_blob, world_hash, input_hash)
  - `hash_checkpoints` (tick, world_hash, verified)
- Engine:
  - Fixed-timestep loop starts
  - Genesis snapshot emitted
  - Per-tick `world_hash` checkpoints
  - crates/world: Core types (Agent, Asset, Terrain, Chunk)
  - crates/events: InputEvent and ObservationEvent schemas
  - crates/persistence: Snapshot format and basic replay
- Server:
  - OIDC auth via local Keycloak (JWKS cached locally)
  - RBAC enforcement
  - Append-only event writer with hash-chain
  - WebSocket fanout
- Web:
  - Login required
  - Live tick + `world_hash`
  - Read-only event timeline
  - Admin InputEvent sender (RBAC gated)

**Determinism Introduced:**
- Fixed timestep
- Canonical event ordering
- Hash-chain enforcement

**Entry Conditions:**
- Repository structure exists
- Cargo.toml files present for all crates
- Docker Compose environment defined
- No external dependencies on cloud services

**Exit Criteria:**
- `docker compose up --build` works with no internet
- Login + RBAC enforced
- Events logged immutably with correct schema
- Engine ticks advance, hashes computed
- Submitted events replay identically
- All Phase Gate Checklist tests pass (§ 9.1)

**Determinism Gates:**
- TEST-DET-001: 100 ticks, same seed, identical hashes across multiple runs
- TEST-SNAPSHOT-EQ-001: Snapshot replay ≡ full replay
- TEST-HASH-CHAIN-001: Hash-chain integrity verified
- TEST-RNG-AUDIT-001: RNG sequences bit-identical across platforms
- TEST-IDENTITY-001: Gem-D/Gem-K decisions match Gemini at ticks 0–100
- TEST-ASSETS-001: House, Shed, Tools, Vehicles, Gem-D, Gem-K all present and valid

**Windsurf Constraints:**
- Must create apps/engine, all required crates (world, events, persistence)
- Must migrate SimLoop from apps/server to apps/engine
- Must implement Genesis content (seeded terrain, House, Shed, Vehicles, Gem-D, Gem-K)
- Must not skip any test in Phase Gate Checklist
- Must escalate if build fails or determinism tests diverge

**Escalation Rules:**
- If Cargo build fails → escalate with build log
- If TEST-DET-001 fails → escalate with divergence report
- If authority boundary violated → escalate with code evidence
- If assets invalid → escalate with validation report

---

### 5.2 PHASE 1: Deterministic Kernel + Replay Harness

**Objective:**
Prove determinism formally via replay and snapshot equivalence.

**Deliverables:**
- Engine:
  - Deterministic scheduler (tick-based event dispatch)
  - Subsystem RNG streams (physics, biology, cognition, environment, governance)
  - Canonical world hashing (blake3, stable serialization)
  - Snapshot write/read (full state, all subsystems)
  - Replay-from-snapshot equality test harness
  - Genesis world:
    - Markenz seed
    - House, shed, tools, vehicles
    - Agents: Gem-D, Gem-K
- Server:
  - Ordered event delivery guarantees (strict (Tick, SourceId, Sequence) ordering)
  - Integrity verification endpoint (`/api/integrity/verify-hash-chain`)
- Web:
  - Hash-chain status panel (shows current hash, tick, verification status)
  - Agent vitals + inner monologue streams (realtime observation)
  - Event log explorer (timeline, filters)

**Determinism Introduced:**
- Snapshot equivalence proof
- Cross-run hash equality
- RNG audit trail

**Entry Conditions:**
- Phase 0 all tests passing
- All crates compiling without errors
- Genesis world fully initialized

**Exit Criteria:**
- Same seed + same events → identical hash timeline
- Snapshot replay == full replay (at multiple snapshots: tick 100, 500, 1000)
- No authority leakage detected
- RNG deterministic across platforms

**Determinism Gates:**
- TEST-SNAPSHOT-EQ-001: Snapshot at tick N + remaining events == full run to same final state
- TEST-HASH-CHAIN-001: Verify unbroken chain from genesis to final tick
- TEST-RNG-AUDIT-001: RNG audit log searchable and correct
- TEST-IDENTITY-001: Gem-D/Gem-K decisions identical under replay

**Windsurf Constraints:**
- Must implement full snapshot format (all state serializable)
- Must implement replay harness (load snapshot, apply events, verify hash)
- Must add RNG audit logging to every DeterministicRng draw
- Must not modify Phase 0 deliverables
- Must pass all Phase 1 tests before Phase 2 entry

**Escalation Rules:**
- If snapshot equivalence fails → escalate with state diff
- If replay produces different hashes → escalate with first divergent tick
- If RNG audit incomplete → escalate with missing fields

---

### 5.3 PHASE 2: World Representation v1 (Terrain + Entities)

**Objective:**
Replace abstract world with deterministic spatial reality.

**Deliverables:**
- Chunked deterministic terrain:
  - Seeded generation using Markenz root seed
  - Chunk coordinates determine terrain type, biome, features
  - Heightmap or voxel grid (implementation choice)
  - No dynamic terrain changes in Phase 2 (static for replay)
- Biomes v1:
  - Grassland, forest, water, mountains (minimal 4 types)
  - Deterministic biome rules by chunk coordinate
- Structures, tools, vehicles, inventories:
  - House: Location deterministic from seed, structure initialized
  - Shed: Tools inventory, durability tracked
  - Tools: Registry with properties (weight, durability, use cases)
  - Vehicles: Ownership, state, mechanics
- Real mechanics:
  - Move: Agent position updates, collision detection, reach constraints
  - Gather: Resource collection (energy cost, tool requirements, biome-dependent)
  - Build: Structure creation (materials, energy, time)
  - Mine: Deep resource extraction (tools, energy, yield)
- Constraints:
  - Reach: Agent can only affect nearby positions
  - Tools: Certain actions require specific tools
  - Energy: All actions consume metabolism/calories
  - Time: Actions take ticks to complete
  - Collisions: Entities cannot occupy same space

**Entry Conditions:**
- Phase 1 all tests passing
- Snapshot/replay framework proven
- RNG streams stable

**Exit Criteria:**
- Actions succeed/fail deterministically
- Full causality trace visible (every action logged with outcome)
- Replay identical to live run
- Terrain generation stable (same seed = same terrain)
- No undefined behavior in physics

**Determinism Gates:**
- TEST-WORLD-GEN-001: Same seed → identical terrain chunks
- TEST-MOVEMENT-001: Move action replays identically
- TEST-GATHER-001: Gather action energy costs and yields deterministic
- TEST-MECHANICS-001: Full test of Move, Gather, Build, Mine under replay
- No divergence between snapshot replay and full run

**Windsurf Constraints:**
- Must implement crates/physics (deterministic movement, collision)
- Must implement deterministic terrain generation (no procedural noise libraries that aren't seeded)
- Must track all action costs (energy, time, materials)
- Must log all state changes to event log
- Must not add dynamic terrain changes (all generation is static seeding)
- Must implement action validation (reject illegal moves, gather, build, mine)

**Escalation Rules:**
- If terrain generation non-deterministic → escalate with seed comparison
- If movement physics diverge on replay → escalate with logs
- If action failures inconsistent → escalate with event trace

---

### 5.4 PHASE 3: Embodied Biology v1

**Objective:**
Enforce biological reality and veto unsafe actions.

**Deliverables:**
- Metabolism (energy, macros):
  - Caloric burn per tick (basal metabolic rate)
  - Nutrient tracking (protein, carbs, fats)
  - Starvation/satiation states
- Hydration + electrolytes:
  - Water level tracking
  - Electrolyte depletion (sodium, potassium)
  - Dehydration symptoms (fatigue, cognitive impairment)
- Thermoregulation:
  - Core body temperature
  - Environmental temperature effects
  - Heat dissipation/retention
- Circadian rhythm + sleep:
  - Sleep/wake cycle (24-tick period, configurable)
  - Sleep deprivation effects on cognition/movement
  - Sleep stages (NREM1/2/3, REM)
- Vitamins/minerals (deficiency/toxicity):
  - Vitamin A, B, C, D, etc.
  - Essential minerals (iron, calcium, magnesium)
  - Deficiency symptoms
  - Toxicity threshold enforcement
- Injury/healing:
  - Damage model (from actions, environment)
  - Healing rate (deterministic per injury type)
  - Ability modifiers (fatigue, pain, mobility loss)
- Immune response:
  - Pathogen exposure model
  - Immune activation (antibodies, inflammation)
  - Recovery trajectory
- Endocrine axes (HPA/HPG):
  - Stress hormones (cortisol, adrenaline)
  - Sex hormones (testosterone, estrogen, progesterone)
  - Reproductive readiness
- BioVeto with logged reasons:
  - Function: `fn bio_veto(action: &Action, bio_state: &BioState) -> Option<String>`
  - Reasons: "Insufficient energy", "Dehydrated", "Injured", "Fatigued", etc.
  - All vetoes logged to event stream

**Entry Conditions:**
- Phase 2 all tests passing
- World mechanics working deterministically
- crates/world fully defined

**Exit Criteria:**
- Agents starve, fatigue, heal, sleep deterministically
- Unsafe actions vetoed with reproducible reasons
- Biology fully deterministic under replay
- No undefined behavior in metabolic calculations

**Determinism Gates:**
- TEST-METABOLISM-001: Caloric burn deterministic
- TEST-SLEEP-001: Sleep/wake cycle deterministic
- TEST-INJURY-001: Healing rates deterministic
- TEST-BIOVETO-001: BioVeto reasons consistent across runs
- TEST-VITALS-001: Vitals (temperature, heart rate, etc.) deterministic
- Full biology system replays identically

**Windsurf Constraints:**
- Must implement crates/biology with all systems above
- Must use only integer or fixed-point math (no floats in authority)
- Must log all biological state changes to event stream
- Must make BioVeto deterministic (pure function of state)
- Must not allow biological state mutations outside of biology crate

**Escalation Rules:**
- If biological calculations diverge on replay → escalate with affected tick
- If BioVeto reason differs → escalate with state snapshot
- If vitals unstable → escalate with timeseries

---

### 5.5 PHASE 4: Cognition Engine (No LLM)

**Objective:**
Deterministic minds and language, fully offline.

**Deliverables:**
- Perception → Drives → Intent → Action queue:
  - Perception: Encode sensory input (what agent sees, hears, feels)
  - Drives: Convert internal state (hunger, fatigue, fear) into goals
  - Intent: Planner selects action to satisfy drives
  - Action queue: Agent executes queued actions (FIFO per tick)
- Deterministic planner (GOAP/HTN):
  - Choose algorithm (GOAP or Hierarchical Task Network)
  - State-based action selection
  - Goal-oriented planning
  - All randomness seeded per subsystem
- Skill trees + habit formation:
  - Skill definitions (hunting, farming, building, etc.)
  - Skill levels (growth via practice)
  - Habit strength (reinforcement over time)
  - No unbounded growth (skills capped at 100)
- Deterministic English:
  - Grammar templates (predefined sentence structures)
  - Lexicon tables (word choices per semantic role)
  - Pragmatics rules (politeness, formality, emotional coloring)
  - No LLM generation (all outputs from deterministic grammars)
- Continuous inner monologue:
  - Agent narrates internal state (thoughts, feelings, decisions)
  - Streamed to observation events (observable but not determinism-breaking)
  - Audit trail of cognition
- Learning and memory growth:
  - Episodic memory (specific events)
  - Semantic memory (facts, concepts)
  - Procedural memory (skill execution)
  - Memory consolidation (learning from experience)

**Entry Conditions:**
- Phase 3 all tests passing
- BioVeto fully functional
- Agent able to perceive environment

**Exit Criteria:**
- Identical thoughts/speech for identical state
- No LLM dependency
- Replay identical
- Planner produces deterministic action selection
- Learning curves deterministic

**Determinism Gates:**
- TEST-PERCEPTION-001: Same environment → same perceptions
- TEST-PLANNER-001: Same goals + state → same plan
- TEST-SPEECH-001: Same emotion + context → same utterance
- TEST-LEARNING-001: Same practice → same skill growth
- TEST-MEMORY-001: Memory consolidation deterministic
- Full cognition replays identically

**Windsurf Constraints:**
- Must implement crates/cognition with all systems above
- Must choose and implement deterministic planner (GOAP or HTN)
- Must define grammar templates for English (no LLM calls)
- Must log all planning decisions to event stream
- Must make learning rates pure functions of practice
- Must not use any external AI/ML services
- Must pass determinism tests over 10,000+ ticks

**Escalation Rules:**
- If planner output differs on replay → escalate with decision trace
- If grammar produces unexpected output → escalate with template
- If learning diverges → escalate with skill timeseries
- If memory consolidation inconsistent → escalate with memory state

---

### 5.6 PHASE 5: Social Dynamics + Scaling

**Objective:**
Emergent society without determinism drift.

**Deliverables:**
- Relationship graph:
  - Graph of agents + relationship strengths
  - Relationship types (kin, friend, rival, neutral)
  - Relationship dynamics (bonding, conflict, healing)
- Attachment styles:
  - Secure, anxious, avoidant, fearful
  - Influence behavior patterns
  - Change slowly via experience
- Trust/conflict/bonding:
  - Trust value (0–100) per pair
  - Conflict events (disagreements, resource competition)
  - Bonding events (cooperation, shared success)
  - Deterministic trust update rules
- Gossip + reputation propagation:
  - Agents exchange information about other agents
  - Reputation spreads deterministically
  - Affects trust formation and group dynamics
- Culture metrics:
  - Shared norms (cooperation level, sharing, honesty)
  - Cultural drift (norms slowly change)
  - Culture affects individual behavior via pressure
- Multi-agent scaling (dozens of agents):
  - Add Gem-L, Gem-M, ... Gem-Z as population
  - All spawn deterministically from genesis
  - All relationships deterministic
  - No agent special-casing (all follow same rules)

**Entry Conditions:**
- Phase 4 all tests passing
- Cognition engine fully functional
- Planner can handle social goals

**Exit Criteria:**
- Social state replay-identical
- Stable tick rate under load (dozen+ agents)
- Relationship graph deterministic
- No divergence between single-agent and multi-agent runs
- Gossip propagation deterministic

**Determinism Gates:**
- TEST-SOCIAL-001: Relationship values deterministic
- TEST-GOSSIP-001: Rumor spreads identically
- TEST-CULTURE-001: Cultural norm values deterministic
- TEST-SCALING-001: 12+ agents, no performance regression
- TEST-MULTIAGENT-001: Same seed + events → identical social state

**Windsurf Constraints:**
- Must implement relationship graph data structure
- Must implement gossip algorithm (deterministic information flow)
- Must define culture metrics (quantify norms)
- Must make all social updates pure functions
- Must handle agent-to-agent interactions deterministically
- Must scale to 20+ agents without tick overruns

**Escalation Rules:**
- If relationships diverge on replay → escalate with interaction log
- If gossip spreads unexpectedly → escalate with propagation trace
- If performance regresses → escalate with profiling data
- If tick rate becomes unstable → escalate with timing data

---

### 5.7 PHASE 6: Genetics + Reproduction

**Objective:**
True population growth with lineage.

**Deliverables:**
- Double-helix genome:
  - 46 chromosomes (23 pairs, human-like)
  - Genes for traits (height, appearance, predispositions)
  - Allelic variants (dominant, recessive)
  - Gene expression thresholds
- Recombination + mutation (policy bounded):
  - Meiosis: Create gametes (combine parent genes)
  - Fertilization: Merge gametes
  - Mutation: Random changes (rate bounded by policy, e.g., 0.1%)
  - All using seeded RNG
- Phenotype expression:
  - Genes → observable traits
  - Development stages (growth, maturation)
  - Epigenetic effects (environment affects expression)
- Reproduction pipeline:
  - Consent: Agents agree to reproduce (requires relationship strength)
  - Intercourse: Reproductive action
  - Probabilistic conception (not guaranteed)
  - Gestation stages (development in womb, 280 ticks = 9 months)
  - Birth: Baby agent spawned with full genetics
- Lineage trees:
  - Parent → child relationships tracked
  - Multi-generational trees
  - Lineage history queryable
- Genetic disorder toggles:
  - Toggle specific disorders (cystic fibrosis, hemophilia, etc.)
  - Affects phenotype and vitals

**Entry Conditions:**
- Phase 5 all tests passing
- Social dynamics including bonding/consent working
- Agents capable of long-term planning

**Exit Criteria:**
- Same parents + seed → same child genome
- Lineage deterministic and inspectable
- Gestation and birth fully deterministic
- Phenotype expression matches genotype
- No undefined behavior in genetic algorithms

**Determinism Gates:**
- TEST-GENOME-001: Same parents + seed → identical child genome
- TEST-MEIOSIS-001: Gamete creation deterministic
- TEST-CONCEPTION-001: Conception probability deterministic
- TEST-GESTATION-001: Gestation timeline deterministic
- TEST-LINEAGE-001: Lineage tree accurate and consistent
- Full reproduction replays identically

**Windsurf Constraints:**
- Must implement crates/genetics with genome data structure
- Must implement meiosis and fertilization algorithms
- Must implement phenotype expression rules (gene → trait mapping)
- Must track lineage history immutably
- Must make all genetic operations pure functions of seed + parents
- Must not allow genetic state mutations outside genetics crate

**Escalation Rules:**
- If child genome differs on replay → escalate with parent genomes
- If gestation diverges → escalate with conception tick
- If phenotype expression unexpected → escalate with genotype
- If lineage tree inconsistent → escalate with tree snapshot

---

### 5.8 PHASE 7: Economy + Governance

**Objective:**
Deterministic rules governing society and resources.

**Deliverables:**
- Property and ownership:
  - Resources assigned to agents or groups
  - Ownership rules (can trade, give, inherit)
  - Theft/sharing mechanics
- Resource markets:
  - Supply/demand curves (deterministic)
  - Price setting (auction or fixed based on scarcity)
  - Trade agreements and contracts
- Farming and animals:
  - Agricultural production (crops, livestock)
  - Breeding programs for animals
  - Harvest cycles (deterministic yields)
- Elections:
  - Agent voting on leaders/policies
  - Voting rules (majority, consensus)
  - Term limits
  - Deterministic vote counting
- Laws and policies:
  - Rules encoded as deterministic modules
  - Policies define allowed/forbidden actions
  - Policy changes via election or decree (via InputEvents)
- Courts and penalties:
  - Violation detection (deterministic)
  - Court hearings (judges evaluate evidence)
  - Penalties (fines, labor, exile)
  - Appeals process

**Entry Conditions:**
- Phase 6 all tests passing
- Reproduction working, population growing
- Social structures stable
- Agents capable of cooperation and group decision-making

**Exit Criteria:**
- Laws constrain actions deterministically
- Governance outcomes replay-identical
- Elections deterministic (same voters + info → same results)
- Economy stable (no hyperinflation, supply/demand balanced)
- No undefined behavior in policy evaluation

**Determinism Gates:**
- TEST-PROPERTY-001: Ownership transfers deterministic
- TEST-MARKETS-001: Trade prices deterministic
- TEST-FARMING-001: Crop yields deterministic
- TEST-ELECTIONS-001: Same voters + info → same election result
- TEST-LAWS-001: Policy violations detected consistently
- TEST-COURTS-001: Court judgments deterministic
- Full governance replays identically

**Windsurf Constraints:**
- Must implement property/ownership model
- Must implement market price algorithm (deterministic)
- Must implement farming/resource production
- Must implement voting and election algorithm
- Must implement policy evaluation (pure function of state + rules)
- Must implement court verdict logic
- Must make all governance operations deterministic
- Must log all governance events to observation stream

**Escalation Rules:**
- If property transfers inconsistent → escalate with transaction log
- If market prices diverge → escalate with supply/demand state
- If election results differ → escalate with voter list and preferences
- If policy violations missed → escalate with state and policy
- If court verdicts inconsistent → escalate with case evidence

---

### 5.9 PHASE 8: WebGPU Renderer + Transparency UI

**Objective:**
Professional visualization without authority leakage.

**Deliverables:**
- WebGPU renderer:
  - 3D visualization of world (terrain, agents, structures)
  - Real-time rendering from snapshot
  - Multi-viewport support
- Render packets derived from snapshots:
  - Render packet schema (deterministic serialization)
  - Packets derived from world_hash (immutable for snapshot)
  - Allows caching and offline rendering
- Multi-monitor layouts:
  - Split-screen views (different regions, different detail levels)
  - Customizable viewport arrangements
- Diff heatmaps:
  - Visualize state changes between ticks
  - Shows which parts of world changed
  - Causality highlighting (what caused changes)
- Causality graph:
  - Node: Event or action
  - Edge: Causality (A caused B)
  - Interactive navigation of causal chains
- Time-travel debugger:
  - Scrub to any tick
  - Inspect world state at that tick
  - Inspect agent cognition at that tick
  - Replay from arbitrary tick forward

**Entry Conditions:**
- Phase 7 all tests passing
- All state fully observable
- Snapshot format mature and stable

**Exit Criteria:**
- Renderer hash-stable for snapshots (same snapshot → same render)
- UI never mutates state
- Time-travel tools work correctly
- Causality graph accurate and queryable
- Performance acceptable (60 FPS for typical view)

**Determinism Gates:**
- TEST-RENDER-HASH-001: Same snapshot → identical render packet hash
- TEST-UI-AUTHORITY-001: UI cannot mutate state (static analysis)
- TEST-TIMETRAVEL-001: Scrub to tick N, result matches original tick N
- TEST-CAUSALITY-001: Causality graph matches event log

**Windsurf Constraints:**
- Must implement WebGPU rendering pipeline
- Must implement render packet serialization (deterministic)
- Must not allow UI to send non-InputEvent commands
- Must implement time-travel scrubbing
- Must build causality graph from event log
- Must not modify engine or server logic (pure visualization layer)

**Escalation Rules:**
- If render packet hashes diverge → escalate with snapshots
- If UI mutation attempted → escalate with code path
- If time-travel inaccurate → escalate with tick comparison
- If causality graph wrong → escalate with event sequence

---

### 5.10 PHASE 9: Security + Integrity Hardening

**Objective:**
Lock security without breaking determinism or offline mode.

**Deliverables:**
- Keycloak primary (WebAuthn/passkeys):
  - Hardware security key support
  - Passkey enrollment for users
  - Passwordless login
  - Offline capability (cached keys)
- Authentik backup:
  - Failover identity provider
  - Never authoritative (Keycloak primary)
  - Same role mappings
- Encryption at rest:
  - Database file encrypted (envelope encryption)
  - Snapshots encrypted before storage
  - Keys managed via Keycloak
  - Decryption via OIDC-authenticated user
- Tamper-evident audit logs:
  - Event log signed/sealed
  - Any modification detected
  - Immutable proof format (blockchain-like hash chain)
- Immutable auth/admin audit trail:
  - All auth events logged immutably
  - All admin InputEvents logged
  - Audit trail itself immutable
- Integrity explorer UI:
  - Query audit logs
  - Verify hash chains
  - Detect tampering
  - Export audit reports

**Entry Conditions:**
- Phase 8 all tests passing
- All systems stable and well-tested
- Determinism proven across all phases

**Exit Criteria:**
- Tampering detected deterministically
- Passkeys work offline
- Replay and hashes still pass
- Encryption-at-rest working
- Audit logs verifiable

**Determinism Gates:**
- TEST-AUTH-001: Keycloak login works offline
- TEST-ENCRYPTION-001: Database unreadable without key
- TEST-AUDITLOG-001: Audit log tamper-detection works
- TEST-INTEGRITY-001: Integrity verification passes on valid logs, detects tampering

**Windsurf Constraints:**
- Must implement WebAuthn/passkey support in Keycloak
- Must implement database encryption (envelope pattern)
- Must implement audit log signing/verification
- Must implement integrity explorer UI
- Must not break determinism (encryption is orthogonal)
- Must not require network for core encryption (keys cached)

**Escalation Rules:**
- If Keycloak offline mode fails → escalate with key cache state
- If encryption breaks replay → escalate immediately (no acceptable mitigation)
- If audit tampering goes undetected → escalate with evidence
- If passkey enrollment fails → escalate with FIDO2 debug logs

---

## 6. RESOLVED AMBIGUITY SPECIFICATIONS

**Binding Law:** The 5 CRITICAL ambiguities identified in AMP_ROADMAP_INTEGRATION_AUDIT.md § 5 are now explicitly resolved below. These specifications are contracts — Windsurf must implement exactly as specified.

### 6.1 PHASE 2 TERRAIN SYSTEM (CRITICAL AMBIGUITY #1)

**Specification:**
- **Representation:** Chunked deterministic grid
- **Chunk Size:** 16×16 blocks (at scale of ~1m per block)
- **Chunk Coordinates:** (x, y, z) tuples, integer
- **Height:** Per-block elevation (integer, 0–256)
- **Biome:** Determined by chunk coordinate via seeded RNG
  - Seed: `blake3(MARKENZ_SEED || chunk_x || chunk_y)`
  - Result: Biome type (Grassland, Forest, Water, Mountain)
- **Generation:** Deterministic, seeded, fully offline
- **Storage:** In snapshots (chunk data persisted with snapshot)
- **Access:** Lazy generation (generate on demand, cache)
- **Physics:** Collision detection per block (agent cannot occupy water/mountain)
- **Reproducibility:** Same seed + chunk coordinates = identical terrain forever

**Test Contract:**
- `TEST-WORLD-GEN-001`: Generate same chunk 10 times, all identical
- `TEST-TERRAIN-REPLAY-001`: Replay includes identical terrain generation

### 6.2 PHASE 4 COGNITION ENGINE (CRITICAL AMBIGUITY #2)

**Specification:**
- **Planner Algorithm:** GOAP (Goal-Oriented Action Planning)
- **State Representation:** Symbolic predicates (agent:hungry=true, agent:near:food=false)
- **Action Preconditions:** Predicates that must be true
- **Action Effects:** Predicates that become true post-action
- **Goal:** Top-level predicate to satisfy
- **Planning:** Backward-chaining search from goal to current state
- **Execution:** Forward-execution of plan (FIFO action queue)
- **RNG:** Subsystem RNG stream for action tie-breaking (deterministic)
- **Language:** Grammar templates (no LLM)
  - Template: "I want to [VERB] the [NOUN]"
  - Choices: VERB from verb list, NOUN from entity list
  - Selection: Deterministic based on state
- **Memory:** Episodic + semantic
  - Episodic: "At tick 500, I gathered berries" (immutable event log)
  - Semantic: "Berries are food" (learned facts)
  - Learning: Count +1 per consistent experience, cap at 100
- **Inner Monologue:** Continuous narration streamed to observation events
  - Topics: Current goal, perceptions, feelings, decisions
  - Deterministic from state

**Test Contract:**
- `TEST-PLANNER-001`: Same state + goal → same plan
- `TEST-SPEECH-001`: Same emotional state + context → same speech
- `TEST-LEARNING-001`: Same practice sequence → same skill level

### 6.3 PHASE 6 GENETICS (CRITICAL AMBIGUITY #3)

**Specification:**
- **Genome:** 46 chromosomes (23 autosomal pairs, 1 sex pair)
- **Gene Representation:** Allele pairs per locus
- **Loci:** ~100 significant loci (height, skin tone, predispositions)
- **Recombination:** Random crossover during meiosis
  - RNG: Subsystem RNG, seeded per parent + meiosis
  - Result: Deterministic gamete for given parent pair
- **Mutation:** Per-base mutation rate 0.1%
  - RNG: Subsystem RNG, seeded
  - Effect: Random allele flip
- **Phenotype Expression:** Locus → trait mapping
  - Examples:
    - Height loci: sum alleles → height (100–200 cm)
    - Skin tone: multiple loci, blended
    - Predisposition: threshold (if 50% alleles for aggression, high aggression)
- **Dominance:** Simple Mendelian (AA, Aa, aa) for some traits; additive for others
- **Conception:** Probabilistic
  - Base rate: 20% per intercourse
  - Modifiers: Health, nutrition, age (affects rate)
  - RNG: Subsystem RNG, seeded per mating
- **Gestation:** 280 ticks (9 months at 20 ticks/day)
  - Stages: Trimester 1/2/3 with different caloric needs
  - Birth: Baby agent spawned with full genetics

**Test Contract:**
- `TEST-GENOME-001`: Same parents + meiosis seed → identical gametes
- `TEST-PHENOTYPE-001`: Same genotype → identical phenotype
- `TEST-CONCEPTION-001`: Same health/conditions + RNG seed → deterministic conception

### 6.4 PHASE 7 GOVERNANCE (CRITICAL AMBIGUITY #4)

**Specification:**
- **Policy Representation:** Deterministic rule modules
  - Input: Agent state, action proposed, world state
  - Output: Allowed/Forbidden + reason
  - Example: "Mining requires tool" → Forbidden if no pick
- **Policy Language:** Simple if/then rules (no Turing-complete execution)
- **Voting:** Majority rule
  - Voter preference: Deterministic from agent state (personality, history, bias)
  - Preference weighted by agent influence (tied to status/relationships)
  - Tie-breaking: RNG (subsystem seeded)
- **Courts:** Deterministic verdict rules
  - Judge: Designated agent (elected or hereditary)
  - Evidence: InputEvents that led to violation
  - Verdict rules: Deterministic evaluation (same evidence → same verdict)
  - Penalty: Deterministic from violation type (labor, fine, exile)
- **Elections:** Term length, candidacy rules, voting rules deterministic
- **Enforcement:** Policies checked at commit time (BioVeto + PolicyVeto)
  - Veto reason logged to event stream

**Test Contract:**
- `TEST-POLICIES-001`: Same violation + rules → same verdict
- `TEST-ELECTIONS-001`: Same voters + preferences → same election result
- `TEST-VOTING-001`: Majority voting deterministic

### 6.5 PHASE 8 WEBGPU RENDERER (CRITICAL AMBIGUITY #5)

**Specification:**
- **Render Packet Schema:** Deterministic serialization
  - Packets derived from world snapshot (immutable for given tick)
  - Fields: Agent positions, terrain chunk data, structure geometry
  - Serialization: Binary format (canonical field order)
- **Authority Isolation:** Renderer reads snapshots only, never modifies state
  - Static analysis enforces: Renderer crate has no `mut` world access
- **Hash Stability:** Same snapshot + time → identical render packet
  - Test: Hash snapshot, render, hash render packet, verify hash stability
- **Diff Heatmaps:** Compute per-chunk delta from previous frame
  - Display intensity proportional to change amount
  - Update: Deterministic from state diff
- **Causality Graph:** Built from event log
  - Nodes: Events
  - Edges: Causal dependencies (event A caused event B)
  - Traversal: Interactive query by UI

**Test Contract:**
- `TEST-RENDER-HASH-001`: Hash(snapshot) == hash(render packet)
- `TEST-DIFF-HEATMAP-001`: Same state diff → same heatmap
- `TEST-CAUSALITY-001`: Causality graph matches event dependencies

---

## 7. RISK & CONTAINMENT MODEL

### Risk Category 1: Determinism Divergence

**Failure Mode:** Seed + InputEvents → different world_hash on rerun. Replay fails.

**Mitigation:**
1. Implement determinism test (TEST-DET-001) in CI/CD
2. Run 100+ ticks, save hash sequence
3. Rerun 3 times, verify all hashes identical
4. If divergence detected:
   - Isolate first divergent tick (binary search)
   - Check RNG audit log at that tick
   - Identify subsystem source (physics, biology, cognition, etc.)
   - Likely causes: wall-clock call, nondeterministic container, floating-point ordering
5. Fix identified root cause
6. Re-run until passing

**Stop Condition:** If determinism divergence cannot be eliminated, STOP immediately. Escalate with full diagnostics.

---

### Risk Category 2: Asset Corruption

**Failure Mode:** House location wrong, Shed inventory missing, Tools durability uninitialized, Vehicles state broken, Gem-D/Gem-K identity diverged.

**Mitigation:**
1. Create `fn validate_all_assets(world: &World) -> Result<()>`
   - Verifies house exists and location valid
   - Verifies shed exists with non-empty inventory
   - Verifies all tools registered with valid durability
   - Verifies all vehicles present with consistent state
   - Verifies Gem-D/Gem-K present with expected consciousness state
2. Call in genesis immediately after world creation
3. Call before every snapshot write
4. Call after every snapshot load
5. If validation fails: Panic with detailed asset report

**Stop Condition:** If asset validation fails at any gate, STOP. Do not proceed without full asset recovery.

---

### Risk Category 3: Authority Boundary Violation

**Failure Mode:** Server or web UI mutates world state.

**Mitigation:**
1. Structural enforcement: World state types **only** in `crates/world`, never in server
2. Static analysis: CI scans server imports for world mutation functions
   - If found: Build fails immediately
3. Code review: All server changes reviewed for state mutation
4. Test: `TEST-AUTHORITY-001` — server cannot create or call any World mutation function
5. Authority graph: Document all inter-crate dependencies, verify no circular authority

**Stop Condition:** If authority boundary violated, STOP. Do not merge without full separation.

---

### Risk Category 4: Identity Loss

**Failure Mode:** Gem-D/Gem-K state corrupted, memories lost, relationships broken, identity fingerprint diverges.

**Mitigation:**
1. Full state snapshot validation (TEST-BRIDGE-001): Round-trip 100 snapshots (Gemini JSON ↔ Markenz binary)
2. Dual comparison (TEST-PRESERVATION-001): Gem-D/Gem-K state at checkpoint tick 0 identical to Gemini
3. Determinism gate (TEST-IDENTITY-001): First 100 ticks produce identical decisions
4. Immutable fingerprinting: Identity hash stored in genesis, never mutated

**Stop Condition:** If TEST-IDENTITY-001 fails, STOP Phase 0. Do not proceed to Phase 1 without full identity recovery.

---

### Risk Category 5: Phase Skipping

**Failure Mode:** Windsurf skips Phase N exit criteria, proceeds to Phase N+1.

**Mitigation:**
1. Explicit phase gates in plan (this document)
2. CI enforces: All tests for phase N must pass before phase N+1 code merges
3. Plan review: AMP auditor approves phase completion before next phase starts
4. Code review: Merge blockers on incomplete phases

**Stop Condition:** Any phase gate failure halts progression. Re-planning required before skip is authorized.

---

## 8. WINDSURF EXECUTION CONTRACT (FINAL)

### 8.1 What Windsurf MAY Do

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

### 8.2 What Windsurf May NOT Do

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

### 8.3 Phase Enforcement Rules

**Strict Ordering:**
- Phase 0 must be 100% complete before Phase 1 begins
- Phase 1 must be 100% complete before Phase 2 begins
- And so on through Phase 9

**Gate Enforcement:**
- No phase may skip its exit criteria
- No phase may proceed if previous phase has failed gates
- All phase test suites cumulative (Phase 2 tests include Phase 0+1 tests)

**Explicit Escalation Triggers:**

Windsurf must escalate to AMP auditor (HALT execution) if:

1. **Ambiguity in Plan:** Any requirement unclear or conflicts with existing code → escalate with evidence
2. **Blocker Not Documented:** If blocker not covered in plan → escalate with full context
3. **Determinism Divergence:** TEST-DET-001 fails → escalate with divergence report
4. **Build Failure:** Cargo fails due to undocumented issue → escalate with build log
5. **Asset Corruption:** Asset validation fails → escalate with asset report
6. **Authority Violation:** Code review detects state mutation outside engine → escalate
7. **Test Failure:** Any specified test fails → escalate with test output
8. **Phase Gate Blocked:** Cannot satisfy exit criteria → escalate with blockers
9. **Divergence on Replay:** Snapshot replay differs from full run → escalate with tick
10. **Performance Regression:** Tick rate drops >20% → escalate with profiling

**Escalation Format:**
```
FROM: Windsurf
TO: AMP Auditor
SUBJECT: Escalation — [Category] — [Phase N]
CONTEXT: [What you were trying to do]
BLOCKER: [The specific issue]
EVIDENCE: [Build logs, code snippets, test output, state diffs, etc.]
PROPOSAL: [Your suggested fix, if any]
```

---

### 8.4 Stop Conditions

**Execution STOPS if:**

1. Any Phase Gate Fails: If test or criterion fails → STOP and escalate
2. Determinism Divergence: If replay produces different hashes → STOP immediately
3. Authority Violation Detected: If server or web mutates state → STOP
4. Asset Loss Detected: If Gem-D/Gem-K/House/Shed incomplete → STOP
5. Build Broken: If cargo build fails → STOP until fixed
6. Phase Skipping Attempted: If Windsurf tries to skip gate → STOP
7. AMP Auditor Directs: AMP can halt at any time with written directive

**Upon STOP:**
- Do not proceed to next phase
- Do not commit incomplete work
- Produce full diagnostic report
- Escalate to AMP with evidence
- Wait for re-planning or approval to proceed

---

## 9. FINAL GO / NO-GO CHECKLIST

### 9.1 Phase 0 Must-Pass Criteria (ALL REQUIRED)

**Build & Compilation:**
- [ ] Cargo build succeeds (release + test builds, zero warnings)
- [ ] All unit tests passing (cargo test --all)
- [ ] All integration tests passing
- [ ] Docker compose boots all services, no errors
- [ ] No clippy warnings in critical paths

**Determinism & Replay:**
- [ ] TEST-DET-001 passing: 100 ticks, same seed, identical hashes across 3+ runs
- [ ] TEST-SNAPSHOT-EQ-001 passing: Snapshot replay ≡ full replay (ticks 50, 500, 1000)
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
- [ ] TEST-SCHEMA-001 passing: Database schema correct (input_events, observation_events, snapshots, hash_checkpoints)
- [ ] Database migration succeeds (no data corruption)
- [ ] Hash-checkpoints table created and functional
- [ ] Append-only constraint enforced (no UPDATE/DELETE on immutable tables)

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
- [ ] Inner monologue streamed to observations

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval obtained in writing

**Failure of ANY criterion = Phase 0 NO-GO. Do not proceed to Phase 1.**

---

### 9.2 Phase 0 No-Go Criteria (STOP IF ANY ARE TRUE)

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

### 9.3 Phase 1–9 Checklists (Template)

For each subsequent phase (1–9), the following checklist applies:

**Build & Tests:**
- [ ] Cargo build succeeds for phase-specific crates
- [ ] All phase-specific unit tests passing
- [ ] All phase-specific integration tests passing
- [ ] No clippy warnings

**Determinism:**
- [ ] Phase-specific determinism test passing (TEST-*-001)
- [ ] Replay identical to live run
- [ ] No divergence on multi-run

**Functionality:**
- [ ] Phase deliverables implemented as specified
- [ ] No stubbed code or TODOs in critical paths
- [ ] Feature toggles for older phases still work

**Integration:**
- [ ] Phase builds on prior phase without regression
- [ ] No breaking changes to prior phase APIs
- [ ] Performance acceptable (tick rate maintained)

**Audit:**
- [ ] Phase-specific audit test passing (tools/audits)
- [ ] Anomaly detection runs without alerts
- [ ] Phase replicable via audit script

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval obtained before Phase N+1 starts

---

## 10. FINAL DECISION & BINDING AUTHORITY

**STATUS: BINDING · EXECUTION-READY · ROADMAP-COMPLETE**

This plan is:
- ✅ **Binding:** All sections are law, not suggestions
- ✅ **Mechanically Enforceable:** Every step has success/failure criteria
- ✅ **Phased & Contained:** Risk bounded per phase with hard gates
- ✅ **Roadmap-Integrated:** All 10 phases (0–9) explicitly detailed
- ✅ **Auditable:** Complete traceability to governing law
- ✅ **Fail-Closed:** Any violation halts execution
- ✅ **Directly Executable:** Windsurf can execute without interpretation
- ✅ **Lossless Synthesis:** All unique features/constraints/guarantees from all input documents preserved
- ✅ **Ambiguity-Resolved:** All 5 CRITICAL and 5 HIGH ambiguities from audit explicitly addressed

**Authority:**
- **Plan Owner:** ANTIGRAVITY (AMP)
- **Execution Authority:** Windsurf
- **Approval Authority:** AMP Principal-Level Auditor
- **Escalation Path:** KAIZA-MCP governing law
- **Supersedes:** All prior plans (v1, integration plan v2, reuse migration v3)

**Next Step:** Execute Phase 0 as specified using this plan as the sole authority. Do not reference prior plans (v1, v2, v3) — this document is the sole source of truth.

---

**END OF PLAN**

---

**SIGNATURE & AUTHORITY**

**Synthesized By:** ANTIGRAVITY (AMP / Planner)  
**Timestamp:** 2026-01-10  
**Authority:** KAIZA-MCP v2  
**Status:** BINDING & EXECUTION-READY & ROADMAP-COMPLETE  
**Plan ID:** MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2  
**Predecessors Fully Superseded:** v1, Gemini Integration v2, Reuse Migration v3
