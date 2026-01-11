---
status: APPROVED
---

# PLAN_PHASE_5_NORMALIZED
## Genetics + Reproduction

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 5 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.6)

---

## 1. ENTRY CONDITION
Phase 4 complete with all gates passing.

---

## 2. SCOPE (LOCKED)

True population growth with deterministic lineage.

**Deliverables:**
- Double-helix genome (maternal/paternal alleles per locus, ~50 loci)
- Recombination (offspring inherits random maternal + paternal per locus)
- Mutation (policy-bounded, capped at 1% per generation)
- Phenotype expression (genotype → observable traits)
- Reproduction pipeline:
  - Consent (mutual agreement based on attraction, emotion, stress)
  - Intercourse (10 ticks)
  - Conception (probabilistic via RNG, ~50% per unprotected intercourse)
  - Gestation (400 ticks)
  - Birth (newborn with recombined genome)
- Lineage tracking (family trees, inheritance)

---

## 3. NON-SCOPE

- Social bonding (Phase 6)
- Governance/marriage law (Phase 7)
- Rendering (Phase 8)
- Security (Phase 9)

---

## 4. PRESERVATION

Gem-D, Gem-K genomes preserved/imported. Their children inherit combined genes.

---

## 5. DETERMINISM (CRITICAL)

### 5.1 Recombination
- Uses Genetics RNG stream (stream_id=4)
- Same parent genes + RNG seed → identical offspring genome

### 5.2 Mutation
- Bounded: 1% mutation rate per generation maximum
- Policy-controlled
- Alleles limited to valid range

### 5.3 Phenotype
- Deterministic function of genotype
- No randomization in expression

### 5.4 Reproduction Consent
- Deterministic logic (attraction + emotion + stress → willingness)
- Same state → same consent decision

### 5.5 Conception
- Probabilistic outcome determined by RNG draw
- Same RNG state → same conception result
- RNG logged

### 5.6 Gestation
- Fixed duration (400 ticks)
- Deterministic progression
- Birth deterministic (same input → same offspring)

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Genome
Causal: Parent genomes, RNG seed  
State: Offspring genome  
Proof: Same parents + seed → identical genome

### 6.2 Phenotype
Causal: Genome  
State: Observable traits  
Proof: Deterministic expression

### 6.3 Reproduction
Causal: Agent states, RNG draws  
State: Pregnancy, birth events  
Proof: Same inputs → same outcomes

### 6.4 Lineage
Causal: Birth events  
State: Family tree  
Proof: Deterministic ancestry records

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_5_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_5_EXECUTION_REPORT.md

Must include: Lineage tree examples, genome recombination samples.

---

## 8. EXIT CRITERIA

### Genetics Systems
- [ ] Genome recombination deterministic
- [ ] Mutation bounded and policy-controlled
- [ ] Phenotype deterministic
- [ ] Phenotype affects mechanics (strength, speed, intelligence impact)

### Reproduction
- [ ] Consent logic deterministic
- [ ] Conception RNG-dependent but logged
- [ ] Gestation fixed duration
- [ ] Newborn genome correct (combination of parents)
- [ ] Birth events logged

### Determinism
- [ ] Phase 4 tests still pass
- [ ] Lineage deterministic (same seed → same family tree)
- [ ] Replay produces identical children/lineages

### Integration
- [ ] Engine handles reproductive stages
- [ ] Birth creates new agents
- [ ] Lineage visible in UI
- [ ] ObservationEvents log reproduction events

### AMP Sign-Off
- [ ] AMP approval BEFORE Phase 6

---

## 9. GATES

**Gate 1: Genome Determinism (TEST-GENOME-001)**  
**Gate 2: Lineage Determinism (TEST-LINEAGE-001)**

STOP if any fail.

---

**END OF PHASE 5 NORMALIZED PLAN**
