---
status: APPROVED
plan_type: reuse-migration
scope: gemini-universe → markenz
authority: antigravity
blocks_execution_without: true
---

# MARKENZ REUSE MIGRATION PLAN

> **AUTHORITY:** Antigravity  
> **INPUTS:** AMP Reuse Audit (Binding), MARKENZ_UNIFIED_MASTER_PLAN (Binding)  
> **MODE:** FAIL-CLOSED  

## 1. Executive Verdict

**CAN MARKENZ REUSE GEMINI UNIVERSE SYSTEMS?**

**NO.** Direct code reuse is **STRICTLY FORBIDDEN**.

The existing Gemini Universe codebase is built on **Node.js runtime assumptions** that violate the core tenets of Markenz:
1.  **Time**: It relies on `Date.now()` and `new Date()`, which are non-deterministic and coupled to wall-clock time.
2.  **Randomness**: It uses `Math.random()` and `crypto.randomUUID()`, which are non-deterministic without a seeded RNG.
3.  **State**: It uses Class-based encapsulation with internal mutable state, whereas Markenz requires **Event-Sourced** or **ECS-based** state.
4.  **Architecture**: It uses `EventEmitter` and global singletons (`transparencyEventBus`), which break offline isolation and determinism.

**YES, with STRICT CONSTRAINTS.**
Markenz may reuse the **logic and algorithms** of certified Gemini Universe systems, but **NOT** the raw implementation files for server-side code. The Master Plan mandates a **Rust** authoritative core, while Gemini Universe is TypeScript. Therefore, "Reuse" is defined as **porting the certified deterministic algorithms to Rust** with 1:1 logical fidelity.
For Web UI components (TypeScript), reuse may be direct copy-paste where applicable.
The Audit certifies that the *logic* of Tier 1 and Tier 2 systems is sound, deterministic, and offline-safe, making them valid candidates for porting. Tier 3 systems are fundamentally broken and must be rewritten from scratch.

**PERMITTED ACTION:**
*   Windsurf is authorized to **READ** legacy TypeScript files to extract **LOGIC, FORMULAS, and CONSTANTS**.
*   Windsurf is authorized to **RE-IMPLEMENT** this logic in Rust (for the Core) or deterministic TypeScript (for the UI/Bridge), subject to the **Mandatory Refactor Specifications** below.

---

## 2. Reuse Classification Table

**LEGEND:**
- **REUSE AS-IS:** Logic is sound; port to Rust exactly as implemented.
- **REUSE WITH CONS:** Logic is sound but requires specific architectural fixes during porting.
- **REWRITE:** Logic is invalid; do not look at original code. Implement from spec.
- **REJECTED:** Forbidden.

