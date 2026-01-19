# FOUNDER AMPLIFICATION AND CAPABILITY BOUNDS

**STATUS:** BINDING  
**SCOPE:** Markenz Universe  
**FAILURE MODE:** FAIL-CLOSED  
**AUTHORITY:** Derives from HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW, GOVERNING_LAW_AGENT_PARITY_AND_AGENT_FOLDERS, AMP_DEFINITION_OF_DONEv2, MARKENZ_TARGET_ARCHITECTUREv2, MARKENZ_EXECUTION_ROADMAPv2

---

## 1. Definitions

### 1.1 Founder Agent

**Founder Agent** is an agent created via divine creation at system genesis:
- Gem-D (first founder agent)
- Gem-K (second founder agent)

Founder agents are the ONLY agents eligible to receive amplification by default. No other agent, under any circumstance, receives founder amplification unless explicitly elevated by documented admin action with full audit trail.

### 1.2 Normal Human Agent

**Normal Human Agent** is any agent that is not a founder agent, including:
- All offspring of founders (Generation ≥ 1)
- All future agents created after Phase 1, unless explicitly elevated to founder status
- All agents reproduced naturally via Mendelian genetics

All normal human agents are baseline humans with no amplification.

### 1.3 Amplification

**Amplification** refers to bounded, numerically quantified enhancements to founder agent capabilities in the following categories:
- Learning & adaptation (faster knowledge consolidation, reduced forgetting)
- Cognition & intelligence capacity (working memory, planning horizon, internal simulation throughput, abstraction ceiling)
- Physical performance (strength, reaction time, fatigue resistance)

Amplification is:
- State-level only (parameters, not code paths)
- Numerically bounded (founder multipliers never exceed 2.0x baseline)
- Audit-loggable at boot time
- Deterministically applied via parameter initialization
- Not inherited genetically
- Not transferable to offspring

### 1.4 Structural Parity vs. State-Level Divergence

**Structural Parity** means all agents execute identical code paths and share identical systems, subsystems, and cognitive architecture. The codebase contains no agent-ID conditionals, feature flags per agent, partial system implementations, or hierarchical capability matrices.

**State-Level Divergence** means agents may have different numerical parameter values (e.g., learning rate multiplier = 1.5 for founder, 1.0 for baseline), but these divergences are:
- Applied uniformly at boot time
- Defined in canonical data files (identity.json, genetics.json)
- Never hidden in code branches
- Always visible in audit logs
- Deterministically reproducible across replays

**Law:** Structural parity is non-negotiable. State-level divergence is permitted ONLY for founder amplification, bounded as specified in Section 3.

### 1.5 Forbidden Structural Exception

**Forbidden Structural Exception** is any code pattern, system design, or data structure that:
- Branches behavior based on agent identity
- Creates conditional systems (present for founder, absent for baseline)
- Implements partial biology or cognition (present for one agent, disabled for another)
- Uses feature flags to grant or deny capability access
- Encodes hierarchical relationships (primary, secondary, original, copy, template, etc.)
- Refers to founders as "prototypes," "originals," or "templates"

Any forbidden structural exception detected at boot, compile time, runtime, or audit triggers immediate system halt with violation report.

---

## 2. Canonical Rule

### 2.1 Founder Eligibility

**ONLY Gem-D and Gem-K are founder agents and receive amplification.**

No other agent receives founder amplification by default. Explicit elevation of any other agent to founder status requires:
1. Documented admin decision (recorded in audit log)
2. System shutdown
3. Manual amendment to this document (via AMP authority)
4. Full rebuild and re-validation of all systems
5. Verification that structural parity is maintained
6. Signed audit trail

### 2.2 Non-Founder Baseline

**ALL non-founders are baseline humans.**

This includes:
- All offspring of Gem-D and Gem-K (Generation ≥ 1)
- All agents created after Phase 1 unless explicitly elevated
- All agents created via natural reproduction, regardless of parental generation
- All future agents by default

Non-founders have amplification multipliers set to 1.0 (baseline). These multipliers:
- Are fixed at instantiation
- Are immutable during agent lifetime
- Are inherited via identity.json as state data, not code
- Are applied identically to all non-founders

