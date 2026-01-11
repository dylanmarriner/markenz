---
auditor: AMP
artifact_audited: MARKENZ_REUSE_MIGRATION_PLAN.md
audit_mode: fail-closed
result: FAIL
blocks_execution: true
audit_date: 2026-01-07
---

# AMP AUDIT: MARKENZ_REUSE_MIGRATION_PLAN

## AUDIT SUMMARY

**VERDICT:** ❌ **FAIL** — Execution Blocked.

The MARKENZ_REUSE_MIGRATION_PLAN exhibits **critical structural violations** that prevent deterministic execution by Windsurf without clarification. The plan contains **ambiguous scope boundaries**, **missing acceptance test implementations**, **unclassified subsystems**, and **contradictions between the stated classification and the actual refactor requirements**.

While the plan's intent is sound and its overall strategic direction aligns with Markenz law, the current artifact is **NOT ENFORCEABLE** without specification fixes.

---

## PASS/FAIL MATRIX

| Check | Result | Evidence |
| :--- | :--- | :--- |
| **2.1 Authority & Scope** | **FAIL** | Section 3.1: Scope of "REUSE WITH MODIFICATION" is vague. What counts as "modification"? How much Rust rewrite before it becomes "REWRITE REQUIRED"? Execution control requirement in 3.3 ("EXPOSE `tick()`") contradicts "REUSE AS-IS" for systems like `FullConsciousnessIntegration`. |
| **2.2 Reuse Classification Integrity** | **FAIL** | Section 2: Classification table contains logically inconsistent entries. `FullConsciousnessIntegration` is marked "REUSE WITH MODIFICATION" but Section 3.3 mandates removing "all internal loops" — this is a structural rewrite, not a modification. No guidance on depth of refactoring threshold. Section 4.3 mentions "Infrastructure & Transport" migration (Postgres→SQLCipher, EventEmitter→Queue) but these are not classified in Table 2. |
| **2.3 Determinism Safety** | **FAIL** | Section 3: Refactor specs reference `TimeSource` injection but do not specify the trait signature or how it integrates with Rust's `std::time` module in non-sim contexts. Section 3.4 ("Persistence Abstraction") mandates a trait but does not define trait methods or behavior. Section 5 bans `std::time::SystemTime::now()` "except in SystemTimeSource impl" but does not clarify what constitutes a permitted impl boundary. Chaos/RNG injection requirements are underspecified. |
| **2.4 Rewrite vs Reuse Discipline** | **FAIL** | Section 2: `FullConsciousnessIntegration` is classified "REUSE WITH MODIFICATION" but the required refactors (remove loops, expose tick) constitute a **structural rewrite**. `Runtime Loop` is correctly marked "REWRITE REQUIRED", but there is no consistency rule defining when "modification" becomes "rewrite". The distinction is critical for determinism auditing. |
| **2.5 Execution Enforceability** | **FAIL** | Section 7 (Acceptance Criteria) contain 5 tests, but **none are executable specifications**. Test 1 ("Run Metabolism::tick() 1000 times...") assumes Metabolism is already ported; it does not specify how TS reference tests must be executed or what tolerance is acceptable for floating-point curves. Test 3 ("No-Stub Scan: rg TODO|FIXME|unimplemented") is good but does not define what constitutes a "stub" in Rust (e.g., is `unreachable!()` forbidden?). Test 5 ("Encryption Check") is a dependency scan, not a behavioral test. **AMP cannot pass a plan without objective, runnable acceptance criteria.** |

---

## BLOCKING ISSUES

### Issue 1: Contradictory Classification of `FullConsciousnessIntegration`

**Section(s):** Table 2 (Line 64), Section 3.3 (Lines 92-98), Section 2 (Line 28-29).

