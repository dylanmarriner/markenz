---
status: APPROVED
plan_type: reuse-migration
authority: antigravity
blocks_execution_without: true
---

# MARKENZ REUSE MIGRATION PLAN v2 (BLOCKER FIX)

> **AUTHORITY:** Antigravity (Sole Planning Authority)  
> **BINDING INPUTS:** AMP Audit (Mandatory), MARKENZ_UNIFIED_MASTER_PLAN (Lockstep/Rust), KAIZA-MCP Rules (Fail-Closed)  
> **MODE:** FAIL-CLOSED Enforced  

---

## 0. OBJECTIVE

This document provides a **deterministic, ambiguity-free specification** for migrating Gemini Universe subsystems to the Markenz architecture. It explicitly resolves all blocking issues raised in the AMP Audit of v1.

**CORE DIRECTIVE:** ANY deviation from this plan is a verified failure. Windsurf MUST STOP and request a new plan if ambiguity is encountered.

---

## 1. DEFINITIONS (MANDATORY)

These definitions are **BINDING**. No other interpretations are permitted.

### Reuse Class Definitions

- **REUSE AS-IS**
  - **Logic:** Unchanged. 1:1 port of algorithms and state machines.
  - **Changes:** Language porting ONLY (TypeScript → Rust).
  - **Prerequisite:** Code must be pure (no side effects, no I/O, no time, no randomness).
  - **Example:** `Metabolism`, `ChaosSys`.

- **REUSE WITH MODIFICATION**
  - **Logic:** Preserved. The core algorithm remains identical.
  - **Changes:** Interfaces refactored to meet Deterministic Law (Section 4).
  - **Prohibited:** Replacing control flow, changing scheduling models.
  - **Example:** `FreeWillDecisionLoop` (needs `TimeSource`), `SomaticBody` (needs `EventBus`).

- **REWRITE REQUIRED**
  - **Condition:** Any of the following:
    - Uses `setInterval`, `setTimeout`, or implicit timers.
    - Relies on global state (`global.bus`).
    - Uses non-deterministic sources (`Math.random`, `Date.now`) without injection capability.
    - Fundamentally architected for Node.js event current loop rather than Tick loop.
  - **Action:** Logic may be *referenced* for intent, but implementation must be built from scratch in Rust.
  - **Example:** `Server.js`, `BootManager`, `RuntimeLoop`.

**RULE:** If unsure → **REWRITE REQUIRED**.

---

## 2. COMPLETE REUSE CLASSIFICATION TABLE

**Scope:** ALL Gemini Universe subsystems found in `apps/backend/src`.

