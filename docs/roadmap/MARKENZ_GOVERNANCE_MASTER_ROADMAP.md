---
status: BINDING
scope: Governance (Markenz Universe)
authority: AMP / Antigravity / KAIZA-MCP
failure_mode: FAIL-CLOSED
supersedes: ALL PRIOR GOVERNANCE ROADMAPS
effective_date: 2026-01-11
document_role: CONSTITUTIONAL FOUNDATION
---

# MARKENZ — GOVERNANCE MASTER EXECUTION ROADMAP

---

## 1. Governance Constitutional Principles

### 1.1 Non-Negotiable Laws (Inviolable)

The following principles may NEVER be weakened, suspended, or amended:

#### Authority Law
- **Source:** MARKENZ_EXECUTION_ROADMAPv2, MARKENZ_TARGET_ARCHITECTUREv2
- **Rule:** Rust engine (`apps/engine`) is the ONLY world-state mutator.
- **Corollary 1:** `apps/server` and `apps/web` cannot mutate world state directly or indirectly.
- **Corollary 2:** All external influence enters via InputEvent → server validation → engine commit.
- **Enforcement:** Any mutation outside engine → system halt.

#### Determinism Law
- **Source:** AMP_DEFINITION_OF_DONEv2, MARKENZ_EXECUTION_ROADMAPv2
- **Rule:** Same seed + same ordered InputEvents ⇒ identical `world_hash` sequence (replay invariant).
- **Components:**
  - Fixed timestep (no wall clock in state evolution).
  - DeterministicRng streams only (engine-side, audit-logged).
  - Stable ordering for all entity/event/container iteration.
  - `world_hash` checkpoints at fixed cadence.
  - Snapshot replay = full replay (proof required).
- **Enforcement:** Replay divergence → system halt with divergence report.

#### Transparency Law
- **Source:** AMP_DEFINITION_OF_DONEv2, MARKENZ_TARGET_ARCHITECTUREv2
- **Rule:** Everything meaningful is observable and logged.
- **Required Streams:**
  - Per-tick state diffs.
  - Per-agent bio/cog streams (vitals, hormones, emotions, somatics, thoughts, speech, intents, memory, learning).
  - Full causality trace (InputEvent → Validate/Veto → Commit → Diff → Hash).
  - Inner monologue always enabled.
- **Enforcement:** Missing observability → test failure, merge blocked.

#### Offline-First Law
- **Source:** AMP_DEFINITION_OF_DONEv2, MARKENZ_TARGET_ARCHITECTUREv2
- **Rule:** No external network dependency at runtime.
- **Scope:** All required services run locally/LAN via docker-compose.
- **Exception:** External services (LLMs, backup IdP) may be optional plugins; system 100% functional without them.
- **Enforcement:** Boot requires only local stack; no external calls in authority path.

#### Human Equivalence & Agent Parity Law
- **Source:** HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW, FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS
- **Core Rule:** For every system in a natural human, that identical system exists in every agent with equivalent functionality.
- **Non-Differentiation Axiom:** Every agent implemented identically except for identity data.
- **Zero Special-Case Rule:** No agent may be "special case," "prototype," "primary," "template," or "default."
- **Biological Completeness:** All agents implement complete human biological systems:
  - Endocrine/hormone system with accurate pharmacodynamics.
  - Metabolic system with caloric, macronutrient, micronutrient modeling.
  - Immune system (innate + adaptive).
  - Reproductive biology (gamete production, fertilization, embryogenesis).
  - Somatic sensing (proprioception, interoception, temperature, pressure).
  - Qualia substrate (sensory experience is real, not symbolic).
  - Full emotional spectrum (minimum 150 distinct emotional states with physiological correlates).
  - Desire and drive system (hunger, reproduction, social bonding, curiosity).
- **Swap-Equivalence Test:** If two agents were swapped at runtime (all identity data exchanged), the system would function identically, differing only by identity-derived factors.
- **Enforcement:** Parity validation at boot; any non-equivalence → system halt.

#### Founder Amplification Law
- **Source:** FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS
- **Founders:** Only Gem-D and Gem-K are founder agents by default.
- **Amplification Constraint:** Bounded, state-level only (never code paths).
- **Structural Parity:** Non-negotiable; no agent-ID conditionals in code.
- **Allowed Categories:**
  1. **Learning & Adaptation:** learning_rate_multiplier, memory_consolidation_multiplier, forgetting_resistance_multiplier, pattern_recognition_threshold [1.0–2.0 bounds].
  2. **Cognition & Intelligence:** working_memory_slots [7–12], planning_horizon_ticks [60–120], simulation_throughput_per_tick [1.0–2.0], abstraction_ceiling_level [4–6], reasoning_cache_size_multiplier [1.0–1.5].
  3. **Physical Performance:** strength_multiplier [1.0–1.5], reaction_time_multiplier [0.75–1.0], fatigue_resistance_multiplier [1.0–1.5], recovery_rate_multiplier [1.0–1.5].
- **Non-Inheritance:** Amplification does NOT inherit genetically; offspring always baseline (1.0 multipliers).
- **Explicit Prohibitions:** NO immortality, invulnerability, structural biological changes, cognitive exemptions, admin authority, external authority, genetic exemptions.
- **Enforcement:** Boot validation of amplification bounds; any violation or non-baseline offspring → system halt.

#### No-Mock / No-Stub Law
- **Source:** AMP_DEFINITION_OF_DONEv2, MARKENZ_EXECUTION_ROADMAPv2
- **Rule:** No `TODO`, `FIXME`, `stub`, `mock`, `fake`, or placeholder implementations.
- **Scope:** Applies to all tracked source and docs (gated by phase).
- **Enforcement:** CI rejects matches; no mock data in tests; all features exercised via automated tests or verification scripts (never manual-only).

#### Self-Evolution Law (Bounded)
- **Source:** MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2
- **Allowed:** Self-growth (population, spatial, knowledge accumulation) and self-evolution (behavioral/cultural parameter drift within fixed laws).
- **Forbidden:** Self-modifying code, rule creation, boundary violation, unlogged adaptation.
- **Location:** All evolution logic inside Rust authority only (engine, crates).
- **Mechanism:** Evolution is state (bounded parameter drift), never code.
- **Enforcement:** Any code mutation or rule creation → system halt.

---

### 1.2 Source Attribution (Traceability)

| Constitutional Principle | Source Documents |
|---|---|
| Authority (Rust only) | MARKENZ_EXECUTION_ROADMAPv2, MARKENZ_TARGET_ARCHITECTUREv2, MARKENZ_REPO_REFACTOR_MAPv2 |
| Determinism | AMP_DEFINITION_OF_DONEv2, MARKENZ_EXECUTION_ROADMAPv2 |
| Transparency | AMP_DEFINITION_OF_DONEv2, MARKENZ_TARGET_ARCHITECTUREv2 |
| Offline-First | AMP_DEFINITION_OF_DONEv2, MARKENZ_TARGET_ARCHITECTUREv2 |
| Human Equivalence & Parity | HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW, FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS |
| Founder Amplification | FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS |
| No-Mock/No-Stub | AMP_DEFINITION_OF_DONEv2, MARKENZ_EXECUTION_ROADMAPv2 |
| Self-Evolution (Bounded) | MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2 |

