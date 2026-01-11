# NORMALIZED PLAN INDEX

**STATUS:** Authority Reference Document  
**TIMESTAMP:** 2026-01-11  
**AUTHORITY:** KAIZA-MCP + MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md

---

## Overview

This index lists all normalized phase plans and their authority hierarchy. It is the **single source of truth for Markenz global phases 0–9**.

**Golden Rule:** Windsurf executes ONLY normalized plans. Prior versions (v1, v2, v3, M1) are SUPERSEDED.

---

## Normalized Phase Plans (Phases 0–9)

| Phase | Filename | Absolute Path | Status | Entry Condition | Primary Deliverables |
|-------|----------|---|---|---|---|
| **0** | PLAN_PHASE_0_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_0_NORMALIZED.md | EXECUTABLE | Repo structure exists | Stack bootstrap, OIDC, event log, genesis |
| **1** | PLAN_PHASE_1_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_1_NORMALIZED.md | EXECUTABLE | Phase 0 gates pass | RNG streams, snapshots, determinism proof |
| **2** | PLAN_PHASE_2_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_2_NORMALIZED.md | EXECUTABLE | Phase 1 gates pass | Terrain, inventory, world actions |
| **3** | PLAN_PHASE_3_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_3_NORMALIZED.md | EXECUTABLE | Phase 2 gates pass | Biology: metabolism, hydration, bio-veto |
| **4** | PLAN_PHASE_4_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_4_NORMALIZED.md | EXECUTABLE | Phase 3 gates pass | Cognition: perception, GOAP planning, language |
| **5** | PLAN_PHASE_5_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_5_NORMALIZED.md | EXECUTABLE | Phase 4 gates pass | Genetics: genome, reproduction, lineage |
| **6** | PLAN_PHASE_6_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_6_NORMALIZED.md | EXECUTABLE | Phase 5 gates pass | Social: relationships, reputation, culture |
| **7** | PLAN_PHASE_7_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_7_NORMALIZED.md | EXECUTABLE | Phase 6 gates pass | Governance: property, markets, laws |
| **8** | PLAN_PHASE_8_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_8_NORMALIZED.md | EXECUTABLE | Phase 7 gates pass | Rendering: WebGPU, time-travel UI |
| **9** | PLAN_PHASE_9_NORMALIZED.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/normalized/PLAN_PHASE_9_NORMALIZED.md | EXECUTABLE | Phase 8 gates pass | Security: encryption, audit hardening |

---

## Authority Hierarchy (Governing Documents)

**Higher authority supersedes lower. Windsurf must defer to ALL of these.**

```
1. KAIZA-MCP (governance system)
   ↓
2. MARKENZ_EXECUTION_ROADMAPv2.md (global roadmap, phases 0–9)
   ↓
3. MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (master plan, phases 0–9)
   ↓
4. MARKENZ_TARGET_ARCHITECTUREv2.md (locked architecture)
   ↓
5. AMP_DEFINITION_OF_DONEv2.md (quality gates, no-mock enforcement)
   ↓
6. MARKENZ_SELF_EVOLUTION_AND_GROWTH_LAW_v2.md (evolution boundaries)
   ↓
7. MARKENZ_REPO_REFACTOR_MAPv2.md (repo structure, authority boundaries)
   ↓
8. KAIZA_COMPLETE_GUIDE.md (MCP execution discipline)
   ↓
9. ADDENDUM_WORLD_PRESERVATION_v1.md (asset preservation mandate)
   ↓
10. ADDENDUM_IDENTITY_CONTINUITY_v1.md (identity preservation mandate)
```

All normalized phase plans defer to this hierarchy. Windsurf **must not** invent higher authorities.

---

## Superseded Documents (DO NOT USE)

These documents are **SUPERSEDED** by normalized plans. Windsurf **MUST NOT** reference them:

- ❌ MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v1.md (incomplete phases)
- ❌ MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v1.md (legacy)
- ❌ MARKENZ_GEMINI_TO_MARKENZ_INTEGRATION_PLAN_v2.md (legacy)
- ❌ MARKENZ_REUSE_MIGRATION_PLAN.md (legacy)
- ❌ MARKENZ_REUSE_MIGRATION_PLAN_v2.md (legacy)
- ❌ MARKENZ_REUSE_MIGRATION_PLAN_v3.md (legacy)
- ❌ MARKENZ_M1_FOUNDATION.md (subordinate milestone; content mapped to global phases)