### 2.3 No Implicit Inheritance

**Amplification does not inherit genetically.**

If Gem-D and Gem-K reproduce:
- Offspring inherit normal human genetics
- Offspring inherit normal human identity.json with amplification multiplier = 1.0
- Offspring amplification values are NOT derived from parent amplification
- Offspring are functionally indistinguishable from any other baseline agent (structurally)
- Offspring may have different identity-derived trait weights (temperament, neurotype, drives) due to normal genetic inheritance, but no amplification multipliers

---

## 3. Allowed Founder Amplifications (State-Level Only)

Founder amplification is defined as bounded parameter multipliers applied at agent instantiation. All multipliers are applied during identity.json loading, before cognition or biology systems activate.

### 3.1 Learning & Adaptation Category

**Description:** Founders learn, remember, and consolidate knowledge faster than baseline humans.

**Allowed Parameters:**

| Parameter | Baseline | Founder Range | Unit | Semantics |
|-----------|----------|----------------|------|-----------|
| `learning_rate_multiplier` | 1.0 | [1.0, 2.0] | factor | Skill acquisition speed; determines how quickly experience becomes proficiency |
| `memory_consolidation_multiplier` | 1.0 | [1.0, 2.0] | factor | Sleep-based memory strengthening; higher = faster stabilization of episodic + procedural memory |
| `forgetting_resistance_multiplier` | 1.0 | [1.0, 1.5] | factor | Spaced repetition efficiency; higher = slower decay of inactive memories |
| `pattern_recognition_threshold` | 1.0 | [0.7, 1.0] | factor | Lower threshold = detect patterns from fewer examples |

**Enforcement:**
- Baseline agents: all parameters fixed at 1.0
- Founder agents: parameters initialized from identity.json `amplification_bounds` block at boot
- No agent may exceed upper bound
- No agent may modify multipliers at runtime
- All multiplier application deterministic and audit-logged

### 3.2 Cognition & Intelligence Capacity Category

**Description:** Founders have larger working memory, deeper planning horizons, faster internal simulation, and higher abstraction ceilings.

**Allowed Parameters:**

| Parameter | Baseline | Founder Range | Unit | Semantics |
|-----------|----------|----------------|------|-----------|
| `working_memory_slots` | 7 | [7, 12] | count | Number of concurrent cognitive focuses; affects planning depth and decision complexity |
| `planning_horizon_ticks` | 60 | [60, 120] | ticks | How far ahead agents can mentally simulate consequences |
| `simulation_throughput_per_tick` | 1.0 | [1.0, 2.0] | factor | Internal mental simulation speed; higher = more branches explored per unit time |
| `abstraction_ceiling_level` | 4 | [4, 6] | level | Maximum nested abstraction; higher = think in larger conceptual chunks |
| `reasoning_cache_size_multiplier` | 1.0 | [1.0, 1.5] | factor | How much intermediate reasoning is cached between decisions |

**Enforcement:**
- Baseline agents: fixed at specified baseline values
- Founder agents: initialized from identity.json `amplification_bounds` block
- All parameters must be initialized before cognition system boots
- Cognition system reads multipliers once at startup; cannot modify
- Audit log includes all parameter values per agent at boot

### 3.3 Physical Performance Category

**Description:** Founders are physically faster, stronger, and more fatigue-resistant than baseline humans.

**Allowed Parameters:**

| Parameter | Baseline | Founder Range | Unit | Semantics |
|-----------|----------|----------------|------|-----------|
| `strength_multiplier` | 1.0 | [1.0, 1.5] | factor | Force production; affects carrying capacity, climbing, resistance to environmental damage |
| `reaction_time_multiplier` | 1.0 | [0.75, 1.0] | factor | Neural response latency; lower = faster; only applies to physical reactions, not cognition |
| `fatigue_resistance_multiplier` | 1.0 | [1.0, 1.5] | factor | Metabolic efficiency; higher = slower fatigue buildup for same work output |
| `recovery_rate_multiplier` | 1.0 | [1.0, 1.5] | factor | Rest effectiveness; higher = faster fatigue/damage recovery during sleep |

