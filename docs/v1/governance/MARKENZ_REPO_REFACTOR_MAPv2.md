# MARKENZ REPO REFACTOR MAP v2
(Gemini Universe → Markenz, Locked Stack, Rust Authority)

STATUS: BINDING  
RULE: Phase-aligned incremental refactor only. No big-bang rewrite permitted.  
PRINCIPLE: Rust engine owns truth from Phase 0 onward.

---

## 1. Core Refactor Doctrine

- **We do not “upgrade” TypeScript into authority.**
- Any existing authoritative logic outside Rust is either:
  1) Rewritten inside `apps/engine` / `crates/*`, or  
  2) Downgraded to non-authoritative input generation or visualization.
- Conceptual hierarchy is preserved:
  **Physics → Biology → Cognition → Society**
- Authority boundaries are enforced structurally by repo layout.

---

## 2. Locked Target Repo Layout (Canonical)

apps/
engine/ # Rust world authority (single-writer)
server/ # Node.js/TypeScript control plane
web/ # React/TypeScript UI + WebGPU renderer

crates/
world/ # Core world state, tick loop, hashing
physics/ # Deterministic physics + constraints
biology/ # Physiology, hormones, immune, reproduction
genetics/ # Genome, recombination, phenotype
cognition/ # Perception, drives, planning, learning, language
events/ # Event schemas, ordering, serialization
persistence/ # Snapshots, replay, hash verification

tools/
audits/ # Python replay audits, anomaly detection, PDF reports

yaml
Copy code

This structure is **non-negotiable**.

---

## 3. Ownership Map (What Lives Where)

### apps/engine (Rust — Authority)
**Moved / Implemented Here**
- Fixed-timestep world loop
- Deterministic scheduler (tick index authoritative)
- Full authority pipeline:
  Perception → Intent → Volition → BioVeto → PhysicsValidate → PolicyValidate → Commit
- Deterministic RNG streams + audit logging
- Canonical world hashing + checkpoints
- Snapshot creation and replay
- Markenz genesis content:
  - Terrain seed
  - House, shed, tools, vehicles
  - Agents: Gem-D, Gem-K

**Explicitly Forbidden**
- Any dependency on wall clock for outcomes
- Any direct DB mutation logic except via persistence crate
- Any UI or server logic

---

### crates/world (Rust)
- Canonical world state types
- Deterministic containers and iteration
- Tick reducers and state transitions
- Hash canonicalization rules
- Snapshot integration hooks

---

### crates/physics (Rust)
- Deterministic movement
- Collision resolution
- Energy and force constraints
- Vehicle mechanics
- No floating-point nondeterminism

---

### crates/biology (Rust)
- Metabolism and homeostasis
- Thermoregulation and circadian rhythm
- Endocrine axes (HPA/HPG)
- Immune response
- Injury and healing
- Nutrition (vitamins/minerals)
- Reproduction stages (pre-genetics)
- BioVeto logic (with stable reasons)

---

### crates/genetics (Rust)
- Double-helix genome representation
- Deterministic recombination
- Bounded mutation (policy controlled)
- Phenotype expression
- Lineage and inheritance tracking

---

### crates/cognition (Rust)
- Perception encoding
- Drives and emotions
- Deterministic planners (GOAP/HTN)
- Skill trees and habit formation
- Memory systems:
  - Episodic
  - Semantic
  - Procedural
- Deterministic language:
  - Grammar templates
  - Lexicon tables
  - Pragmatics rules
- No LLM dependency

---

### crates/events (Rust + shared artifacts)
- Canonical InputEvent schema
- ObservationEvent schema
- Event versioning rules
- Deterministic serialization for hashing and transport

---

### crates/persistence (Rust)
- Snapshot formats
- Replay harness
- Hash equivalence verification
- Read-only DB adapters for replay
- Engine-side truth logic only

---

### apps/server (TypeScript — Control Plane)
**Kept Responsibilities**
- Keycloak OIDC verification (local JWKS)
- RBAC enforcement
- InputEvent schema validation
- Canonical ordering metadata
- Append-only event persistence with hash-chain
- WebSocket fanout of ObservationEvents

**Removed / Prohibited**
- Any authoritative simulation logic
- Any “fix-up” of engine outcomes
- Any hidden state caching that could diverge truth

---

### apps/web (React — Observer / Command Centre)
**Kept Responsibilities**
- Visualization and inspection
- WebGPU renderer (renderer-only)
- Replay viewer
- Time-travel debugger UI
- Diff heatmaps and causality graph
- Admin command forms (InputEvents only)

**Prohibited**
- Any world mutation
- Any bypass of server RBAC
- Any direct DB access

---

### tools/audits (Python)
- Deterministic replay audits
- Hash divergence detection
- Performance and tick stability metrics
- Social/economic anomaly detection
- PDF audit exports

---

## 4. Interface Contracts

### Engine ↔ Server
- Server sends **ordered InputEvents only**
- Engine returns:
  - ObservationEvents
  - State diffs
  - Hash checkpoints
- Engine rejects:
  - Unauthorized events
  - Schema mismatches
  - Version mismatches (deterministically)

### Server ↔ Web
- Web receives:
  - ObservationEvents
  - Snapshot-derived payloads
- Web submits:
  - InputEvent requests only (token-bound)

---

## 5. Data Model Migration Strategy

### Phase 0
- Introduce append-only tables:
  - input_events
  - observation_events
  - snapshots
  - hash_checkpoints

### Phase 1
- Engine emits canonical snapshots and hashes
- tools/audits validate replay equivalence

### Phase 2+
- Expand snapshot payloads:
  - Terrain chunks
  - Entities and inventories
  - Biology and cognition telemetry
- Maintain backward-compatible schema versioning

---

## 6. Incremental Refactor Steps (Phase-Aligned)

- **Phase 0**
  - Stand up locked repo structure
  - Engine boots + ticks + genesis
- **Phase 1**
  - Determinism kernel + replay harness
  - UI panels for ticks, hashes, events
- **Phase 2**
  - Replace abstract rooms/zones with terrain chunks
- **Phase 3**
  - Move BioSys fully into `crates/biology`
- **Phase 4**
  - Move cognition stack into `crates/cognition`
- **Phase 5–9**
  - Expand social, economy, governance, renderer, security
  - Each gated by determinism and audit proof

---

## 7. Explicit Removals and Permanent Prohibitions

- No authoritative logic in `apps/server`
- No physics, biology, cognition mutation in UI
- No “admin patch state” endpoints
- No hidden side channels
- No nondeterministic containers in authority
- No skipping event logs “for convenience”

This refactor map is **binding law** for Markenz.