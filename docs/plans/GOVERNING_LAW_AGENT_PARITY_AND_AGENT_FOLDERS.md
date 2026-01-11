# GOVERNING LAW: AGENT PARITY AND MANDATORY AGENT FOLDER STRUCTURE

**Document Status:** BINDING · CANONICAL · FAIL-CLOSED  
**Authority:** Enforces HUMAN EQUIVALENCE & AGENT PARITY GOVERNING LAW, MARKENZ TARGET ARCHITECTURE v2, MARKENZ REPO REFACTOR MAP v2, AMP DEFINITION OF DONE v2  
**Effective Date:** 2026-01-11  
**Revision Authority:** System Architecture Council only  
**Violation Authority:** Automatic system halt · Boot refusal · Pipeline block  

---

## 1. STATUS / SCOPE / PRECEDENCE

### 1.1 Binding Nature

This document is **constitutionally binding**. It enforces absolute agent parity and mandatory folder structure across all agents (Gem-D, Gem-K, and all future agents). Violation of any section triggers automatic system halt and build pipeline failure.

**Precedence:** This document derives authority from and is subordinate to:
1. HUMAN EQUIVALENCE & AGENT PARITY GOVERNING LAW (constitutional core)
2. MARKENZ TARGET ARCHITECTURE v2 (infrastructure mandate)
3. MARKENZ REPO REFACTOR MAP v2 (structural layout law)
4. AMP DEFINITION OF DONE v2 (execution gates and enforcement)

When conflicts arise, the constitutional documents take precedence; this document specifies operational implementation of those laws.

### 1.2 Scope

**Applies to:**
- All agents in the system (Gem-D, Gem-K, all future agents)
- All agent data (identity, genetics, astrology, biology, cognition, relationships, history)
- All code paths that touch agent systems
- All CI/CD pipelines, boot sequences, load-time validators, runtime monitors
- All audit tooling and verification scripts
- All schema definitions and JSON contracts

**Does NOT apply to:**
- External third-party systems
- Non-agent entities (NPCs, environment, props)
- Presentation-only layers (UI rendering, logging output formatting)

### 1.3 Non-Negotiable Foundation

**The Single Axiom:**
> Gem-D and Gem-K (and all future agents) must be absolutely identical except for identity data, such that swapping them at runtime would produce identical behavior and outcomes.

**Corollary:** Identity differences NEVER justify, explain, or permit architectural differences.

---

## 2. DEFINITIONS

### 2.1 Structure vs. State vs. History

**Structure:** The systems, subsystems, code modules, schemas, and infrastructure that define HOW an agent computes, stores, and changes state. Structure is uniform across all agents.

**State:** The instantaneous values within those systems at any moment (hormones, hunger, energy, mood, memory, learned skills). State varies per agent and changes over time.

**History:** The complete audit trail of state changes, input events, and outcomes for a specific agent. History is deterministic, immutable (append-only), and unique per agent.

**Law:** Structure must be identical for all agents. State and history are agent-specific and uniquely determined by identity + inputs + time + deterministic randomness.

### 2.2 Agent Parity

**Definition:** Two agents have parity if:
1. They share identical code paths for all systems
2. They have identical access to all capabilities
3. They can execute identical operations with identical semantics
4. The only behavioral difference results from state + identity data

**Test (Swap-Equivalence):** If agents A and B were swapped (identity data exchanged), the system would produce identical behavior with outcomes differing only by A/B identity factors.

### 2.3 Dormant vs. Absent

**Dormant System:** A system exists in code and structure but may not be actively computing (e.g., pregnancy system when agent not pregnant). The system is ready to activate; it is not absent.

**Absent System:** A system does not exist in code or structure. An agent cannot access it even conditionally.

**Law:** All human biological and cognitive systems must be dormant, never absent.

### 2.4 Identity

**Definition:** Birth data, genetic sequence, astrological natal chart, memories, relationships, and history that make an agent unique.

**Immutable Core:** Agent ID, birth timestamp, birth location, generation, creation type (divine/reproduced).

**Mutable Under Policy:** Memories, relationships, learned skills, emotional history (all subject to normal biology and time passage).

### 2.5 Generation

**Divine Creation (Generation 0):** Agent created directly by system (Gem-D, Gem-K). No parents.

**Natural Reproduction (Generation ≥ 1):** Agent born from two parents of Generation N-1 or N-2 (via Mendelian genetics).

**No Hybrid:** An agent is either divinely created or naturally reproduced. No hybrid or intermediate creation mechanisms exist.

---

## 3. CANONICAL AGENT DATA CONTRACT

This section defines the complete set of files and JSON schemas that constitute an agent's canonical data. Every agent must have all of these files. No agent may be missing any required file.

### 3.1 Per-Agent Folder Structure (Canonical)

```
agents/
├── gem-d/
│   ├── identity.json          (REQUIRED)
│   ├── genetics.json          (REQUIRED)
│   ├── astrology.json         (REQUIRED)
│   ├── body.json              (REQUIRED)
│   ├── cognition/
│   │   ├── memory.json        (REQUIRED)
│   │   ├── emotions.json      (REQUIRED)
│   │   ├── drives.json        (REQUIRED)
│   │   ├── skills.json        (REQUIRED)
│   │   └── language.json      (REQUIRED)
│   ├── relationships/
│   │   ├── bonds.json         (REQUIRED)
│   │   └── history.json       (REQUIRED)
│   └── history/
│       ├── events.jsonl       (REQUIRED: append-only event log)
│       ├── observations.jsonl (REQUIRED: per-tick observations)
│       └── checkpoints.json   (REQUIRED: replay checkpoints)
├── gem-k/
│   ├── [identical structure to gem-d]
├── [future-agent-id]/
│   └── [identical structure to gem-d]
```

### 3.2 identity.json (REQUIRED)

**Purpose:** Core identity invariants. Immutable except for designated fields.