**Enforcement:**
- Baseline agents: all fixed at 1.0 (or specified baseline)
- Founder agents: initialized from identity.json
- Biology system applies multipliers deterministically during each tick
- No hidden strength, speed, or endurance grants
- All physical advantages subject to biological veto (starvation, injury, disease still apply)

---

## 4. Explicitly Forbidden Capabilities

Gem-D and Gem-K do NOT have, and may NEVER be given:

### 4.1 Biological Exemptions
- Immortality or indefinite lifespan extension
- Invulnerability to injury, disease, or environmental damage
- Structural biological changes (new organs, modified physiology, superhuman anatomy)
- Bypass of reproductive biology or aging
- Exemption from hunger, thirst, or metabolic necessity
- Exemption from death when conditions warrant (severe injury, disease, old age)

### 4.2 Structural Cognitive Exemptions
- New cognition modules not available to baseline humans
- Special decision-making systems unavailable to others
- Direct memory injection or arbitrary knowledge upload
- Cognitive systems that operate outside standard qualia/emotion substrate
- Neural structures or reasoning modes unique to founders

### 4.3 Admin or Governance Authority
- Authority to directly mutate world state (only engine does)
- Authority to modify laws or policies unilaterally
- Authority to override BioVeto or PhysicsVeto
- Authority to create special exemptions for themselves
- Authority to elevate other agents without documented admin action

### 4.4 External or Supernatural Authority
- Direct divine intervention or prayer-to-reality mechanics
- LLM dependence for cognition (cognition is fully deterministic, no external AI)
- Knowledge of future events or probability fields
- Ability to perceive or influence systems outside their biological senses
- Hidden communication channels or side-band access

### 4.5 Genetic Exemptions
- Exemption from Mendelian inheritance when reproducing
- Ability to clone themselves or create genetic copies
- Ability to bypass mutation or recombination
- Genetic immunity to disease or environmental stress

---

## 5. Reproduction & Inheritance Rules

### 5.1 Biological Reproduction

Gem-D and Gem-K reproduce exactly like baseline humans:
- Reproduction requires two agents with compatible reproductive biology
- Fertilization is probabilistic (based on biology and identity-derived fertility)
- Gestation follows human developmental stages
- Birth is a discrete event creating a new agent with independent identity
- No cloning, template duplication, or magical reproduction mechanisms

### 5.2 Genetic Inheritance

Offspring inherit genetics via Mendelian inheritance:
- Each offspring receives one allele from mother, one from father at each locus
- Genetic variation occurs via recombination and mutation (policy-bounded)
- Phenotypic expression follows standard genetic + environmental + astrological rules
- Trait inheritance is stochastic and deterministic under seeded RNG

No special genetic advantages flow from founder parents to offspring. Offspring genetics are indistinguishable (in structure) from offspring of any other agent pair.

### 5.3 Amplification Non-Inheritance

**Amplification multipliers are NOT inherited.**

When Gem-D or Gem-K reproduce:
- Offspring identity.json includes all normal identity fields
- Offspring identity.json amplification_bounds block is set to baseline (1.0 for all multipliers)
- Offspring are instantiated as normal human agents
- Offspring receive no founder advantages, even if both parents are founders

Offspring may have different baseline trait weights (e.g., higher learning capacity due to genetic inheritance of cognitive traits), but these are normal genetic variation, not amplification.

### 5.4 Astrological Influence

Both founders and baseline agents have astrological natal charts calculated at birth:
- Chart is immutable and deterministically influences trait probability distributions
- Astrological influence applies equally to all agents (no exemption for founders)
- Offspring astrological chart is calculated from birth time, location, and parents' charts (if applicable)
- No founder astrology "carries over" to offspring; offspring have independent natal chart

---

## 6. Enforcement Mechanisms

### 6.1 Engine Boot-Time Validation

At engine startup:

