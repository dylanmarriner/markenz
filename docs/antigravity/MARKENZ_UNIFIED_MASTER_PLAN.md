status: APPROVED
plan_id: markenz-master-2026-01-07
title: MARKENZ_UNIFIED_MASTER_PLAN
created_by: Antigravity
created_date: 2026-01-07
---

# MARKENZ — UNIFIED MASTER PLAN (AUTHORITATIVE)

**ONE UNIVERSE. OFFLINE. DETERMINISTIC. EVENT-SOURCED. TRANSPARENT.**  
Markenz is a local-first, fully offline, deterministic 3D voxel universe with embodied agents (gem-d, gem-k initially) that learn, socialize, reproduce, and act autonomously. Multi-user web UI (URL) with WebGPU rendering, god dashboard, and replayable chat.

This plan defines **everything you specified** as binding scope across milestones.

---

## 0) NON-NEGOTIABLE LAWS

### L0. Offline-Only
- No cloud services.
- No API keys.
- No internet required for build/run/test.
- Optional Ollama is **addon only**, never on critical path.

### L1. Single-Universe Authority
- Exactly **one** authoritative universe instance per deployment.
- One sim loop, one event log, one state tree, one timeline.
- Multiple authenticated clients connect concurrently to the same world.

### L2. Deterministic Lockstep + Replay
- State is a pure function of `(seed, genesis config, ordered input events)`.
- Fixed timestep.
- Deterministic ordering for all systems and collections.
- RNG uses explicit streams per subsystem.
- Checkpoint hashing + divergence diagnostics required.

### L3. Zero Stubs / Zero TODO / Zero Mock
- No `TODO`, `FIXME`, `HACK`, placeholder code, or mock world data.
- If it’s mentioned, it’s implemented for the milestone that claims it.

### L4. Transparency
- Every agent perception, goal, plan choice, policy update, belief update, emotion shift, hormone/nutrient update, and utterance is inspectable.
- “WHY” explanations must be produced from logged decision traces.

### L5. Auth + Encryption + Audit Are Mandatory
- OIDC login required (Keycloak default, authentik backup).
- Passkeys/WebAuthn primary method via IdP.
- Encrypted-at-rest storage for: universe DB/event log/snapshots/config secrets/keys/auth DB volumes.
- Every authenticated action is written into the universe event log with identity + role + hash chaining.

---

## 1) EXECUTION GOVERNANCE (KAIZA-MCP)

### Roles
- **AMP / Antigravity**: planners (create APPROVED plans).
- **Windsurf**: executor (writes code only per plan).
- **AMP**: auditor (fail-closed gates).

### Plan Structure Rule
- Execution only allowed from `docs/plans/*.md` with `status: APPROVED`.
- Windsurf must implement **only** the plan referenced by filename.

### Reuse Rule (Gemini Universe)
- Reuse is allowed ONLY if AMP produces a **Reuse Certification Report**:
  - `docs/reports/AMP_REUSE_CERTIFICATION_MARKENZ.md`
- Antigravity then produces:
  - `docs/plans/MARKENZ_REUSE_CERTIFIED_IMPORTS.md`
- Windsurf may reuse code ONLY listed in that plan.

---

## 2) SYSTEM ARCHITECTURE (MANDATORY)

### Language & Runtime
- **Core simulation**: Rust (authoritative).
- **Server**: Rust HTTP + WebSocket (authoritative).
- **Web UI**: TypeScript + React.
- **Renderer**: WebGPU (browser WebGPU API), chunk streaming.

### Deployment
- Local single command start:
  - brings up Rust server + web UI
  - brings up either Keycloak or authentik stack via docker-compose profile
  - enforces encryption unlock
- Localhost bind by default; optional LAN mode with HTTPS.

### Persistence
- **SQLCipher-encrypted SQLite** for:
  - event log (append-only)
  - snapshots
  - checkpoints + hashes
  - simulation metadata (schemas, versions)
- Deterministic replay harness reconstructs identical state hash chain.

---

## 3) AUTHENTICATION / AUTHORIZATION (OIDC-FIRST)

### Dual IdP Selection (NO APP CODE CHANGES)
Deployment-time config switch:
- `AUTH_PROVIDER=keycloak | authentik`

Separate compose profiles:
- `docker compose --profile keycloak up -d`
- `docker compose --profile authentik up -d`

### OIDC Contract (App-Level)
- Authorization Code Flow + PKCE.
- Strict issuer + audience validation.
- Nonce/state validation.
- JWKS local fetch + caching strategy:
  - cache on disk; refresh in background; fail closed if keys unknown.
