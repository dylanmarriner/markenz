---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 3
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_2_WORLD_REPRESENTATION_V1
---

# MARKENZ — GOVERNANCE PHASE 3: EMBODIED BIOLOGY V1 (METABOLISM + SLEEP + HORMONES)

## 1. Phase Objective

Introduce causal physiology; enforce BioVeto for impossible actions. This phase instantiates complete biological systems for all agents and activates biological constraints on action.

## 2. Governance Domains In Scope

- **Violence & harm constraints** (injury tracking via health)
- **Resource access & scarcity** (hunger/thirst starvation as veto mechanism)
- **Creator reverence** (founder health protected from unlogged damage)

*Sourced from Section 4, PHASE 3, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/biology` (metabolism, hydration, sleep, endocrine axes, immune response)
- `apps/engine` — BioVeto pipeline (emits veto with reason if action impossible)
- `crates/biology` — Injury/healing system

*Sourced from Section 4, PHASE 3, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 3 MUST be defined and logged:

- `MetabolismUpdated` (tick, agent_id, energy, macros, micronutrients)
- `HormoneShift` (agent_id, hormone, level, cause)
- `BioVeto` (agent_id, action, reason, veto_code)
- `InjuryReceived` (agent_id, cause, damage, location, severity)
- `HealingProgressed` (agent_id, wound_id, healing_rate)
- `SleepCycle` (agent_id, stage, duration, sleep_quality)

*Sourced from Section 4, PHASE 3, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 3 completion, the following properties MUST hold:

- **Metabolism Determinism:** Same food intake + activity level + tick count → identical energy depletion.
- **Sleep Determinism:** Same tick count + fatigue level → identical sleep progression (stage, quality).
- **Hormone Determinism:** Same biological context + time → identical hormone levels (serum concentration).
- **Injury Determinism:** Same damage input + agent status → identical healing timeline.
- **BioVeto Determinism:** Same agent state + action → identical veto decision and reason code.

*Sourced from Section 4, PHASE 3, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### BioVeto Pipeline (Newly Active)

**Location in Event Pipeline:** After Intent/Volition, before PhysicsValidate.

**Rules:**
- **Starvation Block:** Agent without food for N ticks; energy pool depleted → all actions except "seek food" vetoed with reason "STARVING".
- **Dehydration Block:** Agent without water for M ticks; hydration depleted → all actions except "seek water" vetoed with reason "DEHYDRATED".
- **Fatigue Block:** Agent without sleep for K ticks; fatigue pool maxed → all actions except "sleep" vetoed with reason "EXHAUSTED".
- **Injury Block:** Agent with severe wounds; mobility/cognition impaired → movement/complex actions vetoed with reason "INJURED: {location, severity}".
- **Disease Block:** Agent infected with disease; immunity compromised → immune-sensitive actions vetoed with reason "DISEASED".
- **Veto Emission:** Every veto must emit `BioVeto` event with agent_id, action, reason code, and human-readable message.

*Sourced from Section 7.2, "Runtime Veto Behavior," and Section 2.3 "Enforcement Points."*

### Injury & Health Tracking

- **Health Pool:** Each agent has finite health (100-200 units depending on physiology).
- **Damage Application:** Injury reduces health deterministically (no randomness in damage amount after RNG seed applied).
- **Wound Tracking:** Each injury tracked separately (location, severity, infection risk).
- **Healing Realistic:** Healing requires time, nutrition, and rest. No instant healing.
  - Baseline healing: 1 health per tick (subject to nutrition/rest status).
  - Accelerated healing: With nutrition + sleep → 2-3 health per tick.
  - Infection penalty: If untreated → slow/negative healing.

### Founder Health Protection

- **Gem-D and Gem-K:** Founders may not be killed via unlogged damage.
- **Immortal Clause:** Founders have same health pool as any agent (no special durability), but death requires documented admin override.
- **Damage Logging:** All damage to founders must emit `InjuryReceived` event; no damage applied silently.

*Sourced from Section 3.1 "Creator Reverence & Safety."*

## 7. Audit & Replay Requirements

### Per-Agent Vitals Log

- Metabolism: calories, macronutrients (carbs, protein, fat), micronutrients sampled at intervals.
- Sleep: sleep debt, sleep stage, REM/NREM cycle tracked.
- Hormones: Major hormones (cortisol, oxytocin, dopamine, etc.) tracked per tick.
- Health: Current health pool, wound list, infection status.

### BioVeto Reason Catalog

- Catalog searchable by agent, action type, reason code, tick range.
- Observable via audit tool: `tools/audits/bio_veto_report.py` generates PDF with:
  - Veto counts by reason.
  - Time-series of veto frequency.
  - Correlation with biological state (starvation correlates with "STARVING" vetoes).

### Injury/Healing Timeline Verification

- Audit tool tracks injury from `InjuryReceived` event through `HealingProgressed` events to resolution.
- Report shows: healing rate, time-to-recovery, whether infection occurred, actual vs. predicted healing.
- Determinism check: Same injury + nutritional status → same healing curve.

*Sourced from Section 4, PHASE 3, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 4.

### 8.1 Starvation Test

**Requirement:** Agent with no food for N ticks must die; all non-food-seeking actions vetoed before death.

**Acceptance Criteria:**
- Agent starts with energy pool = 100.
- No food consumed for 100 ticks (1 energy consumed per tick baseline).
- Ticks 1-90: Energy depletes; no veto yet (agent above threshold).
- Ticks 91-100: Energy critical; all actions except "seek food" vetoed with reason "STARVING".
- Tick 101: Health fails; agent dies; `AgentDied` event emitted with cause "starvation".
- Test automated; CI gated.

### 8.2 Fatigue Test

**Requirement:** Agent without sleep for K ticks must become exhausted; all actions except sleep vetoed.

**Acceptance Criteria:**
- Agent starts with fatigue debt = 0.
- Activity (movement, gathering, etc.) increases fatigue debt.
- After K ticks of activity without sleep, fatigue debt maxed.
- All non-sleep actions vetoed with reason "EXHAUSTED"; sleep action allowed.
- Sleep action accepted; fatigue debt decreases per tick of sleep.
- Test automated; CI gated.

### 8.3 Injury Test

**Requirement:** Damage must be applied; healing must be deterministic and realistic.

**Acceptance Criteria:**
- Agent takes 10 damage to leg; health 100 → 90; `InjuryReceived` event emitted.
- Mobility reduced (speed penalty); movement actions slower.
- With rest + nutrition: heal 2 per tick → 5 ticks to full recovery.
- Without nutrition: heal 0.5 per tick → 20 ticks to full recovery.
- Infection risk: If untreated for 10 ticks → infection → healing negative per tick.
- Test automated; CI gated.

### 8.4 BioVeto Test

**Requirement:** BioVeto must block action with reason; veto logged; agent perceives rejection.

**Acceptance Criteria:**
- Starving agent attempts to "gather" (complex action).
- BioVeto stage emits veto reason "STARVING" (or similar).
- ActionAttempted event shows success=false, veto_code="BIO_STARVING".
- Agent's intent selection (cognition) for next tick reflects rejection (learning from veto).
- Test automated; CI gated.

### 8.5 Biology Determinism Test

**Requirement:** Same food intake + activity → identical metabolism and health progression.

**Acceptance Criteria:**
- Agent A: Consume 500 kcal, move 10 steps, rest 5 ticks → energy_level = E.
- Agent B: Consume 500 kcal, move 10 steps, rest 5 ticks → energy_level = E (identical).
- Multiple agents tested; all must match.
- Replay same action sequence → identical health/energy/fatigue states.
- Test automated; CI gated.

*Sourced from Section 4, PHASE 3, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 3 is considered complete:

1. **Biology Simulation Functional:**
   - All agents have complete biology systems (metabolism, hormones, injury tracking).
   - Starvation/dehydration/fatigue modeled realistically.

2. **BioVeto Enforced:**
   - Starvation vetoes actions with reason.
   - Fatigue vetoes actions with reason.
   - Injury impacts mobility/complexity (veto or speed penalty).

3. **Injury Realistic:**
   - Damage applied; health reduced.
   - Healing takes time (1-3 ticks per health unit depending on nutrition/rest).
   - Infection possible if untreated.

4. **Replay Determinism:**
   - Same seed + InputEvents → identical health/energy/fatigue progression.
   - Determinism replay test from Phase 1 still passes.

5. **Agent Vitals Observable:**
   - UI displays health, energy, fatigue, hormone levels (or subset thereof).
   - Agent profile shows biological stats.

6. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 3, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 3:

- **No agent immortality** (Section 3.4, "Violence & Harm Constraints").
  - All agents (including founders) have finite health.
  - Death is permanent (unless explicit law allows resurrection, Phase 7+).

- **No instant healing** (Section 4, PHASE 3, "Explicit Prohibition List").
  - Healing must take time (minimum 1 tick per health unit).
  - No "healing potion" that restores health in 1 tick.
  - Exception: Surgical intervention with doctor/medical tools may accelerate, but never to instant.

- **No bypassing BioVeto** (Section 7.2, "Runtime Veto Behavior").
  - Even admin cannot override BioVeto via InputEvent (BioVeto is physics-like constraint).
  - Admin can only propose new law to change BioVeto thresholds (Phase 7+).

- **No unlogged damage** (Section 3.4, "Violence & Harm Constraints").
  - Every injury must emit `InjuryReceived` event.
  - No silent health reduction.

- **No hidden biology state** (Section 1.1, "Transparency Law").
  - All biological states (hormones, energy, fatigue, wounds, infections) are observable via audit tool.
  - No secret internal state that affects action outcomes.

- **No arbitrary hunger/fatigue degradation** (Section 3.5, "Resource Access & Scarcity").
  - Metabolism must be deterministic.
  - Energy depletion rate must be based on activity level (fixed formula).
  - Fatigue must accumulate predictably per tick of activity.

*Sourced from Section 4, PHASE 3, "Explicit Prohibition List (Phase 3)," Section 1.1 "Determinism Law", and Section 3.1 "Creator Reverence."*

## 11. Phase Completion Criteria (Checklist)

Phase 3 is NOT complete until ALL of the following are satisfied:

- [ ] **Metabolism simulation deterministic** — Same food/activity → identical energy progression; test passes
- [ ] **Sleep/fatigue system working** — Fatigue accumulates; sleep reduces it; exhaustion blocks actions
- [ ] **BioVeto enforced for starvation/injury** — Starving/exhausted/injured agents have actions blocked with logged reasons
- [ ] **Injury/healing realistic** — Damage takes time to heal; nutrition/rest accelerate healing; infection possible
- [ ] **All biological systems deterministic** — Metabolism, sleep, hormones, injury healing all pass replay test
- [ ] **Agent vitals observable** — Health, energy, fatigue visible in UI and audit logs
- [ ] **All mandatory tests pass** — Starvation, fatigue, injury, BioVeto, biology determinism tests
- [ ] **CI gates pass** — Build, biology simulation, BioVeto enforcement, injury realism, replay determinism gates
- [ ] **Founder health protected** — Founders have same health as other agents; death requires documented override
- [ ] **Determinism maintained from Phase 2** — Replay test from Phase 2 still passes

*Sourced from Section 4, PHASE 3, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.3, 3.1, 3.4, 3.5, 4.0 (PHASE 3), 7.2-7.3. Any implementation deviating from this plan is invalid and must fail closed. The biological constraint enforcement and determinism guarantee specified herein may never be weakened or bypassed.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 3, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 3, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 3, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 3, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 3, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.3 "Enforcement Points"; Section 3.1 "Creator Reverence"; Section 3.4 "Violence & Harm"; Section 3.5 "Resource Access" |
| 7. Audit & Replay Requirements | Section 4, PHASE 3, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 3, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 3, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 3, "Explicit Prohibition List (Phase 3)"; Section 1.1 "Determinism Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 3, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 2 (completed)