---

## Reference Documents (EVIDENCE ONLY)

These documents are **reference-only**. They are NOT execution authority but provide context:

- 📋 M1_TO_GLOBAL_PHASE_CROSSWALK.md (maps M1 to global phases; clarifies supersession)
- 📋 AMP_PLAN_NORMALIZATION_REPORT.md (normalization summary and audit trail)
- 📋 All audit reports in /media/linnyux/development3/developing/gemini_universe/markenz/docs/audits/ (evidence of prior work)
- 📋 All execution reports in /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/ (evidence of prior execution)

---

## Required Windsurf Outputs

**Windsurf MUST produce exactly ONE execution report per phase, at the path specified:**

| Phase | Report Filename | Absolute Path |
|-------|---|---|
| 0 | WINDSURF_PHASE_0_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_0_EXECUTION_REPORT.md |
| 1 | WINDSURF_PHASE_1_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_1_EXECUTION_REPORT.md |
| 2 | WINDSURF_PHASE_2_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_2_EXECUTION_REPORT.md |
| 3 | WINDSURF_PHASE_3_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_3_EXECUTION_REPORT.md |
| 4 | WINDSURF_PHASE_4_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_4_EXECUTION_REPORT.md |
| 5 | WINDSURF_PHASE_5_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_5_EXECUTION_REPORT.md |
| 6 | WINDSURF_PHASE_6_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_6_EXECUTION_REPORT.md |
| 7 | WINDSURF_PHASE_7_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_7_EXECUTION_REPORT.md |
| 8 | WINDSURF_PHASE_8_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_8_EXECUTION_REPORT.md |
| 9 | WINDSURF_PHASE_9_EXECUTION_REPORT.md | /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_9_EXECUTION_REPORT.md |

---

## Execution Rules (BINDING)

1. **Phase Ordering:** Phase 0 must 100% complete before Phase 1 begins. Same for all phases.
2. **Gate Enforcement:** All exit criteria in each phase plan are MANDATORY. Failure blocks next phase.
3. **No Skipping:** Windsurf may not skip phases or gates.
4. **Authority Deference:** If conflict detected, refer to higher authority in hierarchy above.
5. **No-Mock / No-Stub:** Per AMP_DEFINITION_OF_DONEv2.md, all code must be production-ready.
6. **Escalation:** If Windsurf cannot satisfy a criterion, escalate to AMP immediately with evidence.

---

## Normalized Plan Structure (LOCKED)

Every normalized phase plan contains (in order):

1. Title & metadata (phase number, authority, status)
2. Authority declaration (higher authorities deferred to)
3. Phase scope (LOCKED deliverables)
4. Explicit non-scope (forbidden in this phase)
5. Preservation clauses (assets, identities)
6. Determinism requirements (RNG, hashing, replay)
7. Implementation obligations (anti-fake checklist)
8. Required artifacts (Windsurf output path + filename)
9. Exit criteria (verifiable, all required)
10. Determinism & replay gates (specific tests)
11. Hard stop conditions (execution halts)

---

## How to Use This Index

**For Windsurf (Executor):**
1. Read NORMALIZED_PLAN_INDEX.md (this file) first
2. For each phase in sequence:
   - Read PLAN_PHASE_N_NORMALIZED.md completely
   - Understand all higher authorities it defers to
   - Execute against specifications
   - Produce report at path specified in phase plan
3. Do NOT reference superseded documents
4. Escalate blockers to AMP immediately

**For AMP (Auditor):**
1. Review this index to confirm all phases present
2. Review each normalized plan for compliance with governing documents
3. Verify all superseded documents are no longer in use
4. Sign off on each phase after Windsurf reports

**For Other Readers:**
1. Use AUTHORITY HIERARCHY section to understand governance
2. Use M1_TO_GLOBAL_PHASE_CROSSWALK.md to understand why M1 is superseded
3. Refer to AMP_PLAN_NORMALIZATION_REPORT.md for normalization decisions

---

**This document is the single source of truth for Markenz phase ordering and authority.**

END OF INDEX.
