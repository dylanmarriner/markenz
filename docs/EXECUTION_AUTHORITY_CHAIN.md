---
status: BINDING · EXECUTION GOVERNING LAW
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
document_id: EXECUTION_AUTHORITY_CHAIN
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
---

# EXECUTION AUTHORITY CHAIN
## (Binding Governance · Escalation Hierarchy · Executor Constraints)

**AUTHORITY LEVEL:** Principal  
**AUDIENCE:** All executors (Windsurf, AMP, Antigravity)  
**ENFORCEMENT:** KAIZA-MCP · Immutable

---

## 1. GOVERNING LAW HIERARCHY

### Tier 1: Constitutional Authority (Immutable)

1. **KAIZA_COMPLETE_GUIDE.md** — System architecture (planning + execution phases)
2. **AMP_DEFINITION_OF_DONEv2.md** — Binding acceptance criteria (determinism, no-mocks)
3. **MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md** — Supreme roadmap authority (phases 0-9)

### Tier 2: Phase Execution Authority (Binding)

- **PLAN_PHASE_0_BOOTSTRAP.md** — Offline stack, deterministic tick
- **PLAN_PHASE_1_DETERMINISM.md** — Seeded RNG, replay harness
- **PLAN_PHASE_2_WORLD.md** — Terrain, physics, actions
- **PLAN_PHASE_3_BIOLOGY.md** — Metabolism, bio-veto
- **PLAN_PHASE_4_COGNITION.md** — Perception, intent, planning
- **PLAN_PHASE_5_GENETICS.md** — Heritable traits
- **PLAN_PHASE_6_SOCIAL.md** — Reproduction, bonds, culture
- **PLAN_PHASE_7_GOVERNANCE.md** — Economy, justice, law
- **PLAN_PHASE_8_RENDERING.md** — WebGPU, chunk streaming
- **PLAN_PHASE_9_SCALING.md** — Security, production readiness

### Tier 3: Audit & Enforcement Authority (Verifying)

- **AMP_NO_PLACEHOLDER_AUDIT_PHASE_0_1_2.md** — Current state audit (blocks execution until remediated)
- **AMP_NORMALIZATION_REPORT.md** — This document (traceability)

---

## 2. EXECUTION ROLES & AUTHORITY

### AMP (Principal Planner)
- **Authority:** Creates binding plans via bootstrap_create_foundation_plan
- **Responsibility:** Ensures plans are complete, unambiguous, executable
- **Constraint:** CANNOT execute code (planning only)
- **Escalation Authority:** HALT entire project if blocker detected

### Windsurf (Executor)
- **Authority:** Implements code per plan specifications only
- **Responsibility:** Exact code implementation, test verification, escalation
- **Constraint:** CANNOT make architectural decisions (plan specifies all)
- **Escalation Authority:** MUST escalate if ambiguity or blocker encountered

### AMP Auditor (Verifying Authority)
- **Authority:** Verifies each phase exit criteria before proceeding
- **Responsibility:** Sign-off on phase completion, audit trails
- **Constraint:** CANNOT override failing tests (automatic no-go)
- **Escalation Authority:** Can HALT phase advance if gaps detected

---

## 3. PHASE EXECUTION SEQUENCE (NON-NEGOTIABLE)

### Strict Ordering Rule
```
Phase 0 MUST be 100% complete AND signed off
  ↓
Phase 1 MUST be 100% complete AND signed off
  ↓
Phase 2 MUST be 100% complete AND signed off
  ↓
... (Phases 3-9 same pattern) ...
  ↓
Phase 9 MUST be 100% complete AND signed off
  ↓
PRODUCTION DEPLOYMENT AUTHORIZED
```

### No Phase May Proceed If:
- Previous phase exit criteria NOT met
- Any blocker from audit NOT resolved
- Any test NOT passing
- Determinism divergence detected
- Authority violation discovered
- AMP auditor has not signed off in writing