| Subsystem | Original Path (Gemini) | Classification | Determinism Risks | Required Refactors (During Port) | Acceptance Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Metabolism** | `core/biology/metabolism.ts` | **REUSE AS-IS** | None (Pure State Machine) | None. Port 1:1. | Glucose/ATP curves match TS original. |
| **Hormones** | `core/biology/hormones.ts` | **REUSE AS-IS** | None (Explicit decay) | None. Port 1:1. | 9 hormone vectors fluctuate identically. |
| **Interoception** | `core/senses/interoception.ts` | **REUSE AS-IS** | None (Aggregator) | None. Port 1:1. | Urgency scores match inputs 1:1. |
| **Proprioception** | `core/senses/proprioception.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Postural strain calc matches. |
| **Tactile System** | `core/senses/tactile-system.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Sensitivity mapping preserved. |
| **Somatic Body** | `core/somatic/SomaticBody.ts` | **REUSE WITH MODIFICATION** | Event Bus Side-effects | Remove global event bus. Inject EventBus trait/struct. | No global state access. |
| **Vitals System** | `core/biology/vitals.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Pulse/BP/SpO2 calc matches. |
| **Immune System** | `core/biology/immune-system.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Antibody prod rate matches. |
| **Granular Emotions** | `core/psychology/granular-emotions.ts` | **REUSE AS-IS** | None | None. Port 1:1. | 150+ emotions map currectly. |
| **Dark Triad** | `core/psychology/dark-triad.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Narcissism scores evolve identically. |
| **Free-Will Loop** | `core/free-will-decision-loop.ts` | **REUSE WITH MODIFICATION** | `Date.now()`, Chaos Seed | Inject `TimeSource`. Explicitly seed ChaosSys. | Replay produces identical choices. |
| **Time Source** | `core/time-source.ts` | **REUSE AS-IS** | None | None. Port 1:1 (SystemTime vs ReplayTime). | Time can be frozen/stepped. |
| **Event Replay** | `core/event-replay-engine.ts` | **REUSE WITH MODIFICATION** | DB Dependency, Stub hash | Abstract DB (Trait). Implement SHA-256 state hash. | Hash chain verification passes. |
| **Runtime Loop** | `core/runtime/loop.ts` | **REWRITE REQUIRED** | `setInterval`, Broken | Do not port using `setInterval`. Use fixed timestep loop. | Fixed 20Hz tick supported. |
| **State Container** | `core/runtime/state-container.ts` | **REUSE WITH MODIFICATION** | Missing Somatic/Brain calls | Implement missing `processSomatic`/`processBrain` calls. | State tree updates fully. |
| **Consciousness Kernel** | `core/consciousness-kernel-enhanced.ts` | **REUSE WITH MODIFICATION** | `Date.now()` | Inject `TimeSource`. Inject `EventBus`. | Tick structure preserved. |
| **Full Consciousness** | `core/full-consciousness-integration.ts` | **REUSE WITH MODIFICATION** | `setInterval`, Weak World API | Remove loop. Expose `tick(dt)`. Enforce World trait. | Driven by external tick only. |
| **Homestead** | `world/homestead.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Decay/weather logic matches. |
| **Shed** | `world/shed.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Crafting recipes work. |
| **World Service** | `domains/cosmos/world/services/world-service.ts` | **REUSE WITH MODIFICATION** | DB Dep, `Date.now()` | Inject `TimeSource`. Abstract DB. Use `tick()`. | Offline-safe world state. |
| **Self-Reflection** | `domains/psychology/cognition/self-reflection.ts` | **REWRITE REQUIRED** | Stubs | Logic missing. Implement new. | N/A |
| **ChaosSys** | `chaos/ChaosSys.ts` | **REUSE AS-IS** | None | None. Port 1:1. | Same seed = same output. |

---

## 3. Mandatory Refactor Specifications (During Port)

**⚠️ Critical:** Since migration involves a port to Rust, "Refactor" means "Implement the corrected logic in Rust".
Any logic ported from Gemini Universe MUST adhere to these architectural laws:

### 3.1 Time & Determinism Injection
- **Target:** `FreeWillDecisionLoop`, `ConsciousnessKernel`, `WorldService`.
- **Requirement:**
  - **REMOVE:** Any calls to system clock (`Date.now()`, `SystemTime::now()`).
  - **INJECT:** A `TimeSource` trait/struct that returns simulation time.
  - **TEST:** Verify that mocking `TimeSource` allows arbitrary time-travel in tests.

### 3.2 Event Bus & Side Effects
- **Target:** `SomaticBody`, `ConsciousnessKernel`.
- **Requirement:**
  - **REMOVE:** Global singleton access (`global.bus`).
  - **INJECT:** `EventBus` trait (or returning Events in the return type).
  - **TEST:** Assert emitted events appear in the Output vector or EventLog.

### 3.3 Execution Control (The "Tick" Law)
- **Target:** `FullConsciousnessIntegration`, `WorldService`.
- **Requirement:**
  - **REMOVE:** All internal loops (`setInterval`, `loop {}`, `thread::sleep`).
  - **EXPOSE:** `pub fn tick(&mut self, dt: Duration)` methods.
  - **INVAR:** System state advances ONLY when `tick()` is called.

### 3.4 Data Persistence Abstraction
- **Target:** `EventReplayEngine`, `WorldService`.
- **Requirement:**
  - **REMOVE:** Hard dependency on SQL/Postgres pools.
  - **INJECT:** `Persistence` trait (for Events/Snapshots).
  - **GOAL:** Enable in-memory fake persistence for unit tests.

---

## 4. Rewrite-Required Systems

The following systems are **technologically incompatible** and must be built from scratch in Rust/Wasm:

### 4.1 Runtime Loop (`core/runtime/loop.ts`)
- **Reason:** Original relied on `setInterval` (non-deterministic) and was audited as broken/orphaned.
- **Replacement:** `markenz-server` (Rust) main loop.
- **Milestone:** M1 (Foundation).

### 4.2 Self-Reflection Engine (`.../self-reflection-engine.ts`)
- **Reason:** Original was pure stubs ("Implementation to be added in Phase 3").
- **Replacement:** New implementation based on Master Plan "Explainability" spec.
- **Milestone:** M6 (Learning).

### 4.3 Infrastructure & Transport
- **Persistence Layer**: Migrate from Postgres/Redis to **SQLCipher** (Wasm-compatible, encrypted local storage).
- **Networking/Bus**: Migrate from Node.js `EventEmitter` to **Event Queue** (passed between Rust Core and UI).
- **Boot**: Migrate from `BootManager` env-vars to explicit **Universe File** loading.

---

## 5. Forbidden Patterns & Imports

Windsurf is **strictly prohibited** from introducing the following patterns or dependencies into Markenz code (Rust or TS):

1.  **System Time Calls in Core:** `Date.now()`, `std::time::SystemTime::now()` (Except in `SystemTimeSource` impl).
2.  **Uncontrolled Sleep/Timers:** `setInterval`, `setTimeout`, `thread::sleep()` (inside simulation logic).
3.  **Global RNG:** `Math.random()`, `rand::thread_rng()` (Use injected chaos/rng streams).
4.  **Network/Cloud Calls in Sim:** `fetch()`, `reqwest::*`, `WebSocket` (inside agent/world logic).
5.  **Stub Macros:** `todo!()`, `unimplemented!()`, `panic!("TODO")`.
6.  **Undefined Types:** `any` (TS), `serde_json::Value` (unless strictly typed schema is impossible).
7.  **Original File Imports:** Do NOT `import` the original TS files directly into the build. They are references only.

---

## 6. Migration Sequence

**Phase 1: Foundation (Rust Port)**
1.  **ChaosSys**: Port `chaos/ChaosSys.ts` to Rust (`crates/deterministic/src/chaos.rs`).
2.  **TimeSource**: Port `core/time-source.ts` to Rust (`crates/deterministic/src/time.rs`).
3.  **Metabolism**: Port `core/biology/metabolism.ts` to Rust.
4.  **Hormones**: Port `core/biology/hormones.ts` to Rust.
5.  **Vitals**: Port `core/biology/vitals.ts` to Rust.
6.  *Integration Point:* Biology Unit Tests (Verify curves match TS reference).

**Phase 2: Sensory & Body**
7.  **Interoception**: Port `core/senses/interoception.ts`.
8.  **Proprioception**: Port `core/senses/proprioception.ts`.
9.  **SomaticBody**: Port `core/somatic/SomaticBody.ts` (Apply Refactor 3.2).

**Phase 3: World & Items**
10. **Homestead/Shed**: Port `world/homestead.ts` and `world/shed.ts`.
11. **WorldService**: Port `world-service.ts` logic to Rust ECS/System (Apply Refactors 3.1, 3.3, 3.4).

**Phase 4: Agent Mind**
12. **Emotions/DarkTriad**: Port `granular-emotions.ts`, `dark-triad.ts`.
13. **FreeWillLoop**: Port logic to Rust Planner/ActionSelector (Apply Refactor 3.1).
14. **ConsciousnessKernel**: Port `consciousness-kernel-enhanced.ts` (Apply Refactors 3.1, 3.2).

---

## 7. Acceptance Criteria (Auditor-Gated)

Audit will fail unless these tests exist and pass:

1.  **Determinism Test:**
    - Run `Metabolism::tick()` 1000 times with same seed.
    - Assert final state hash is identical across 5 runs.
2.  **Replay Test:**
    - Record inputs from `FreeWillLoop`.
    - Replay inputs.
    - Assert `Decision` output is identical.
3.  **No-Stub Scan:**
    - `rg "TODO|FIXME|unimplemented!"` returns 0 results.
4.  **Time Isolation:**
    - Search for `SystemTime::now()` in `apps/server/src/sim`.
    - Must return 0 results (allow only in `infra` or `time_source` impl).
5.  **Encryption Check:**
    - Verify `sqlcipher` dependency is used, not `sqlite`.

---

## 8. Windsurf Execution Constraints

1.  **Reference Only:** Windsurf may read Gemini Universe TS files to understand the logic.
2.  **Porting Authority:** Windsurf MUST port the logic to Rust as specified in the Master Plan. Copy-pasting TS to Rust is invalid.
3.  **Refactor Enforcement:** Windsurf MUST apply all "Required Refactors" (injecting TimeSource, removing side effects) *during* the port.
4.  **Deviation Ban:** Any deviation from the algorithm's logic requires a new Antigravity Plan.
5.  **Stop on Violation:** If AMP detects a `todo!()` or `Date.now()`, execution halts immediately.

**APPROVED FOR EXECUTION.**