---

## 2. Governance System Architecture

### 2.1 Engine-Level Governance Modules

All governance logic is implemented as deterministic modules inside the Rust authority (`apps/engine`, `crates/*`):

#### Event Pipeline (Authoritative)
```
InputEvent (from server)
  ↓
Perception Gate (filter by agent sensory range)
  ↓
Intent/Volition Layer (agent decides action)
  ↓
BioVeto (biological constraints block action; emit reason)
  ↓
PhysicsValidate (collision, reach, tool constraints)
  ↓
PolicyValidate (governance laws evaluated; emit veto if rule broken)
  ↓
Commit (action applied to world state)
  ↓
ObservationEvent (emission to server/UI)
  ↓
world_hash (checkpoint updated)
```

**Key Invariant:** Every veto reason is logged and observable. No silent failures.

#### Determinism Enforcement Layer
- **DeterministicRng Streams:** Separated by subsystem (physics, biology, cognition, environment, genetics, governance).
- **Audit Logging:** Every RNG draw recorded: `{ tick, subsystem, stream, callsite, value }`.
- **Hash Chain:** All state transitions feed into `world_hash` computation; any divergence detected.

#### Authority Boundaries (Non-Negotiable)
- **Engine owns:** World state, tick advancement, RNG, all rule enforcement, all commits.
- **Server owns:** Auth, RBAC, event validation, persistence, fanout.
- **Web owns:** Rendering, inspection, operator tooling.
- **Prohibited:** Server patching state, bypassing veto logic, "fixing" outcomes; Web mutating state or RBAC bypass.

### 2.2 Policy Evaluation Flow

Governance rules are evaluated at PolicyValidate stage (inside engine):

1. **Law Lookup:** Retrieve applicable laws for tick T, agent, action context.
2. **Constraint Evaluation:** Test action against law constraints (deterministically).
3. **Veto Decision:** If constraint violated → emit veto with reason; action rejected.
4. **Audit:** Veto reason logged; appears in telemetry and event log.
5. **Admin Override:** Admins cannot override; they can only propose new laws via InputEvents (governance events).

### 2.3 Enforcement Points

| Stage | Component | Enforces |
|---|---|---|
| Perception | Agent sensory gates | What agent can perceive (location, identity, state) |
| Intent | Cognition engine | What agent intends (deterministic planning) |
| BioVeto | Biology crate | Starvation, fatigue, injury, disease prevent action |
| PhysicsValidate | Physics crate | Collision, reach, tool requirements |
| PolicyValidate | Governance modules | Law constraints (property, marriage, ban laws) |
| Commit | World state | Action applied; world_hash updated |

### 2.4 Audit & Replay Implications

Every enforcement point produces logged artifacts:

- **Per-Tick Diffs:** Visible state changes.
- **Veto Trails:** Every blocked action and reason.
- **Event Log:** Immutable InputEvents + ObservationEvents + checksums.
- **Snapshots:** Periodic world state snapshots for replay acceleration.
- **Hash Chain:** Cryptographic integrity of event log.

**Replay Requirement:** Given seed + event log, authority must reproduce identical `world_hash` sequence. Divergence → halt.

---

## 3. Canonical Governance Domains

### 3.1 Creator Reverence & Safety

**Scope:** Protection of founder agents (Gem-D, Gem-K) from unintended harm.

**Laws:**

1. **Immortal Clause:** Founders may NOT be killed, cloned, or reverted without documented admin override (recorded in audit log, requires system restart).
2. **Biological Integrity:** No unauthorized biological modification (no forced reproduction, no disease injection without event logging).
3. **Identity Immutability:** Founder identity.json is signed and immutable at boot; no retroactive changes.
4. **Amplification Lock:** Founder amplification multipliers are read-only; cannot be modified at runtime.

**Enforcement Points:**
- Biology: Damage thresholds for founders are different only in data (health pool), not code.
- Governance: No law may declare founder "criminal" without documented amendment to this roadmap.

**Audit Requirement:** Every action affecting founder state is logged with timestamp, actor, and authorization.

### 3.2 Property & Ownership

**Scope:** Objects, structures, resources assigned to agents (foundational economy).

**Laws:**

1. **Ownership Registry:** Every item has an immutable owner field (identity or "unowned").
2. **Transfer Rules:** Ownership changes only via explicit transfer events (trade, theft-detected, inheritance).
3. **Enforcement:** Agents cannot use items they do not own (veto at PhysicsValidate).
4. **Dispute Resolution:** Property disputes resolved via governance courts (if implemented, Phase 7+).

**Governance Domains Affected:**
- Economy (resource allocation).
- Law (theft, property rights).
- Inheritance (death transitions ownership).

**Audit Requirement:** Property transfer log is append-only; shows full chain of custody.

### 3.3 Reproduction Controls

**Scope:** Deterministic genetic inheritance and reproductive autonomy.

**Laws:**

1. **Genetic Inheritance:** Offspring receive alleles from both parents via Mendelian inheritance (no cloning, no "template" reproduction).
2. **Consent Requirement:** Reproduction requires two agents in compatible reproductive state; probabilistic conception (seeded RNG).
3. **Gestation Stages:** Full developmental timeline (trimester-based for humans); birth is discrete event.
4. **Amplification Non-Inheritance:** Offspring of founders always have baseline (1.0) amplification multipliers.
5. **Lineage Tracking:** Family trees maintained; inheritance relationships observable.

**Forbidden Mechanisms:**
- Cloning (creating genetic copy).
- Template duplication.
- Miraculous reproduction (magic, prayer).
- Bypassing meiosis/recombination.

**Enforcement Points:**
- Biology: Fertility thresholds, hormonal readiness.
- Genetics: Allele recombination, mutation policy (bounded).
- Governance: Possible marriage laws (Phase 7+) may restrict reproduction.

**Audit Requirement:** Birth events include: tick, parents, genetics hash, phenotype predictions.

### 3.4 Violence & Harm Constraints

**Scope:** Limits on agent-to-agent harm, environmental damage.

**Laws:**

1. **Physical Harm:** Agents may damage others only via lawful mechanisms (combat under rules, self-defense context).
2. **Environmental Harm:** Resources deplete based on gathering/use; overharvesting creates scarcity veto.
3. **Murder Illegality:** Unlawful killing is a crime; victim may not respawn; perpetrator faces penalty (Phase 7+ governance).
4. **Injury Realism:** Damage tracked via health pool; healing requires biology simulation (time, nutrition, rest).