---

## 4. GATE ENFORCEMENT (MANDATORY)

### Phase Entry Gate
**Before starting Phase N:**
- [ ] Phase N-1 exit criteria ALL TRUE
- [ ] Phase N-1 audit sign-off obtained
- [ ] All Phase N-1 tests passing
- [ ] No critical blockers in Phase N-1

### Phase Exit Gate
**Before advancing to Phase N+1:**
- [ ] All Phase N success criteria TRUE
- [ ] Phase N tests passing (cumulative: all prior phases still pass)
- [ ] Determinism verified (replay matches live)
- [ ] Authority boundaries verified (no violations)
- [ ] No TODO/FIXME/stub code in critical paths
- [ ] AMP auditor sign-off obtained in writing

### No Exceptions
- Gates are automatic (not subjective)
- Tests are executable (not manual)
- Sign-off is required (not optional)

---

## 5. WINDSURF EXECUTION CONSTRAINTS

### Windsurf MAY:
- Implement code per plan specification
- Choose implementation details (algorithms, libraries) that preserve determinism/authority
- Request clarification if plan is ambiguous
- Escalate if plan has gap or blocker
- Run tests to verify correctness

### Windsurf MUST NOT:
- Deviate from plan specifications
- Invent scope or architecture decisions
- Implement TODO/FIXME/mock/stub code
- Skip any phase gate criterion
- Merge code if any test fails
- Proceed if AMP auditor has not signed off
- Add external dependencies without explicit plan instruction
- Implement non-deterministic operations in authority code

---

## 6. ESCALATION PROTOCOL

### Escalation Trigger
Windsurf escalates to AMP Auditor if:

1. **Ambiguity in Plan:** Requirement unclear or contradictory
2. **Blocker Not Documented:** Issue not covered in plan
3. **Determinism Divergence:** Hashes mismatch on replay
4. **Authority Violation:** Server/Web mutates state
5. **Test Failure:** Any specified test fails
6. **Build Failure:** Cargo fails due to undocumented issue
7. **Asset Corruption:** Identity/asset data lost
8. **Performance Regression:** Tick rate drops >20%
9. **RBAC Bypass:** Unauthorized access possible
10. **Audit Finding:** Placeholder code found

### Escalation Format
```
FROM: Windsurf
TO: AMP Auditor
SUBJECT: Escalation — [Category] — Phase N
CONTEXT: [What you were trying to do]
BLOCKER: [Specific issue]
EVIDENCE: [Build log, code snippet, test output, state diff]
PROPOSAL: [Suggested fix, if any]
```

### AMP Auditor Response Options
1. **Resolve:** Provide clarification or update plan
2. **Escalate-Up:** Forward to AMP Principal for re-planning
3. **HALT:** Stop execution pending investigation

---

## 7. AUDIT AUTHORITY

### AMP Auditor Verification Checklist

**Before Signing Off Phase N:**

- [ ] All tests specified in plan are passing
- [ ] Code implements plan specification exactly
- [ ] No deviations detected (code review)
- [ ] No placeholder/mock/stub code in critical paths
- [ ] Determinism verified (same seed → same hashes)
- [ ] Authority boundaries enforced (server cannot mutate)
- [ ] All ObservationEvents generated (state is observable)
- [ ] Assets/identity preserved (no data loss)
- [ ] Performance acceptable (no major regressions)
- [ ] Regression tests passing (all prior phases)

### Auditor Authority
- **Can approve:** Phase completion (written sign-off)
- **Can reject:** Phase completion (if any item above fails)
- **Cannot override:** Failing tests (automatic no-go)
- **Can escalate:** To AMP Principal if critical issues found

---

## 8. COMMIT & DEPLOYMENT AUTHORITY

### Git Commit Requirements