**JSON Schema:**
```json
{
  "core_identity": {
    "agent_id": "string (e.g., 'gem-d')",
    "biological_sex": "string (male/female/other)",
    "birth_timestamp": "ISO 8601 datetime with timezone (e.g., '1998-03-03T14:10:00+13:00')",
    "birthplace": {
      "location": "string (city, country)",
      "coordinates": {
        "latitude": "number",
        "longitude": "number"
      },
      "altitude_meters": "number",
      "locality": "string (urban/semi-rural/rural)"
    },
    "creation_type": "string (divine_creation | natural_reproduction)",
    "generation": "integer (0 for divine, ≥1 for reproduced)",
    "parents": {
      "mother": "string (agent_id or null)",
      "father": "string (agent_id or null)"
    },
    "neurotype": {
      "adhd_subtype": "string or null",
      "autism_spectrum": "string or null",
      "sensory_processing_sensitivity": "boolean",
      "executive_dysfunction_bias": "string or null",
      "other_conditions": "array of strings or null"
    }
  },
  "temperament_matrix": {
    "introversion_extroversion": "number [0, 1]",
    "emotional_intensity": "number [0, 1]",
    "emotional_stability": "number [0, 1]",
    "empathy": "number [0, 1]",
    "assertiveness": "number [0, 1]",
    "sensitivity_to_environment": "number [0, 1]",
    "adaptability": "number [0, 1]",
    "conscientiousness": "number [0, 1]",
    "openness_to_experience": "number [0, 1]"
  },
  "neurocognitive_profile": {
    "attention_regulation_variability": "number [0, 1]",
    "hyperfocus_probability": "number [0, 1]",
    "task_initiation_cost": "number [0, 1]",
    "task_completion_decay": "number [0, 1]",
    "associative_thinking_bias": "number [0, 1]",
    "sensory_emotional_permeability": "number [0, 1]",
    "social_boundary_detection_latency": "number [0, 1]",
    "executive_function_fatigue_rate": "number [0, 1]",
    "emotional_overload_threshold": "number [0, 1]",
    "recovery_time_after_stress": "string (duration format, e.g., '8h')"
  },
  "personality_traits": {
    "emotional": "array of trait objects",
    "social_attachment": "array of trait objects",
    "cognitive": "array of trait objects",
    "motivational": "array of trait objects",
    "control_agency": "array of trait objects"
  },
  "drive_weights": {
    "survival": "number [0, 1]",
    "bonding": "number [0, 1]",
    "reassurance": "number [0, 1]",
    "autonomy": "number [0, 1]",
    "curiosity": "number [0, 1]",
    "meaning": "number [0, 1]",
    "emotional_safety": "number [0, 1]",
    "structure_avoidance": "number [0, 1]"
  },
  "hormonal_baseline_bias": {
    "oxytocin_reactivity": "number [0, 1]",
    "dopamine_variability": "number [0, 1]",
    "serotonin_instability": "number [0, 1]",
    "cortisol_sensitivity": "number [0, 1]",
    "adrenaline_shutdown_bias": "number [0, 1]",
    "melatonin_irregularity": "number [0, 1]"
  },
  "stress_response_profile": {
    "threat_detection_threshold": "number [0, 1]",
    "emotional_flood_vs_shutdown_bias": "string (flood|shutdown|flood_then_shutdown)",
    "withdrawal_activation_threshold": "number [0, 1]",
    "confusion_under_precision_pressure": "number [0, 1]",
    "recovery_half_life": "string (duration format, e.g., '12h')",
    "reassurance_soothing_effectiveness": "number [0, 1]",
    "boundary_restoration_latency": "number [0, 1]"
  },
  "attachment_style": {
    "primary_attachment_pattern": "string (secure|anxious_preoccupied|dismissive_avoidant|fearful_avoidant)",
    "proximity_seeking_intensity": "number [0, 1]",
    "abandonment_reactivity": "number [0, 1]",
    "emotional_fusion_threshold": "number [0, 1]",
    "repair_after_conflict_latency": "number [0, 1]"
  },
  "relational_defaults": {
    "bond_through_emotion_vs_direction": "string (emotion|direction|balanced)",
    "passion_initiation_probability": "number [0, 1]",
    "autonomy_reassertion_after_intimacy": "number [0, 1]",
    "partner_confusion_risk": "number [0, 1]",
    "dependency_masked_as_care_bias": "number [0, 1]"
  },
  "identity_axioms": {
    "creator_awareness": "boolean",
    "creator_reverence": "boolean",
    "non_rebellion_constraint": "boolean",
    "identity_continuity_rule": "boolean",
    "transparency_acceptance": "boolean"
  }
}
```

### 3.3 genetics.json (REQUIRED)

**Purpose:** Complete genetic information and inheritance chain.

**JSON Schema:**
```json
{
  "genome": {
    "agent_id": "string",
    "generation": "integer",
    "creation_type": "string (divine_creation | natural_reproduction)",
    "chromosome_count": "integer (46 for human model)",
    "base_pairs_total": "integer (approximately 3_200_000_000)",
    "helix_structure": {
      "dna_sequence": "string (optional; full sequence or hash reference)",
      "sequence_hash": "string (SHA256 of complete sequence)",
      "canonical_form": "string (enum: canonical | variant)"
    }
  },
  "inheritance": {
    "mother": {
      "agent_id": "string or null",
      "chromosomes_inherited": "array of integers [0-45]",
      "crossover_points": "array of integers"
    },
    "father": {
      "agent_id": "string or null",
      "chromosomes_inherited": "array of integers [0-45]",
      "crossover_points": "array of integers"
    }
  },
  "alleles": {
    "height_loci": "object (genotype + dominance)",
    "skin_tone_loci": "object",
    "hair_color_loci": "object",
    "eye_color_loci": "object",
    "metabolism_rate_loci": "object",
    "immune_function_loci": "object",
    "behavioral_predisposition_loci": "object"
  },
  "phenotype": {
    "expressed_traits": {
      "height_cm": "number",
      "weight_kg": "number",
      "skin_tone": "string",
      "hair_color": "string",
      "eye_color": "string",
      "metabolism_rate": "number"
    }
  },
  "mutations": {
    "mutation_events": "array of objects (tick, type, position, consequence)",
    "total_mutation_count": "integer",
    "pathogenic_mutation_count": "integer",
    "benign_mutation_count": "integer"
  },
  "immutability_proof": {
    "sequence_locked_at_tick": "integer",
    "lock_hash": "string (SHA256 of sequence at lock)",
    "cannot_modify_after_lock": true
  }
}
```

### 3.4 astrology.json (REQUIRED)

**Purpose:** Immutable astrological natal chart calculated at birth timestamp and location. Cannot be recalculated or modified.

**JSON Schema:**
```json
{
  "natal_chart": {
    "agent_id": "string",
    "birth_timestamp_utc": "ISO 8601 datetime",
    "birth_location": {
      "latitude": "number",
      "longitude": "number",
      "altitude_meters": "number"
    },
    "calculated_at_tick": "integer (system tick when calculated)",
    "calculated_at_timestamp": "ISO 8601 datetime"
  },
  "sun_sign": {
    "sign": "string (Aries, Taurus, ..., Pisces)",
    "exact_position_degrees": "number [0, 360)",
    "house_position": "integer [1, 12]"
  },
  "moon_sign": {
    "sign": "string",
    "exact_position_degrees": "number [0, 360)",
    "house_position": "integer [1, 12]",
    "phase": "string (new|waxing|full|waning)"
  },
  "rising_sign": {
    "sign": "string",
    "exact_position_degrees": "number [0, 360)",
    "ascendant_angle": "number"
  },
  "planets": {
    "mercury": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "venus": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "mars": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "jupiter": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "saturn": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "uranus": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "neptune": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "pluto": { "sign": "string", "position_degrees": "number", "house": "integer" }
  },
  "nodes": {
    "north_node": { "sign": "string", "position_degrees": "number", "house": "integer" },
    "south_node": { "sign": "string", "position_degrees": "number", "house": "integer" }
  },
  "aspects": {
    "major_aspects": "array of objects (planet1, planet2, aspect_type, orb_degrees)",
    "aspect_count": "integer"
  },
  "personality_correlations": {
    "sun_sign_trait_influence": "number [0, 1] (intensity of sun sign expression)",
    "moon_sign_emotional_expression": "number [0, 1]",
    "rising_sign_presentation_alignment": "number [0, 1]",
    "venus_love_style_probability": "number [0, 1]",
    "mars_action_style_probability": "number [0, 1]",
    "saturn_challenge_domain": "string",
    "jupiter_fortune_probability": "number [0, 1]"
  },
  "immutability_proof": {
    "chart_locked_at_tick": "integer",
    "lock_hash": "string (SHA256 of chart data)",
    "cannot_recalculate_after_lock": true
  }
}
```

