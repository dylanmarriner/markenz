---
status: APPROVED
---

# PLAN_PHASE_3_NORMALIZED
## Embodied Biology v1

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 3 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.4)

---

## 1. ENTRY CONDITION
Phase 2 complete with all gates passing.

---

## 2. SCOPE (LOCKED)

Implement biological constraints and deterministic veto of unsafe actions.

**Deliverables:**
- Metabolism (energy, macronutrients, vitamins/minerals)
- Hydration (dehydration rate per activity/temperature)
- Thermoregulation (body temperature, thermoregulatory costs)
- Circadian rhythm + sleep (sleep debt, fatigue veto)
- Immune system (disease progression, infection risk)
- Injury/healing (wounds, healing rates, bandaging)
- Endocrine axes (hormones affecting stress, mood, reproduction readiness)
- BioVeto (enum-based veto reasons, not freeform strings)

---

## 3. NON-SCOPE

- Genetics (Phase 5)
- Reproduction mechanics (Phase 5)
- Cognition (Phase 4)
- Social systems (Phase 6)
- Governance (Phase 7)
- Rendering (Phase 8)
- Security (Phase 9)

---

## 4. PRESERVATION

Gem-D, Gem-K identities preserved. Initial bio states set from import.

---

## 5. DETERMINISM

### 5.1 BioVeto Reasons
- Enum-based (not freeform strings)
- Deterministic logic (same state → same veto)
- Variants: InsufficientEnergy, Fatigued, Dehydrated, Sick, Injured, Starving, etc.

### 5.2 Bio Progression
- All rates deterministic (energy cost per action, dehydration per tick, healing progress)
- Integer math only
- No randomization except where RNG explicitly used (e.g., infection risk from RNG draw)

### 5.3 State Hashing
- All bio state changes affect world_hash
- Replay produces identical bio state progression

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Metabolism
Causal: Action type + agent state  
State: Energy, nutrients  
Proof: Same action → same cost; replay produces identical values

### 6.2 Hydration
Causal: Activity, temperature, tick count  
State: Hydration level  
Proof: Dehydration rate deterministic

### 6.3 Circadian/Sleep
Causal: Sleep debt, tick count, sleep action  
State: Sleep debt, sleep quality, energy recovery  
Proof: Sleep debt progression deterministic

### 6.4 Immune System
Causal: Disease contract event, immune strength, tick count  
State: Disease progression, immune response  
Proof: Disease duration deterministic

### 6.5 Injury/Healing
Causal: Injury event, treatment, tick count  
State: Injury severity, heal progress, infection risk  
Proof: Healing timeline deterministic

### 6.6 BioVeto
Causal: Agent bio state, action  
State: Veto decision (enum)  
Proof: Same state → same veto reason

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_3_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_3_EXECUTION_REPORT.md

---

## 8. EXIT CRITERIA

### Biology Systems
- [ ] Metabolism deterministic
- [ ] Hydration deterministic
- [ ] Thermoregulation deterministic
- [ ] Circadian/sleep deterministic
- [ ] Immune progression deterministic
- [ ] Injury healing deterministic
- [ ] Hormone updates deterministic

### BioVeto
- [ ] VetoReason enum complete
- [ ] Veto logic deterministic
- [ ] Vetoed actions logged with reason
- [ ] No freeform strings in veto

### Determinism
- [ ] Phase 2 tests still pass
- [ ] Bio determinism test passes
- [ ] Veto consistency test passes
- [ ] World hash unaffected regression

### Integration
- [ ] Engine runs bio_tick per tick
- [ ] Veto pass integrated into pipeline
- [ ] Veto rejections logged as ObservationEvents
- [ ] UI shows vitals and health status

### AMP Sign-Off
- [ ] AMP approval BEFORE Phase 4

---

## 9. GATES

**Gate 1: Bio Determinism (TEST-BIO-001)**  
**Gate 2: Veto Consistency (TEST-VETO-001)**  
**Gate 3: Phase 2 No Regression (TEST-P2-REGRESS-001)**

STOP if any fail.

---

**END OF PHASE 3 NORMALIZED PLAN**