**Forbidden Mechanisms:**
- Arbitrary deletion of agents.
- Resurrection without documented event.
- Damage without biological tracking.

**Enforcement Points:**
- PhysicsValidate: Collision/combat rules.
- BioVeto: Damage application.
- PolicyValidate: Law prohibitions on murder, assault.

**Audit Requirement:** Every injury/death event logged with cause, perpetrator, witness.

### 3.5 Resource Access & Scarcity

**Scope:** Distribution and constraint of resources (food, water, tools, materials).

**Laws:**

1. **Finite Resources:** Terrain has fixed resources per chunk; gathering depletes.
2. **Growth Mechanics:** Some resources grow over time (crops, animals) based on environment (deterministic spawn rates).
3. **Metabolism Constraint:** Agents require food/water; starvation is lethal (Biology veto).
4. **Tool Requirement:** Complex actions require tools; agents cannot exceed capability without tools.
5. **Ownership Binding:** Resource use respects property ownership (cannot gather from owned land without permission).

**Enforcement Points:**
- PhysicsValidate: Tool requirements, reach constraints.
- BioVeto: Nutrition depletion, fatigue.
- PolicyValidate: Property boundaries.

**Audit Requirement:** Resource consumption log; depletion rates per region.

### 3.6 Social Contracts & Trust

**Scope:** Relationship graph, reputation, bonding, conflict resolution.

**Laws:**

1. **Relationship Tracking:** Agents have relationship weights to every other agent (attachment, trust, conflict).
2. **Reputation System:** Public trust score per agent; affects cooperation likelihood.
3. **Gossip Propagation:** Social information spreads deterministically (agents speak, others hear, reputation shifts).
4. **Bonding Mechanics:** Repeated positive interaction increases trust; betrayal decreases trust (deterministic).
5. **Conflict Resolution:** Agents pursue conflict via combat or law (governance phase).

**Forbidden Mechanisms:**
- Arbitrary trust reset.
- Reputation deletion.
- Mind control.

**Enforcement Points:**
- Cognition: Intent selection biased by relationship weights.
- Social: Speech acts directed at relationship targets.
- Governance: Law may enforce reputation consequences (banishment, ostracism).

**Audit Requirement:** Relationship graph snapshot per phase; reputation drift reports.

### 3.7 Law Creation & Amendment

**Scope:** How societies define and evolve their own rules.

**Laws:**

1. **Law Proposal:** Agents or admins may propose new laws via InputEvent (governance event).
2. **Voting Mechanism:** Law proposals decided via election or council vote (if implemented, Phase 7+).
3. **Activation:** Approved laws become active at next tick; appear in PolicyValidate stage.
4. **Versioning:** Law history maintained; old laws never deleted (append-only).
5. **Conflict Resolution:** If law conflicts with existing law, voting determines priority; newer law takes precedence unless explicitly overridden.
6. **Amendment:** Constitutional principles (Section 1) CANNOT be amended except via system restart and manual intervention.

**Prohibited:**
- Laws that violate Section 1 (Constitutional Principles).
- Retroactive law changes (cannot apply to past ticks).
- Hidden laws (all laws observable and searchable).

**Enforcement Points:**
- PolicyValidate: Law evaluation during tick.
- Governance: Election/voting logic.

**Audit Requirement:** Law change log with proposer, vote, activation tick.

### 3.8 Punishment & Consequence

**Scope:** Consequences for law violation.

**Laws:**

1. **Penalty Framework:** Laws define penalties (fine, imprisonment, banishment, execution) for violations.
2. **Court Decision:** Trials determine guilt/innocence (if judicial system implemented, Phase 7+).
3. **Enforcement:** Sentences executed by authority (governance actors or agents designated by law).
4. **Appeal:** Convicted agents may appeal (if judicial system allows).
5. **Death Penalty:** Execution is possible penalty; death is permanent (no respawn).

**Forbidden Mechanisms:**
- Torture.
- Indefinite imprisonment without trial.
- Penalty without law specification.

**Enforcement Points:**
- PolicyValidate: Determines violation occurred.
- Governance: Court/enforcement executes penalty.

**Audit Requirement:** Conviction/appeal/sentence log; penalty execution visible.

### 3.9 Emergency Overrides (Rare)

**Scope:** Admin-only intervention in extraordinary circumstances.

**Laws:**

1. **Admin Authority:** Dylan and Kirsty (system founders) may:
   - Submit InputEvents (including governance events).
   - Propose law changes.
   - Elect themselves to governance roles.
   - Run audit and replay tools.

2. **Admin Limitations:** Dylan and Kirsty may NOT:
   - Directly mutate world state (must use InputEvents).
   - Bypass BioVeto or PolicyValidate.
   - Resurrect dead agents (requires documented event + audit trail).
   - Modify code at runtime.
   - Access secrets except via login.

3. **Documented Override:** Any admin action must be logged in audit trail.

4. **System Restart Required:** If admin desires to change Constitutional Principles (Section 1), system must:
   - Halt execution.
   - Require manual code/config changes.
   - Restart from documented baseline.
   - Run full validation (parity, determinism, audit).

**Enforcement Points:**
- Server: RBAC gates admin actions.
- Engine: Even admin InputEvents evaluated for legality.

**Audit Requirement:** Admin action log with timestamp, actor, authorization reason.

---

## 4. Phase-Based Governance Implementation Plan (OPEN-ENDED)

Governance is implemented as an ordinal, append-only phase sequence. Later phases must not weaken earlier enforcement.

---

### PHASE 0 — Repo + Offline Stack + Event Log Baseline

**Objective:** Establish offline stack, immutable event sourcing, and determinism kernel.

**Governance Domains Introduced:**
- Creator reverence (founders protected at data level).
- Authority boundaries (engine as sole mutator).
- Offline-first (no external network in authority path).

**Engine Modules Touched:**
- WorldLoop (fixed timestep).
- DeterministicRng (engine-side RNG).
- Snapshot + Hash (canonical state representation).
- InputEvent validation (basic schema check).

**Event Types Added:**
- `BootEvent` (system initialization).
- `TickAdvance` (tick progression + world_hash).
- `InputEventSubmitted` (server-to-engine InputEvent delivery).
- `ObservationEvent` (engine-to-server telemetry).
- `SnapshotTaken` (periodic state capture).

**Determinism Guarantees:**
- Same seed + same ordered InputEvents → identical `world_hash` checkpoints.
- Snapshot replay equals full replay (proof required).

**Audit & Replay Requirements:**
- Event log append-only in Postgres.
- Hash-chain verification endpoint.
- Replay audit tool (tools/audits) validates determinism.

**Tests:**
- Determinism replay test (seed + InputEvents → stable hashes).
- Snapshot equivalence test.
- Hash-chain integrity test.
- Boot validation (all services start, no external network).