- Sessions stored server-side (signed cookie session id -> server session store).

### Passkeys/WebAuthn
- Enabled in IdP login flows as primary method.
- App UI provides link to IdP account settings for passkey enrollment/management.
- Password fallback allowed (bootstrap/recovery), configurable.

### Roles
- Dylan: `creator_admin`
- Kirsty: `creator_admin`
- Optional: `observer` (default disabled)

### Enforcement
- All mutating endpoints require `creator_admin`:
  - god actions
  - governance scripts
  - chat injections (god channel)
- Observers can read dashboards/logs and follow entities only.

### Audit Logging
Every authenticated action appended to universe event log:
- `user_id`, `role`, `wall_time`, `sim_time`, `request_id`, `input_hash`, chained hash.

---

## 4) ENCRYPTION AT REST (MANDATORY)

### Universe DB
- SQLCipher encrypted SQLite.
- Key management:
  - Generate master key on first run.
  - Store in OS keychain where available; otherwise encrypted keyfile unlocked via passphrase at server start.
- Key rotation:
  - SQLCipher `rekey` operation + rewrap secrets.
- Encrypted backup/restore:
  - export includes: encrypted db, manifest, signatures, hashes.

### Secrets
- All config secrets are encrypted at rest under the master key policy:
  - OIDC client secrets
  - TLS keys (LAN mode)
  - governance script library (if stored)
  - any local identity fields flagged “local-only”

### Auth DB Volumes (IdP)
- Compose enforces encrypted volume posture where feasible:
  - default: host directory under an “encrypted storage root” (operator-provided) + startup posture check.
- Startup refuses if encryption posture check fails (within realistic container limits).

---

## 5) DETERMINISM DESIGN (HARDCORE)

### Simulation Loop
- Fixed timestep, e.g. 20Hz authoritative.
- Single-threaded sim update (deterministic).
- Rendering and networking are non-authoritative; cannot mutate state.

### Ordering Rules
- All entity iteration uses stable ordering:
  - deterministic entity IDs
  - BTreeMap/BTreeSet for indexed sets
- All input events ordered by server sequence number.

### RNG
- ChaCha-based deterministic RNG.
- Explicit streams:
  - `rng_world`, `rng_agents`, `rng_weather`, `rng_genetics`, etc.
- All randomness recorded as events if user-driven; otherwise derived deterministically from seed + tick + subsystem stream.

### Checkpoint Hashing
- At configurable intervals (e.g. every 100 ticks):
  - compute state hash (blake3 over canonical serialization)
  - store checkpoint hash
- Replay verifier:
  - replays from genesis + events
  - compares hashes at each checkpoint
  - divergence tool outputs first mismatch tick + diff summary.

### Tests
- Golden replay tests.
- Determinism tests across multiple runs on same platform.
- Property tests for genetics/reproduction.
- Fuzz tests for event streams.

---

## 6) WORLD REPRESENTATION (3D VOXEL FROM DAY ONE)

### Voxel Terrain
- Chunked voxel grid (configurable, default 32^3 or 16^3).
- Deterministic procedural generation with seed.
- Editable voxels (dig/build).
- Resource layers: ore strata, soil, rock, groundwater table.
- Biomes: temperature/humidity, seasons, day/night.

### Meshing + Rendering
- Chunk streaming protocol server -> client.
- Client-side meshing for rendering only:
  - culled face meshing (MVP)
  - greedy meshing later
- Collision derived from voxel occupancy (server authoritative).

### Weather System
- Variables: temperature, humidity, wind vectors, precipitation, storms, cloud cover.
- Deterministic evolution.
- Affects:
  - agent comfort
  - crop growth
  - visibility
  - sound propagation (later milestone)

### Fluids (Water)
- Stable deterministic fluid model (cellular / heightfield hybrid).
- Must be complete and non-explosive.

### Objects + Structures
- World starts with:
  - house
  - shed
  - tools
  - vehicles
- Agents can expand via gather/mine/craft/build.

---

## 7) CRAFTING / BUILDING / VEHICLES

### Inventory + Logistics
- Item stacks, containers, storage, hauling.
- Tools with wear/maintenance.
- Multi-stage construction (blueprints + build steps).
- Repair system (tools and vehicles).

### Vehicles
- Deterministic drivetrain + fuel/energy.
- Collisions and movement.
- Vehicle repair.

### Power/Energy (where relevant)
- Fuel consumption, generators (later), basic energy accounting.

---

## 8) AGENTS (gem-d, gem-k) + UNLIMITED ADDABLE

