# Reality Lock & Code Quality Standards

**Status:** BINDING  
**Authority:** `tools/ci/reality_lock.sh`  
**Parent Doc:** [README.md](../../README.md)

---

## 1. The Core Philosophy: "Rust Reality Lock"

The **Rust Reality Lock** is an enforcement mechanism designed to prevent the seepage of "fake" code into the authoritative simulation. In a system claiming "human equivalence," a placeholder is a lie.

**Rule:** If it is in the `release` build, it must be a real, functioning implementation.

---

## 2. Banned Patterns (Production Code)

The following are **strictly prohibited** in any code path reachable by the production binary (`apps/engine`).

### 2.1 Banned Macros (The "Five Horsemen")

1. `todo!()` - Immediate CI Failure.
2. `unimplemented!()` - Immediate CI Failure.
3. `panic!()` - Forbidden. Recoverable errors only.
4. `dbg!()` - Logging allowed in local testing only. Remove before commit.
5. `mock!()` / `stub!()` - No mocking frameworks in production logic.

### 2.2 Banned Idioms

* **Silent Unwrapping:** `option.unwrap()` or `result.expect()` are banned.
  * *Correction:* Handle the error. If the state is invalid, return a `BioVeto` or log a critical health metric drop. Do not crash the universe.
* **Dead Code attributes:** `#[allow(dead_code)]`.
  * *Correction:* Delete the code or use it.

### 2.3 Banned Dependencies

* No "Test-Only" crates in the main dependency graph (e.g., `mockall`, `fake`, `proptest`).
* These must be scoped to `[dev-dependencies]`.

---

## 3. Allowed Patterns (Test Code)

Inside `#[cfg(test)]` modules or `tests/` directories:

* `unwrap()` / `expect()`: **ALLOWED**. (Tests should panic on failure).
* `mockall`: **ALLOWED**.
* `todo!()`: **ALLOWED** (for sketching tests).

---

## 4. Error Handling Philosophy

**Biological vs. Technical Errors:**

* **Technical Error:** Database disconnected, Out of Memory. -> **System Halt (Fail Closed).**
* **Biological Error:** Agent tried to eat Apple but Inventory is Empty. -> **System Continue.**
  * This is not a Rust `Err` to be unwrapped. This is a simulation result.
  * Emit `ObservationEvent { context: "Eat", result: "Failed: MissingItem" }`.

---

## 5. CI Gates

The repository is guarded by four gates. If any gate fails, the build is rejected.

1. **Format Gate:** `cargo fmt -- --check`.
2. **Lint Gate:** `cargo clippy -- -D warnings`. (Warnings = Errors).
3. **Reality Gate:** `tools/ci/reality_lock.sh` scans for banned macros/crates.
4. **Determinism Gate:** `verify_determinism.sh` runs the double-replay test.

---

## 6. Contributor Obligations

As a Senior Engineer contributing to Markenz:

1. **You do not stub.** You implement the feature or you don't commit it.
2. **You do not silence lints.** You fix the code.
3. **You prove reality.** Every biological system added must include a scenario test proving it has physiological consequences (e.g., "If I remove the stomach, the agent stops gaining calories").

---

## 7. Versioning & Breaking Changes

* All changes to `apps/engine` authority logic that affect the `WorldHash` are **Breaking Changes**.
* They require a `Snapshot` migration or a strict protocol for "New Genesis."
* Never modify logic retroactively. History is immutable.
