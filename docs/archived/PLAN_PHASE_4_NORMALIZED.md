---
status: APPROVED
---

# PLAN_PHASE_4_NORMALIZED
## Cognition Engine (No LLM)

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 4 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.5)

---

## 1. ENTRY CONDITION
Phase 3 complete with all gates passing.

---

## 2. SCOPE (LOCKED)

Deterministic minds and language, fully offline.

**Deliverables:**
- Perception encoding (world state → logical predicates)
- Drives/motivation system (hunger, thirst, fatigue, exploration, social, safety, curiosity)
- Emotions (valence, arousal, emotions as state)
- Goal selection (drive → goal mapping, deterministic priority)
- GOAP planner (Goal-Oriented Action Planning, depth-first deterministic search)
- Skill trees (skill progression, habit formation)
- Memory systems (episodic, semantic, procedural)
- Deterministic English language (lexicon + grammar templates, NO LLM)
- Inner monologue (continuous thought stream, logged as ObservationEvents)

---

## 3. NON-SCOPE

- Genetics (Phase 5)
- Reproduction (Phase 5)
- Social dynamics (Phase 6)
- Governance (Phase 7)
- Rendering (Phase 8)
- Security (Phase 9)

---

## 4. PRESERVATION

Gem-D, Gem-K skills/memories preserved from Phase 1 import.

---

## 5. DETERMINISM (CRITICAL)

### 5.1 GOAP Planner
- Deterministic search (depth-first, fixed action ordering)
- Same (goal, perception, actions) → same plan
- No randomized search, no probabilistic selection

### 5.2 Perception Encoding
- Same world state → same perception predicates
- Integer math, no floating-point sight calculations

### 5.3 Drive/Emotion Updates
- Deterministic progression (same bio state → same drives)
- Emotion decay deterministic (fixed rate per tick)

### 5.4 Language Generation
- Deterministic word selection from lexicon (indexed, not random)
- Grammar rules deterministic
- Same thought → same utterance

### 5.5 Inner Monologue
- Continuous (one thought per tick minimum)
- Deterministic content (same state → same thought)
- Streamed as ObservationEvents

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Perception
Causal: Agent position, world state, tick  
State: Perception predicates  
Proof: Same world → same perception

### 6.2 Drives
Causal: Bio state (energy, hydration, sleep debt)  
State: Drive intensities  
Proof: Drives update deterministically per tick

### 6.3 GOAP Planning
Causal: Current state, goal, action library  
State: Action plan  
Proof: Same inputs → same plan (or same failure)

### 6.4 Language
Causal: Thought content, emotion state  
State: Utterance string  
Proof: Same thought → same utterance (deterministic lexicon selection)

### 6.5 Inner Monologue
Causal: Perception, drives, emotion, recent events  
State: Thought log  
Proof: Replay produces identical thoughts

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_4_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_4_EXECUTION_REPORT.md

Must include: GOAP plan examples, language generation samples, thought stream samples.

---

## 8. EXIT CRITERIA

### Cognition Systems
- [ ] Perception deterministic
- [ ] Drives deterministic
- [ ] Emotions deterministic
- [ ] Goals deterministic
- [ ] GOAP planning deterministic
- [ ] Skill progression deterministic
- [ ] Memory deterministic

### Planning
- [ ] GOAP planner produces valid plans
- [ ] Plans achieve goals
- [ ] Planning determinism test passes
- [ ] No randomized plan selection

### Language
- [ ] Language generation deterministic
- [ ] Lexicon-based (no LLM)
- [ ] Grammar deterministic
- [ ] Inner monologue every tick
- [ ] No LLM API calls
- [ ] Language determinism test passes

### Integration
- [ ] Engine runs cognition_tick per tick
- [ ] Thoughts emitted as ObservationEvents
- [ ] Inner monologue in web UI
- [ ] Action queue populated from plans
- [ ] Goals affect action selection

### Determinism
- [ ] Phase 3 tests still pass
- [ ] Cognition does not affect world_hash (observational only until action commit)
- [ ] Inner monologue replay identical

### AMP Sign-Off
- [ ] AMP approval BEFORE Phase 5

---

## 9. GATES

**Gate 1: Planning Determinism (TEST-PLAN-001)**  
**Gate 2: Language Determinism (TEST-LANG-001)**  
**Gate 3: Inner Monologue Replay (TEST-MONO-001)**

STOP if any fail or LLM call detected.

---

**END OF PHASE 4 NORMALIZED PLAN**