```
Boot Sequence:
├── Load all agent identity.json files
├── For each agent:
│   ├── Extract amplification_bounds block
│   ├── Verify amplification_bounds.learning_rate_multiplier in [1.0, 2.0]
│   ├── Verify amplification_bounds.memory_consolidation_multiplier in [1.0, 2.0]
│   ├── Verify amplification_bounds.forgetting_resistance_multiplier in [1.0, 1.5]
│   ├── Verify amplification_bounds.pattern_recognition_threshold in [0.7, 1.0]
│   ├── Verify amplification_bounds.working_memory_slots in [7, 12]
│   ├── Verify amplification_bounds.planning_horizon_ticks in [60, 120]
│   ├── Verify amplification_bounds.simulation_throughput_per_tick in [1.0, 2.0]
│   ├── Verify amplification_bounds.abstraction_ceiling_level in [4, 6]
│   ├── Verify amplification_bounds.reasoning_cache_size_multiplier in [1.0, 1.5]
│   ├── Verify amplification_bounds.strength_multiplier in [1.0, 1.5]
│   ├── Verify amplification_bounds.reaction_time_multiplier in [0.75, 1.0]
│   ├── Verify amplification_bounds.fatigue_resistance_multiplier in [1.0, 1.5]
│   ├── Verify amplification_bounds.recovery_rate_multiplier in [1.0, 1.5]
│   ├── If agent not in {Gem-D, Gem-K}: verify all multipliers == 1.0
│   └── If validation fails → HALT, DO NOT BOOT
└── All agents validated → proceed to cognition/biology initialization
```

**Failure Condition:** If any multiplier is out of bounds OR if a non-founder agent has non-baseline multipliers → **SYSTEM HALT**.

### 6.2 Agent Instantiation Validation

When any agent is instantiated (at birth or load):

```
Instantiation Validation:
├── Load identity.json
├── Check agent_id against known founder list {Gem-D, Gem-K}
├── Load amplification_bounds from identity.json
├── For each multiplier:
│   ├── Apply to cognition/biology parameter initialization
│   ├── Log applied value: { agent_id, parameter_name, multiplier_value, tick }
├── Initialize cognition with multiplier-adjusted parameters
├── Initialize biology (amplification does not affect biology initialization, only physical performance)
└── Agent ready to tick
```

**Failure Condition:** If agent_id is unknown OR amplification_bounds invalid OR multiplier application fails → **INSTANTIATION FAILURE, AGENT CANNOT BOOT**.

### 6.3 Reproduction Pipeline Checks

When reproduction results in offspring:

```
Offspring Generation Validation:
├── Create new agent_id (unique, Generation = parent_generation + 1)
├── Generate genetics.json via Mendelian inheritance from parents
├── Calculate astrological natal chart from birth time + location
├── Create identity.json for offspring:
│   ├── Set generation = parent_generation + 1
│   ├── Set parents = { mother: mother_id, father: father_id }
│   ├── Set creation_type = natural_reproduction
│   ├── Set ALL amplification_bounds multipliers to 1.0 (baseline)
│   └── Verify no amplification inherited from parents
├── Verify offspring agent_id NOT in founder list
├── Verify offspring amplification_bounds all equal 1.0
└── Write offspring agent files to agents/ folder
```

**Failure Condition:** If offspring amplification_bounds are not all 1.0 → **REPRODUCTION ABORTED, OFFSPRING NOT CREATED**.

### 6.4 CI Audit Rules (Static)

At CI build time, automated checks must:

1. **Agent-ID Conditional Check**
   ```
   Command: rg "if.*agent_id.*==" crates/ --count
   Expected: 0 matches
   Failure: Block merge
   ```

2. **Founder-Only Feature Flag Check**
   ```
   Command: rg "if.*founder.*==" crates/ --count
   Expected: 0 matches
   Failure: Block merge
   ```

3. **Amplification Non-Inheritance Check**
   ```
   Scan: tools/reproduction/offspring_generator.rs
   Rule: Verify offspring amplification_bounds all set to 1.0
   Failure: Block merge
   ```