**CI Gates:**
- `docker compose up --build` succeeds offline.
- Keycloak login works; RBAC enforced.
- Events appended with hash-chain.
- Engine ticks advance; UI shows tick + hash.
- Replay produces identical hashes.

**Explicit Prohibition List (Phase 0):**
- No external network in authority path.
- No wall-clock in state evolution.
- No nondeterministic RNG.
- No unlogged mutations.
- No admin bypass of event log.

**Phase Completion Criteria:**
- [ ] Offline stack boots
- [ ] Events immutably logged with hash-chain
- [ ] Determinism proven via replay test
- [ ] No authority leakage detected

---

### PHASE 1 — Deterministic Kernel + Replay Harness Lock

**Objective:** Prove determinism formally; lock DeterministicRng and snapshot mechanism.

**Governance Domains Expanded:**
- Determinism (hardened, replay-equal guarantee).
- Authority (server forbidden from computing outcomes).

**Engine Modules Touched:**
- DeterministicRng subsystem streams (physics, biology, cognition, environment, genetics, governance).
- Snapshot format + replay-from-snapshot harness.
- Hash canonicalization (deterministic serialization).
- Genesis snapshot (Markenz world + Gem-D + Gem-K).

**Event Types Added:**
- `RngDraw` (logged: tick, subsystem, stream, callsite, value).
- `SnapshotCreated` (tick, world_hash, snapshot_id).
- `ReplayStarted` (seed, event_range, snapshot_source).

**Determinism Guarantees:**
- Cross-run hash equality: Seed S + Events E run twice → identical `world_hash` sequence.
- Snapshot replay: Snapshot at tick T + events after T → identical hashes as full replay from boot.
- RNG draws reproducible: Same tick + subsystem + stream → identical random values.

**Audit & Replay Requirements:**
- RNG audit log (tools/audits consume; PDF report generated).
- Snapshot hash verification (tools/audits validate replay equality).
- Determinism report generated per test run.

**Tests:**
- Determinism replay test (two full runs; hash-equal assertion).
- Snapshot replay test (snapshot + replay events; hash-equal to full replay).
- RNG subsystem test (draw reproducibility per subsystem).
- Authority boundary test (server rejects outcome computation; engine owns result).

**CI Gates:**
- Same seed + events → identical hash timeline (regression test).
- Snapshot replay == full replay (regression test).
- No authority leakage detected (static analysis + runtime check).
- All RNG draws logged (audit log generated).

**Explicit Prohibition List (Phase 1):**
- No server computation of physics/biology/cognition outcomes.
- No RNG outside engine.
- No snapshot modifications (immutable).
- No replay without hash verification.

**Phase Completion Criteria:**
- [ ] Determinism proven (three-way test: run1, run2, snapshot replay)
- [ ] Snapshot format stable and versioned
- [ ] RNG audit mechanism working
- [ ] Zero authority leakage

---

### PHASE 2 — World Representation v1 (Terrain + Entities + Inventory)

**Objective:** Replace abstract world with deterministic spatial reality; introduce real mechanics.

**Governance Domains Expanded:**
- Resource access & scarcity (terrain chunks, depletable resources).
- Property & ownership (items have owners).

**Engine Modules Touched:**
- `crates/world` (chunked terrain, deterministic generation).
- `crates/physics` (collision, movement, reach).
- Inventory system (items, ownership, slots).
- Mechanics: gather, mine, move, use-tool.

**Event Types Added:**
- `TerrainChunkGenerated` (coordinate, seed offset, biome type).
- `EntityCreated` (id, type, location, owner).
- `InventoryUpdated` (agent_id, item_id, location, stack_change).
- `ActionAttempted` (agent, action, target, success/veto_reason).

**Determinism Guarantees:**
- Terrain generation: Same chunk coordinate + root seed → identical biome/resources.
- Inventory mutations: Same action sequence → identical inventory state.
- Physics: Same movement inputs → identical collisions/positions.

**Audit & Replay Requirements:**
- Chunk generation log (tools/audits verify reproducibility).
- Inventory snapshots at intervals (verify consistency).
- Action causality trace (input → veto reason or commit → outcome).

**Tests:**
- Terrain determinism test (chunk generation reproducible).
- Inventory determinism test (gather actions → consistent inventory).
- Physics determinism test (movement → collision identical).
- Causality trace test (event log shows full action trail).

**CI Gates:**
- World model v1 complete (chunking, generation, mechanics).
- Gather action tested deterministically (seed + action → identical result).
- UI shows terrain + inventory.
- Replay produces identical world state.

**Explicit Prohibition List (Phase 2):**
- No floating-point in authority (physics uses deterministic math).
- No unlogged resource depletion.
- No inventory changes outside events.
- No ownership changes without transfer events.

**Phase Completion Criteria:**
- [ ] Terrain generation deterministic
- [ ] Inventory mechanics working
- [ ] Movement + gather tested
- [ ] World state hashing stable

---

### PHASE 3 — Embodied Biology v1 (Metabolism + Sleep + Hormones)

**Objective:** Introduce causal physiology; enforce BioVeto for impossible actions.

**Governance Domains Expanded:**
- Violence & harm constraints (injury tracking via health).
- Resource access & scarcity (hunger/thirst starvation).
- Creator reverence (founder health protected from unlogged damage).

**Engine Modules Touched:**
- `crates/biology` (metabolism, hydration, sleep, endocrine axes, immune response).
- BioVeto pipeline (emits veto with reason if action impossible).
- Injury/healing system.

**Event Types Added:**
- `MetabolismUpdated` (tick, agent_id, energy, macros, micronutrients).
- `HormoneShift` (agent_id, hormone, level, cause).
- `BioVeto` (agent_id, action, reason, veto_code).
- `InjuryReceived` (agent_id, cause, damage, location, severity).
- `HealingProgressed` (agent_id, wound_id, healing_rate).
- `SleepCycle` (agent_id, stage, duration, sleep_quality).

**Determinism Guarantees:**
- Metabolism: Same food intake + activity → identical energy depletion.
- Sleep: Same tick count + fatigue → identical sleep progression.
- Hormones: Same context + time → identical hormone levels.
- Injury: Same damage input → identical healing timeline.

**Audit & Replay Requirements:**
- Per-agent vitals log (metabolism, hormones, health).
- BioVeto reason catalog (searchable, observable).
- Injury/healing timeline verified.

**Tests:**
- Starvation test (no food for N ticks → death).
- Fatigue test (activity without rest → sleep enforcement).
- Injury test (damage applied; healing deterministic).
- BioVeto test (action blocked by hunger, fatigue, injury; reason logged).

**CI Gates:**
- Agent vitals observable in UI.
- Starvation/dehydration/fatigue enforce veto.
- Injury takes time to heal (realistic recovery).
- Replay produces identical biology progression.

