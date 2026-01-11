---
status: APPROVED
authority: MARKENZ_GOVERNANCE_PHASE_1_DETERMINISTIC_KERNEL_AND_REPLAY_HARNESS_LOCK
enforcement: CI + local verification scripts
rule: If any item fails → NO MERGE
---

# RUST REALITY LOCK

**Purpose:** Enforce absolute separation between production code (real, auditable, deterministic) and test code (allowed to use unsafe patterns for verification).

**Binding Law:** No stubs, mocks, fakes, or placeholder implementations in production Rust code paths.

---

## A. BANNED IN PRODUCTION CODE

### Banned Macros (Hard Fail)
- `todo!()`
- `unimplemented!()`
- `panic!(...)` when used as placeholder (e.g., `panic!("todo")`)
- `dbg!()` (logging only; tracing allowed in tests)

### Banned Methods
- `.unwrap()` (outside `#[cfg(test)]`)
- `.expect(...)` (outside `#[cfg(test)]`)

### Banned Attributes (in production paths)
- `#[allow(unused)]`
- `#[allow(dead_code)]`
- `#[allow(unreachable_code)]`
- `#[allow(clippy::unwrap_used)]`
- `#[allow(clippy::expect_used)]`
- `#[allow(clippy::todo)]`
- `#[allow(clippy::unimplemented)]`
- `#[allow(clippy::panic)]`

These pragmas are categorically forbidden because they mask incomplete implementations.

### Banned Dependencies (in production graph)
Any crate that is fundamentally a mock/stub/fake/test-only framework:
- mockall
- fake
- double
- rstest
- proptest
- quickcheck
- arbitrary
- fakeit

These are allowed ONLY if they are dev-dependencies and never linked into release builds.

### Banned Patterns
- Empty impl bodies indicating stubs: `impl Trait for Type {}` with no methods
- Dead code (unreachable statements, unused variables without cfg(test))

---

## B. ALLOWED IN TESTS AND TEST CODE

### Test Paths (cfg(test) and **/*test*)
- `unwrap()` and `expect()` permitted for clarity (test setup should panic if preconditions fail)
- `todo!()` in test stubs (document the incomplete test)
- `panic!()` for assertions
- Logging via `dbg!()` for debugging test failures

### Dev Dependencies
- test frameworks (criterion, proptest as dev-dep only, arbitrary as dev-dep only)
- mocking frameworks (mockall as dev-dep only)
- These must NOT appear in release dependency graph

---

## C. ENFORCEMENT MECHANISMS

### C.1 Rust Lints (Primary)
Workspace-level lint configuration enforces:
```toml
[lints.rust]
unsafe_code = "deny"
missing_docs = "warn"

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
todo = "deny"
unimplemented = "deny"
panic = "deny"
```

Per-crate lints override workspace defaults and are applied to runtime crates:
- apps/engine/Cargo.toml
- crates/*/Cargo.toml (except crates that are test harnesses)

### C.2 Guard Script (Secondary: tools/ci/reality_lock.sh)
Automated scan that fails CI if:
1. Any banned macro found in src/**, crates/**/src/**, apps/**/src/**
2. Excludes: `**/tests/`, **/*test*, lines under `#[cfg(test)]`
3. Any `#[allow(unused)]`, `#[allow(dead_code)]`, `#[allow(unreachable_code)]` in production paths
4. Any `unwrap()`/`expect()` in production paths
5. Any `dbg!()` outside tests
6. Cargo.lock shows banned crates in non-dev dependency tree

### C.3 cargo-deny (Tertiary: deny.toml)
Scans dependency graph for banned crate names in release profile.

### C.4 Cargo Clippy (Tertiary: CI)
Run: `cargo clippy -- -D warnings` to catch lint violations not in Cargo.toml

---

## D. REMEDIATION RULES

### If Production Code Violates Lock

| Violation | Remediation |
|-----------|-------------|
| `todo!()` / `unimplemented!()` | Implement the feature or split into separate tracked issue. |
| `.unwrap()` / `.expect()` | Use `?` operator, `.unwrap_or()`, or handle error explicitly. |
| `panic!("todo")` | Replace with proper error handling or state validation. |
| `dbg!()` | Replace with `tracing::debug!()` or remove. |
| `#[allow(...)]` on production code | Fix the underlying issue; do NOT suppress. |
| `impl Trait for X {}` (stub) | Either complete the impl or mark as `#[cfg(test)]`. |

### If Test Code Violates Lock
Tests MAY use unsafe patterns for clarity.
Document the test's intent clearly with comments.

---

## E. CI GATES (Required to Pass)

1. **cargo build --all** must succeed without warnings or errors
2. **cargo clippy -- -D warnings** must pass
3. **tools/ci/reality_lock.sh** must pass (no banned patterns found)
4. **cargo deny check bans** must pass
5. **cargo test --all** must pass (unit + integration tests)

All gates are FAIL-CLOSED: if any gate fails, build is rejected.

---

## F. LOCAL DEVELOPMENT

Recommended workflow:
```bash
./tools/ci/guard.sh           # Run the guard script first
cargo fmt                      # Format code
cargo test --all              # Unit and integration tests
cargo clippy -- -D warnings   # Check all lints
cargo build --release         # Build release binary
```

If guard.sh fails, review the output and fix violations before committing.

---

## G. FAILURE MODES (What Can Go Wrong)

### What This Lock PREVENTS
- Accidental merging of unfinished code
- Silent placeholders in production
- Nondeterministic APIs leaking into authority code
- Mocking frameworks silently linked into release builds

### What This Lock DOES NOT PREVENT
- Logic bugs in correct implementations
- Intentional panics for true error conditions
- Design issues (only catches format/pattern violations)

### Bypass Risk
A developer could manually suppress warnings or bypass the guard script.
This is a social contract enforcement: we trust devs to run the full gate.
CI enforces it automatically on all pushes.

---

## H. COMPLIANCE CHECKLIST

Before any PR is merged:

- [ ] Guard script passes locally: `./tools/ci/guard.sh`
- [ ] Clippy clean: `cargo clippy -- -D warnings`
- [ ] Tests pass: `cargo test --all`
- [ ] All new code includes auditable comments (header + inline)
- [ ] No new mock/stub/fake crates added as runtime dependencies
- [ ] Any dev-dependencies using test frameworks are marked `[dev-dependencies]`

---

## I. AUDIT TRAIL

Every code change that touches production paths MUST be auditable by a random competent Rust dev reading the code:
- Every file starts with a header explaining purpose and rule enforcement
- Every non-trivial block has inline comments
- Every decision about error handling is explicit (not hidden behind unwrap/expect)

---

## FINAL RULE

**No code with the following characteristics will merge:**
- Compilation warnings
- Banned macros in production paths
- Missing error handling (unwrap/expect outside tests)
- Suppressed lints in production code
- Stub or mock implementations
- Mock/fake/test frameworks in release dependency graph

**Everything that compiles and passes tests MUST be real, complete, and auditable.**