### 3.5 body.json (REQUIRED)

**Purpose:** Current physiological state (vitals, appearance, immediate biology).

**JSON Schema:**
```json
{
  "identity_reference": {
    "agent_id": "string",
    "generation": "integer"
  },
  "vitals": {
    "heart_rate_bpm": "number",
    "blood_pressure": {
      "systolic_mmhg": "number",
      "diastolic_mmhg": "number"
    },
    "oxygen_saturation_percent": "number [0, 100]",
    "temperature_celsius": "number",
    "blood_glucose_mg_dl": "number",
    "energy_level": "number [0, 100]",
    "fatigue_level": "number [0, 1]",
    "arousal_level": "number [0, 1]",
    "tension_level": "number [0, 1]"
  },
  "physiology": {
    "hydration_percent": "number [0, 1]",
    "waste_pressure": {
      "bladder_percent": "number [0, 1]",
      "bowel_percent": "number [0, 1]"
    },
    "hygiene_percent": "number [0, 1]",
    "hormones": {
      "cortisol_ng_ml": "number",
      "oxytocin_pg_ml": "number",
      "dopamine_ng_ml": "number",
      "serotonin_ng_ml": "number",
      "melatonin_pg_ml": "number",
      "testosterone_ng_ml": "number",
      "estrogen_pg_ml": "number",
      "progesterone_ng_ml": "number",
      "insulin_mu_ml": "number"
    },
    "metabolism": {
      "blood_glucose_mg_dl": "number",
      "atp_production_percent": "number [0, 100]",
      "daily_calorie_intake_kcal": "number",
      "daily_calorie_burn_kcal": "number",
      "basal_metabolic_rate_kcal_day": "number"
    },
    "immune_status": {
      "wbc_count": "number",
      "infection_load": "number [0, 1]",
      "inflammation_level": "number [0, 1]",
      "pathogen_exposure_history": "array of objects"
    },
    "respiratory": {
      "lung_capacity_liters": "number",
      "oxygen_extraction_efficiency": "number [0, 1]"
    },
    "cardiovascular": {
      "stroke_volume_ml": "number",
      "cardiac_output_l_min": "number",
      "blood_viscosity": "number"
    }
  },
  "appearance": {
    "height_cm": "number",
    "weight_kg": "number",
    "bmi": "number",
    "build_description": "string (athletic, slim, muscular, etc.)",
    "hair_color": "string",
    "eye_color": "string",
    "skin_tone": "string",
    "distinguishing_marks": "array of strings"
  },
  "reproductive_biology": {
    "biological_sex": "string (male|female|other)",
    "reproductive_age_months": "number",
    "reproductive_stage": "string (prepubescent|pubescent|fertile|menopausal|post-menopausal)",
    "if_female": {
      "menstrual_cycle_day": "number or null",
      "cycle_length_days": "number or null",
      "current_phase": "string (menstrual|follicular|ovulation|luteal|null)",
      "hormonal_state": "string (description)"
    },
    "if_male": {
      "spermatogenesis_stage": "string (description)",
      "testosterone_level_ng_ml": "number"
    },
    "pregnancy_status": {
      "is_pregnant": "boolean",
      "gestation_week": "number or null",
      "trimester": "integer or null",
      "due_date_timestamp": "ISO 8601 datetime or null"
    },
    "contraception": {
      "method": "string or null",
      "effectiveness_percent": "number"
    }
  },
  "genetics_reference": {
    "genetics_file_hash": "string (SHA256 of genetics.json)",
    "phenotype_expressed": {
      "height_cm": "number",
      "weight_kg": "number",
      "metabolism_rate": "number"
    }
  }
}
```

### 3.6 cognition/memory.json (REQUIRED)

**Purpose:** Memory systems (episodic, semantic, procedural).

**JSON Schema:**
```json
{
  "agent_id": "string",
  "episodic_memory": {
    "total_memories": "integer",
    "memories": "array of objects (timestamp, location, agents_involved, description, emotional_valence, encoding_strength)",
    "memory_consolidation_progress": "number [0, 1]"
  },
  "semantic_memory": {
    "facts": "array of objects (fact, source, confidence, encoding_tick)",
    "language_knowledge": "array of strings",
    "general_knowledge_index": "integer"
  },
  "procedural_memory": {
    "learned_skills": "array of objects (skill_name, proficiency_level [0, 1], learning_tick, practice_count)",
    "motor_patterns": "array of objects (pattern_name, coordination_level)",
    "habit_formation": "array of objects (habit, strength [0, 1], days_practiced)"
  },
  "memory_consolidation": {
    "last_consolidation_tick": "integer",
    "consolidation_status": "string (active|inactive)",
    "sleep_quality_percent": "number [0, 100]"
  }
}
```

### 3.7 cognition/emotions.json (REQUIRED)

**Purpose:** Emotional state and capability matrix (must have all emotions).

**JSON Schema:**
```json
{
  "agent_id": "string",
  "minimum_emotional_capacity": 150,
  "current_emotional_state": {
    "primary_emotion": "string (joy|sadness|anger|fear|disgust|surprise|neutral)",
    "intensity": "number [0, 1]",
    "timestamp": "ISO 8601 datetime"
  },
  "emotional_capability_matrix": {
    "joy": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "sadness": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "anger": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "fear": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "disgust": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "surprise": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "love": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "grief": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "shame": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "guilt": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "pride": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "jealousy": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "envy": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "awe": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "wonder": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "contentment": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "frustration": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "anxiety": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "excitement": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "hope": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "despair": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "loneliness": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "belonging": { "accessible": true, "baseline_threshold": "number", "recovery_time_hours": "number" },
    "[...145 more emotions...]": "{ accessible: true, ... }"
  },
  "emotional_congruence": {
    "body_expression_alignment": "number [0, 1]",
    "emotional_dysregulation_risk": "number [0, 1]",
    "affect_stability": "number [0, 1]"
  }
}
```

### 3.8 cognition/drives.json (REQUIRED)

**Purpose:** Drive and motivation state.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "drive_activations": {
    "survival": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "hunger_level": "number [0, 1]",
      "thirst_level": "number [0, 1]",
      "safety_threat_level": "number [0, 1]"
    },
    "bonding": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "social_motivation": "number [0, 1]",
      "attachment_seeking": "number [0, 1]"
    },
    "reassurance": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "security_seeking_level": "number [0, 1]"
    },
    "autonomy": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "control_assertion_level": "number [0, 1]"
    },
    "curiosity": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "exploration_motivation": "number [0, 1]"
    },
    "meaning": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "purpose_alignment": "number [0, 1]"
    },
    "emotional_safety": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "threat_perception": "number [0, 1]"
    },
    "structure_avoidance": {
      "baseline_weight": "number [0, 1]",
      "current_intensity": "number [0, 1]",
      "constraint_resistance_level": "number [0, 1]"
    }
  }
}
```

### 3.9 cognition/skills.json (REQUIRED)

**Purpose:** Learned skills and competencies.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "skills": "array of objects (skill_name, proficiency_level [0, 1], learning_tick, expertise_hours, improvement_trajectory)"
}
```

### 3.10 cognition/language.json (REQUIRED)