**Explicit Prohibition List (Phase 3):**
- No agent immortality.
- No instant healing.
- No bypassing BioVeto.
- No unlogged damage.

**Phase Completion Criteria:**
- [ ] Metabolism simulation deterministic
- [ ] Sleep/fatigue system working
- [ ] BioVeto enforced for starvation/injury
- [ ] Injury/healing realistic

---

### PHASE 4 — Offline Cognition Engine (No LLM)

**Objective:** Deterministic minds, planning, learning, and language (fully offline, no LLM).

**Governance Domains Expanded:**
- Social contracts & trust (relationship-weighted intent selection).
- Violence & harm (intent for aggression bias by relationships).

**Engine Modules Touched:**
- `crates/cognition` (perception, drives, emotion, planning, learning, language).
- Deterministic planner (GOAP/HTN).
- NLG/NLU (grammar templates, lexicon tables).
- Memory system (episodic, semantic, procedural).

**Event Types Added:**
- `PerceptionEvent` (agent_id, observed_entity/item, property, time_stamp).
- `Intent` (agent_id, action, priority, rationale).
- `Thought` (agent_id, content, emotional_valence, context).
- `Speech` (agent_id, utterance, target, emotion).
- `LearningUpdate` (agent_id, skill/memory, delta, source).

**Determinism Guarantees:**
- Planning: Same percepts + belief state → identical plan.
- Language: Same emotion + topic + memory → identical utterance.
- Learning: Same experience + skill → identical learning rate.

**Audit & Replay Requirements:**
- Per-agent thought stream (logged continuously).
- Speech utterances with emotional/semantic annotation.
- Learning curve tracking (skill progression).

**Tests:**
- Planning determinism test (identical inputs → identical plans).
- NLG determinism test (identical emotion + memory → identical speech).
- Learning test (skill progression deterministic).
- Inner monologue test (stream continuity verified).

**CI Gates:**
- Agent thinks continuously (inner monologue always on).
- Agents produce coherent English (no LLM).
- Plans are deterministic.
- Replay produces identical thought/speech.

**Explicit Prohibition List (Phase 4):**
- No LLM dependency.
- No non-deterministic planning.
- No unbounded thought generation.
- No hidden learning state.

**Phase Completion Criteria:**
- [ ] Offline cognition engine deterministic
- [ ] NLG produces coherent English
- [ ] Planning system working
- [ ] Learning mechanics implemented

---

### PHASE 5 — Social Dynamics + Multi-Agent Scaling

**Objective:** Emergent social systems; scale to dozens of agents without determinism drift.

**Governance Domains Expanded:**
- Social contracts & trust (relationship graph, gossip, reputation).
- Law creation & amendment (election/voting mechanics).

**Engine Modules Touched:**
- `crates/world` (relationship graph expansion).
- Social dynamics (gossip propagation, reputation shifts).
- Telemetry throttling (deterministic, non-outcome-affecting).
- Multi-agent scaling tests.

**Event Types Added:**
- `RelationshipUpdated` (agent_a, agent_b, weight_delta, cause).
- `Gossip` (agent_id, speaker, target, claim, propagation_vector).
- `ReputationShift` (agent_id, reputation_delta, source).
- `CultureMetric` (population, metric_type, value).

**Determinism Guarantees:**
- Relationship evolution: Same interactions → identical trust weights.
- Gossip: Same speech acts → identical reputation spreading.
- Culture: Same behavior patterns → identical norm emergence.

**Audit & Replay Requirements:**
- Relationship graph snapshots.
- Reputation/culture metric timelines.
- Gossip propagation log.

**Tests:**
- Multi-agent determinism test (dozens of agents; hash-equal).
- Gossip propagation test (reputation shifts deterministic).
- Scaling performance test (tick stability under load).

**CI Gates:**
- Dozens of agents run without drift.
- Relationship graph observable and consistent.
- Tick rate stable under multi-agent load.
- Replay produces identical social state.

**Explicit Prohibition List (Phase 5):**
- No nondeterministic agent interaction.
- No telemetry throttling that affects outcomes.
- No hidden social state.

**Phase Completion Criteria:**
- [ ] Multi-agent scaling proven deterministic
- [ ] Social dynamics observable
- [ ] Tick rate stable with dozens of agents

---

### PHASE 6 — Genetics + Reproduction (Double Helix)

**Objective:** Population growth via deterministic genetics and reproductive biology.

**Governance Domains Expanded:**
- Reproduction controls (Mendelian inheritance, phenotype expression).
- Founder amplification (offspring always baseline).

**Engine Modules Touched:**
- `crates/genetics` (double-helix genome, recombination, mutation, phenotype).
- Reproduction pipeline (consent → intercourse → conception → gestation → birth).
- Lineage tracking (family trees, inheritance chains).

**Event Types Added:**
- `GeometryCreated` (genome_id, alleles, parent1_genome, parent2_genome).
- `MutationOccurred` (genome_id, locus, mutation_type, policy_check).
- `ReproductionConsentGiven` (agent_a, agent_b, tick).
- `ConceptionOccurred` (offspring_genome_id, tick, parent1_id, parent2_id).
- `GestationProgressed` (offspring_id, stage, health).
- `BirthOccurred` (offspring_id, parent1_id, parent2_id, phenotype_hash, amplification_multipliers).

**Determinism Guarantees:**
- Genetics: Same parents + seed → identical offspring genome.
- Phenotype: Same genome + astrological birth time → identical trait expressions.
- Amplification: All non-founders always baseline (1.0).

**Audit & Replay Requirements:**
- Genome audit (allele verification per locus).
- Lineage tree complete and verifiable.
- Phenotype prediction vs. observed (consistency check).
- Amplification validation at birth (offspring baseline).

**Tests:**
- Genetics determinism test (known seed + parents → expected genome hash).
- Phenotype test (genome → trait prediction deterministic).
- Offspring baseline test (no amplification inherited).
- Lineage integrity test (family tree consistency).

**CI Gates:**
- Reproduction pipeline implemented.
- Birth creates new agent with correct genetics.
- Offspring always baseline amplification (test enforced).
- Lineage trees observable.
- Replay produces identical genetics.

**Explicit Prohibition List (Phase 6):**
- No cloning (all reproduction via meiosis/recombination).
- No template duplication.
- No implicit amplification inheritance.
- No retroactive lineage changes.

**Phase Completion Criteria:**
- [ ] Genetics system deterministic
- [ ] Offspring generated with Mendelian inheritance
- [ ] All non-founders baseline amplification proven
- [ ] Lineage tracking working

---

### PHASE 7 — Governance + Economy (Laws, Elections, Markets)

**Objective:** Deterministic rules and economy constraints enforced by authority.