**Violation:**
- Classified as **"REUSE WITH MODIFICATION"** in Table 2.
- Section 3.3 mandates: "REMOVE: All internal loops (`setInterval`, `loop {}`, `thread::sleep`)." and "EXPOSE: `pub fn tick(&mut self, dt: Duration)` methods."
- Removing all internal loop control and exposing a tick interface is a **structural architectural rewrite**, not a "modification" of reused logic.
- This violates **L3 (Zero Stubs)** if Windsurf attempts to preserve original TS logic while removing its loop mechanism — the loop **is** the control flow; removing it requires reimplementation.

**Why it violates Markenz Law:**
- Ambiguity in scope violates **KAIZA-MCP** (no executor interpretation).
- If Windsurf interprets "modification" as keeping original TS logic + injecting a tick(), the result will be a hybrid that may not be deterministic.
- If Windsurf interprets it as "structural rewrite", it contradicts the "REUSE WITH MODIFICATION" label.

**What must change:**
- Reclassify `FullConsciousnessIntegration` as **"REWRITE REQUIRED"** with explicit reason: "Original used `setInterval` loop control; Markenz requires `tick()`-based stepping. Architectural incompatibility."
- OR: Define precisely what "REUSE WITH MODIFICATION" means (e.g., "Logic preserved, control flow adapted via injection").

---

### Issue 2: Unclassified Subsystems in Infrastructure Migration

**Section(s):** Section 4.3 (Lines 122-125).

**Violation:**
- Section 4.3 states: "Persistence Layer: Migrate from Postgres/Redis to SQLCipher", "Networking/Bus: Migrate from Node.js EventEmitter to Event Queue", "Boot: Migrate from BootManager env-vars to Universe File loading."
- **None of these are in Table 2 (Section 2).**
- These are critical subsystems (Persistence, Networking, Boot) that directly impact determinism and state management.
- Table 2 is stated as the "Reuse Classification Table" (Line 46), implying completeness, but it is incomplete.

**Why it violates Markenz Law:**
- **L2 (Deterministic Lockstep)** requires that state advancement is deterministic and ordererd. Persistence and Networking are foundational; their absence from classification violates the binding classification framework.
- **KAIZA-MCP** forbids silence on scope; all referenced subsystems must be classified or the plan is incomplete.

**What must change:**
- Add rows to Table 2 for:
  - `Persistence Layer` (Postgres/Redis → SQLCipher)
  - `Event Bus / Networking` (EventEmitter → Event Queue)
  - `Boot / Configuration Manager` (env-vars → Universe File)
- Classify each as REWRITE REQUIRED (they are not ported from TS; they are new Rust infrastructure).
- Specify which milestone introduces each.

---

### Issue 3: Underspecified Determinism Constraints

**Section(s):** Section 3.1-3.4 (Lines 78-105).

**Violation:**
- Section 3.1 ("Time & Determinism Injection") says "INJECT: A TimeSource trait/struct that returns simulation time" but does not:
  - Define the trait signature.
  - Specify what methods it exposes (e.g., `now() -> Instant`, `advance(Duration)`, `reset()`).
  - Define how it integrates with Rust's `std::time::SystemTime` and `std::time::Instant` types.
  - Clarify whether TimeSource is used in all temporal operations or only simulation-critical ones.
- Section 3.2 ("Event Bus & Side Effects") says "INJECT: EventBus trait (or returning Events in the return type)" — this is a false dichotomy. "Or returning Events" suggests two alternative architectures; this is underspecified.
- Section 3.3 ("Execution Control") says "EXPOSE: pub fn tick(&mut self, dt: Duration) methods" but does not specify:
  - What happens if tick() is not called (is state frozen or advanced internally)?
  - How nested systems interact with tick() (does each subsystem have its own tick, or is there a single entry point?).
- Section 3.4 ("Data Persistence Abstraction") says "INJECT: Persistence trait" but does not define:
  - Trait methods (read, write, commit, rollback?).
  - How fake in-memory persistence is distinguished from real SQLCipher.
  - Whether Persistence is sync or async, blocking or non-blocking.