### Identity & Creator Priors
- gem-d and gem-k initial “Adam/Eve” of Markenz.
- Creator/admin identities (Dylan, Kirsty) exist as editable value-system entries + deontic rules:
  - authority priors
  - benevolence priors
- Creator identity fields are **local-only** and never exported unless explicitly requested.

### Embodied Body & Biology (COMPLETE SYSTEM, MILESTONED)
**Final state requirements** (implemented across milestones, never stubbed):
- Anatomy sufficient for locomotion, manipulation, fatigue, injury, pain, sickness, reproduction, thermoregulation.
- Somatic senses:
  - touch/pressure
  - temperature
  - pain
  - proprioception
  - vestibular/balance proxy
  - nausea
  - itch
  - hunger
  - thirst
  - dyspnea proxy
- Metabolism:
  - macronutrients (carb/fat/protein), digestion timelines
  - glycogen/adipose storage
  - BMR + activity expenditure
  - hydration + electrolytes (Na/K/Cl/Mg/Ca) with renal-like regulation proxy
  - vitamins/minerals complete tracked set with deficiency/excess syndromes
  - nutrient composition DB for all foods present (data-driven, no placeholders)
- Circadian rhythm:
  - sleep pressure, chronotype, hormonal cycles, cognitive curve
- Endocrine system:
  - cortisol, catecholamine proxy, dopamine/serotonin proxies
  - oxytocin/vasopressin
  - testosterone/estrogen/progesterone
  - melatonin
  - insulin/glucagon proxies
  - thyroid proxy
- Health:
  - immune proxy, infection/fever, wound healing, pain modulation
  - nutrient deficiency syndromes, fertility health

### Emotions & Affect
- Appraisal-based emotion engine:
  - mood baseline
  - coping strategies
  - rumination
  - affective forecasting
- Emotions modulate:
  - learning rate
  - memory consolidation
  - language tone

### Reproduction
- Consent + social context required.
- Fertility cycles, conception probability.
- Pregnancy timeline + fetal development proxy.
- Childbirth + lactation proxy if applicable.
- Offspring development stages:
  - infant -> child -> adolescent -> adult
  - learning + dependency modeling.

### Genetics (Double Helix)
- Diploid two-strand genome representation.
- Recombination + mutation.
- Genotype -> phenotype mapping:
  - physiology baselines
  - temperament priors
  - disease predispositions
  - fertility
  - sensory thresholds
- Lineage tracking + visualization.

---

## 9) SOCIAL INTELLIGENCE (EMERGENT SOCIETY STACK)

### Social Graph
- relationships, trust, attachment, kinship, reputation, status
- reciprocity ledgers, alliances, grudges

### Theory of Mind
- belief models about others’ beliefs/intentions
- empathy, jealousy, fairness
- guilt/shame/pride
- moral reasoning

### Coordination
- task allocation
- joint intentions
- commitments/promises tracked in event log
- negotiation, conflict resolution, norms formation

### Culture System
- emergent norms/values
- rituals
- taboo system
- institutions as population grows.

---

## 10) LEARNING (OFFLINE, DETERMINISTIC)

### Unified Learning Stack
- procedural skill learning (practice curves + fatigue)
- habit learning (deterministic RL style updates)
- semantic learning (concept formation, causal models)
- episodic memory (salience, reconsolidation, forgetting, trauma toggle)
- curriculum engine (self-generated goals + exploration + long projects)

### Explainability
Every update logs:
- input features
- scores
- selected action
- outcome
- delta to policy/beliefs/memory

---

## 11) LANGUAGE (NO LLM; HIGH-QUALITY ENGLISH)

### Symbolic / NLG Pipeline
- intent -> dialogue act -> content planning -> sentence planning -> surface realization
- feature-based grammar:
  - agreement, tense/aspect, modality, negation, questions, anaphora
- lexicon:
  - morphology, synonyms, register/tone, idioms where appropriate
- discourse manager:
  - turn-taking, topic tracking, grounding, repair, clarification, summarization, narrative recall
- inner monologue uses same pipeline tagged as private thought.

### Optional Ollama Addon (Strict Boundary)
- may propose paraphrases/summaries/plan critiques only.
- cannot influence deterministic state unless explicitly recorded as an external input event with hash.
- must be disableable with identical sim outcomes when disabled.

---

## 12) GOVERNANCE TOOLS (MANDATORY)

### Governance DSL
- deterministic scenario control, triggers, schedules, assertions
- permission-checked and event-sourced
- versioned script library

### Macro Recorder
- record god actions into scripts
- replay identically

