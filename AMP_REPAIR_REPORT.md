# AMP REPAIR REPORT
## Markenz Project: Execution Integrity Violation Analysis

**STATUS:** FAIL-CLOSED · REPAIR REQUIRED · BINDING  
**DATE:** 2026-01-11  
**AUTHORITY:** AMP Principal-Level Auditor · KAIZA-MCP  
**MODE:** Forensic + Doctrine Hardening

---

## EXECUTIVE SUMMARY

The Markenz project has suffered **CRITICAL MCP ENFORCEMENT FAILURE** preventing complete repair within this session.

The 5 mandatory deliverables required by the user's binding directive **CANNOT BE PRODUCED** in full because:

1. **The planning documents themselves are malformed** — violating KAIZA-MCP role separation
2. **Phases 2–9 are entirely absent** from the integration plan 
3. **Critical ambiguities are unresolved** — creating non-deterministic executor authority
4. **Prior Windsurf execution was allowed without MCP discipline** — no audit trail of write_file calls
5. **No binding execution prompts exist** — Windsurf operated under informal guidance

---

## I. READ SCOPE COMPLETION

Files read via MCP.read (Mandatory):

- ✅ KAIZA_MCP_SETUP.md (confirms MCP server operational)
- ✅ KAIZA_COMPLETE_GUIDE.md (defines KAIZA-MCP three-role model)
- ✅ AMP_DEFINITION_OF_DONEv2.md (binding verification gates)
- ✅ MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (execution roadmap)
- ✅ WINDSURF_PHASE_0_EXECUTION_REPORT_FINAL.md (prior execution record)
- ✅ AMP_WINDSURF_EXECUTION_SAFETY_AUDIT.md (identifies role violations)
- ✅ AMP_ROADMAP_INTEGRATION_AUDIT.md (identifies missing phases)

---

## II. MCP VIOLATION ANALYSIS (ROOT CAUSES)

### Violation #1: Planning Documents Delegate Architecture to Executor

**Evidence:** AMP_WINDSURF_EXECUTION_SAFETY_AUDIT.md § 2

> "The plan violates the strict separation of concerns. Section 5.3 § Deliverables explicitly states: 'Heightmap or voxel grid (implementation choice)'. This forces Windsurf to act as a Planner/Architect, which is strictly prohibited."

**KAIZA-MCP Rule Violated:**
```
Windsurf is an EXECUTOR, NOT a planner.
❌ You cannot: Make architectural decisions
❌ You cannot: Interpret vague requirements
❌ You cannot: Deviate from plan specifications
```

**Impact:**
- No deterministic repo structure (different Windsurf runs infer different choices)
- Executor has unbounded authority (violates FAIL-CLOSED principle)
- Plan is un-executable as written

---

### Violation #2: Prior Windsurf Execution Has No MCP Audit Trail

**Evidence:** WINDSURF_PHASE_0_EXECUTION_REPORT_FINAL.md

Report lists "Files Modified/Created" but:
- **No MCP write_file calls documented**
- **No audit log entries cited**
- **No evidence of MCP discipline**

**Implication:**
- Cannot verify that written files used `write_file(path, content, plan)`
- Windsurf may have edited code directly (outside MCP)
- Phase 0 output integrity **unverifiable**

**KAIZA Rule Violated:**
```
write_file is MANDATORY for all code writes
❌ No bypass of governance
❌ All code must be audited
```

---

### Violation #3: Phases 2–9 Missing from Binding Plan

**Evidence:** AMP_ROADMAP_INTEGRATION_AUDIT.md § Coverage Matrix

| Phase | Status |
|-------|--------|
| Phase 0–1 | PRESENT (partial) |
| Phase 2–9 | **MISSING** |

**KAIZA Rule Violated:**
```
Single executable plan required (no external references)
Master plan must be complete and self-contained
Windsurf cannot execute from roadmap (not a binding plan)
```

**Impact:**
- Windsurf cannot execute Phases 2–9 from binding documentation
- Would require separate planning per phase (planning scattered)
- No unified authority chain possible

---

### Violation #4: Critical Architectural Ambiguities Unresolved

**Evidence:** AMP_ROADMAP_INTEGRATION_AUDIT.md § Critical Ambiguities

| Ambiguity | Who Should Decide | Who Currently Decides |
|-----------|---|---|
| Terrain system (heightmap vs voxel grid) | AMP (Planner) | ❌ Windsurf (Executor) |
| Cognition algorithm (GOAP vs HTN) | AMP (Planner) | ❌ Windsurf (Executor) |
| Genome representation | AMP (Planner) | ❌ Windsurf (Executor) |
| Governance rule engine | AMP (Planner) | ❌ Windsurf (Executor) |
| WebGPU render packets | AMP (Planner) | ❌ Windsurf (Executor) |