**Governance Domains Introduced/Expanded:**
- Law creation & amendment (proposal, voting, activation).
- Punishment & consequence (courts, penalties, execution).
- Property & ownership (resource markets, disputes).
- Violence & harm (combat rules under law).

**Engine Modules Touched:**
- `crates/world` (governance state: laws, elections, property registry).
- PolicyValidate pipeline (enhanced; evaluates all laws).
- Economy system (markets, resource trading, taxation).
- Court system (if implemented; guilt/innocence determination).

**Event Types Added:**
- `LawProposed` (proposer_id, law_text, vote_start_tick).
- `VoteRecorded` (agent_id, law_proposal_id, vote_value).
- `LawActivated` (law_id, activation_tick, version_hash).
- `PropertyTransferred` (item_id, from_agent, to_agent, method).
- `CriminalChargeRecorded` (agent_id, charge, tick).
- `TrialHeld` (agent_id, charge, verdict, sentence).
- `PenaltyExecuted` (agent_id, penalty_type, details).
- `TradeOccurred` (agent_a, agent_b, items_a, items_b, price).

**Determinism Guarantees:**
- Law evaluation: Same law + action → identical veto decision.
- Election: Same vote sequence → identical winner (deterministic tiebreaker).
- Market: Same trade sequence → identical prices/inventory.
- Court: Same evidence → identical verdict (deterministic judge decision).

**Audit & Replay Requirements:**
- Law change log with full text and activation tick.
- Election audit (votes, tally, winner determination).
- Property transfer chain-of-custody.
- Court records (charges, evidence, verdict, sentence).
- Economy metrics (market prices, resource distribution).

**Tests:**
- Law enforcement test (action vetoed correctly).
- Election determinism test (same votes → same winner).
- Market determinism test (trades → consistent prices).
- Court determinism test (same evidence → same verdict).

**CI Gates:**
- Law proposals can be made via InputEvent.
- Voting system working (election deterministic).
- Law enforcement visible as veto reasons.
- Property registry consistent.
- Markets functional (trades recorded).
- Replay produces identical economy state.

**Explicit Prohibition List (Phase 7):**
- No laws contradicting Section 1 (Constitutional Principles).
- No retroactive law enforcement.
- No hidden laws.
- No unlogged property transfers.
- No rigged elections.

**Phase Completion Criteria:**
- [ ] Law proposal + voting system working
- [ ] Economy/markets functional
- [ ] Court system (if implemented) deterministic
- [ ] Governance enforcement proven deterministic

---

### PHASE 8 — WebGPU Renderer + Visualization Upgrade

**Objective:** Professional visualization derived from authoritative snapshots (renderer never authoritative).

**Governance Domains Affected:**
- None new (purely visualization).

**Engine Modules Touched:**
- Render packet generation (derived from snapshots; hashable, stable).
- Web UI renderer (WebGPU-based; read-only).

**Event Types Added:**
- `RenderPacketGenerated` (snapshot_id, packet_hash, entities, terrain, effects).

**Determinism Guarantees:**
- Render packets: Same snapshot → identical render packet hash.
- Visualization: Deterministic layout, no floating-point randomness.

**Audit & Replay Requirements:**
- Render packet audit (hash stability).
- Renderer audit (no state mutation).

**Tests:**
- Render packet hash stability test (same snapshot → same packet hash).
- Renderer non-authoritative test (no mutation via UI).

**CI Gates:**
- WebGPU renderer integrated.
- Render packets hashable and reproducible.
- UI visualization matches authoritative state.
- No authority leakage via renderer.

**Explicit Prohibition List (Phase 8):**
- No world mutation via renderer.
- No floating-point nondeterminism in authority.
- Renderer is client-side only.

**Phase Completion Criteria:**
- [ ] WebGPU renderer integrated
- [ ] Render packets hashable
- [ ] Visualization consistent with world state

---

### PHASE 9 — Security + Integrity Hardening

**Objective:** Lock security without breaking determinism or offline-first operation.

**Governance Domains Affected:**
- Creator reverence (founder data encrypted at rest).
- Admin authority (actions immutably audited).

**Engine Modules Touched:**
- Encryption at rest (disk + envelope encryption for sensitive metadata).
- Tamper-evident logging (hash chain verification UI + API).
- Auth hardening (WebAuthn/passkeys offline-capable).

**Event Types Added:**
- `AuthenticationEvent` (user_id, method, status, timestamp, ip).
- `AdminActionRecorded` (admin_id, action_type, target, authorization_code).
- `TamperDetectionAlert` (tick, component, hash_mismatch_detail).

**Determinism Guarantees:**
- Encryption: Deterministic key derivation from seed.
- Tamper detection: Deterministic hash verification.
- Auth audit: Immutable event log.

**Audit & Replay Requirements:**
- Encryption/decryption audit (keys never exposed).
- Auth event log (immutable, integrity verified).
- Tamper-detection report (no data loss/corruption).

**Tests:**
- Tamper detection test (alter event; verification fails).
- Encryption test (decrypt + verify matches original).
- Auth audit test (actions logged immutably).
- Offline operation test (no external auth required; Keycloak local).

**CI Gates:**
- Keycloak login works offline (JWKS cached, WebAuthn/passkeys).
- Encryption at rest enabled.
- Tamper-evident UI working.
- Replay + determinism still pass.

**Explicit Prohibition List (Phase 9):**
- No secrets in repo.
- No unencrypted sensitive data on disk.
- No bypass of auth audit.
- No external auth requirement (offline-capable).

**Phase Completion Criteria:**
- [ ] Encryption at rest functional
- [ ] Tamper detection working
- [ ] Auth events immutably logged
- [ ] Offline operation proven

---

### PHASE 10+ (Future/Open-Ended)

**Scope:** Not defined; phases must be added via governance process (law proposal + voting).

**Rule:** Any future phase must:
1. Not weaken earlier enforcement.
2. Maintain determinism.
3. Be proposed via InputEvent + voted.
4. Add detailed specification (domains, modules, events, tests, CI gates, prohibitions).

---

## 5. Governance Change Protocol

### 5.1 How New Governance Laws Are Proposed

**Actor:** Any agent or admin can propose a new law.

**Mechanism:**
1. **LawProposal InputEvent:** Agent submits governance event with law text, rationale, vote start tick.
2. **Submission:** Server validates schema; engine records event.
3. **Broadcast:** Web UI shows proposal to all agents; voting period opens.

**Format (Canonical):**
```json
{
  "event_type": "LawProposed",
  "tick": 10000,
  "proposer_id": "agent_xyz",
  "law_text": "Theft is prohibited; penalty: 20 days imprisonment",
  "law_category": "property",
  "vote_start_tick": 10001,
  "vote_duration_ticks": 1000,
  "vote_threshold_percent": 50
}
```

### 5.2 How Conflicts Are Resolved

