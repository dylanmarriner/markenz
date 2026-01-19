# 2. Docs-as-a-Product Strategy

**Status:** Accepted
**Date:** 2026-01-19
**Deciders:** Principal Architect
**Consulted:** Product Leadership

## Context and Problem Statement

Documentation in technical repositories often rots, becoming out of sync with the product. For Markenz, which claims "auditability" and "scientific rigor," inaccurate documentation is a critical failure mode. We need a strategy to ensure documentation quality matches code quality.

## Decision Drivers

* Enterprise readiness requires trustworthy docs.
* Documentation must support multiple inputs (GitHub view, Static Site Generators).
* Changes to docs must be auditable and linked to releases.

## Considered Options

* **Docs-as-a-Product:** Treat docs as code, with tests, versioning, and release gates.
* **Best Effort:** Update docs when possible.
* **External Knowledge Base:** Hire technical writers to maintain a separate portal.

## Decision Outcome

Chosen option: **Docs-as-a-Product**, because it enforces the same rigor on explanation as we place on implementation.

### Positive Consequences

* Building the docs can be part of CI/CD.
* Documentation structure is vetted and designed (Information Architecture).
* Changelogs are mandatory.

### Negative Consequences

* Higher friction for developers (must write docs to ship code).
* Requires strict linting and style guides.

## Pros and Cons of the Options

### Docs-as-a-Product

* Good, because it aligns with the "Reality Lock" philosophy of Markenz.
* Good, because it eliminates "hidden knowledge."
* Bad, because it slows down "fast and loose" prototyping (which is already forbidden in Markenz).

### Best Effort

* Bad, because it inevitably leads to drift and mistrust.

### External Knowledge Base

* Bad, because it introduces a synchronization latency.