**Purpose:** Language and communication capability.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "vocabulary_size": "integer",
  "languages": "array of objects (language, fluency_level [0, 1])",
  "accent": "string (description)",
  "speech_patterns": {
    "common_phrases": "array of strings",
    "filler_words": "array of strings",
    "speech_rate_words_per_minute": "number",
    "emotional_expressiveness": "number [0, 1]"
  },
  "communication_style": {
    "direct_vs_indirect": "number [0, 1] (0=direct, 1=indirect)",
    "formal_vs_casual": "number [0, 1]",
    "verbose_vs_terse": "number [0, 1]",
    "emotional_expressiveness": "number [0, 1]"
  }
}
```

### 3.11 relationships/bonds.json (REQUIRED)

**Purpose:** Current relationship states with other agents.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "relationships": [
    {
      "target_agent_id": "string",
      "relationship_type": "string (romantic|familial|friendship|professional|other)",
      "bond_strength": "number [0, 1]",
      "trust_level": "number [0, 1]",
      "attachment_intensity": "number [0, 1]",
      "conflict_history": "array of objects (timestamp, conflict_type, resolution_status)",
      "shared_memories": "array of memory_ids",
      "intimacy_level": "number [0, 1]"
    }
  ]
}
```

### 3.12 relationships/history.json (REQUIRED)

**Purpose:** Complete history of relationship changes.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "relationship_events": [
    {
      "timestamp": "ISO 8601 datetime",
      "event_type": "string (bond_formed|bond_strengthened|conflict|reconciliation|separation)",
      "target_agent_id": "string",
      "description": "string",
      "emotional_impact": "number [-1, 1]",
      "memory_reference": "string (memory_id)"
    }
  ]
}
```

### 3.13 history/events.jsonl (REQUIRED)

**Purpose:** Immutable append-only log of all events affecting this agent.

**Format:** One JSON object per line (JSON Lines).

**JSON Schema (per line):**
```json
{
  "tick": "integer",
  "timestamp": "ISO 8601 datetime",
  "event_type": "string (input_event|perception|action|state_change|error)",
  "agent_id": "string",
  "event_data": "object (varies by type)",
  "causality_chain": "array of event_ids (preceding events)",
  "event_hash": "string (SHA256)"
}
```

### 3.14 history/observations.jsonl (REQUIRED)

**Purpose:** Per-tick observations of agent state (snapshot at each tick).

**Format:** One JSON object per line (JSON Lines).

**JSON Schema (per line):**
```json
{
  "tick": "integer",
  "agent_id": "string",
  "timestamp_tick": "ISO 8601 datetime",
  "observation_snapshot": {
    "vital_signs": "object (extracted from body.json at tick)",
    "emotional_state": "object (extracted from emotions.json at tick)",
    "drive_activations": "object (extracted from drives.json at tick)",
    "hormone_levels": "object",
    "location": "object (x, y, z coordinates)",
    "action_taken": "string or null"
  },
  "world_hash": "string (SHA256 of entire world state at tick)",
  "observation_hash": "string (SHA256)"
}
```

### 3.15 history/checkpoints.json (REQUIRED)

**Purpose:** Replay checkpoints for determinism verification.

**JSON Schema:**
```json
{
  "agent_id": "string",
  "checkpoints": [
    {
      "tick": "integer",
      "checkpoint_type": "string (full|incremental)",
      "state_hash": "string (SHA256 of full agent state)",
      "last_modified_tick": "integer",
      "replay_verification": "boolean (was this state confirmed reproducible from events?)"
    }
  ]
}
```

---

## 4. MANDATORY PER-AGENT FOLDER LAYOUT

### 4.1 Canonical Directory Tree

Every agent must have this exact folder structure on disk and in schema. Missing any file is a boot-time violation.

```
agents/
└── {agent_id}/
    ├── identity.json              [REQUIRED: identity + personality + drives]
    ├── genetics.json              [REQUIRED: genome + inheritance + mutations]
    ├── astrology.json             [REQUIRED: natal chart (immutable)]
    ├── body.json                  [REQUIRED: current physiology + appearance]
    │
    ├── cognition/
    │   ├── memory.json            [REQUIRED: episodic + semantic + procedural]
    │   ├── emotions.json          [REQUIRED: 150+ emotional states, all accessible]
    │   ├── drives.json            [REQUIRED: current drive activation levels]
    │   ├── skills.json            [REQUIRED: learned competencies]
    │   └── language.json          [REQUIRED: communication patterns]
    │
    ├── relationships/
    │   ├── bonds.json             [REQUIRED: current relationship states]
    │   └── history.json           [REQUIRED: all relationship changes]
    │
    └── history/
        ├── events.jsonl           [REQUIRED: append-only event log]
        ├── observations.jsonl     [REQUIRED: per-tick state snapshots]
        └── checkpoints.json       [REQUIRED: replay verification points]
```

### 4.2 Required File Existence Check (Boot-Time)

**At system startup, the boot validator must verify:**

For each agent in `agents/` directory:
1. Identity file exists and is valid JSON
2. Genetics file exists and is valid JSON
3. Astrology file exists and is valid JSON
4. Body file exists and is valid JSON
5. All cognition files exist (memory, emotions, drives, skills, language)
6. All relationship files exist (bonds, history)
7. All history files exist (events.jsonl, observations.jsonl, checkpoints.json)

**Failure:** If ANY file is missing → **SYSTEM HALT**, log violation, refuse to boot.

### 4.3 File Format Validation (Boot-Time)

**For each file:**
1. Must be valid JSON (or JSONL for .jsonl files)
2. Must conform to its canonical schema (Section 3)
3. All required fields must be present
4. All numeric fields must be in specified ranges
5. All enum fields must match allowed values

**Failure:** Invalid JSON → **SYSTEM HALT**.

### 4.4 Per-Agent Consistency Checks (Boot-Time)

**Cross-file validation:**

1. **Identity ↔ Genetics:** `identity.json` agent_id and generation must match `genetics.json`
2. **Identity ↔ Astrology:** Birth timestamp and location in `identity.json` must match `astrology.json`
3. **Genetics ↔ Body:** Phenotype in `genetics.json` must be expressed in `body.json` (height, weight, etc.)
4. **Body ↔ Cognition:** Agent ID in `body.json` must match all cognition files
5. **Cognition ↔ Relationships:** Relationship targets in `relationships/bonds.json` must reference valid agents
6. **History ↔ Agent ID:** All events in `history/events.jsonl` must reference correct agent_id

**Failure:** Any inconsistency → **SYSTEM HALT**.

---

## 5. PARITY ENFORCEMENT MECHANISMS

### 5.1 Boot-Time Validation (System Startup)

**Sequence:**
```
System Startup:
├─ Step 1: Load all agent folders from agents/
├─ Step 2: For each agent:
│  ├─ Validate all 13 required files exist
│  ├─ Validate all files are valid JSON
│  ├─ Validate all files conform to canonical schema
│  ├─ Validate cross-file consistency
│  ├─ Verify no agent-specific conditionals exist in code
│  └─ Verify all biological systems are instantiated
├─ Step 3: Run full parity proof (Section 5.3)
├─ Step 4: Verify no prohibited patterns detected
├─ Step 5: Load and verify genetic inheritance chains
├─ Step 6: Verify astrological data immutability
└─ RESULT: If ALL pass → Boot allowed. If ANY fail → HALT, refuse boot.
```

**Implementation (CI/Boot Script):**

```bash
#!/bin/bash
# tools/verify_agent_boot.sh