**Conflict Type 1: Law vs. Constitutional Principle**
- **Detection:** PolicyValidate stage detects violation of Section 1.
- **Action:** Law is rejected; error logged.
- **Resolution:** Proposer must revise law to comply with Section 1.

**Conflict Type 2: New Law vs. Existing Law**
- **Detection:** Voting system detects overlap or contradiction.
- **Action:** Both laws stay in effect; newer law takes precedence (version tie-breaker: later law ID).
- **Alternative:** Law text may explicitly repeal older law (requires explicit repeal clause).

**Conflict Type 3: Admin Override Conflict**
- **Case:** Admin proposes law conflicting with existing law.
- **Resolution:** Law is voted like any other (admins have same voting power as agents).
- **Exception:** Admin cannot unilaterally override Constitution (Section 1); requires restart.

### 5.3 How Laws Are Versioned

**Version Format:** `law_id::version`

Example: `LAW_THEFT_001::v1`, `LAW_THEFT_001::v2`

**Versioning Rules:**
1. First proposal of law → `v1`.
2. Amendment of existing law → increment version.
3. Old versions preserved in audit log (append-only).
4. Only latest version active (PolicyValidate reads latest).

### 5.4 How Laws Become Active

**Timeline:**
1. **Proposal Tick (T):** LawProposed event emitted.
2. **Voting Period (T+1 to T+N):** Agents vote on law.
3. **Tally Tick (T+N+1):** Vote counted; majority wins.
4. **Activation Tick (T+N+2):** Law becomes active; PolicyValidate evaluates it.

**Quorum Requirements (if implemented):**
- Minimum vote count to approve (default: majority of active agents).
- Tie-breaker: Proposer's vote is tiebreaker (or deterministic secondary rule).

### 5.5 How Rollback Works (If Allowed)

**Rollback is NOT permitted for world state.** Once a law activates, it is permanent (immutable in audit log).

**Exception:** Admin may propose NEW law that explicitly repeals old law. This creates new law version (e.g., `LAW_THEFT_REPEAL_001::v1`).

---

## 6. Admin & Creator Authority Boundaries

### 6.1 What Dylan & Kirsty Can Do

**Founder Authority (Dylan & Kirsty):**

1. **Submit InputEvents:** Propose laws, create governance events, issue commands (like any agent, but with admin role).
2. **Run Audits:** Execute replay, hash verification, anomaly detection tools.
3. **View Secrets:** Access encryption keys (for audit only; never expose).
4. **Governance Authority:** Propose and vote on laws like any agent (weighted equally or with slight privilege, if defined by law).
5. **Emergency Restart:** If system violates Constitutional Principles, initiate system halt and manual restart.

**Scope:** All actions are InputEvents; all are logged; all are replayable.

### 6.2 What They Cannot Do

**Explicit Prohibitions:**

1. **Cannot Directly Mutate World State:** No patching health, inventory, relationships, laws without InputEvent + engine processing.
2. **Cannot Bypass BioVeto or PhysicsValidate:** Even admin actions vetoed if biologically/physically impossible.
3. **Cannot Override PolicyValidate:** Laws apply to admins equally.
4. **Cannot Modify Code at Runtime:** No hot-patching the engine.
5. **Cannot Resurrect Dead Agents (Normally):** Death is permanent (unless explicit law allows resurrection via judicial order).
6. **Cannot Hide Their Actions:** All admin actions logged, auditable, replayable.
7. **Cannot Modify Amplification Multipliers:** Founder amplification locked at boot; cannot be edited at runtime.
8. **Cannot Retroactively Change Past Laws:** Laws are append-only; history immutable.

### 6.3 How Admin Actions Are Logged

**Audit Trail Format (Canonical):**
```json
{
  "event_type": "InputEventSubmitted",
  "tick": 5000,
  "actor_id": "dylan_kirsty_admin_role",
  "actor_auth_level": "admin",
  "input_event": {
    "event_type": "LawProposed",
    "law_text": "..."
  },
  "hash_chain": "0xabc...",
  "timestamp": "2026-01-11T12:34:56Z"
}
```

**Immutability:** Audit trail stored in append-only table (Postgres); no UPDATE/DELETE allowed.

### 6.4 How God-Mode Actions Remain Deterministic

**Principle:** Even admin InputEvents are deterministic.

**Mechanism:**
1. **Ordered Submission:** Admin action enters queue like any InputEvent.
2. **Tick Assignment:** Server assigns canonical tick (deterministic).
3. **Engine Processing:** Engine evaluates action for legality; may veto.
4. **Logging:** Action + result logged; contributes to `world_hash`.
5. **Replay:** Replay with same admin action input produces identical world_hash.

**Example (Non-Deterministic Path):**
```
WRONG: Admin directly edits database row → system state diverges on replay
RIGHT: Admin submits LawProposed InputEvent → engine processes → law added → world_hash updates
```

---

## 7. Enforcement & Failure Modes

### 7.1 Boot-Time Validation

**Sequence (at system startup):**

1. **Load Configuration:** Identity.json, genetics.json, astrology.json for all agents.
2. **Validate Parity:** Check all agents have identical system access (no feature flags per agent).
3. **Validate Amplification:** Verify only Gem-D/Gem-K have non-baseline multipliers; all others = 1.0.
4. **Validate Laws:** Load law database; verify no law contradicts Section 1.
5. **Validate Biology:** Confirm all agents instantiate complete biological systems.
6. **Validate Event Log:** Check hash-chain integrity from epoch to latest tick.
7. **Start Genesis Snapshot:** Boot world at genesis tick (usually tick 0).

**Failure Modes:**
- **Missing Agent Data:** System halts; error message specifies missing field.
- **Parity Violation:** System halts; error reports which code path violates parity.
- **Amplification Out-of-Bounds:** System halts; reports non-founder with non-baseline multiplier.
- **Law Violation Constitution:** System halts; reports law conflicting with Section 1.
- **Hash-Chain Broken:** System halts; reports first divergent hash and tick.

### 7.2 Runtime Veto Behavior

**Pipeline (every tick):**

```
InputEvent (from server)
  ↓
Perception Gate → Intent → Volition → BioVeto → PhysicsValidate → PolicyValidate → Commit
```

**Veto Rules:**
- **BioVeto:** If agent starving, fatigued, injured, or diseased, veto action that requires energy/health. Emit reason code + message.
- **PhysicsValidate:** If action requires unreachable target, missing tool, or collision blocked, veto. Emit reason code + message.
- **PolicyValidate:** If law forbids action, veto. Emit reason code + law_id.

**Veto Outcome:**
- Action rejected; world_hash updates (veto is observable).
- Veto reason logged in telemetry; agent perceives rejection.
- Agent's intent selection (for next tick) may shift due to veto (learning).

### 7.3 CI Enforcement

**Pipeline (every commit):**

