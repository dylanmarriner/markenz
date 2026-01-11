---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 4
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_3_EMBODIED_BIOLOGY_V1
---

# MARKENZ — GOVERNANCE PHASE 4: OFFLINE COGNITION ENGINE (NO LLM)

## 1. Phase Objective

Implement deterministic minds, planning, learning, and language (fully offline, no LLM). All agents must have complete cognitive systems and produce coherent English utterances deterministically.

## 2. Governance Domains In Scope

- **Social contracts & trust** (relationship-weighted intent selection)
- **Violence & harm** (intent for aggression bias by relationships)

*Sourced from Section 4, PHASE 4, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/cognition` (perception, drives, emotion, planning, learning, language)
- `crates/cognition` — Deterministic planner (GOAP/HTN or equivalent)
- `crates/nlg` (NLG/NLU: grammar templates, lexicon tables)
- `crates/memory` (episodic, semantic, procedural memory systems)

*Sourced from Section 4, PHASE 4, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 4 MUST be defined and logged:

- `PerceptionEvent` (agent_id, observed_entity/item, property, timestamp)
- `Intent` (agent_id, action, priority, rationale)
- `Thought` (agent_id, content, emotional_valence, context)
- `Speech` (agent_id, utterance, target, emotion)
- `LearningUpdate` (agent_id, skill/memory, delta, source)

*Sourced from Section 4, PHASE 4, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 4 completion, the following properties MUST hold:

- **Planning Determinism:** Same percepts + belief state → identical plan (action sequence with priorities).
- **Language Determinism:** Same emotion + topic + memory → identical utterance (word sequence).
- **Learning Determinism:** Same experience + skill → identical learning rate and skill progression.
- **Emotion Determinism:** Same external context + internal state → identical emotional response (valence, intensity).
- **Thought Stream Determinism:** Same history + current percepts → identical inner monologue (sequence of thoughts).

*Sourced from Section 4, PHASE 4, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### Perception Gate (Fully Implemented)

- **Agent Sensory Range:** Agent perceives only entities within sensory distance (deterministic spatial constraint).
- **Identity Perception:** Agent recognizes familiar agents by learned patterns; strangers remain unknown until interaction.
- **State Perception:** Agent perceives observable properties (location, health, inventory type) based on sensory modality (visual, olfactory, etc.).
- **Perception Filtering:** Perception gate emits only observable properties to downstream cognition; non-observable properties blocked.

### Intent/Volition Layer (Newly Active)

- **Drives:** Agent has fundamental drives (hunger, reproduction, social bonding, curiosity, safety).
- **Planning:** Agent plans deterministically using planner (GOAP/HTN) to satisfy drives.
- **Intent Biasing:** Plan execution biased by relationship weights (friendly target favored over enemy).
- **Emotion Integration:** Emotional state (fear, joy, anger) influences plan probability (not outcome).

### Speech Generation (Deterministic NLG)

- **No LLM:** All utterances generated via templates + lexicon (grammar-based NLG).
- **Emotion Modulation:** Emotional state modulates word choice (angry speech uses intense words; sad speech uses softer words).
- **Topic Selection:** Utterance topic biased toward current goals (hungry agent talks about food more).
- **Grammar Stability:** Same emotion + topic → same sentence structure (deterministic template selection).

*Sourced from Section 2.1 "Event Pipeline", Section 2.3 "Enforcement Points", and Section 3.6 "Social Contracts & Trust."*

## 7. Audit & Replay Requirements

### Per-Agent Thought Stream

- Logged continuously; every `Thought` event emitted per tick (or per decision point).
- Stream shows: internal monologue content, emotional context, decision rationale.
- Observable via UI or audit tool; no hidden thought.

### Speech Utterances with Annotation

- Every `Speech` event includes: speaker, utterance (word sequence), target, emotion, topic.
- Utterances searchable; can reconstruct agent's conversation history.
- Audit report: agent's communication patterns, relationship effects on speech, learning through dialogue.

### Learning Curve Tracking

- `LearningUpdate` events show: skill name, delta (improvement magnitude), source (practice, observation, teaching).
- Aggregate report: skill progression timeline, learning rate per skill, correlation with experience.

*Sourced from Section 4, PHASE 4, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 5.

### 8.1 Planning Determinism Test

**Requirement:** Same percepts + belief state must produce identical plan (same action sequence, same priorities).

**Acceptance Criteria:**
- Agent has goal "acquire food"; perceives: forest (contains berries), river (contains fish), home (has stored grain).
- Belief state: agent is hungry, has fishing rod, knows berry patch location.
- Plan 1: [go to forest, gather berries, return home] with [priority: 1, 2, 3].
- Replay same context → Plan 2: identical action sequence and priorities.
- Multiple agents, multiple goal contexts tested.
- Test automated; CI gated.

### 8.2 NLG Determinism Test

**Requirement:** Same emotion + memory + topic must produce identical utterance (identical word sequence).

**Acceptance Criteria:**
- Agent emotion: joy; memory: positive interaction with friend; topic: greeting.
- Utterance 1: "Hello, friend! I'm so happy to see you."
- Replay same context → Utterance 2: identical word sequence (character-for-character).
- Multiple emotion × topic combinations tested.
- Test automated; CI gated.

### 8.3 Learning Test

**Requirement:** Skill progression must be deterministic; same experience → same learning curve.

**Acceptance Criteria:**
- Agent learns "fishing" skill via 10 practice attempts.
- Skill progression: [0.0, 0.1, 0.18, 0.25, 0.31, ...] (learned from practice).
- Replay same 10 practice attempts → identical skill progression (bit-for-bit).
- Test automated; CI gated.

### 8.4 Inner Monologue Test

**Requirement:** Thought stream must be continuous and deterministic; same history → same thoughts.

**Acceptance Criteria:**
- Agent has history: [perceive forest, hunger drive activated, recall fishing knowledge, decide to fish].
- Current percept: river visible, fishing rod in inventory.
- Thought stream 1: [consider fishing, remember successful catch yesterday, anticipate food, decide to fish].
- Replay same history + percept → Thought stream 2: identical sequence.
- Test automated; CI gated.

### 8.5 Speech Coherence Test

**Requirement:** Generated speech must be coherent English; no nonsensical output.

**Acceptance Criteria:**
- Agent generates 100 random utterances via NLG across all emotional/topic combinations.
- All utterances are grammatically correct English.
- All utterances are semantically coherent (meaningful given context).
- No placeholder tokens remain (e.g., "{subject}" not replaced).
- Test automated; manual review of sample utterances.

*Sourced from Section 4, PHASE 4, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 4 is considered complete:

1. **Cognition Engine Complete:**
   - Perception gate implemented and tested.
   - Intent/planning system working.
   - Emotion system integrated.

2. **Deterministic NLG:**
   - All agents produce coherent English.
   - No LLM dependency.
   - Speech is deterministic.
   - Grammar templates + lexicon complete (no placeholders).

3. **Planning Determinism:**
   - Agent planning deterministic.
   - Same percepts → identical plan.
   - Test automated; CI gated.

4. **Learning Mechanics:**
   - Agents learn from experience deterministically.
   - Skill progression reproducible via replay.

5. **Inner Monologue Always On:**
   - Every agent produces continuous thought stream.
   - Thoughts logged and observable.
   - No silent thinking (all thinking is auditable).

6. **Replay Determinism:**
   - Same seed + InputEvents → identical cognition state, thought streams, utterances.
   - Determinism replay test from Phase 3 still passes.

7. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 4, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 4:

- **No LLM dependency** (Section 4, PHASE 4, "Explicit Prohibition List").
  - Speech generation must use only deterministic NLG (templates + lexicon).
  - No calls to external LLM (OpenAI, Anthropic, etc.).
  - Exception: Optional post-processing for visualization, but never for authoritative speech.

- **No non-deterministic planning** (Section 1.1, "Determinism Law").
  - Planner (GOAP/HTN) must be deterministic.
  - Same inputs → same plan (no randomness in plan generation).
  - Exception: RNG for action probability (affect weighting), but not plan structure.

- **No unbounded thought generation** (Section 4, PHASE 4, "Explicit Prohibition List").
  - Thought generation must be bounded per tick.
  - Example: Agent thinks at most 5 thoughts per tick (configurable, not infinite).
  - No runaway cognition that delays tick advancement.

- **No hidden learning state** (Section 1.1, "Transparency Law").
  - All learning updates must emit `LearningUpdate` event.
  - No silent skill progression.
  - Memory consolidation is deterministic and observable.

- **No mind control or personality override** (Section 3.6, "Social Contracts & Trust").
  - Admin cannot force agent to like/dislike another agent (must use relationship event).
  - Admin cannot rewrite agent memories or beliefs (must use learning/experience events).
  - All personality changes must flow through deterministic cognition.

- **No placeholder NLG tokens** (Section 4, PHASE 4, "Explicit Prohibition List").
  - All utterances must be fully formed English.
  - No "{subject}" or "{verb}" tokens in output.
  - Template substitution must be complete.

*Sourced from Section 4, PHASE 4, "Explicit Prohibition List (Phase 4)," Section 1.1 "Determinism Law", and Section 3.6 "Social Contracts."*

## 11. Phase Completion Criteria (Checklist)

Phase 4 is NOT complete until ALL of the following are satisfied:

- [ ] **Offline cognition engine deterministic** — Same percepts + state → identical plans, thoughts, utterances
- [ ] **NLG produces coherent English** — No LLM; grammar templates + lexicon complete; 100% valid utterances
- [ ] **Planning system working** — GOAP/HTN planner implemented; agents form goals and plan sequences
- [ ] **Learning mechanics implemented** — Agents learn from experience; skill progression deterministic
- [ ] **Inner monologue always on** — Every tick produces thought stream; thoughts logged; no silent thinking
- [ ] **Speech generation deterministic** — Same emotion/memory/topic → identical utterance
- [ ] **All mandatory tests pass** — Planning, NLG, learning, thought stream, coherence tests
- [ ] **CI gates pass** — Build, cognition completeness, NLG determinism, planning determinism, learning, replay determinism gates
- [ ] **Relationship weighting integrated** — Friendship/enmity biases intent selection (affects plan priorities)
- [ ] **Determinism maintained from Phase 3** — Replay test from Phase 3 still passes

*Sourced from Section 4, PHASE 4, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.1, 2.3, 3.6, 4.0 (PHASE 4), and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The determinism guarantee, offline requirement, and transparency guarantee specified herein may never be weakened or suspended.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 4, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 4, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 4, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 4, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 4, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.1 "Event Pipeline"; Section 2.3 "Enforcement Points"; Section 3.6 "Social Contracts & Trust" |
| 7. Audit & Replay Requirements | Section 4, PHASE 4, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 4, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 4, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 4, "Explicit Prohibition List (Phase 4)"; Section 1.1 "Determinism Law", "Offline-First Law", "Transparency Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 4, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 3 (completed)