Before any commit is allowed:
- [ ] All files written via KAIZA-MCP (audit-logged)
- [ ] All tests passing
- [ ] Phase exit criteria met
- [ ] AMP auditor sign-off obtained
- [ ] Pre-commit hooks pass (no mock code, no TODOs)

### Pre-Commit Hook (Enforced)

```bash
#!/bin/bash

# Reject any commit with placeholder code
if grep -r "TODO\|FIXME\|mock\|stub\|HACK\|XXX" src/; then
    echo "COMMIT REJECTED: Placeholder code detected"
    exit 1
fi

# Reject if tests failing
cargo test --all || exit 1

# Reject if clippy warnings in critical paths
cargo clippy --all -- -D warnings || exit 1

exit 0
```

---

## 9. DETERMINISM AUTHORITY

### Determinism is Non-Negotiable

- **Fixed timestep:** Tick index (u64), never wall-clock
- **Canonical RNG:** ChaCha20 (RFC 7539) with seeded streams
- **Immutable ordering:** BTreeMap, sorted iteration, stable serialization
- **Hash verification:** blake3 checksums on every state change
- **Replay proof:** Identical seed + events → identical hash sequence
- **Platform independence:** Linux x64/arm64, macOS all produce same hashes

### Determinism Violation = Hard Fail

If any test shows determinism divergence:
- [ ] STOP execution immediately
- [ ] Escalate with divergence report (first tick where hashes differ, subsystem RNG state)
- [ ] Do NOT proceed to next phase

---

## 10. AUTHORITY BOUNDARY ENFORCEMENT

### Rust Engine Authority (Immutable)
- Sole writer of world state
- Fixed-timestep deterministic loop
- All randomness via RNG subsystems
- All mutations via authority pipeline (10 passes)

### TypeScript Server (Stateless Control Plane)
- CANNOT mutate world state
- CANNOT import state mutation functions (code review enforces)
- Identity, auth, persistence only
- InputEvent validation and ordering

### React Web (Read-Only Observer)
- CANNOT mutate state directly or indirectly
- Submits InputEvents only (never direct mutations)
- Visualization and replay viewer only

### Violation = Hard Fail
If code review detects authority boundary violation:
- [ ] HALT execution
- [ ] Escalate with static analysis evidence
- [ ] Require architectural redesign

---

## 11. APPROVAL CHAIN

### Windsurf Execution Flow

```
Phase N Plan Ready
  ↓ [Windsurf implements]
  → Code written via MCP.write (audit-logged)
  → Tests run: pass/fail
  ↓ [If FAIL]
  → Windsurf fixes code
  → Tests rerun
  ↓ [If PASS]
  → Escalation report if any issues
  ↓ [AMP Auditor reviews]
  → Verify all exit criteria
  → Check for authority violations
  → Sign off or reject
  ↓ [If SIGN-OFF]
  → Phase N+1 planning begins
  ↓ [If REJECT]
  → Windsurf escalates or refixes
```

---

## 12. FINAL AUTHORITY

**Final deployment authority rests with:**
- AMP Principal Planner (final sign-off)
- Only after ALL phases 0-9 signed off
- Only after complete system integration test
- Only after no critical security issues remain

---

## 13. FAILURE MODES

### If Phase Fails:
- STOP execution
- Produce diagnostic report
- Escalate to AMP with evidence
- AMP re-plans or modifies phase
- Restart phase with updated plan

### If Critical Blocker Found:
- HALT entire project
- Produce forensic audit
- AMP Principal determines fix
- Restart from affected phase

### If Security Vulnerability Detected:
- STOP immediately
- Escalate to AMP Principal
- Do NOT deploy until resolved
- Update governance if needed

---

## END OF DOCUMENT

**Authority:** KAIZA-MCP · AMP Principal Planner  
**Status:** BINDING · EXECUTION-GOVERNING  
**Enforces:** Phases 0-9 sequential execution, gate compliance, auditor sign-off  
**Timestamp:** 2026-01-11