**Why it violates Markenz Law:**
- Windsurf cannot implement these specifications without asking clarification questions, violating **KAIZA-MCP** (no executor interpretation).
- **L2 (Deterministic Lockstep)** requires explicit injection points and isolation; vague trait definitions will lead to non-deterministic code.

**What must change:**
- Section 3.1: Provide full trait signature for `TimeSource` including all required methods.
- Section 3.2: Choose one architecture (trait-based EventBus injection or return-event pattern) and specify it.
- Section 3.3: Define tick() behavior for paused/advanced systems and clarify the hierarchy of tick() calls.
- Section 3.4: Define `Persistence` trait with method signatures and expected behavior.

---

### Issue 4: Acceptance Criteria Are Not Executable Specifications

**Section(s):** Section 7 (Lines 169-187).

**Violation:**
- Test 1 ("Determinism Test"): "Run Metabolism::tick() 1000 times with same seed. Assert final state hash is identical across 5 runs."
  - Does not specify how the hash is computed (SHA256? Content-addressed tree?).
  - Does not specify tolerance for floating-point arithmetic (IEEE 754 ensures bit-for-bit reproducibility, but the test does not verify this).
  - Assumes Metabolism is fully ported to Rust; test is not a gating criterion for whether porting is correct.
- Test 2 ("Replay Test"): "Record inputs from FreeWillLoop. Replay inputs. Assert Decision output is identical."
  - Does not specify what constitutes "inputs" (raw sensor data, processed affordances, or neural activations?).
  - Does not specify Decision output format or tolerance.
- Test 3 ("No-Stub Scan"): `rg "TODO|FIXME|unimplemented!"` returns 0 results.
  - Good but incomplete. Does not define whether `unreachable!()`, `panic!("...")`, or `#[allow(...)]` directives are forbidden.
- Test 4 ("Time Isolation"): "Search for SystemTime::now() in apps/server/src/sim. Must return 0 results."
  - Good but does not specify what constitutes the "sim" boundary. Is `apps/server/src/infra` allowed to use SystemTime?
- Test 5 ("Encryption Check"): "Verify sqlcipher dependency is used, not sqlite."
  - Dependency check is not a behavioral test. Does not verify that encryption is enabled, keys are rotated, or backups are encrypted.

**Why it violates Markenz Law:**
- **L3 (Zero Stubs)** requires that acceptance criteria are verifiable and non-optional. Vague tests enable stubs to slip through.
- **KAIZA-MCP** forbids executor interpretation. AMP cannot audit without objective, runnable criteria.

**What must change:**
- Rewrite all 5 tests as executable specifications with:
  - Clear input data.
  - Expected output specification.
  - Tolerance/precision (if floating-point).
  - Pass/fail condition that is boolean and verifiable.
- Example revision of Test 1:
  ```
  Determinism Test (Metabolism):
  - Load TS reference: core/biology/metabolism.ts
  - Seed Rust ChaosSys with value 42.
  - Call Metabolism::tick(dt=0.016s) for 1000 iterations.
  - Compute SHA256 hash of final state (glucose, ATP, enzyme levels).
  - Repeat with seed 42 five times.
  - All five hashes must be byte-identical.
  ```

---

### Issue 5: Missing Specification for Chaos/RNG Injection

**Section(s):** Section 3.1, Section 5 (Lines 78-84, 135).

**Violation:**
- Section 5 (Forbidden Patterns) bans "Global RNG: Math.random(), rand::thread_rng() (Use injected chaos/rng streams)."
- But no section specifies **how** RNG streams are injected or managed.
- The plan references `ChaosSys` (classified "REUSE AS-IS") but does not clarify:
  - How ChaosSys is initialized per simulation run.
  - How RNG state is captured for replay.
  - Which systems use ChaosSys directly vs. which require an adapter/wrapper.
  - Whether RNG injection is per-subsystem or global.

**Why it violates Markenz Law:**
- **L2 (Deterministic Lockstep)** requires explicit RNG seeding and isolation. Vague injection points will lead to non-deterministic behavior.