| Module | Gemini Path | Classification | Determinism Risk | Mandatory Action | Acceptance Criteria |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Metabolism** | `core/biology/metabolism.ts` | **REUSE AS-IS** | Low (Pure Calc) | Port 1:1 to Rust. | Glucose curves match ±0.01%. |
| **Hormones** | `core/biology/hormones.ts` | **REUSE AS-IS** | Low | Port 1:1. | Decay rates identical. |
| **Immune** | `core/biology/immune-system.ts` | **REUSE AS-IS** | Low | Port 1:1. | Antibody generation matches. |
| **Vitals** | `core/biology/vitals.ts` | **REUSE AS-IS** | Low | Port 1:1. | BP/Pulse algo matches. |
| **Chaos** | `chaos/ChaosSys.ts` | **REUSE AS-IS** | None | Port 1:1. | Seed N produces Stream N. |
| **Homestead** | `world/homestead.ts` | **REUSE AS-IS** | Low | Port 1:1. | Upgrade costs/logic match. |
| **Shed** | `world/shed.ts` | **REUSE AS-IS** | Low | Port 1:1. | Recipe outputs match. |
| **Interoception** | `core/senses/interoception.ts` | **REUSE WITH CONS** | Aggregation Order | Port 1:1 using sorted inputs. | Urgency scores match. |
| **Proprioception**| `core/senses/proprioception.ts`| **REUSE AS-IS** | None | Port 1:1. | Strain calc matches. |
| **Tactile** | `core/senses/tactile-system.ts` | **REUSE AS-IS** | None | Port 1:1. | Sensitivity map matches. |
| **Emotions** | `core/psychology/granular-emotions.ts`| **REUSE AS-IS** | None | Port 1:1. | Intensity vectors match. |
| **Dark Triad** | `core/psychology/dark-triad.ts` | **REUSE AS-IS** | None | Port 1:1. | Trait evolution matches. |
| **Free Will** | `core/free-will-decision-loop.ts` | **REUSE WITH MODIFICATION** | `Date.now`, RNG | Inject `TimeSource`, `ChaosStream`. | Replay of inputs = Same Choice. |
| **Consciousness**| `core/consciousness-kernel-enhanced.ts`| **REUSE WITH MODIFICATION** | Side Effects | Inject `EventBus` (return events). | Tick produces predictable events. |
| **Somatic Body** | `core/somatic/SomaticBody.ts` | **REUSE WITH MODIFICATION** | Global Bus | Remove `global.bus`. Inject Sink. | State updates are pure. |
| **Event Replay** | `core/event-replay-engine.ts` | **REUSE WITH MODIFICATION** | DB Dependency | Abstract `EventStore` trait. | Hash chain verifies correctly. |
| **Language** | `core/language-system.js` | **REUSE WITH MODIFICATION** | Large/Legacy | Extract logic, wrap in Rust. | Deterministic token output. |
| **Twin Init** | `core/twin-system-initializer.ts` | **REWRITE REQUIRED** | Side Effects/Boot | Re-impl as `UniverseBuilder`. | Deterministic Genesis Block. |
| **Boot Manager** | `core/boot-manager.ts` | **REWRITE REQUIRED** | Env/Filesystem | Re-impl as Config Loader. | Config object is read-only. |
| **Server** | `core/server.js` | **REWRITE REQUIRED** | Node.js HTTP | Replace with Rust Axum/Loop. | 20Hz fixed tick enforced. |
| **Transport** | `core/server/frontend-server.js`| **REWRITE REQUIRED** | WebSocket State | Replace with Event Queue. | No socket state in Sim. |
| **Networking** | `core/services/` | **REWRITE REQUIRED** | Promises/Async | Replace with IPC/Bus. | Zero async I/O in tick. |
| **Human Integ** | `core/human-systems-integration.js`| **REWRITE REQUIRED** | External API | Mock for Offline/Sim mode. | Zero external calls. |
| **Self-Reflect** | `core/psychology/self-reflection.ts`| **REWRITE REQUIRED** | Stubs/Missing | Implement new spec. | N/A |

---

## 3. DETERMINISTIC INTERFACE SPECIFICATION

You MUST use these interfaces. NO EXCEPTIONS.

### 3.1 TimeSource
```rust
pub trait TimeSource {
    /// Returns the current simulation time (NOT wall clock).
    fn now(&self) -> SimTime;
    /// Returns the delta since last tick.
    fn dt(&self) -> Duration;
}
// PROHIBITED: std::time::SystemTime::now(), Date.now()
```

### 3.2 RNG / Chaos System
```rust
pub trait ChaosStream {
    /// Returns a deterministic float [0.0, 1.0) derived from Seed + Tick + Salt.
    fn next_float(&mut self) -> f64;
    /// Returns a deterministic UUID.
    fn next_uuid(&mut self) -> Uuid;
}
// ALGORITHM: PCG or ChaCha20 (Seed derived from Genesis Hash).
// PROHIBITED: rand::thread_rng(), Math.random(), UUID v4 (random).
```

### 3.3 Event Bus
```rust
pub trait EventBus {
    /// Enqueues an event for the NEXT frame. No immediate callbacks.
    fn publish(&mut self, event: SimEvent);
}
// CONSTRAINT: Single Authority Queue. Ordered by (Tick, SourceID).
// PROHIBITED: async await in publish(), immediate side-effects.
```

### 3.4 Persistence Interface
```rust
pub trait Persistence {
    /// Appends events to the immutable log.
    fn append(&mut self, events: &[SimEvent]) -> Result<LogHash>;
}
// CONSTRAINT: All writes are append-only.
// SNAPSHOTS: Created deterministically every N ticks (e.g., 1000).
```

---

## 4. ACCEPTANCE TESTS (EXECUTABLE)

These tests are **MANDATORY** for every migrated subsystem.

### 4.1 Determinism Replay Test
- **Input:** Specific Seed (e.g., `0xDEADBEEF`), Sequence of 100 Ticks.
- **Execution:** Run system.
- **Output:** State Hash at Tick 100.
- **Fail Condition:** Hash differs on subsequent runs or across machines.

### 4.2 Time Isolation Scan
- **Execution:** `rg "Date\.now|SystemTime::now|check_time" src/core`
- **Fail Condition:** Any match outside of `RealRealTimeSource` implementation.

### 4.3 Offline-Only Scan
- **Execution:** `rg "fetch\(|axios|reqwest|http::" src/core`
- **Fail Condition:** Any network call found in simulation logic.