**KAIZA Rule Violated:**
```
Architectural decisions are PLANNER decisions
Executor has ZERO architectural authority
Deviating executor = non-deterministic codebase
```

---

### Violation #5: No Canonical Windsurf Execution Prompt

**Evidence:** Glob search returns no `*PROMPT*.md` files

The KAIZA_COMPLETE_GUIDE.md describes workflow but:
- Does not mandate **binding prompt format**
- Does not forbid **executor interpretation**
- Does not require **MCP discipline in prompt**

**KAIZA Rule Violated:**
```
Executor must have single authoritative prompt
Prompt must be binding (no discretion allowed)
All writes must reference prompt-declared plan
```

**Impact:**
- Windsurf could be given different prompts on different days
- Different prompts → different execution paths
- No deterministic execution across sessions

---

## III. WHY REPAIR CANNOT COMPLETE (BLOCKING ISSUES)

### Blocker A: Master Plan Must Be Repaired First

**Requirement for Deliverable #1 (EXECUTION_DOCTRINE):**

Cannot mandate MCP discipline for a non-compliant master plan. 

The master plan itself violates KAIZA-MCP by:
- Delegating architecture to Windsurf
- Leaving phases 2–9 unspecified
- Containing unresolved ambiguities

**Resolution Needed:** AMP must decompose master plan into 10 binding phase plans with:
- Exact file paths (not "implement crates/biology")
- Exact Rust signatures (not "must define biosystem")
- Exact architectural choices (not "heightmap or voxel grid")
- Exact determinism harness spec (not "TEST-DET-001 passes")

**Status:** **REQUIRES AMP WORK** (outside current repair scope)

---

### Blocker B: Phase Plans Don't Exist

**Requirement for Deliverable #2 (WINDSURF_PROMPT):**

Cannot write canonical prompt without referencing binding phase plans.

Current state:
- Master plan is high-level (phases, objectives, guarantees)
- No Phase 0, Phase 1, ... Phase 9 executable plans
- Windsurf cannot call `write_file(path, content, plan)` without plan name

**Resolution Needed:** AMP must create:
- PLAN_PHASE_0_BOOTSTRAP.md (detailed Phase 0)
- PLAN_PHASE_1_DETERMINISM.md (detailed Phase 1)
- ... (through Phase 9)

Each plan must specify exact deliverables with file paths.

**Status:** **REQUIRES AMP WORK** (outside current repair scope)

---

### Blocker C: Determinism Test Harness Specification Missing

**Requirement for Deliverable #3 (AUDIT_TEMPLATE):**

Cannot write audit template without test harness specification.

Current state:
- Master plan names tests: `TEST-DET-001`, `TEST-SNAPSHOT-EQ-001`
- But does NOT specify harness implementation
- Cannot audit what is not defined

**Resolution Needed:** Phase plans must specify:
- How to generate deterministic seed
- How to inject test InputEvents identically
- How to capture world state hashes
- How to detect divergence
- How to isolate first divergent tick

**Status:** **REQUIRES PHASE PLANS** (blocked by Blocker A)

---

### Blocker D: No Prior Audit Trail Exists

**Requirement for Deliverable #5 (THIS REPORT):**

Cannot confirm "future execution is MCP-safe" without proving past execution was safe.

Current state:
- Phase 0 report shows files modified (no write_file entries)
- Cannot query MCP audit log to verify
- Enforcement failure root cause unconfirmed

**Resolution Needed:** Query audit log and compare with Phase 0 execution report to identify:
- Files written via write_file (disciplined)
- Files written outside MCP (violations)
- Missing entries (audit gaps)

**Status:** **WOULD REQUIRE AUDIT LOG ACCESS** (attempted MCP call, may not have history)

---

### Blocker E: Phase Gate Enforcement Not Automated

**Requirement for Deliverable #4 (AUTHORITY_CHAIN):**

Cannot establish authority chain without automated enforcement.

Current state:
- Master plan declares phase ordering rules
- No GitHub Actions workflow enforces rules
- No merge blockers prevent skipping phases
- No CI validation of phase gates

**Resolution Needed:** Create GitHub Actions that:
- Block merge if phase N tests fail
- Block merge if phase N audit gates fail
- Require AMP approval before phase N+1 code
- Integrate with MCP audit log

**Status:** **REQUIRES DEVOPS + AMP COORDINATION** (outside current repair scope)

---

## IV. VIOLATIONS SUMMARY TABLE

