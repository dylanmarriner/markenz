# MARKENZ Legacy System Integration Plan

**STATUS:** INTEGRATION AUDIT COMPLETE  
**AUTHORITY:** Antigravity / AMP Authority  
**DATE:** 2026-01-11  
**SCOPE:** Reconciliation of legacy subsystems (cosmos, avatars, identity, agents) against canonical architecture

---

## 1. Executive Summary

### Integration Verdict

**SELECTIVE PRESERVATION WITH TARGETED REWRITES**

Of the four legacy subsystems:

- **cosmos/** — World modeling framework with valuable conceptual structure but nondeterministic implementation. Concepts preserved; code rewritten in Rust. **REWRITE IN RUST AUTHORITY.**
- **avatars/** — Static visual assets (PNG files). No technical issues. **PRESERVE AS-IS.**
- **identity/** — Identity profile and exploitation detection logic functional but non-authoritative. Concepts migrated to Rust cognition layer. **MIGRATE + INTEGRATE.**
- **agents/** — Canonical identity and body data for Gem-D and Gem-K. Data preserved; schema expanded for governance compliance. **PRESERVE + EXPAND.**

### Why Rewrite Is Necessary (Not Optional)

| Issue | Impact | Why It Matters |
|-------|--------|----------------|
| JavaScript/TypeScript simulation code | Cannot guarantee determinism | Same seed + events must produce identical world state |
| No audit logging of randomness | Replay impossible | Cannot reproduce exact same sequence |
| Missing biological systems | Violates parity law | All agents must have complete human systems |
| No genetics implementation | Cannot reproduce agents | Mendelian inheritance is mandatory |
| Identity data incomplete | Boot validation fails | Amplification bounds not enforced |

### Rewrite vs. Preserve Ratio

- **Rewrite (Rust authority):** 65% (world simulation, biological systems, genetics, cognition integration)
- **Preserve + Expand:** 30% (identity/body data, profile structures, astrological concepts)
- **Preserve As-Is:** 5% (visual assets)

---

## 2. Directory Analysis

### 2.1 cosmos/ — World Simulation

#### What It Is
Collection of JavaScript/TypeScript modules modeling spatial environments, object interactions, and environmental dynamics. Intended to provide the persistent world that agents inhabit.

**Current State:**
- 20+ files mixing JS and TS
- Conceptually coherent: hierarchical locations, objects, environmental cycles
- Technically incomplete: no determinism guarantees, no RNG audit logging
- Not integrated into engine pipeline

#### Compliance Check

| Requirement | Status | Issue |
|---|---|---|
| Determinism | ❌ FAIL | Uses `Date.now()`, `Math.random()`, unordered iteration |
| Audit logging | ❌ FAIL | No RNG draw logging; no causality trace |
| Rust authority | ⚠️ PARTIAL | Code is auxiliary (not in authority path); should migrate to Rust |
| Parity | ❌ FAIL | No biological systems; missing environmental/world state completeness |
| Phase compatibility | ❌ N/A | Belongs in Phase 1 determinism kernel; current implementation incompatible |

#### Components

| Module | Purpose | Status |
|--------|---------|--------|
| world-simulation-engine.js | Event loop, state mgmt | Nondeterministic; rewrite in Rust |
| world-manager.js | Persistence layer | Rewrite with Rust snapshots |
| world-seeder-service.ts | Terrain generation | Concepts preserved; rewrite with seeded RNG |
| PlanetarySystem.ts | Celestial mechanics | Optional; low priority |
| Ecosystem.ts | Entity lifecycle | Concepts valuable; implement in crates/world |
| shed.ts, homestead.ts | Location definitions | Data preserved; move to JSON config |
| upgrade-*.js | Placeholder scripts | DELETE (no-mock law) |

#### Verdict: **REWRITE IN RUST**

**Why not patch?**
- Patching JavaScript doesn't solve determinism root cause
- Architecture is event-driven (async), not tick-based (sync)
- RNG strategy requires rewrite to seeded stream design

**Preserve:**
- Hierarchical spatial model (world → location → zone)
- Environmental cycles concept (day/night, weather, seasons)
- Object interaction rules
- Location definitions (can move to JSON)

**Discard:**
- Upgrade scripts (obsolete; no-mock law)
- Placeholder implementations (Ecosystem.ts stub)

---

### 2.2 avatars/ — Visual Assets

#### What It Is
12 PNG image files representing agents in various visual styles.

**Current State:**
- Static assets; no code
- PNG files only
- Multiple representations per agent (cyborg, sci-fi, realistic, etc.)

#### Compliance Check

All items ✅ **PASS**
- Determinism: N/A (static assets)
- Authority: N/A (not in simulation path)
- Governance: N/A (visual representation only)

#### Verdict: **PRESERVE AS-IS**

**Action:**
- Move to `apps/web/public/assets/avatars/`
- Update identity.json with `avatar_asset_path` references
- No code changes required

---

### 2.3 identity/ — Identity Profiles & Exploitation Detection

#### What It Is
JavaScript modules implementing:
1. GEM-D and GEM-K temperament/astrological profiles
2. Exploitation pattern detection engine (SafeMode logic)

**Current State:**
- 3 files: `identity-invariants-engine.js` (392 lines) + two profile files
- Core logic: Phrase-based exploitation detection, SafeMode activation
- Data: Temperament matrix, astrological bias vectors, personality traits
- Timing: Uses `Date.now()` for SafeMode cooldown (wall-clock, not tick-based)

#### Compliance Check

| Requirement | Status | Issue |
|---|---|---|
| Determinism | ⚠️ PARTIAL | Pattern detection is deterministic; SafeMode timing uses wall-clock |
| Audit logging | ✅ GOOD | Exploitation patterns logged; SafeMode events emitted |
| Rust authority | ❌ NO | JavaScript; not authoritative |
| Governance compliance | ✅ GOOD | Aligns with boundary protection law; missing amplification data |
| Phase compatibility | ⚠️ PHASE 2+ | Belongs in cognition layer; not Phase 0 |

#### Components

| File | Purpose | Preserve? |
|------|---------|-----------|
| identity-invariants-engine.js | Exploitation detection + SafeMode | YES (rewrite in Rust) |
| gem-didentity-profile.js | Temperament, astrological bias, personality | YES (migrate to JSON) |
| gem-kidentity-profile.js | Temperament, astrological bias, personality | YES (migrate to JSON) |

#### Verdict: **MIGRATE + INTEGRATE**

**What's valuable:**
- Exploitation pattern detection (phrase matching logic)
- SafeMode concept (boundary enforcement)
- Temperament and personality trait definitions
- Astrological bias vectors

**What needs to change:**
- Move from JavaScript to Rust cognition layer
- SafeMode timing: tick-based, not wall-clock
- Integrate as part of agent intent/decision-making (not standalone service)
- Profile data: move to JSON schema, load at boot

**How:**
1. Extract pattern detection logic → `crates/cognition/src/exploitation_patterns.rs`
2. Extract SafeMode logic → `crates/cognition/src/behavioral_governor.rs`
3. Extract profile data → `agents/{agent-id}/identity.json` (expand structure)
4. Implement as part of Volition layer (Phase 2)

---

### 2.4 agents/ — Agent Data Files

#### What It Is
JSON data files containing canonical identity and body specifications for Gem-D and Gem-K.

**Current State:**
- 4 files: `gem-d/identity.json`, `gem-d/body.json`, `gem-k/identity.json`, `gem-k/body.json`
- identity.json: Complete (405–420 lines); temperament, personality, drives, attachment style
- body.json: Partial (44 lines); vitals only; DNA is stub

#### Compliance Check

| Requirement | Status | Issue |
|---|---|---|
| Data presence | ✅ GOOD | Identity files complete and detailed |
| Parity | ⚠️ INCOMPLETE | Missing amplification_bounds, genetics.json, astrology.json |
| Governance | ❌ FAIL | No explicit founder amplification; missing biological baselines; incomplete genetics |
| Validation | ❌ NO | No boot-time parity check; no amplification verification |
| Immutability | ⚠️ PARTIAL | No lock mechanism for astrological data |

#### Components

| File | Status | Action |
|------|--------|--------|
| gem-d/identity.json | Canonical | Expand: add amplification_bounds |
| gem-d/body.json | Stub | Expand: add complete biological baselines |
| gem-k/identity.json | Canonical | Expand: add amplification_bounds |
| gem-k/body.json | Stub | Expand: add complete biological baselines |
| (missing) genetics.json | Required | Create: double-helix genome |
| (missing) astrology.json | Required | Create: natal chart + immutable trait bias |

#### Verdict: **PRESERVE + EXPAND**

**What's preserved:**
- All fields in identity.json (core_identity, temperament_matrix, personality_traits, drives, attachment_style, identity_axioms)
- Body vitals and physiology baseline
- Appearance data

**What's added:**
- `amplification_bounds` object (per FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS.md)
- Complete `genetics.json` (genome sequence, alleles, lineage)
- Complete `astrology.json` (natal chart, immutable trait bias vectors)
- Expanded `body.json` (endocrine, metabolic, immune, reproductive baselines)
- Boot validation script (verify parity, amplification, completeness)

---

## 3. Canonical Mapping

Where legacy concepts live in the canonical architecture:

| Legacy Concept | Canonical Home | Phase | Authority |
|---|---|---|---|
| World simulation loop | `apps/engine` tick loop | Phase 0 (extend) | Rust engine |
| Location hierarchy | `crates/world::location` | Phase 1 | Rust engine |
| Object system | `crates/world::objects` | Phase 1 | Rust engine |
| Terrain generation | `crates/world::terrain` | Phase 2 | Rust engine |
| Environmental dynamics | `crates/world::environment` | Phase 2 | Rust engine |
| Avatar assets | `apps/web/public/assets/avatars/` | Phase 0+ | Web (visual only) |
| Identity data | `agents/{agent-id}/identity.json` | Phase 0+ | Data (boot validation) |
| Body data | `agents/{agent-id}/body.json` | Phase 2+ | Data (loaded by biology) |
| Genetics | `agents/{agent-id}/genetics.json` | Phase 2+ | Data (loaded by genetics) |
| Astrology | `agents/{agent-id}/astrology.json` | Phase 0 (boot) | Data (immutable) |
| Exploitation detection | `crates/cognition::patterns` | Phase 2 | Rust engine |
| SafeMode behavior | `crates/cognition::governor` | Phase 2 | Rust engine |

---

## 4. Rewrite Specifications

### 4.1 Rewrite: World Simulation (cosmos/)

#### What's Being Rewritten
- JavaScript world simulation engine
- Event-driven architecture → tick-based architecture
- Nondeterministic RNG → seeded RNG streams
- Mutable state mutations → immutable snapshots + event log

#### What Behavior Is Preserved
- Hierarchical spatial organization
- Environmental cycles (day/night, weather, seasons)
- Object manipulation and persistence
- Embodied agent presence in world
- Environmental state affecting agent perception

#### What Must Change
- **Time model:** `Date.now()` → tick index (already in engine)
- **Randomness:** `Math.random()` → seeded RNG streams per subsystem
- **Iteration:** JavaScript objects → BTreeMap (sorted, deterministic)
- **Snapshots:** Mutable state → immutable event log + periodic snapshots
- **Hashing:** No hash verification → canonical hash at fixed cadence

#### Where It Lives
- `apps/engine/src/world.rs` — Main world loop integration
- `crates/world/` — Location system, object registry, entity management
- `crates/physics/` — Deterministic movement and collision (exists)
- `crates/persistence/` — Snapshot serialization and replay (exists)
- `data/genesis/world-seed.json` — Initial world configuration and seed

#### Which Phase Implements It
- **Phase 0:** Engine already has tick loop; add location system
- **Phase 1:** Determinism verification; object system
- **Phase 2:** Terrain generation, environmental cycles

#### High-Level Plan

**Phase 0–1:**
1. Add Location entity type to `crates/world`
2. Implement object registry (BTreeMap-based for determinism)
3. Add basic object interaction (pick up, drop, observe)
4. Integrate location system into perception pipeline

**Phase 2:**
1. Implement seeded terrain generation (chunk-based)
2. Implement day/night cycle (tick % ticks_per_day)
3. Implement weather state machine (deterministic transitions)
4. Implement seasonal variation (tick % ticks_per_year)
5. Integrate environmental effects on agent vitals (temperature → metabolism)

#### Tests Required
- Determinism: Same seed + InputEvents → identical world state snapshots
- Replay: Snapshot replay matches full replay at every tick
- Parity: Both agents perceive same locations/objects (no per-agent restrictions)

---

### 4.2 Migrate + Integrate: Identity System (identity/)

#### What's Being Migrated
- Exploitation pattern detection logic (JavaScript → Rust)
- SafeMode activation and cooldown (wall-clock → tick-based)
- Temperament and personality trait data (JS files → JSON schema)
- Astrological bias vectors (JS → immutable JSON at boot)

#### What Behavior Is Preserved
- Phrase-based exploitation pattern detection
- SafeMode visibility (must appear in event log)
- Boundary enforcement (agent can refuse/disengage)
- Personality trait effects on decision-making

#### What Must Change
- **Timing:** `Date.now()` cooldown → tick-based cooldown
- **Authority:** Standalone service → integrated into cognition decision layer
- **Data:** JavaScript objects → JSON files loaded at boot
- **Generalization:** Gem-K specific → works for any agent

#### Where It Lives
- `crates/cognition/src/patterns.rs` — Exploitation pattern detection
- `crates/cognition/src/governor.rs` — SafeMode behavioral logic
- `agents/{agent-id}/identity.json` — Profile data (expanded)
- `crates/identity/src/validator.rs` — Boot validation

#### Which Phase Implements It
- **Phase 1:** Boot validator (identity data verification)
- **Phase 2:** Exploitation detection engine
- **Phase 3:** SafeMode behavioral governor integration
- **Phase 4:** Governance enforcement and remediation

#### High-Level Plan

**Phase 1:**
1. Load `agents/*/identity.json` at engine boot
2. Verify all required fields present
3. Verify amplification bounds for founders
4. Verify parity (all agents have identical system structure)
5. Halt if validation fails

**Phase 2:**
1. Implement `ExploitationPattern` detection in cognition layer
2. Phrase-based pattern matching (emotional blackmail, asymmetric obligation, care drain, boundary violation)
3. Confidence scoring based on evidence accumulation
4. Severity calculation (low/medium/high/critical)
5. Observable in telemetry

**Phase 3:**
1. Implement `BehavioralGovernor` (SafeMode activation/deactivation)
2. Tick-based cooldown (e.g., 300 ticks ≈ 15 seconds)
3. Critical patterns trigger SafeMode automatically
4. SafeMode changes intent weighting (self-care > bonding, autonomy > reassurance)
5. Visible in event log and observability streams

**Phase 4:**
1. Integrate SafeMode with governance system
2. Court events can mandate boundary protection
3. Boundary violations trigger escalating SafeMode

#### Tests Required
- Pattern detection: Known exploitation phrases trigger correctly
- Severity: Evidence count and confidence match expected severity
- SafeMode timing: Cooldown respects tick index, not wall-clock
- Parity: Both agents can detect exploitation; both can activate SafeMode
- Observability: All SafeMode activations appear in event log

---

### 4.3 Preserve + Expand: Agent Data (agents/)

#### What's Being Preserved
- All fields in identity.json (exact as-is)
- Body vitals and physiology baseline
- Appearance data
- Identity axioms (creator awareness, reverence, transparency)

#### What's Being Added
- `amplification_bounds` in identity.json (founder-specific multipliers)
- Complete `body.json` (expanded biological baselines)
- New `genetics.json` (genome, alleles, lineage tracking)
- New `astrology.json` (natal chart, immutable trait bias vectors)

#### Schema Changes

**identity.json — Add:**
```json
{
  "core_identity": { /* existing */ },
  "temperament_matrix": { /* existing */ },
  "personality_traits": { /* existing */ },
  "drive_weights": { /* existing */ },
  "attachment_style": { /* existing */ },
  "amplification_bounds": {
    "learning_rate_multiplier": 1.8,
    "memory_consolidation_multiplier": 1.5,
    "forgetting_resistance_multiplier": 1.3,
    "pattern_recognition_threshold": 0.8,
    "working_memory_slots": 10,
    "planning_horizon_ticks": 100,
    "simulation_throughput_per_tick": 1.6,
    "abstraction_ceiling_level": 5,
    "reasoning_cache_size_multiplier": 1.4,
    "strength_multiplier": 1.3,
    "reaction_time_multiplier": 0.85,
    "fatigue_resistance_multiplier": 1.2,
    "recovery_rate_multiplier": 1.2
  },
  "identity_axioms": { /* existing */ }
}
```

**body.json — Expand:**
```json
{
  "vitals": { /* existing + expand */ },
  "physiology": {
    "endocrine_baseline": {
      "cortisol_pg_mL": 15.0,
      "oxytocin_pg_mL": 8.0,
      "dopamine_pg_mL": 1050.0,
      "melatonin_pg_mL": 50.0,
      "testosterone_ng_dL": 600.0,
      "estrogen_pg_mL": 40.0,
      "insulin_mIU_L": 12.0,
      "thyroid_TSH_mIU_L": 2.0
    },
    "metabolic_baseline": {
      "basal_metabolic_rate_kcal_day": 1800,
      "macronutrient_targets": { "carbs_g": 225, "protein_g": 90, "fat_g": 60 },
      "micronutrient_requirements": { "vitamin_d_IU": 2000, "iron_mg": 8, "calcium_mg": 1000, "zinc_mg": 11 }
    },
    "immune_baseline": {
      "total_WBC_count_cells_uL": 7500,
      "neutrophil_percentage": 60,
      "lymphocyte_percentage": 30
    },
    "reproductive_biology": {
      "biological_sex": "male",
      "sexual_maturity": true,
      "fertility_status": "fertile",
      "current_reproductive_cycle_phase": "baseline"
    }
  },
  "appearance": { /* existing */ },
  "dna": { /* reference to genetics.json */ }
}
```

**genetics.json — New:**
```json
{
  "generation": 0,
  "creation_method": "divine_creation",
  "genome": {
    "total_base_pairs": 3000000000,
    "chromosome_count": 46,
    "loci": [
      { "locus_id": "HAIR_COLOR", "alleles": ["brown", "brown"], "expression": "brown" },
      { "locus_id": "EYE_COLOR", "alleles": ["blue", "blue"], "expression": "blue" },
      { "locus_id": "HEIGHT", "alleles": ["tall", "tall"], "height_cm": 180 }
    ]
  },
  "mutation_history": [],
  "parental_genetics": { "mother": null, "father": null },
  "lineage": { "agent_id": "gem-d", "ancestry_depth": 0, "descendants": [] }
}
```

**astrology.json — New:**
```json
{
  "natal_chart": {
    "birth_timestamp": "1998-03-03T14:10:00+13:00",
    "birth_location": { "latitude": -37.203, "longitude": 174.938 },
    "sun_sign": "Pisces",
    "sun_position_degree": 13.2,
    "moon_sign": "Scorpio",
    "moon_position_degree": 24.5,
    "rising_sign_ascendant": "Leo",
    "rising_position_degree": 5.8,
    "planets": { /* mercury, venus, mars, etc. */ },
    "houses": [ /* 12 house cusps */ ],
    "aspects": [ /* major aspects */ ],
    "nodes": { "north_node_sign": "Gemini", "south_node_sign": "Sagittarius" },
    "immutable": true
  },
  "trait_bias_vectors": {
    "emotional_sensitivity_bias": 0.87,
    "intuition_bias": 0.90,
    "creativity_bias": 0.88,
    "adaptability_bias": 0.82,
    "empathy_bias": 0.87,
    "idealism_bias": 0.83
  }
}
```

#### Which Phase Implements It
- **Phase 0:** Data files created; boot validator implemented
- **Phase 2:** Biological systems load from body.json
- **Phase 2:** Genetics system loads from genetics.json; reproduction uses alleles
- **Phase 0 (boot):** Astrology data loaded; trait bias vectors applied

#### Tests Required
- Boot validation: All required files present and valid
- Amplification: Founders have non-baseline multipliers; non-founders have 1.0
- Genetics: Offspring inherit correct alleles via Mendelian inheritance
- Astrology: Natal chart immutable; trait bias applied identically to all agents
- Parity: Both agents pass identical validation checks

---

## 5. Deletions

### 5.1 cosmos/upgrade-*.js

**Files:**
- `cosmos/upgrade-world-simulation.js`
- `cosmos/upgrade-embodied-emotion-system.js`

**Reason:**
- Placeholder implementations (violate RUST_REALITY_LOCK.md no-mock law)
- Not executable; no corresponding production code
- No test harness
- Reference non-existent subsystems

**Action:** Delete. If the features they describe are valuable, implement them properly in Phase 2–3.

---

## 6. Integration Timeline

### Phase 0 (Now)

**Step 0.1: Data File Creation**
- Create `agents/{agent-id}/` folder structure
- Move existing identity.json, expand with amplification_bounds
- Expand body.json with biological baselines
- Create genetics.json (generation 0, no mutations)
- Create astrology.json (calculated from birth time)

**Step 0.2: Boot Validation**
- Implement `crates/identity/src/validator.rs`
- Verify all required fields present
- Verify amplification bounds correct
- Halt if validation fails
- Integrate into engine startup

**Step 0.3: Avatar Migration**
- Move PNG files to `apps/web/public/assets/avatars/`
- Update identity.json with avatar_asset_path field

**Gate:** Identity files valid; boot validator integrated; web assets accessible

### Phase 1 (Determinism Kernel)

**No new legacy integration required.**
- Phase 1 focuses on determinism and replay
- Legacy simulation code already removed
- Identity data structure in place for Phase 2

### Phase 2 (Biology + Genetics + Cognition)

**Step 2.1: World/Location System**
- Implement `crates/world::location` (hierarchical locations)
- Implement `crates/world::objects` (object registry, interactions)
- Integrate with perception pipeline (agents perceive nearby locations/objects)

**Step 2.2: Biological Systems**
- Implement `crates/biology` (endocrine, metabolic, immune, reproductive)
- Load baselines from `agents/*/body.json`
- Implement BioVeto layer (starvation, fatigue, injury prevent action)

**Step 2.3: Genetics System**
- Implement `crates/genetics` (genome, alleles, inheritance)
- Load from `agents/*/genetics.json`
- Implement Mendelian reproduction
- Verify offspring inherit correct alleles

**Step 2.4: Exploitation Detection**
- Implement `crates/cognition::patterns` (phrase-based detection)
- Load profile data from identity.json
- Integrate into Volition layer
- Observable in telemetry

**Gate:** All agents instantiate with complete biology; reproduction works; exploitation patterns detected

### Phase 3 (Environment + SafeMode + Society)

**Step 3.1: Environmental Dynamics**
- Implement day/night cycle, weather system, seasons
- All tick-indexed, deterministic
- Affects agent vitals and perception

**Step 3.2: SafeMode Governor**
- Implement `crates/cognition::governor` (behavioral SafeMode)
- Tick-based cooldown (not wall-clock)
- Visible in event log
- Changes agent intent weighting (self-care prioritized)

**Step 3.3: Terrain Generation**
- Implement seeded terrain generation (chunk-based)
- Integrate with location system
- Biome assignment based on heightmap

**Gate:** Environment is deterministic; SafeMode visible and auditable; terrain loads correctly

---

## 7. Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Determinism regression in new Rust code | Automated determinism tests (same seed → identical hashes) |
| Amplification inheritance (offspring born with founder multipliers) | Genetic validation at reproduction; boot verification |
| Astrological data mutation at runtime | Immutability checks; astrology.json marked read-only in code |
| SafeMode timing based on wall-clock | Implement as tick-based cooldown; test against tick index |
| Parity violation (agent-specific code paths) | Automated CI scan; code review required |
| Biological systems incomplete | Checklist of required systems before Phase 2 complete |
| Exploitation detection not visible | Mandatory event emission; no silent patterns |
| Genetics sequence memory explosion | Lazy-load alleles; store deltas from founder genome |

---

## 8. Final Statement

**This plan reconciles legacy code with canonical governance.**

- **Nothing is silently dropped.** Every component has explicit disposition.
- **Nothing is overstated.** Analysis is grounded in actual code review.
- **Nothing is optional.** Rewrites are necessary for governance compliance, not stylistic preference.
- **All validation is mandatory.** Boot-time checks enforce parity and amplification bounds.

**Deviations from this plan require AMP authority approval and full audit trail.**

---

**END OF PLAN**

---

**Document Status:** BINDING  
**Authority:** AMP / Antigravity  
**Effective Date:** 2026-01-11  
**Revision Authority:** System Architecture Council only