### Experiment Runner
- batch runs
- parameter sweeps
- metrics outputs
- comparisons
- offline deterministic

---

## 13) POPULATION SCALING (1000+ AGENTS)

### Requirements
- region streaming + cognition LOD tiers
- deterministic workload scheduling
- load tests + scaling dashboard
- deterministic hashes must still match in scaling runs.

---

## 14) UI / UX (WEB APP + WEBGPU)

### Mandatory UI Modules
- Login via OIDC (Keycloak/authentik swap)
- 3D WebGPU voxel view:
  - free camera + follow agent
  - chunk debug overlay
  - biome/weather overlays
  - streaming status
  - perf HUD
- Entity inspector:
  - searchable state tree (agents/objects/vehicles/voxels)
- Agent “Mind Dashboard”:
  - perceptions
  - goals
  - plan stack
  - action candidates + utilities
  - emotions
  - hormones
  - nutrients
  - memories
  - beliefs
  - social graph
  - WHY explanations (decision trace)
- Event log viewer:
  - filter/search by agent/type/time
  - export
  - diff mode
  - replay scrubber timeline
- God controls (auth-gated):
  - pause/step/fast-forward
  - weather/time/season edits
  - voxel edit/spawn
  - spawn items/resources
  - edit traits/recipes
  - inject events
  - toggle subsystems
  - run scenarios/experiments
- Multi-user presence:
  - connected users + roles
  - show who executed actions

### Chat (Mandatory)
- Real-time chat window:
  - message any agent/group
  - tag “in-world speech” vs “god message”
- Messages are input events, logged, replayable, identity-tagged.
- Multi-client sees consistent chat + state streams.

---

## 15) DATA SCHEMAS (AUTHORITATIVE)

### Event Log Record (SQLCipher)
- `seq: u64` (monotonic)
- `tick: u64`
- `wall_time_utc: i64`
- `actor_user_id: string`
- `actor_roles: string[]`
- `event_type: string`
- `payload: bytes` (canonical serialization)
- `prev_hash: bytes32`
- `hash: bytes32`

### Snapshot Record
- `tick: u64`
- `state_blob: bytes` (canonical)
- `state_hash: bytes32`
- `created_at: i64`

### Checkpoint Record
- `tick: u64`
- `state_hash: bytes32`
- `event_seq: u64`

### Core Entity IDs
- deterministic IDs (incrementing allocator from genesis)
- never random UUIDs inside sim state

### Agent State (minimum; expands later)
- body state (energy, hydration, electrolytes, pain, fatigue, injury)
- endocrine vectors
- nutrient vectors
- emotion state + mood baseline
- drives
- memory stores (episodic/semantic/procedural)
- social graph references
- world model beliefs
- decision trace (last N decisions)

---

## 16) CONTROL LOOPS (MANDATORY)

### Server Loop (authoritative)
1. dequeue events by seq
2. apply to state at tick boundary
3. run sim tick:
   - physics/world update
   - biology update
   - cognition update
   - social update
4. compute checkpoints at interval
5. broadcast diffs/events via WS

### Client Loop (non-authoritative)
- subscribe to snapshots + event stream
- render world (WebGPU)
- show inspectors + panels
- send authenticated input events

---

## 17) MILESTONE PLAN (SHIPPABLE BUILDS WITH TEST GATES)

### M0 — Reuse Audit Gate (Gemini Universe)
**Deliverable**
- `docs/reports/AMP_REUSE_CERTIFICATION_MARKENZ.md`
- `docs/plans/MARKENZ_REUSE_CERTIFIED_IMPORTS.md`

**Acceptance**
- reuse table produced with evidence
- binding reuse rules defined
- fail-closed if anything lacks code evidence

---

### M1 — Foundation Build (YOU DEMANDED THIS AS “FIRST WORKING BUILD”)
**Must ship as fully working, no stubs**
- Rust server: authoritative sim loop + event log + snapshots + checkpoints + replay verify
- Web UI by URL: React + WebGPU voxel render + chunk streaming + overlays
- OIDC login (Keycloak or authentik via config switch)
- Passkeys supported via IdP (primary auth)
- Encrypted-at-rest posture enforced:
  - SQLCipher for universe
  - secrets encrypted at rest
  - IdP DB volume encryption posture checks
- Two agents: gem-d + gem-k
- Transparency panels: state inspector + mind dashboard (v1)
- God controls: pause/step/time/weather/voxel edit (auth-gated)
- Chat: replayable input events + deterministic agent replies (symbolic NLG v1)
- Save/load
- Deterministic replay verification + divergence report