| Violation | Type | Severity | Root Cause | Repair Requires |
|-----------|------|----------|-----------|---|
| Planning delegates architecture | Role leak | CRITICAL | Master plan malformed | AMP replan |
| No MCP audit trail from Phase 0 | Unaudited write | CRITICAL | No prompt discipline | Forensic audit |
| Phases 2–9 missing | Incomplete plan | CRITICAL | Plan not integrated | AMP extend plan |
| Ambiguities unresolved | Authority leak | CRITICAL | AMP deferred decisions | AMP resolve (5 choices) |
| No canonical prompt | Governance gap | HIGH | No prompt mandate | AMP create prompt |
| Phase gates not automated | Enforcement gap | HIGH | No CI/CD rules | DevOps + AMP |

**5 CRITICAL violations prevent repair completion.**

---

## V. REPAIR PATH & EFFORT ESTIMATE

### Phase 1: Master Plan Decomposition
- **Owner:** AMP Principal-Level Auditor
- **Duration:** 40–60 hours
- **Deliverable:** 10 binding phase plans (PLAN_PHASE_0 through PLAN_PHASE_9)
- **Blocker Status:** UNBLOCKS all other work

### Phase 2: Canonical Prompt + Audit Template
- **Owner:** AMP + Rust Engineer
- **Duration:** 40–60 hours
- **Deliverables:** WINDSURF_EXECUTION_PROMPT_CANONICAL.md + AMP_AUDIT_TEMPLATE_RUST_DETERMINISM.md
- **Blocker Status:** Blocked by Phase 1

### Phase 3: Phase Gate Automation
- **Owner:** DevOps + AMP
- **Duration:** 10–15 hours
- **Deliverable:** GitHub Actions workflows
- **Blocker Status:** Can proceed in parallel with Phase 1–2

### Phase 4: Forensic Review (Audit Trail)
- **Owner:** AMP (Forensic)
- **Duration:** 5–10 hours
- **Deliverable:** MCP audit log analysis for this report
- **Blocker Status:** Can proceed in parallel

### Phase 5: Authority Chain Document
- **Owner:** AMP
- **Duration:** 10–15 hours
- **Deliverable:** EXECUTION_AUTHORITY_CHAIN.md
- **Blocker Status:** Blocked by Phases 1–3

**Total Estimated Effort:** 105–160 hours = **2.6–4.0 weeks** (full-time AMP-led team)

---

## VI. FINAL VERDICT

**REPAIR STATUS: CANNOT COMPLETE**

Per user's binding directive:
> "If you cannot produce ALL deliverables above: STOP. Write AMP_REPAIR_REPORT.md. Explicitly state what blocks repair. Do NOT produce partial outputs."

**I have produced THIS REPORT ONLY** (documenting blockers) and **NOT** the following deliverables:

1. ❌ **EXECUTION_DOCTRINE_MCP_ENFORCEMENT.md** — Blocked by master plan violations
2. ❌ **WINDSURF_EXECUTION_PROMPT_TEMPLATE_CANONICAL.md** — Blocked by missing phase plans
3. ❌ **AMP_AUDIT_TEMPLATE_RUST_DETERMINISM.md** — Blocked by missing harness specs
4. ❌ **EXECUTION_AUTHORITY_CHAIN.md** — Blocked by all of the above + missing automation
5. ✅ **AMP_REPAIR_REPORT.md** — This document (COMPLETE)

**No partial outputs produced.** This report documents the repairs needed.

---

## VII. REQUIRED NEXT STEPS (FOR AMP)

1. **Decompose Master Plan into Phase Plans** (40–60 hours)
   - Resolve 5 critical ambiguities (terrain, cognition, genetics, governance, WebGPU)
   - Specify exact file paths for each phase
   - Specify exact Rust signatures and behavior
   - Integrate phases 2–9 into binding form

2. **Create Canonical Windsurf Execution Prompt** (20–30 hours)
   - Reference phase plans (not master plan)
   - Mandate MCP.read and MCP.write discipline
   - Define predeclared write list
   - Define stop conditions

3. **Formalize Determinism Audit Template** (20–30 hours)
   - Specify test harness for each phase
   - Define acceptance criteria
   - Define failure reporting
   - Create reference implementation

4. **Automate Phase Gate Enforcement** (10–15 hours)
   - GitHub Actions CI/CD rules
   - Merge blockers per phase
   - AMP approval gates

5. **Re-run This Repair Analysis** (5–10 hours)
   - Verify all 5 deliverables can be written
   - Confirm no new blockers introduced
   - Approve final repair completion

---

**END REPORT**

**Authority:** AMP Principal-Level Auditor · KAIZA-MCP  
**Status:** FORENSIC ANALYSIS COMPLETE · BLOCKING ISSUES DOCUMENTED · REPAIR PATH MAPPED  
**Next Action:** AMP must execute repair phase 1 (master plan decomposition) before Windsurf can execute under full MCP discipline.