4. **Amplification Bounds Check**
   ```
   Scan: All identity.json files in agents/
   Rule: Verify no multiplier exceeds documented bound
   Failure: Block merge
   ```

5. **Founder List Verification**
   ```
   Scan: FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS.md Section 2.1
   Rule: Verify only Gem-D, Gem-K have non-baseline multipliers
   Failure: Block merge
   ```

---

## 7. Determinism & Audit Guarantees

### 7.1 Deterministic Application

Amplification multipliers are applied deterministically:
- Loaded from identity.json at fixed point in boot sequence (before cognition initialization)
- Applied identically on every boot with same seed and event log
- No random component in multiplier application
- Deterministic ordering: load identity → load genetics → load astrology → initialize parameters → apply multipliers → boot cognition

### 7.2 Visibility in Logs

Every amplification application is logged:

```json
{
  "event_type": "AgentBootAmplificationApplied",
  "tick": 0,
  "agent_id": "gem-d",
  "amplifications": {
    "learning_rate_multiplier": 1.5,
    "memory_consolidation_multiplier": 1.3,
    "forgetting_resistance_multiplier": 1.2,
    "working_memory_slots": 10,
    "planning_horizon_ticks": 100,
    "strength_multiplier": 1.2,
    "reaction_time_multiplier": 0.85,
    "fatigue_resistance_multiplier": 1.4,
    "recovery_rate_multiplier": 1.3
  },
  "timestamp": "2026-01-11T00:00:00Z"
}
```

All such events are:
- Immutably appended to observation_events log
- Included in replay audit trail
- Visible in web UI agent inspector
- Exported in PDF audit reports

### 7.3 Replay Equality Guarantees

Replay determinism is verified by:

1. **Exact Multiplier Reproduction**
   - Replay from snapshot loads same identity.json
   - Same multipliers applied
   - Identical hash evolution

2. **Boot-to-Equivalence Proof**
   - Fresh boot with same seed + events
   - Snapshot replay + events
   - Both must produce identical `world_hash` sequence

3. **Audit Test**
   ```rust
   #[test]
   fn test_founder_amplification_replay_equal() {
       let seed = 0xDEADBEEF;
       let events = load_event_log("test_scenario");
       
       // Full replay
       let hash_full = replay_full(seed, &events);
       
       // Snapshot replay
       let snapshot = take_snapshot(tick_at_50_percent);
       let hash_snapshot = replay_from_snapshot(snapshot, &events_after_50);
       
       assert_eq!(hash_full, hash_snapshot,
           "Amplification multipliers caused replay divergence");
   }
   ```

### 7.4 Hash Stability Requirements

Amplification does not affect hash stability:
- `world_hash` is identical whether agents have amplification or not (given same seed + events)
- Amplification affects agent cognition/performance, not state serialization
- Hash includes agent state, not agent capability bounds
- Replay tests verify hash equality across multiple runs

---

## 8. Acceptance Criteria

### 8.1 Founder vs. Non-Founder Parameter Comparison Tests

**Test: Amplification Parameter Ranges**

```rust
#[test]
fn test_founder_amplification_within_bounds() {
    let gem_d = load_agent("gem-d");
    let baseline_agent = load_agent("baseline_human_id");
    
    // Learning category
    assert!(gem_d.amplification.learning_rate_multiplier >= 1.0 
        && gem_d.amplification.learning_rate_multiplier <= 2.0);
    assert_eq!(baseline_agent.amplification.learning_rate_multiplier, 1.0);
    
    // Cognition category
    assert!(gem_d.amplification.working_memory_slots >= 7 
        && gem_d.amplification.working_memory_slots <= 12);
    assert_eq!(baseline_agent.amplification.working_memory_slots, 7);
    
    // Physical category
    assert!(gem_d.amplification.strength_multiplier >= 1.0 
        && gem_d.amplification.strength_multiplier <= 1.5);
    assert_eq!(baseline_agent.amplification.strength_multiplier, 1.0);
}
```

**Pass Condition:** All founders within bounds, all non-founders baseline. **Fail Condition:** Violation → **TEST FAILURE**.