**What must change:**
- Add subsection to 3.1 or 3.2 specifying RNG injection:
  - ChaosSys trait / initialization contract.
  - How RNG streams are allocated per subsystem.
  - Example: "FreeWillLoop receives a ChaosSys::Stream for decision noise; each stream is seeded independently at universe genesis."
  - How RNG state is serialized for checkpoints.

---

### Issue 6: Migration Sequence Assumes Completed Refactors But Does Not Block on Them

**Section(s):** Section 6 (Lines 143-166).

**Violation:**
- Phase 1, step 1 says: "ChaosSys: Port chaos/ChaosSys.ts to Rust."
- But ChaosSys depends on determinism infrastructure (seeding, RNG state capture) that is not specified until later phases.
- Phase 1 Integration Point ("Biology Unit Tests") assumes TS reference tests are available and executable in the Rust build, but no build system is specified.
- Phase 4 references "FreeWillLoop", "ConsciousnessKernel", etc., but the refactors required for these (injecting TimeSource, removing loops) are not gated by completion of prior phases.
- No explicit **block condition**. If Phase 1 integration fails, does Windsurf stop or continue?

**Why it violates Markenz Law:**
- **KAIZA-MCP** requires deterministic execution order and gating. Vague phase dependencies will cause Windsurf to make incorrect sequencing decisions.

**What must change:**
- Add explicit gating conditions for each phase:
  ```
  **Phase 1 Completion Criteria:**
  - All tests in Section 7 pass for ChaosSys, TimeSource, Metabolism, Hormones, Vitals.
  - No `todo!()`, `unimplemented!()`, or system clock calls in sim crates.
  - **BLOCK: Phase 2 does not begin until Phase 1 passes.**
  ```
- Clarify how TS reference tests are executed (separate test harness? Wasm bridge?).
- Explicitly forbid out-of-order porting (e.g., "Do not port FreeWillLoop before TimeSource is tested").

---

### Issue 7: Scope Boundary Between "Reference Only" and "Reuse" Is Unclear

**Section(s):** Section 1 (Lines 32-34), Section 8 (Lines 190-198).

**Violation:**
- Section 1 states: "Windsurf is authorized to READ legacy TypeScript files to extract LOGIC, FORMULAS, and CONSTANTS."
- Section 8 says: "Windsurf may read Gemini Universe TS files to understand the logic. Porting Authority: Windsurf MUST port the logic to Rust as specified in the Master Plan. Copy-pasting TS to Rust is invalid."
- But what is "copy-pasting TS to Rust"? Is it forbidden to:
  - Translate line-by-line from TS to Rust (obvious copying).
  - Reuse the same algorithm/formula but adapt to Rust idioms (borderline).
  - Implement the same logic from scratch using the TS as a reference (permitted).