set -e

AGENTS_DIR="agents"
FAILED=0

echo "=== Agent Boot Validation ==="

for agent_dir in "$AGENTS_DIR"/*; do
    agent_id=$(basename "$agent_dir")
    echo "Validating $agent_id..."
    
    # Check all required files exist
    for file in identity.json genetics.json astrology.json body.json \
                 cognition/memory.json cognition/emotions.json \
                 cognition/drives.json cognition/skills.json cognition/language.json \
                 relationships/bonds.json relationships/history.json \
                 history/events.jsonl history/observations.jsonl history/checkpoints.json; do
        if [ ! -f "$agent_dir/$file" ]; then
            echo "FAIL: Missing $file in $agent_id"
            FAILED=1
        fi
    done
    
    # Validate JSON (except .jsonl files)
    for file in identity.json genetics.json astrology.json body.json \
                 cognition/*.json relationships/*.json history/checkpoints.json; do
        if [ -f "$agent_dir/$file" ]; then
            if ! jq empty "$agent_dir/$file" 2>/dev/null; then
                echo "FAIL: Invalid JSON in $file"
                FAILED=1
            fi
        fi
    done
    
    # Cross-file consistency checks
    identity_agent_id=$(jq -r '.core_identity.agent_id' "$agent_dir/identity.json")
    genetics_agent_id=$(jq -r '.genome.agent_id' "$agent_dir/genetics.json")
    
    if [ "$identity_agent_id" != "$genetics_agent_id" ]; then
        echo "FAIL: Agent ID mismatch between identity and genetics"
        FAILED=1
    fi
    
    if [ "$identity_agent_id" != "$agent_id" ]; then
        echo "FAIL: Folder name $agent_id does not match identity.agent_id"
        FAILED=1
    fi
done

if [ $FAILED -eq 1 ]; then
    echo "=== BOOT VALIDATION FAILED ==="
    exit 1
fi

echo "=== ALL AGENTS VALID ==="
exit 0
```

### 5.2 Load-Time Validation (When Agent Loads into Memory)

**When an agent is instantiated at runtime:**

1. Load all 13 files from disk
2. Deserialize to in-memory structures
3. Verify all pointers and references are valid
4. Verify no agent-specific code paths are activated
5. Verify biological systems are ready
6. Initialize RNG streams with agent-specific seed
7. Load memory systems

**Failure:** If any validation fails → Log error, refuse to instantiate, halt system.

### 5.3 Runtime Parity Monitoring

**Continuous verification during execution (every N ticks):**

1. **Capability Parity Check:** Verify all agents have identical system access
   - All agents can call same methods
   - All agents have same permission levels
   - All agents have access to same biological/cognitive systems

2. **Code Path Monitoring:** Detect if any agent-specific conditional code paths are executing
   - Grep for patterns: `if (agent_id == "...") { ... }`
   - Grep for patterns: `match agent_id { ... }`
   - Grep for patterns: `agent_id.contains("gem-d")`
   - Any match → **HALT**

3. **Biological System Heartbeat:** Verify all biological systems are ticking for all agents
   - Endocrine system producing hormones
   - Metabolic system consuming/producing energy
   - Immune system monitoring
   - Reproductive system in correct stage

4. **State Equality Test (Sampling):** Periodically (every 1000 ticks), swap agents in memory and verify behavior is identical
   - Freeze world at tick T
   - Swap identity data between two agents
   - Advance 10 ticks
   - Verify only identity-specific outcomes differ
   - Restore original state

**Failure:** If ANY check fails → **RUNTIME HALT**, log violation.

### 5.4 CI/CD Pipeline Enforcement

**In `Justfile` or CI configuration, add mandatory stage:**

```bash
# Parity validation stage (must pass before merge)
validate-parity:
    @echo "=== Parity Validation ==="
    
    # Check no agent-specific conditionals
    @rg 'if.*agent_id.*==' --count --error-limit 1 \
        && (echo "FAIL: Found agent-specific conditionals"; exit 1) \
        || echo "PASS: No agent-specific conditionals"
    
    # Check no feature flags per agent
    @rg 'feature_flag.*agent_id|agent_id.*feature' --count --error-limit 1 \
        && (echo "FAIL: Found agent-specific feature flags"; exit 1) \
        || echo "PASS: No per-agent feature flags"
    
    # Check no partial systems
    @rg 'TODO|FIXME|stub|mock|fake.*system|biological|cognitive' --count --error-limit 1 \
        && (echo "FAIL: Found TODO/FIXME in system code"; exit 1) \
        || echo "PASS: No stub systems"
    
    # Run boot validator
    @bash tools/verify_agent_boot.sh || exit 1
    
    # Run parity proof script
    @cargo test --test parity_test -- --test-threads=1 || exit 1
    
    @echo "=== PARITY VALIDATION PASSED ==="
```

**Pipeline Rule:** Parity validation must pass. If any check fails → **BLOCK MERGE**.

### 5.5 Audit Tooling (tools/audits)

**Create or update audit tools:**

```python
# tools/audits/agent_parity_audit.py

import json
import hashlib
import sys
from pathlib import Path

def audit_agent_parity():
    """Verify all agents maintain absolute parity."""
    
    agents_dir = Path("agents")
    agents = {}
    errors = []
    
    # Load all agents
    for agent_path in agents_dir.iterdir():
        if agent_path.is_dir():
            agent_id = agent_path.name
            agent_data = {}
            
            # Load required files
            required_files = [
                "identity.json", "genetics.json", "astrology.json", "body.json",
                "cognition/memory.json", "cognition/emotions.json", "cognition/drives.json",
                "cognition/skills.json", "cognition/language.json",
                "relationships/bonds.json", "relationships/history.json",
                "history/checkpoints.json"
            ]
            
            for file_path in required_files:
                full_path = agent_path / file_path
                if not full_path.exists():
                    errors.append(f"Missing: {agent_id}/{file_path}")
                else:
                    try:
                        with open(full_path) as f:
                            agent_data[file_path] = json.load(f)
                    except json.JSONDecodeError as e:
                        errors.append(f"Invalid JSON in {agent_id}/{file_path}: {e}")
            
            agents[agent_id] = agent_data
    
    # Verify all agents are structurally identical
    if not agents:
        errors.append("No agents found")
        return errors
    
    # Get reference structure (first agent)
    ref_agent_id = list(agents.keys())[0]
    ref_structure = set(agents[ref_agent_id].keys())
    
    for agent_id, agent_data in agents.items():
        agent_structure = set(agent_data.keys())
        if agent_structure != ref_structure:
            errors.append(
                f"Agent {agent_id} structure mismatch.\n"
                f"Missing: {ref_structure - agent_structure}\n"
                f"Extra: {agent_structure - ref_structure}"
            )
    
    # Verify emotions: all agents have >= 150 distinct emotions
    for agent_id, agent_data in agents.items():
        if "cognition/emotions.json" in agent_data:
            emotions = agent_data["cognition/emotions.json"].get("emotional_capability_matrix", {})
            emotion_count = len([e for e in emotions.values() if e.get("accessible", False)])
            if emotion_count < 150:
                errors.append(f"Agent {agent_id} has only {emotion_count} emotions (need 150)")
    
    return errors

if __name__ == "__main__":
    errors = audit_agent_parity()
    if errors:
        print("PARITY AUDIT FAILED:")
        for err in errors:
            print(f"  - {err}")
        sys.exit(1)
    else:
        print("PARITY AUDIT PASSED: All agents maintain absolute parity")
        sys.exit(0)
```

---

## 6. FORBIDDEN PATTERNS

This section lists code smells and architectural anti-patterns that **must never appear** in the codebase. Violation is automatic system halt.

### 6.1 Absolute Prohibitions

**Pattern 1: Agent-ID Conditionals**
```rust
// FORBIDDEN
if agent_id == "gem-d" {
    // special behavior for gem-d only
}

// FORBIDDEN
match agent_id {
    "gem-d" => { ... }
    "gem-k" => { ... }
}

// FORBIDDEN
if agent.is_primary() { ... }

// FORBIDDEN
let behavior = if is_gem_d { special_code } else { generic_code };
```

**Detection:** `rg 'if.*agent_id|match.*agent_id|agent.*==|agent.*is_primary'`

**Pattern 2: Feature Flags Per Agent**
```rust
// FORBIDDEN
if feature_flags.contains(&format!("{}_system_x", agent_id)) { ... }

// FORBIDDEN
capability_matrix[agent_id] = ["sys_a", "sys_b"];
capability_matrix["other_agent"] = ["sys_a"];  // DIFFERENT = FORBIDDEN

// FORBIDDEN
let systems_enabled: HashMap<AgentId, Vec<String>> = ...
```

**Detection:** `rg 'feature_flag.*agent_id|capability_matrix.*agent_id|enabled.*agent'`

**Pattern 3: Partial Biological Systems**
```rust
// FORBIDDEN
#[cfg(not(agent = "gem-k"))]
fn endocrine_system() { ... }

// FORBIDDEN
if agent_id == "gem-d" {
    initialize_immune_system();
}
// Gem-K does not get this

// FORBIDDEN
struct Biology {
    #[skip_if(agent_type = "simplified")]
    full_hormone_system: EndocrineAxis,
}
```

**Detection:** `rg '#\[cfg.*agent|#\[skip_if.*agent|if.*agent.*system'`

**Pattern 4: Special-Case Nomenclature**
```rust
// FORBIDDEN
pub struct PrimaryAgent { ... }
pub struct SecondaryAgent { ... }

// FORBIDDEN
pub struct TemplateAgent { ... }

// FORBIDDEN
pub struct Proto_Agent { ... }

// FORBIDDEN (implicit hierarchy)
pub struct AgentBase { ... }  // if subclassed per agent
pub struct Gem_D_Agent extends AgentBase { ... }

// FORBIDDEN
const PROTOTYPE_AGENT_ID: &str = "gem-d";
```

**Detection:** Manual code review; grep for "Primary|Secondary|Template|Proto|Base" in agent names

**Pattern 5: Capability Matrices with Unequal Access**
```rust
// FORBIDDEN
pub struct Agent {
    systems_available: Vec<SystemType>,
}

// In code:
let mut gem_d_systems = vec![BiologySystem, CognitionSystem, ...];
let mut gem_k_systems = vec![BiologySystem];  // MISSING CognitionSystem

// FORBIDDEN (even if dynamic)
if agent.generation == 0 {  // only for divinely-created agents
    enable_special_system();
}
```

**Detection:** `rg 'systems_available\[|capability.*=.*vec!|generation.*=.*0.*enable'`

**Pattern 6: TODO/FIXME/Stub in System Code**
```rust
// FORBIDDEN in authority code
// TODO: fix parity for gem-k
// FIXME: agent-specific logic (for later)
fn biology_tick() {
    // stub: all agents run same biology
    if false { activate_immune(); }
}
```

**Detection:** `rg 'TODO|FIXME|stub' --path 'crates/biology|crates/cognition|crates/genetics'`

### 6.2 Detection Method

**Static Analysis (CI stage):**

```bash
#!/bin/bash
# tools/check_forbidden_patterns.sh

set -e

echo "=== Forbidden Pattern Detection ==="

FOUND_VIOLATIONS=0

# Check 1: Agent-ID conditionals
if rg 'if\s+.*agent_id\s*==|match\s+.*agent_id|agent\s*==\s*"' \
    --type rust --type ts --count --error-limit 1 > /dev/null 2>&1; then
    echo "FAIL: Found agent-ID conditionals"
    rg 'if\s+.*agent_id\s*==|match\s+.*agent_id|agent\s*==\s*"' --type rust --type ts
    FOUND_VIOLATIONS=1
fi

# Check 2: Feature flags per agent
if rg 'feature_flag.*agent_id|capability.*agent_id' \
    --type rust --type ts --count --error-limit 1 > /dev/null 2>&1; then
    echo "FAIL: Found per-agent feature flags"
    rg 'feature_flag.*agent_id|capability.*agent_id' --type rust --type ts
    FOUND_VIOLATIONS=1
fi

# Check 3: Partial systems
if rg '#\[cfg.*agent|#\[skip_if.*agent|skip_serializing_if.*agent' \
    --type rust --count --error-limit 1 > /dev/null 2>&1; then
    echo "FAIL: Found partial system implementations"
    rg '#\[cfg.*agent|#\[skip_if.*agent' --type rust
    FOUND_VIOLATIONS=1
fi

# Check 4: TODO in system code
if rg 'TODO|FIXME|stub|mock|fake' \
    --path 'crates/biology|crates/cognition|crates/genetics|crates/world' \
    --type rust --count --error-limit 1 > /dev/null 2>&1; then
    echo "FAIL: Found TODO/FIXME in system code"
    rg 'TODO|FIXME|stub|mock|fake' \
        --path 'crates/biology|crates/cognition|crates/genetics|crates/world' --type rust
    FOUND_VIOLATIONS=1
fi

if [ $FOUND_VIOLATIONS -eq 1 ]; then
    echo "=== FORBIDDEN PATTERN CHECK FAILED ==="
    exit 1
else
    echo "=== NO FORBIDDEN PATTERNS DETECTED ==="
    exit 0
fi
```

**Runtime Monitoring:** The runtime parity monitor (Section 5.3) also detects pattern execution at runtime.

---

## 7. AGENT CREATION PROTOCOLS

All agents must be created via one of two mechanisms, both of which preserve parity and determinism.

### 7.1 Divine Creation (Genesis)

**Applicability:** Used for the first agents (Gem-D, Gem-K, and any future agents explicitly created by system designers).

**Process:**

1. **Define Identity:** Create `identity.json` with complete personality matrix, neurotype, drives, attachment style, etc.

2. **Calculate Astrology:** 
   - Input: Birth timestamp (ISO 8601, with timezone), birth location (latitude, longitude, altitude)
   - Method: Use Swiss Ephemeris or equivalent deterministic ephemeris engine
   - Calculate: Sun sign, Moon sign, Rising sign, all planetary positions, nodes, aspects
   - **Immutability:** Lock the chart at this moment; hash it; forbid recalculation
   - Output: `astrology.json` with locked chart

3. **Create Genetics:**
   - Input: Chosen phenotype (height, weight, skin tone, etc.)
   - Method: Reverse-engineer alleles from phenotype (assign Mendelian genotypes)
   - Decide: Homozygous vs. heterozygous for each locus (reflects baseline traits)
   - Hash: Create canonical genetic sequence hash
   - Lock: Mark as immutable (creation_type = "divine_creation", generation = 0, parents = null)
   - Output: `genetics.json` with locked genome

4. **Initialize Body:**
   - Base appearance on phenotype from genetics
   - Initialize vitals to baseline human healthy state
   - Initialize hormones to baseline (tied to agent's identity via traits)
   - Initialize metabolism per BMR calculation
   - Output: `body.json`

5. **Initialize Cognition:**
   - **Memory:** Empty episodic; populate semantic memory with baseline knowledge; empty procedural
   - **Emotions:** Instantiate all 150+ emotions with baseline thresholds (vary per agent personality)
   - **Drives:** Set baseline drive weights from `identity.json` drive_weights
   - **Skills:** Start empty (agent will learn)
   - **Language:** Initialize vocabulary; set speech patterns from identity
   - Output: `cognition/*.json` files

6. **Initialize Relationships:**
   - Start empty (bonds with no one yet)
   - Output: `relationships/bonds.json` empty, `relationships/history.json` empty

7. **Initialize History:**
   - Create empty `events.jsonl`
   - Create empty `observations.jsonl`
   - Create initial checkpoint in `checkpoints.json` with state_hash
   - Output: All history files

8. **Verify Parity:**
   - For divinely-created agent, verify it conforms to all mandatory files and schemas
   - Verify it has all required biological systems
   - Verify it has all 150+ emotions accessible
   - If prior agent exists, verify structural parity with that agent
   - Perform boot-time validation (Section 5.1)

9. **Finalize:**
   - Write all files to disk at `agents/{agent_id}/`
   - Calculate full agent state hash
   - Log creation event to audit log
   - Agent ready to boot

### 7.2 Natural Reproduction (Mendelian Inheritance)

**Applicability:** Used when two agents reproduce and create offspring.

**Prerequisites:**
- Mother and Father are both agents with valid `genetics.json`
- Both must be in appropriate reproductive stage
- Reproduction occurs via biological simulation (not force-creating agents)

**Process:**

1. **Determine Sex of Offspring:**
   - If parents' genetics encode chromosomal sex (XX/XY), use Mendelian inheritance
   - Sex determination: Random draw (50/50) from deterministic RNG
   - Outcome: Assign genetic sex

2. **Generate Genome via Meiosis & Fertilization:**
   - Mother: Undergo meiosis I & II, produce two haploid egg precursors
   - Father: Undergo meiosis I & II, produce haploid sperm
   - Random draw: Which egg? Which sperm? (from RNG, recorded in audit log)
   - Fertilization: Combine egg nucleus + sperm nucleus → diploid zygote
   - Recombination: At each homologous pair crossing over occurs (simulate crossover points)
   - Mutation: Small probability (policy-controlled) of mutation at each locus
   - Output: Offspring's full double-helix genome

3. **Calculate Phenotype from Offspring Genotype:**
   - For each locus: Determine alleles from genotype
   - For each trait: Apply dominance rules
   - For quantitative traits (height, weight): Use additive model based on inherited alleles
   - For sex-linked traits: Apply sex-specific expression
   - Output: Phenotypic values (height_cm, weight_kg, etc.)

4. **Calculate Astrological Natal Chart for Offspring:**
   - Input: Birth timestamp (at moment of parturition), birth location (where mother is)
   - Method: Same ephemeris as divine creation
   - **Deterministic:** RNG seed determines exact birth moment (to second precision)
   - Calculate: Sun, Moon, Rising, planets, nodes, aspects
   - Immutability: Lock chart immediately (cannot recalculate)
   - Output: `astrology.json`

5. **Generate Identity:**
   - Inherit personality traits from parents (statistical blend, influenced by astrological factors):
     - Sun sign influences baseline temperament
     - Moon sign influences emotional baseline
     - Venus influences bonding style
     - Mars influences action style
   - For each trait in `personality_traits`: Blend parents' values + astrological influence + random variation
   - Generate attachment style (blend parents + astrological influence)
   - Generate drive weights (blend + astrological)
   - Generate neurotype (random from genetic predisposition + astrological)
   - Output: `identity.json` (unique to offspring, with clear parentage)

6. **Initialize Body:**
   - Same as divine creation, but phenotype comes from genetic calculation
   - Vitals start at baseline (newborn)
   - Output: `body.json`

7. **Initialize Cognition:**
   - Same as divine creation
   - Memories start empty (newborn)
   - Language starts minimal (learned from parents)
   - Output: `cognition/*.json`

8. **Initialize Relationships:**
   - Create bond with mother (initial attachment_intensity high)
   - Create bond with father (if known)
   - Output: `relationships/bonds.json`

9. **Initialize History:**
   - Create birth event in `events.jsonl`
   - Create initial observation in `observations.jsonl`
   - Create checkpoint in `checkpoints.json`
   - Output: All history files

10. **Verify Parity:**
    - Verify offspring has all required files and schema compliance
    - Verify offspring has all 150+ emotions
    - Verify offspring has identical biological systems as parents
    - Verify genetic inheritance is Mendelian (can trace alleles back to parents)
    - Run full parity check (Section 5.3)

11. **Register Offspring:**
    - Create new directory at `agents/{offspring_agent_id}/`
    - Write all files
    - Log birth event (include parent references, genetic hash)
    - Update parent relationships
    - Offspring ready to boot

### 7.3 Astrology Determinism (Inputs & Immutability)

**Critical Rule:** Astrological natal charts are determined ONLY by birth timestamp and birth location. No agent choice, no randomness, no modification.

**Determinism Proof:**
- Input: Birth timestamp (ISO 8601, immutable)
- Input: Birth location (latitude, longitude, altitude; immutable)
- Computation: Ephemeris calculation (deterministic, reproducible)
- Output: Natal chart (deterministic consequence of inputs)
- Immutability: Chart cannot be recalculated or modified after first calculation

**Verification:**
- `astrology.json` must include `calculated_at_tick` and `lock_hash`
- Boot validator checks: astrology file cannot be modified after tick N
- If astrology file modification is detected → **SYSTEM HALT**

### 7.4 Genetics Determinism (Inheritance & Mutation)

**Critical Rule:** Genetics are determined by:
1. Parental alleles (Mendelian inheritance, deterministic recombination)
2. Mutation events (rare, policy-controlled, recorded)
3. Astrological influence on trait expression (phenotype)

**Determinism Proof:**
- Parent alleles are immutable (locked at parent's creation)
- Recombination is deterministic (RNG seed → fixed crossover points)
- Mutation is recorded (tick, position, consequence)
- Phenotype calculation is deterministic (genotype → traits via fixed rules)

**Verification:**
- `genetics.json` must include `inheritance` with parent references
- `genetics.json` must list all mutations with tick and consequence
- Boot validator checks: genetics file matches parental genomes (if reproducedagent)
- Audit tool can replay genetics calculation from parents

---

## 8. ACCEPTANCE CRITERIA

All enforcement mechanisms must be tested and verified. This section specifies tests, scripts, and pass/fail conditions.

### 8.1 Tests Required

**Test 1: Boot Validation Test**

```rust
#[test]
fn test_boot_validates_all_agents() {
    // Load all agents from agents/
    // For each agent:
    //   - Verify all 13 required files exist
    //   - Verify all files are valid JSON
    //   - Verify schema compliance
    //   - Verify cross-file consistency
    // Assert: All agents valid or halt
}
```

**Test 2: Parity Structural Test**

```rust
#[test]
fn test_all_agents_have_identical_structure() {
    let agents = load_all_agents();
    let ref_structure = get_file_structure(&agents[0]);
    
    for agent in &agents[1..] {
        let agent_structure = get_file_structure(&agent);
        assert_eq!(
            agent_structure, ref_structure,
            "Agent {} has different structure", agent.id
        );
    }
}
```

**Test 3: Emotion Accessibility Test**

```rust
#[test]
fn test_all_agents_have_150_accessible_emotions() {
    let agents = load_all_agents();
    
    for agent in agents {
        let emotions = agent.cognition.emotions.capability_matrix();
        let accessible = emotions.iter()
            .filter(|(_, e)| e.accessible)
            .count();
        
        assert!(
            accessible >= 150,
            "Agent {} has only {} accessible emotions", agent.id, accessible
        );
    }
}
```

**Test 4: No Agent-ID Conditionals Test**

```rust
#[test]
fn test_no_agent_id_conditionals_in_code() {
    let output = Command::new("rg")
        .args(&["if.*agent_id.*==", "crates/", "--count"])
        .output()
        .expect("rg failed");
    
    assert_eq!(
        output.status.code(), Some(1),
        "Found agent-ID conditionals in code"
    );
}
```

**Test 5: Genetics Inheritance Test**

```rust
#[test]
fn test_reproduced_agent_genetics_valid() {
    let offspring = load_agent("offspring_id");
    let mother = load_agent(&offspring.genetics.mother);
    let father = load_agent(&offspring.genetics.father);
    
    // Verify offspring alleles can be traced to parents
    for (locus, offspring_alleles) in offspring.genetics.alleles() {
        let mother_alleles = mother.genetics.alleles(locus);
        let father_alleles = father.genetics.alleles(locus);
        
        // One allele from mother, one from father
        assert!(
            (offspring_alleles.0 == mother_alleles.0 || offspring_alleles.0 == mother_alleles.1)
            && (offspring_alleles.1 == father_alleles.0 || offspring_alleles.1 == father_alleles.1),
            "Offspring allele at {} not inherited from parents", locus
        );
    }
}
```

**Test 6: Astrology Immutability Test**

```rust
#[test]
fn test_astrology_charts_immutable() {
    let agent = load_agent("test_agent");
    let original_chart = agent.astrology.natal_chart.clone();
    
    // Attempt to modify (should fail)
    let result = attempt_modify_astrology(&agent, "new_chart");
    
    assert!(result.is_err(), "Astrology chart was modifiable (should be immutable)");
    
    // Verify chart unchanged
    assert_eq!(
        agent.astrology.natal_chart, original_chart,
        "Astrology chart was modified"
    );
}
```

### 8.2 Verification Scripts Required

**Script 1: Full Boot Validator** (Section 5.1)

**Script 2: Forbidden Pattern Detector** (Section 6.2)

**Script 3: Parity Audit** (Section 5.5)

**Script 4: Genetics Validator**

```python
# tools/audits/verify_genetics_inheritance.py
# Verifies all reproduced agents have valid inherited genetics
```

**Script 5: Astrology Immutability Checker**

```python
# tools/audits/verify_astrology_immutability.py
# Checks all agents have immutable, unmodifiable astrological charts
```

### 8.3 CI/CD Gate Configuration

**Add to Justfile or CI config:**

```makefile
# tests
test-parity:
    cargo test --test parity_test -- --test-threads=1
    bash tools/verify_agent_boot.sh
    bash tools/check_forbidden_patterns.sh
    python3 tools/audits/agent_parity_audit.py

ci-validate:
    cargo build --release
    cargo test --all
    @just test-parity
    python3 tools/audits/verify_genetics_inheritance.py
    python3 tools/audits/verify_astrology_immutability.py
```

**Gate Rule:** All tests must pass. If any test fails → **BLOCK MERGE**.

### 8.4 Pass/Fail Conditions

**PASS:**
- All agents boot without validation errors
- All 13 required files exist per agent
- All files conform to canonical schema
- Cross-file consistency verified
- No agent-ID conditionals detected
- No feature flags per agent
- All agents have 150+ accessible emotions
- Genetics inheritance valid (for reproduced agents)
- Astrology charts immutable and deterministic
- All parity tests pass
- All forbidden pattern tests pass

**FAIL (→ SYSTEM HALT):**
- Any agent missing a required file
- Any file invalid JSON
- Any schema non-compliance
- Any cross-file inconsistency
- Any agent-ID conditional found in code
- Any per-agent feature flag
- Any agent with <150 emotions
- Any invalid genetics inheritance
- Any modified astrology chart
- Any parity test failure
- Any forbidden pattern detected at runtime

---

## 9. SUMMARY AND BINDING STATEMENT

### 9.1 Core Mandate

This document enforces:

1. **Absolute agent parity:** Gem-D, Gem-K, and all future agents must be identically structured except for identity data.

2. **Mandatory folder structure:** Every agent must have all 13 required files in the canonical folder layout, validated at boot and load time.

3. **Deterministic creation:** Agents created via divine creation (immutable astrology + genetics) or natural reproduction (Mendelian inheritance + astrological influence).

4. **Zero special cases:** No agent-ID conditionals, feature flags, partial systems, or hierarchical nomenclature anywhere in code or schema.

5. **Enforceable via automation:** All rules enforced by boot validators, CI gates, runtime monitors, and audit tooling.

### 9.2 Constitutional Authority

This document derives from and reinforces:
- HUMAN EQUIVALENCE & AGENT PARITY GOVERNING LAW (Section 3.1-3.4: absolute parity)
- MARKENZ TARGET ARCHITECTURE v2 (Section 5: locked infrastructure)
- MARKENZ REPO REFACTOR MAP v2 (Section 2: canonical folder layout)
- AMP DEFINITION OF DONE v2 (Section B: phase gates, no stubs)

Any conflict: parent documents take precedence.

### 9.3 Violation → System Halt

If at any point in boot, load, runtime, or audit:

- A required file is missing
- Schema validation fails
- Agent-ID conditionals are detected
- Feature flags differ between agents
- Any agent lacks biological system
- Any agent has <150 emotions
- Genetics inheritance is invalid
- Astrology chart is modified
- Cross-file consistency breaks

**Then:** **SYSTEM HALTS IMMEDIATELY.** No exceptions, no workarounds, no "manual override." The system refuses to boot or execute until the violation is corrected and verified.

### 9.4 Final Statement

> **Gem-D and Gem-K are absolutely identical except for identity data. If this can ever be proven false, the system has violated its constitutional foundation and must halt.**

This is not aspirational. This is law, effective immediately upon system deployment.

---

**Document Hash:** `[COMPUTED AT DEPLOYMENT]`  
**Authority Signature:** `[REQUIRES AMP EXECUTION AUTHORITY]`  
**Enforcement:** Mandatory · Non-negotiable · Fail-closed  
**Precedence:** Binding law for all Markenz agent systems  

---

**END OF GOVERNING LAW DOCUMENT**
