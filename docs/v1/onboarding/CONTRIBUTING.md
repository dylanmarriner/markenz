# Contributing to Markenz

**Status:** Active
**Enforcement:** CI/CD + Code Review

## The "Reality Lock"

Markenz enforces a "Reality Lock" on all contributions:

1. **No Mocks:** You cannot mock the database or the physics engine. Your code must work within the simulation.
2. **No "TODO" in Logic:** Do not commit `unimplemented!` or `todo!` in critical paths.
3. **Determinism:** Any PR that breaks deterministic replay is automatically rejected.

## Pull Request Process

1. **Issue:** Ensure an issue describes the feature or bug.
2. **Branch:** Create a branch `feat/` or `fix/`.
3. **ADR:** If changing architecture, include an ADR in `docs/adr/`.
4. **Tests:** Add unit tests.
5. **Review:** Request review from @markenz-architects.

## Style Guide

* **Rust:** Follow `rustfmt` and strict `clippy` settings.
* **Markdown:** Use semantic line breaks (one sentence per line) for better diffs.