### 4.4 Chaos Stability Test
- **Execution:** Record 1000 RNG calls from `ChaosSys`.
- **Fail Condition:** Sequence changes when `TimeSource` is mocked/advanced differently.

---

## 5. MIGRATION SEQUENCE WITH HARD GATES

**STOP RULE:** If Phase N fails acceptance, Phase N+1 is **ILLEGAL** to start.

### Phase 1: Foundation (The Time & Chaos Anchor)
- **Goal:** Establish deterministic universe clock and RNG.
- **Modules:** `TimeSource`, `ChaosSys`, `EventBus`.
- **Entry:** Empty Rust Crate.
- **Exit:** `cargo test` passes 4.1, 4.2.

### Phase 2: Biological Substrate (Pure Logic)
- **Goal:** Port metabolic and somatic systems.
- **Modules:** `Metabolism`, `Hormones`, `Immune`, `Vitals`.
- **Entry:** Phase 1 Passed.
- **Exit:** Biology Unit Tests match TS reference outputs.

### Phase 3: The Conscious Loop
- **Goal:** Implement the decision engine.
- **Modules:** `FreeWillDecisionLoop`, `ConsciousnessKernel`.
- **Entry:** Phase 2 Passed.
- **Exit:** Replay Test (4.1) passes for full decision cycle.

### Phase 4: World & Persistence
- **Goal:** Ground the agent in the world.
- **Modules:** `WorldService`, `Homestead`, `Shed`, `Persistence`.
- **Entry:** Phase 3 Passed.
- **Exit:** World State Hash is deterministic.

---

## 6. TYPESCRIPT REFERENCE BOUNDARY

- **LOGIC REFERENCE (OK):** Reading TS files to understand formulas, state transitions, and constants.
- **CODE REUSE (RESTRICTED):** Copying TS code to Rust is **FORBIDDEN**. You must *rewrite* it in Rust syntax.
- **DIRECT COPY (OK - UI ONLY):** Types/interfaces may be copied for Frontend/Bridge use.
- **VERIFICATION:** AMP verifies by checking for `// ported from [path]` annotations.
- **AMBIGUITY RULE:** If TS code assumes `global` or `closure` state -> **REWRITE** using explicit `struct` state.

---

## 7. MASTER PLAN LAW ENFORCEMENT

Automated checks to enforce the Master Plan:

| Law | Enforcement Check |
| :--- | :--- |
| **Offline-Only** | CI fails if `apps/backend/src` imports any cloud SDKs. |
| **Single Universe** | All systems share ONE `UniverseState` struct. No split brains. |
| **Lockstep** | Execution crashes if `Tick(N)` starts before `Tick(N-1)` completes. |
| **No Global State** | `static mut` and global `let` are banned via linting. |
| **Encryption** | DB driver MUST be `sqlcipher`. `sqlite3` is banned. |

---

## 8. FORBIDDEN IMPORTS & PATTERNS (BINDING)

**CI-ENFORCEABLE BAN LIST:**

1.  `Date.now()` / `new Date()` (Use `ctx.time()`)
2.  `setInterval` / `setTimeout` (Use `ctx.scheduler()`)
3.  `Math.random()` / `crypto.randomUUID()` (Use `ctx.chaos()`)
4.  `global`, `process.env` (Use `Config` struct)
5.  `fetch`, `axios`, `http` (Strictly Banned)
6.  `fs.readFile` (Use `Persistence` trait)
7.  `console.log` (Use `tracing::info!` with structured fields)
8.  `any` / `unwrap()` (Strict typing only, handle all errors)

---

## 9. WINDSURF EXECUTION LAW

> **WARNING:** VIOLATION OF THIS SECTION REVOKES AGENT AUTHORITY.

### WINDSURF EXECUTION CONSTRAINTS

1.  **Scope Verification:** Windsurf may ONLY touch modules marked **REUSE AS-IS** or **REUSE WITH MODIFICATION** in Section 2.
2.  **Mandatory Refactors:** Windsurf MUST apply interfaces from Section 3. Leaving legacy `Date.now` calls is a **FAILURE**.
3.  **Ambiguity Stop:** If a TS file is unclear or relies on "magic" global state not defined here, Windsurf **MUST STOP** and ask Antigravity for a decision.
4.  **No Invention:** Windsurf CANNOT invent new features. If logic is missing in TS, mark as `TODO` and **FAIL** the task (requiring Planner intervention).
5.  **New Plan Trigger:** Any deviation from the Sequence (Section 5) requires a NEW Antigravity Plan.

**VERDICT:** PROCEED WITH CAUTION. FAIL CLOSED.