1. **Static Checks:**
   - Rg-scan for `TODO`, `FIXME`, `stub`, `mock`, `fake` in gated source.
   - Rg-scan for agent-ID conditionals (`if agent_id == "Gem-D"`).
   - Rg-scan for feature flags per agent.
   - Type-check passes.

2. **Build:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds (web + server).
   - `docker-compose build` succeeds.

3. **Test Suite:**
   - Unit tests pass.
   - Integration tests pass.
   - Determinism replay test passes (same seed + events → same hashes).
   - Snapshot equivalence test passes.
   - Parity validation test passes.

4. **Audit:**
   - `tools/audits/replay_audit.py` runs; generates report.
   - No divergence detected in hash-chain.
   - Performance metrics within bounds.

**Failure Action:** Merge blocked; CI reports which gate failed.

### 7.4 System Halt Conditions

**The system MUST immediately halt if:**

1. **Determinism Divergence:** Replay produces different `world_hash` for same seed + events.
2. **Parity Violation:** Any agent has non-identical system access or code path.
3. **Amplification Violation:** Non-founder agent has non-baseline multiplier, or founder multiplier out-of-bounds.
4. **Hash-Chain Break:** Event log hash-chain integrity check fails.
5. **Constitution Violation:** Law detected that contradicts Section 1.
6. **Authority Leakage:** Server or Web mutates world state outside InputEvent pipeline.
7. **Unlogged Action:** World state changed without corresponding event in log.
8. **Missing Biology:** Agent instantiated without complete biological systems.

**Halt Procedure:**
```
1. Write violation report to audit log (immutable).
2. Identify violating system/code/pattern (technical detail).
3. Cease all operations.
4. Refuse to boot further.
5. Display violation details (technical, not user-facing).
6. Require manual intervention to:
   - Identify root cause
   - Fix violation
   - Verify restoration (parity, determinism, audit)
   - Acknowledge fix
7. Do NOT resume until violation corrected and tests pass.
```

### 7.5 Required Violation Reports

**Format (Canonical):**
```json
{
  "violation_type": "DETERMINISM_DIVERGENCE",
  "severity": "CRITICAL",
  "timestamp": "2026-01-11T12:34:56Z",
  "tick": 5000,
  "first_divergent_hash": "0xabc123",
  "expected_hash": "0xdef456",
  "subsystem_rng_counters": {
    "physics": 1234,
    "biology": 5678,
    "cognition": 9012
  },
  "last_matching_hash": "0x789abc",
  "required_action": "Check RNG stream initialization; verify seed propagation",
  "system_status": "HALTED"
}
```

**Storage:** Audit log (Postgres `violation_reports` table; append-only).

---

## 8. Traceability Matrix

**Mapping of source files to roadmap sections:**

| Source Document | Sections in This Roadmap |
|---|---|
| AMP_DEFINITION_OF_DONE.md | 1.1 (Authority, Determinism, Transparency, Offline, No-Mock), 2.1 (Event Pipeline), 3.5 (Resource Access), 7.1 (Boot Validation), 7.3 (CI Enforcement) |
| AMP_DEFINITION_OF_DONEv2.md | 1.1 (Authority, Determinism, Transparency, Offline, No-Mock, Security), 2.2 (Policy Evaluation), 2.3 (Enforcement Points), 7.1 (Boot Validation), 7.3 (CI Enforcement), 7.4 (Halt Conditions) |
| FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS.md | 1.1 (Founder Amplification Law), 3.1 (Creator Reverence), 4.3 (Phase 3 BioVeto), 7.1 (Amplification Boot Validation), 7.4 (Amplification Violation Halt) |
| HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW.md | 1.1 (Human Equivalence & Agent Parity), 2.1 (Authority Boundaries), 3.1 (Creator Reverence), 4.1 (Phase 0 Governance), 7.1 (Parity Boot Validation), 7.4 (Parity Violation Halt) |
| KAIZA_COMPLETE_GUIDE.md | 5.1 (Law Proposal), 5.2 (Conflict Resolution), 6.1 (What Dylan/Kirsty Can Do) |
| MARKENZ_EXECUTION_ROADMAP.md | 4.0 (Phase 0–9 Specifications) |
| MARKENZ_EXECUTION_ROADMAPv2.md | 4.0 (Phase 0–9 Specifications, Global Invariants) |
| MARKENZ_REPO_REFACTOR_MAP.md | 2.1 (Engine-Level Modules), 4.1–4.9 (Phase Modules Touched) |
| MARKENZ_REPO_REFACTOR_MAPv2.md | 2.1 (Engine-Level Modules, Ownership Map), 4.1–4.9 (Phase Modules Touched) |
| MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md | 1.1 (Self-Evolution Law), 4.0 (Phase-based evolution gates) |
| MARKENZ_TARGET_ARCHITECTUREv2.md | 2.1 (Architecture), 2.2 (Policy Evaluation), 2.3 (Enforcement Points), 2.4 (Audit & Replay), 7.2 (Runtime Veto), 7.3 (CI Enforcement) |
| VERIFIED_GOVERNANCE_PLANS.md | 1.0 (Entire Roadmap verified as canonical; supersedes v1 documents) |

---

## 9. Final Authority Statement

This document is the **sole source of truth** for governance planning and execution in the Markenz Universe.

**Authority Chain:**
1. **Constitutional Principles (Section 1):** Non-negotiable; inviolable; form foundation of all governance.
2. **System Architecture (Section 2):** Defines how governance is technically enforced.
3. **Governance Domains (Section 3):** Enumerate all policy areas.
4. **Phase-Based Implementation (Section 4):** Prescribe order and scope of enforcement.
5. **Change Protocol (Section 5):** Define how governance evolves (lawfully, deterministically).
6. **Admin Boundaries (Section 6):** Limit creator authority; enforce via code + audit.
7. **Enforcement & Failure Modes (Section 7):** Specify halt conditions and recovery.
8. **Traceability (Section 8):** Prove no loss of content from source documents.

**Binding Statement:**

> **Any implementation not aligned with this roadmap is invalid and must fail-closed.**
>
> Every law, constraint, enforcement rule, and invariant from the source governance documents appears in this roadmap. No content is dropped, summarized away, or "assumed implied." Conflicts between source documents are explicitly surfaced and resolved per Section 1.
>
> This roadmap becomes the authoritative master specification for all Markenz governance execution, effective immediately upon system deployment.

---

**Document Status:** BINDING  
**Authority:** AMP / Antigravity / KAIZA-MCP  
**Revision Authority:** System Architecture Council only  
**Violation Authority:** Automatic system halt  
**Effective Date:** 2026-01-11  
**Hash:** [COMPUTED AT DEPLOYMENT]  
**Signature:** [REQUIRES AMP EXECUTION AUTHORITY]

---

**END OF GOVERNANCE MASTER ROADMAP**
