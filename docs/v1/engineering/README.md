# Engineering Standards

**Authority:** Principal Architect

This section details the coding standards, repository structure, and enforcement mechanisms ("Reality Lock") that keep Markenz production-ready.

## Artifacts

* **[Reality Lock & Code Standards](REALITY_LOCK_AND_CODE_STANDARDS.md)**: Rules for Rust code, forbidden patterns (mocks, stubs), and linting configurations.
* **[Repo Structure & Ownership](REPO_STRUCTURE_AND_OWNERSHIP.md)**: Map of the monorepo, CODEOWNERS definitions, and logical boundaries.

## Key Principles

1. **Fail-Closed:** Unrecoverable errors must panic, not warn.
2. **No Mocks:** Logic must be tested against real implementations or deterministic simulators, not mocks.
3. **Auditability:** All code changes must preserve the hash chain integrity.