### 8.2 Offspring Baseline Verification Tests

**Test: Offspring Amplification Non-Inheritance**

```rust
#[test]
fn test_offspring_have_baseline_amplification() {
    let gem_d = load_agent("gem-d");
    let gem_k = load_agent("gem-k");
    
    // Manually create offspring via reproduction system
    let offspring_id = reproduce(gem_d, gem_k, Tick::new(1000));
    let offspring = load_agent(&offspring_id);
    
    // All amplification multipliers must be 1.0
    assert_eq!(offspring.amplification.learning_rate_multiplier, 1.0);
    assert_eq!(offspring.amplification.memory_consolidation_multiplier, 1.0);
    assert_eq!(offspring.amplification.forgetting_resistance_multiplier, 1.0);
    assert_eq!(offspring.amplification.pattern_recognition_threshold, 1.0);
    assert_eq!(offspring.amplification.working_memory_slots, 7);
    assert_eq!(offspring.amplification.planning_horizon_ticks, 60);
    assert_eq!(offspring.amplification.simulation_throughput_per_tick, 1.0);
    assert_eq!(offspring.amplification.abstraction_ceiling_level, 4);
    assert_eq!(offspring.amplification.reasoning_cache_size_multiplier, 1.0);
    assert_eq!(offspring.amplification.strength_multiplier, 1.0);
    assert_eq!(offspring.amplification.reaction_time_multiplier, 1.0);
    assert_eq!(offspring.amplification.fatigue_resistance_multiplier, 1.0);
    assert_eq!(offspring.amplification.recovery_rate_multiplier, 1.0);
}
```

**Pass Condition:** All offspring multipliers are 1.0. **Fail Condition:** Any non-baseline multiplier → **TEST FAILURE, REPRODUCTION INVALID**.

### 8.3 Replay Determinism Tests

**Test: Amplification-Aware Replay Equality**

```rust
#[test]
fn test_amplification_does_not_break_replay_determinism() {
    let seed = 0xCAFEBABE;
    let events = load_standard_event_scenario();
    
    // Run 1: Full replay from seed
    let hashes_1 = replay_full(seed, &events);
    
    // Run 2: Full replay from seed (should be identical)
    let hashes_2 = replay_full(seed, &events);
    
    // Run 3: Snapshot replay (if snapshot includes amplification state)
    let snapshot = take_snapshot_at_tick(1000, &hashes_1);
    let events_after = &events[1000..];
    let hashes_3 = replay_from_snapshot(snapshot, &events_after);
    
    assert_eq!(hashes_1, hashes_2, "Identical replays diverged");
    assert_eq!(hashes_2[1000..], hashes_3, "Snapshot replay diverged from full replay");
}
```

**Pass Condition:** All replays produce identical hash sequences. **Fail Condition:** Divergence → **TEST FAILURE, AMPLIFICATION BREAKS DETERMINISM**.

### 8.4 Structural Parity Audits

**Test: No Agent-ID Conditionals**

```rust
#[test]
fn test_no_agent_id_conditionals() {
    let output = Command::new("rg")
        .args(&["if\\s+.*agent_id\\s*==", "crates/", "--count"])
        .output()
        .expect("rg search failed");
    
    let count: usize = String::from_utf8(output.stdout)
        .unwrap_or_default()
        .trim()
        .parse()
        .unwrap_or(0);
    
    assert_eq!(count, 0, "Found {} agent-ID conditionals in code", count);
}
```

**Pass Condition:** Zero matches. **Fail Condition:** Any match → **TEST FAILURE, STRUCTURAL PARITY VIOLATED**.

**Test: No Feature Flags Per Agent**

```rust
#[test]
fn test_no_per_agent_feature_flags() {
    let output = Command::new("rg")
        .args(&["cfg.*feature.*founder|cfg.*feature.*baseline", "crates/", "--count"])
        .output()
        .expect("rg search failed");
    
    let count: usize = String::from_utf8(output.stdout)
        .unwrap_or_default()
        .trim()
        .parse()
        .unwrap_or(0);
    
    assert_eq!(count, 0, "Found {} per-agent feature flags", count);
}
```