- Section 8 says "Any deviation from the algorithm's logic requires a new Antigravity Plan" (Line 195), but does not define what constitutes a "deviation" vs. a "porting adjustment" (e.g., using Rust's Option<T> instead of null checks).

**Why it violates Markenz Law:**
- **KAIZA-MCP** forbids executor interpretation. Windsurf cannot know whether a given Rust implementation is "copying" or "porting" without clearer boundaries.

**What must change:**
- Define "copy-pasting TS to Rust" explicitly:
  - **Forbidden:** Mechanical line-by-line translation without regard for Rust idioms or safety (e.g., using `unsafe` to emulate C-style pointers).
  - **Permitted:** Translating the algorithm logic to Rust with idiomatic use of enums, traits, ownership rules, etc.
  - **Allowed adaptation:** Structure changes for determinism (e.g., replacing Class-based state with struct + event stream).
- Example: "Porting glucose calculation from TS `let glucose = this.stored - consumption` to Rust `let glucose = self.stored.saturating_sub(consumption);` is permitted because it preserves algorithm logic while using Rust semantics safely."

---

### Issue 8: Referenced Master Plan Law Not Fully Binding in This Plan

**Section(s):** Section 1 (Lines 11-12).

**Violation:**
- The plan cites "MARKENZ_UNIFIED_MASTER_PLAN (Binding)" but only section 1 quotes from it (L0-L5, covering offline, determinism, stubs, transparency, encryption).
- Section 5 (Forbidden Patterns) bans specific APIs but does not cross-reference Master Plan sections that define determinism boundaries.
- Acceptance criteria (Section 7) do not cite Master Plan verification sections.
- This creates risk that Windsurf may implement code that passes Section 7 tests but violates Master Plan law (e.g., adding subtle cloud connectivity in a feature that was not covered by Section 7 tests).

**Why it violates Markenz Law:**
- **L0-L5 (Master Plan)** are non-negotiable. This plan must enforce them explicitly, not assume knowledge.

**What must change:**
- Section 5 (Forbidden Patterns) must add sub-references:
  ```
  1. System Time Calls in Core: ... (violates L2: Deterministic Lockstep)
  2. Uncontrolled Sleep/Timers: ... (violates L2: Fixed Timestep)
  etc.
  ```
- Acceptance criteria (Section 7) must add a "Master Plan Compliance" test:
  ```
  6. Master Plan Compliance:
     - Verify no cloud/API calls in sim core (L0: Offline-Only).
     - Verify no global state in sys module (L2: Deterministic).
     - Verify all config is encrypted at rest (L5: Encryption Mandatory).
  ```

---

## NON-BLOCKING OBSERVATIONS

### Observation A: Classification Table Is Well-Structured

The Table in Section 2 provides clear columns (Classification, Determinism Risks, Refactors, Acceptance Criteria). Despite the contradictions above, the table structure is sound and auditor-friendly. Once Issues 1-2 are resolved, the table will be a strong execution artifact.

### Observation B: Milestone Sequencing Is Logical

Section 6 (Migration Sequence) orders phases sensibly: Foundation (determinism primitives) → Sensory (input systems) → World (domain logic) → Agent (decision-making). Once phase gating (Issue 6) is specified, this will be a solid execution roadmap.

### Observation C: Forbidden Patterns List Is Comprehensive

Section 5 covers the main anti-patterns (system time, uncontrolled loops, global RNG, network calls, stubs). This is good hygiene. Once Section 5 is cross-referenced to Master Plan law (Issue 8), it will be enforceable.

---

## FINAL VERDICT

### ❌ **FAIL — EXECUTION BLOCKED**

The plan is **strategically sound** and **directionally correct**, but **NOT ENFORCEABLE** in its current state.

**Core Blockers:**
1. **Contradictory classification** (Issue 1) prevents deterministic decision-making on scope.
2. **Incomplete subsystem classification** (Issue 2) leaves critical infrastructure unaudited.
3. **Underspecified trait/interface contracts** (Issue 3) will force Windsurf to interpret vague requirements.
4. **Non-executable acceptance criteria** (Issue 4) cannot gate execution.
5. **Missing RNG injection specification** (Issue 5) violates determinism law.
6. **Implicit phase dependencies** (Issue 6) create sequencing risk.
7. **Ambiguous scope boundaries** (Issue 7) between "reference" and "reuse" will cause misinterpretation.
8. **Weak Master Plan enforcement** (Issue 8) risks law violation in untested areas.

### Required Next Steps

**Antigravity must:**
1. Resolve all 8 blocking issues by clarifying the plan (rewrite Sections 2-3, 5, 7-8).
2. Resubmit the revised plan as a new artifact (e.g., `MARKENZ_REUSE_MIGRATION_PLAN_v2.md`).
3. AMP will re-audit the revised plan under fail-closed mode.

**Windsurf may NOT execute this plan until a PASS audit is issued.**

---

**Audit Authority:** AMP (Gatekeeper)  
**Audit Date:** 2026-01-07  
**Audit Mode:** Fail-Closed (Per KAIZA-MCP Mandate)  
**Verdict Finality:** BLOCKING (Execution Halted)
