# AMP WINDSURF EXECUTION SAFETY AUDIT

---

## 1. Audit Header

- **STATUS:** FAIL
- **AUTHORITY:** KAIZA-MCP · AMP
- **SCOPE:** Windsurf execution safety & MCP compliance
- **FAIL MODE:** FAIL-CLOSED
- **DATE:** 2026-01-10

---

## 2. KAIZA-MCP Role Compliance Check

- **Plan Author:** AMP (Antigravity) ✅
- **Executor:** Windsurf ✅
- **No Planner Actions Required:** ❌ **FAIL**
  - **Explanation:** The plan violates the strict separation of concerns defined in `KAIZA_COMPLETE_GUIDE.md`. It repeatedly assigns architectural decisions to the Executor (Windsurf).
  - **Evidence:** Section 5.3 § Deliverables explicitly states: "Heightmap or voxel grid (implementation choice)". This forces Windsurf to act as a Planner/Architect, which is strictly prohibited by KAIZA-MCP ("Windsurf is an EXECUTOR, NOT a planner... ❌ You cannot: Make architectural decisions").

---

## 3. Write-Path Verifiability

- **Explicit File Paths:** ❌ **FAIL**
  - **Verification:** The plan lists high-level "Deliverables" (e.g., "crates/biology", "Metabolism", "Hydration") but fails to specify the exact file paths (e.g., `crates/biology/src/metabolism.rs`).
- **Unambiguous Write Targets:** ❌ **FAIL**
  - The plan requires Windsurf to "implement crates/biology" but does not define the internal module structure. Windsurf would have to invent filenames, creating a non-deterministic codebase structure.
- **Verdict:** **BLOCKING**. Windsurf cannot call `write_file` without explicit paths provided by the plan.

---

## 4. Executor Ambiguity Scan

- **Detected Ambiguities:**
  - **Section 5.3:** "Heightmap or voxel grid (implementation choice)" → **UNSAFE** (Requires architectural decision).
  - **Section 5.4:** "BioVeto with logged reasons... Reasons: 'Insufficient energy', etc." → **UNSAFE** (Data structure for "Reasons" not defined; enum vs string vs struct left to executor).
  - **Section 5.5:** "Choose algorithm (GOAP or Hierarchical Task Network)" → **UNSAFE** (Explicitly asks executor to choose algorithm).
  - **General:** "Must implement crates/..." without file breakdown → **UNSAFE** (Requires inferred directory structure).
- **Result:** **UNSAFE**. The plan contains multiple "Choice Points" where Windsurf executes non-deterministic judgment.

---

## 5. KAIZA-MCP Tool Compatibility

- **Tool:** `write_file`
- **Compatibility:** ❌ **FAIL**
  - KAIZA-MCP requires: `read_file(plan) → write_file(path, content, plan)`.
  - Since `path` is not specified in the plan, Windsurf cannot form a valid `write_file` call without inventing the path.
- **Tool:** `list_plans` / `read_file`
  - **Compatibility:** ✅ PASS. The plan is readable.

---

## 6. Phase Gate Enforceability

- **Entry Conditions:** defined ✅
- **Exit Criteria:** defined ✅
- **Testable Gates:** defined ✅
  - Examples: `TEST-DET-001`, `TEST-SNAPSHOT-EQ-001`.
- **Enforcement:** The gates are rigorous and testable.
- **Result:** **PASS**. The verification layer is solid; the failure is in the implementation specifications.

---

## 7. Determinism & Authority Safety Check

- **Rust-only authority preserved?** YES.
  - Plan explicitly locks authority to `apps/engine`.
- **Server/UI mutation impossible?** YES.
  - Plan correctly downgrades server/web to read-only/input-only.
- **Replay + hash gates executable?** YES.
  - Verification logic is sound.

---

## 8. Execution Readiness Verdict

- **READY FOR WINDSURF EXECUTION:** **NO**
- **BLOCKING ISSUES:**
  1. **Missing File-Level Specifications:** Plan describes *features* not *files*. Windsurf cannot map requirements to `write_file` calls.
  2. **Architectural Delegation:** Plan explicitly asks Executor to make choices (Voxel vs Heightmap, GOAP vs HTN), violating KAIZA-MCP Role separation.
  3. **Implied Structure:** Internal crate structure is undefined, leading to non-deterministic repo states depending on Executor interpretation.

---

## 9. Mandatory Actions

1. **Decompose into Execution Plans:** `MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md` must be treated as a **Strategic Architecture Document**, not an executable plan. It must be broken down into granular, file-specific Execution Plans (e.g., `PLAN_PHASE_0_BOOTSTRAP.md`, `PLAN_PHASE_1_DETERMINISM.md`).
2. **Resolve Architectural Choices:** AMP (Planner) must make all pending decisions (Heightmap vs Voxel, GOAP vs HTN) and document them in the Execution Plans.
3. **Define File Maps:** Execution Plans must list every target file path (e.g., `crates/biology/src/metabolism.rs`) and its specific exports/behavior, enabling unambiguous `write_file` calls.
