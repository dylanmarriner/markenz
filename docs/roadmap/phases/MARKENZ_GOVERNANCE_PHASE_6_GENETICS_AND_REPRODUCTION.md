---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 6
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_5_SOCIAL_DYNAMICS_AND_MULTI_AGENT_SCALING
---

# MARKENZ — GOVERNANCE PHASE 6: GENETICS AND REPRODUCTION (DOUBLE HELIX)

## 1. Phase Objective

Implement population growth via deterministic genetics and reproductive biology. This phase introduces Mendelian inheritance, phenotype expression, and the complete reproduction pipeline from conception through birth.

## 2. Governance Domains In Scope

- **Reproduction controls** (Mendelian inheritance, phenotype expression, consent-based conception)
- **Founder amplification** (offspring of founders always have baseline amplification multipliers)

*Sourced from Section 4, PHASE 6, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/genetics` (double-helix genome, recombination, mutation, phenotype expression)
- `crates/biology` — Reproduction pipeline (consent → intercourse → conception → gestation → birth)
- `crates/world` — Lineage tracking (family trees, inheritance chains)

*Sourced from Section 4, PHASE 6, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 6 MUST be defined and logged:

- `GenomeCreated` (genome_id, alleles, parent1_genome, parent2_genome)
- `MutationOccurred` (genome_id, locus, mutation_type, policy_check)
- `ReproductionConsentGiven` (agent_a, agent_b, tick)
- `ConceptionOccurred` (offspring_genome_id, tick, parent1_id, parent2_id)
- `GestationProgressed` (offspring_id, stage, health)
- `BirthOccurred` (offspring_id, parent1_id, parent2_id, phenotype_hash, amplification_multipliers)

*Sourced from Section 4, PHASE 6, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 6 completion, the following properties MUST hold:

- **Genetics Determinism:** Same parents + seed → identical offspring genome (allele sequence).
- **Phenotype Determinism:** Same genome + astrological birth time (seed) → identical trait expressions.
- **Amplification Determinism:** All non-founder offspring always have baseline (1.0) amplification multipliers; founders' children always baseline (never inherit amplification).
- **Reproduction Determinism:** Same consent + fertile state → identical conception probability (seeded RNG).
- **Gestation Determinism:** Same conception tick + parental health → identical gestation timeline and birth outcome.

*Sourced from Section 4, PHASE 6, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### Mendelian Reproduction

- **Double-helix genome:** Each agent has diploid genome (two alleles per locus).
- **Meiosis:** Reproduction via meiosis (not cloning). Offspring receive one allele from each parent per locus (recombination).
- **Allele Selection:** Allele inheritance determined by RNG (seeded), recorded in `GenomeCreated` event.
- **Forbidden:** Cloning, template duplication, miraculous reproduction (magic, prayer).

*Sourced from Section 3.3 "Reproduction Controls."*

### Phenotype Expression

- **Genotype → Phenotype:** Alleles determine phenotype (trait expression) deterministically.
- **Epistasis & Dominance:** Allele interactions (dominance, recessiveness, epistasis) are pre-computed.
- **Birth Time Modulation:** Astrological birth time (seed offset) modulates phenotype expression (e.g., birth under certain astro conditions slightly increases trait value).

### Consent Requirement

- **Two Agents Involved:** Reproduction requires two agents in compatible reproductive state (fertility window, health, willingness).
- **Explicit Consent:** Both agents must emit `ReproductionConsentGiven` event for same tick (mutual agreement).
- **Conception Probability:** After consent, conception is probabilistic (seeded RNG); fertile window increases conception probability.

### Amplification Non-Inheritance (MANDATORY)

- **Founder Offspring Baseline:** If both parents are founders (Gem-D, Gem-K), offspring have baseline (1.0) amplification.
- **Non-Founder Offspring Baseline:** All non-founder offspring ALWAYS have baseline amplification, regardless of parent status.
- **No Amplification Inheritance:** Amplification multipliers are NOT inherited genetically; they are state-only (parent's multiplier does not affect offspring).
- **Enforcement:** `BirthOccurred` event must validate that offspring amplification multipliers are all 1.0; birth fails if not.

*Sourced from Section 1.1 "Founder Amplification Law," Section 3.3 "Reproduction Controls."*

### Gestation Stages

- **Trimester-based:** Full developmental timeline for humans (9 gestational months = 360 ticks, approximately).
- **Stage Progression:** Gestation progresses through stages (early, mid, late); each stage has milestones.
- **Health Tracking:** Offspring health monitored; poor nutrition/stress on mother may cause miscarriage.
- **Birth Event:** Discrete birth event at gestation completion; offspring becomes independent agent.

*Sourced from Section 3.3 "Reproduction Controls."*

### Lineage Tracking

- **Family Trees:** Lineage maintained; each agent knows parents, siblings, offspring.
- **Inheritance Relationships:** Marital status (if law implements marriage) and lineage used for inheritance (Phase 7+).
- **Immutable Lineage:** Lineage cannot be retroactively changed (parents are fixed at birth).

## 7. Audit & Replay Requirements

### Genome Audit

- `tools/audits/genome_audit.py` verifies alleles per locus.
- Report shows: allele frequencies in population, deviation from Hardy-Weinberg (if modeled).
- Verification: Known parents + seed → expected offspring genome matches actual.

### Lineage Tree Completeness

- Audit report: full lineage tree with birth ticks, parents, siblings, offspring.
- Verification: no orphaned agents (all agents have valid parents in log or are founders).
- Pedigree analysis: population diversity, genetic drift over generations.

### Phenotype Prediction vs. Observed

- Audit tool predicts phenotype from genome + birth time.
- Comparison: predicted traits vs. actual agent traits (should match).
- Report shows: accuracy of phenotype prediction; any divergences flagged.

### Amplification Validation

- At birth, verify offspring amplification multipliers are 1.0.
- Audit report: amplification values across all agents; flag any non-baseline for non-founders.
- CI gate: no non-baseline amplification in population except Gem-D and Gem-K.

*Sourced from Section 4, PHASE 6, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 7.

### 8.1 Genetics Determinism Test

**Requirement:** Known seed + parents → expected offspring genome (allele sequence).

**Acceptance Criteria:**
- Parent A genome: [A1, a1], [B1, B1], [c1, c2] (example loci).
- Parent B genome: [A2, A2], [B1, b1], [c1, c1].
- Seed S, conception tick T → Offspring genome (deterministic RNG from seed).
- Offspring 1: [A1, A2], [B1, B1], [c1, c1] (one allele from each parent per locus).
- Replay same seed → Offspring 2: identical genome.
- Multiple families, multiple crosses tested.
- Test automated; CI gated.

### 8.2 Phenotype Test

**Requirement:** Genome → trait prediction deterministic.

**Acceptance Criteria:**
- Genome [A1, A1], [B1, b1], [c1, c2] + birth time T → Trait values: height=180cm, eye_color=brown, intelligence=110.
- Replay same genome + birth time → identical trait values.
- Multiple agents, multiple trait combinations tested.
- Test automated; CI gated.

### 8.3 Offspring Baseline Amplification Test

**Requirement:** All non-founder offspring have baseline (1.0) amplification; founders' children also baseline.

**Acceptance Criteria:**
- Parent 1: Gem-D (founder, amplification={learning_rate: 1.5, strength: 1.2}).
- Parent 2: Regular agent (baseline amplification={all: 1.0}).
- Birth event: Offspring has amplification={all: 1.0} (baseline, not inherited).
- Replay same birth → offspring always baseline amplification.
- Test: Gem-D × Gem-D (both founders) → offspring baseline.
- Test: Gem-D × regular → offspring baseline.
- Test: regular × regular → offspring baseline.
- Test automated; CI gated; fails if any non-founder has non-baseline or any offspring has inherited amplification.

### 8.4 Lineage Integrity Test

**Requirement:** Family tree consistency; no orphaned agents or impossible lineages.

**Acceptance Criteria:**
- After 1000 ticks of population growth, generate lineage report.
- Verify: all agents have valid parents (except founders at genesis).
- Verify: no agent is their own ancestor (no cycles).
- Verify: parent ages > offspring ages (temporal consistency).
- Verify: agent can have at most 2 parents (no cloning).
- Test automated; audit report generated.

### 8.5 Reproduction Mechanics Test

**Requirement:** Conception requires consent; gestation takes deterministic time; birth creates new agent.

**Acceptance Criteria:**
- Two agents in fertile state and compatible (different sex if binary, or appropriate for species).
- Both emit `ReproductionConsentGiven` for same tick.
- Engine processes conception: `ConceptionOccurred` event emitted; offspring genome created.
- Gestation progresses deterministically: 360 ticks (or species-specific time).
- Birth event: new agent created with offspring genome, phenotype, baseline amplification.
- Replay same conception → identical offspring (genome, birth time, phenotype).
- Test automated; CI gated.

*Sourced from Section 4, PHASE 6, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 6 is considered complete:

1. **Reproduction Pipeline Implemented:**
   - Consent mechanism working.
   - Conception/gestation/birth events produced.
   - New agents created at birth with correct genetics.

2. **Birth Creates Agent with Correct Genetics:**
   - Offspring genome deterministic (seeded RNG).
   - Phenotype matches genome + birth time.
   - Agent created with correct name, identity, relationships.

3. **Offspring Always Baseline Amplification (Test Enforced):**
   - CI gate: verify all non-founder agents have amplification={all: 1.0}.
   - Gate fails if any non-founder has non-baseline multiplier.
   - Gate fails if any offspring inherited parent's amplification.

4. **Lineage Trees Observable:**
   - Audit tool generates lineage report.
   - UI shows family relationships (if implemented).
   - No orphaned agents detected.

5. **Replay Produces Identical Genetics:**
   - Same seed + parents + conception → identical offspring.
   - Determinism replay test from Phase 5 still passes.

6. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 6, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 6:

- **No cloning** (Section 3.3 "Reproduction Controls").
  - Offspring must be genetically unique (via meiosis/recombination).
  - Creating genetic copy of agent is forbidden.

- **No template duplication** (Section 3.3 "Reproduction Controls").
  - Cannot duplicate agent agent as shortcut to reproduction.
  - Each agent must be born via conception/gestation/birth pipeline.

- **No miraculous reproduction** (Section 3.3 "Reproduction Controls").
  - Reproduction requires two biological parents (no magic, prayer, intervention).
  - No "virgin birth" or spontaneous generation.

- **No bypassing meiosis/recombination** (Section 3.3 "Reproduction Controls").
  - Offspring must inherit via allele recombination (not genome copy).
  - Mutation policy (if any) must be explicitly modeled (not "oops, wrong allele").

- **No implicit amplification inheritance** (Section 1.1 "Founder Amplification Law").
  - Amplification NEVER inherited genetically.
  - Offspring amplification ALWAYS baseline, regardless of parent status or amplification.
  - Code MUST enforce: offspring_amplification = {all: 1.0}.

- **No retroactive lineage changes** (Section 4, PHASE 6, "Explicit Prohibition List").
  - Parents are immutable at birth (recorded in `BirthOccurred` event).
  - Cannot reassign parents retroactively.

- **No unlogged births** (Section 1.1, "Transparency Law").
  - Every birth MUST emit `BirthOccurred` event.
  - No silent agent creation.

*Sourced from Section 4, PHASE 6, "Explicit Prohibition List (Phase 6)," Section 3.3 "Reproduction Controls," Section 1.1 "Founder Amplification Law".*

## 11. Phase Completion Criteria (Checklist)

Phase 6 is NOT complete until ALL of the following are satisfied:

- [ ] **Genetics system deterministic** — Known seed + parents → expected offspring genome; test passes
- [ ] **Offspring generated with Mendelian inheritance** — Meiosis/recombination working; alleles properly inherited
- [ ] **All non-founders baseline amplification proven** — CI gate enforces; no non-founder has non-baseline multiplier
- [ ] **Founder offspring always baseline** — Gem-D × anyone → offspring baseline amplification (test passes)
- [ ] **Lineage tracking working** — Family trees complete; no orphaned agents; UI shows relationships
- [ ] **Phenotype expression deterministic** — Genome + birth time → trait values reproducible
- [ ] **Reproduction consent mechanism working** — Two agents can consent; conception follows; gestation deterministic
- [ ] **Birth creates independent agent** — New agent spawned with correct genetics, identity, relationships
- [ ] **All mandatory tests pass** — Genetics, phenotype, amplification baseline, lineage, reproduction tests
- [ ] **CI gates pass** — Build, reproduction pipeline, amplification baseline, lineage, replay determinism gates
- [ ] **Determinism maintained from Phase 5** — Replay test from Phase 5 still passes; multi-agent hashes stable

*Sourced from Section 4, PHASE 6, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 3.3, 4.0 (PHASE 6), and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The genetics determinism guarantee, non-inheritance of amplification, and baseline amplification enforcement specified herein are inviolable.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 6, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 6, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 6, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 6, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 6, "Determinism Guarantees"; Section 1.1 "Determinism Law", "Founder Amplification Law" |
| 6. Enforcement Rules | Section 3.3 "Reproduction Controls"; Section 1.1 "Founder Amplification Law" |
| 7. Audit & Replay Requirements | Section 4, PHASE 6, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 6, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 6, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 6, "Explicit Prohibition List (Phase 6)"; Section 1.1 "Founder Amplification Law"; Section 3.3 "Reproduction Controls" |
| 11. Phase Completion Criteria | Section 4, PHASE 6, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 5 (completed)
