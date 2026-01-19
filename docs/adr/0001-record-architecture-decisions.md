# 1. Record Architecture Decisions

**Status:** Accepted
**Date:** 2026-01-19
**Deciders:** Principal Architect
**Consulted:** Engineering Team

## Context and Problem Statement

We need to record architectural decisions for the Markenz project. As the system grows in complexity (biological simulation, physics, governance), relying on tacit knowledge or scattered chat logs is insufficient for an enterprise-grade system. A lack of formalized decision history makes onboarding difficult and architectural regression likely.

## Decision Drivers

* Traceability of design decisions.
* Need for "why" documentation, not just "what".
* Support for future audits and compliance reviews.

## Considered Options

* **ADR (Architecture Decision Records):** Lightweight, version-controlled text files.
* **Wiki/Confluence:** Centralized but separated from code versioning.
* **Governance Docs:** Expanding the existing heavily formalized governance documents.

## Decision Outcome

Chosen option: **ADR**, because it keeps the decision record checked in with the code, ensuring they are versioned together and easily reviewable in Pull Requests.

### Positive Consequences

* Decisions are immutable parts of the history.
* Review process is integrated into the standard development workflow.
* Format is standard and tool-compatible.

### Negative Consequences

* Requires discipline to write and update.
* Potential for drift if decision status isn't updated (e.g., Superseded).

## Pros and Cons of the Options

### ADR

* Good, because it lives with the code (Git).
* Good, because it uses Markdown (universal).
* Bad, because it requires manual index management (mitigated by tooling conventions).

### Wiki

* Good, because it is easy to edit.
* Bad, because it is desynchronized from the codebase state at any given commit.

### Governance Docs

* Good, because Markenz already uses them.
* Bad, because they are too heavy for smaller, technical architectural choices.