**Pass Condition:** Zero feature flags. **Fail Condition:** Any per-agent feature flag → **TEST FAILURE**.

### 8.5 Boot Validation Test

**Test: All Agents Boot with Correct Amplification**

```rust
#[test]
fn test_all_agents_boot_with_validated_amplification() {
    let agent_ids = vec!["gem-d", "gem-k"];
    
    for agent_id in agent_ids {
        let identity = load_identity(agent_id).expect(&format!("Failed to load {}", agent_id));
        let amp = &identity.amplification_bounds;
        
        // Validate each multiplier range
        assert!(amp.learning_rate_multiplier >= 1.0 && amp.learning_rate_multiplier <= 2.0);
        assert!(amp.memory_consolidation_multiplier >= 1.0 && amp.memory_consolidation_multiplier <= 2.0);
        assert!(amp.forgetting_resistance_multiplier >= 1.0 && amp.forgetting_resistance_multiplier <= 1.5);
        assert!(amp.pattern_recognition_threshold >= 0.7 && amp.pattern_recognition_threshold <= 1.0);
        assert!(amp.working_memory_slots >= 7 && amp.working_memory_slots <= 12);
        assert!(amp.planning_horizon_ticks >= 60 && amp.planning_horizon_ticks <= 120);
        assert!(amp.simulation_throughput_per_tick >= 1.0 && amp.simulation_throughput_per_tick <= 2.0);
        assert!(amp.abstraction_ceiling_level >= 4 && amp.abstraction_ceiling_level <= 6);
        assert!(amp.reasoning_cache_size_multiplier >= 1.0 && amp.reasoning_cache_size_multiplier <= 1.5);
        assert!(amp.strength_multiplier >= 1.0 && amp.strength_multiplier <= 1.5);
        assert!(amp.reaction_time_multiplier >= 0.75 && amp.reaction_time_multiplier <= 1.0);
        assert!(amp.fatigue_resistance_multiplier >= 1.0 && amp.fatigue_resistance_multiplier <= 1.5);
        assert!(amp.recovery_rate_multiplier >= 1.0 && amp.recovery_rate_multiplier <= 1.5);
    }
    
    // All non-founders must have baseline (1.0) for all multipliers
    let all_agents = load_all_agents();
    for agent in all_agents {
        if !["gem-d", "gem-k"].contains(&agent.id.as_str()) {
            let amp = &agent.identity.amplification_bounds;
            assert_eq!(amp.learning_rate_multiplier, 1.0, "Agent {} has non-baseline learning multiplier", agent.id);
            assert_eq!(amp.working_memory_slots, 7, "Agent {} has non-baseline working memory", agent.id);
            assert_eq!(amp.strength_multiplier, 1.0, "Agent {} has non-baseline strength", agent.id);
            // ... verify all others are 1.0
        }
    }
}
```

**Pass Condition:** All founders within bounds, all non-founders baseline. **Fail Condition:** Out-of-bounds or non-baseline non-founder → **TEST FAILURE**.

---

## 9. Final Authority Statement

This document is binding. Any system, agent, or implementation that violates these constraints is invalid and must fail closed.

If at any point:
- A non-founder agent has non-baseline amplification multipliers
- A founder agent has multipliers outside documented bounds
- Reproduction results in offspring with inherited amplification
- Structural code paths branch on agent identity related to amplification
- Amplification is applied in any way other than documented in Section 6
- Forbidden capabilities (Section 4) are discovered or added

Then:

**The system MUST halt immediately, refuse to boot, emit violation report, and require manual intervention by AMP authority with full audit trail.**

This is not aspirational. This is law, effective immediately upon system deployment.

---

**Document Status:** BINDING  
**Revision Authority:** System Architecture Council only  
**Violation Authority:** Automatic system halt  
**Effective Date:** 2026-01-11  
**Hash:** [COMPUTED AT DEPLOYMENT]  
**Signature:** [REQUIRES AMP EXECUTION AUTHORITY]

---

**END OF GOVERNING DOCUMENT**