**M1 Acceptance Tests (auditor-gated)**
1. `unauth` cannot access `/api/state` or WS streams (401).
2. `observer` cannot mutate (403 on god endpoints).
3. Both IdPs work:
   - Keycloak profile login works
   - authentik profile login works
   - same app build, config switch only
4. Passkeys:
   - enrollment reachable via IdP
   - login works using passkey
5. Encryption:
   - server refuses start if SQLCipher key not provided/unlocked
6. Replay:
   - run 500 ticks, record hashes, restart, replay -> identical hashes
7. WebGPU:
   - chunk streaming renders visible voxels; overlays display stats
8. Repo scan:
   - no TODO/FIXME/stubs/@ts-nocheck

---

### M2 — World Systems: Mining/Crafting/Building + Tools + Vehicles v1
- gather/mine/craft/build loop complete
- tool wear + repair
- vehicle drivetrain + fuel + repair
- deterministic collision integration

**Acceptance**
- agents autonomously complete a build task chain
- event log shows WHY for each plan selection
- replay hashes match

---

### M3 — Weather + Seasons + Biomes + Crops v1
- weather affects comfort, crops, visibility
- deterministic storm events
- overlays in UI

---

### M4 — Biology v1 (Somatic + Metabolism + Hormones Core)
- continuous somatic signals
- macro metabolism + hydration/electrolytes
- endocrine vectors and coupling to cognition/emotion
- UI graphs + inspector

---

### M5 — Language v2 (Full symbolic pipeline)
- dialogue acts + grammar + discourse manager
- inner monologue pipeline
- lexicon expansion tooling

---

### M6 — Learning v1 (procedural/habit/semantic/episodic)
- explainable updates
- curriculum engine

---

### M7 — Reproduction + Genetics + Development
- diploid genome + recombination + mutation
- fertility/pregnancy/offspring lifecycle
- lineage visualization

---

### M8 — Society v1 (ToM, norms, institutions)
- trust/attachment/reputation
- commitments/contracts in event log
- culture emergence scaffolding

---

### M9 — Scaling to 1000+ Agents
- region streaming
- cognition LOD tiers
- deterministic scheduling
- load tests + perf HUD

---

## 18) REPO STRUCTURE (AUTHORITATIVE)

markenz/
docs/
plans/
MARKENZ_MASTER_PLAN.md # this file (to be created via KAIZA-MCP bootstrap)
MARKENZ_REUSE_CERTIFIED_IMPORTS.md # generated after AMP audit
MARKENZ_M1_FOUNDATION.md # M1 execution authority
reports/
AMP_REUSE_CERTIFICATION_MARKENZ.md
AMP_M1_AUDIT_REPORT.md
infra/
compose.yml # profiles: keycloak/authentik
keycloak/
realm-export.json # offline import
authentik/
bootstrap/ # offline bootstrap artifacts
server/ # Rust authoritative sim + API + WS
web/ # React + WebGPU
tools/
auth_bootstrap/ # CLI to configure IdP + users + roles
keymgr/ # init/unlock/rotate/backup/restore
replay/ # replay verifier + diff tools
audit/ # no-stub scanner + determinism checks
data/
nutrients/ # nutrient db for foods (fully specified by milestone)
recipes/
world_genesis/

yaml
Copy code

---

## 19) SUCCESS CRITERIA (FINAL)

Markenz is “complete” only when:
- fully offline, deterministic, replayable
- multi-user OIDC with passkeys
- encryption at rest enforced
- WebGPU voxel world with chunk streaming
- agents are embodied, biological, social, learning-capable
- reproduction + genetics + lineage
- governance DSL + macro recorder + experiment runner
- 1000+ agents scaling
- total transparency and WHY traces
- no stubs/mocks/TODOs anywhere

---

## 20) NEXT EXECUTABLE STEPS (DO NOT SKIP)

1. **AMP** runs the reuse audit using the AMP prompt you created:
   - Output: `docs/reports/AMP_REUSE_CERTIFICATION_MARKENZ.md`

2. **Antigravity** uses that report to create:
   - `docs/plans/MARKENZ_REUSE_CERTIFIED_IMPORTS.md`
   - `docs/plans/MARKENZ_M1_FOUNDATION.md` (scope-locked to M1)

3. **Windsurf** executes **M1 only** from `MARKENZ_M1_FOUNDATION.md`.

4. **AMP** audits M1 and blocks until all M1 acceptance tests pass.

---

# Approved By
Antigravity (Planner / Authority)  
Date: 2026-01-07





