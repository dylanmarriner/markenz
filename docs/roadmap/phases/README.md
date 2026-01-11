# MARKENZ Governance Phase Plans

This directory contains the complete breakdown of the MARKENZ_GOVERNANCE_MASTER_ROADMAP into executable phase plans, one per phase.

## Overview

The master roadmap defines 10 sequential phases (0-9) plus a framework for future phases (10+). Each phase plan is a complete, standalone specification suitable for direct Windsurf execution.

**Total Content Loss:** ZERO  
**Content Reordering:** NONE  
**Phase Collapsing:** NONE  
**Prohibition Relaxation:** NONE

All original requirements, prohibitions, and enforcement rules are preserved exactly as specified.

## Phase Plans (Complete)

| Phase | File | Objective | Status |
|---|---|---|---|
| 0 | `MARKENZ_GOVERNANCE_PHASE_0_REPO_AND_EVENT_LOG_BASELINE.md` | Establish offline stack, immutable event sourcing, determinism kernel | EXECUTABLE |
| 1 | `MARKENZ_GOVERNANCE_PHASE_1_DETERMINISTIC_KERNEL_AND_REPLAY_HARNESS_LOCK.md` | Prove determinism formally; lock DeterministicRng and snapshot mechanism | EXECUTABLE |
| 2 | `MARKENZ_GOVERNANCE_PHASE_2_WORLD_REPRESENTATION_V1.md` | Replace abstract world with deterministic spatial reality; introduce real mechanics | EXECUTABLE |
| 3 | `MARKENZ_GOVERNANCE_PHASE_3_EMBODIED_BIOLOGY_V1.md` | Introduce causal physiology; enforce BioVeto for impossible actions | EXECUTABLE |
| 4 | `MARKENZ_GOVERNANCE_PHASE_4_OFFLINE_COGNITION_ENGINE.md` | Deterministic minds, planning, learning, and language (fully offline, no LLM) | EXECUTABLE |
| 5 | `MARKENZ_GOVERNANCE_PHASE_5_SOCIAL_DYNAMICS_AND_MULTI_AGENT_SCALING.md` | Emergent social systems; scale to dozens of agents without determinism drift | EXECUTABLE |
| 6 | `MARKENZ_GOVERNANCE_PHASE_6_GENETICS_AND_REPRODUCTION.md` | Population growth via deterministic genetics and reproductive biology | EXECUTABLE |
| 7 | `MARKENZ_GOVERNANCE_PHASE_7_GOVERNANCE_AND_ECONOMY.md` | Deterministic rules and economy constraints enforced by authority | EXECUTABLE |
| 8 | `MARKENZ_GOVERNANCE_PHASE_8_WEBGPU_RENDERER_AND_VISUALIZATION_UPGRADE.md` | Professional visualization derived from authoritative snapshots | EXECUTABLE |
| 9 | `MARKENZ_GOVERNANCE_PHASE_9_SECURITY_AND_INTEGRITY_HARDENING.md` | Lock security without breaking determinism or offline-first operation | EXECUTABLE |
| 10+ | `MARKENZ_GOVERNANCE_PHASE_10_PLUS_FUTURE_GOVERNANCE_PLACEHOLDER.md` | Framework for future governance phases (reserved; to be defined via law proposal) | PLACEHOLDER |

## Constitutional Principles (Always Active)

The following principles from Section 1 of the master roadmap apply to ALL phases and may NEVER be weakened:

1. **Authority Law** — Rust engine is ONLY world-state mutator
2. **Determinism Law** — Same seed + ordered InputEvents → identical `world_hash` sequence
3. **Transparency Law** — Everything meaningful is observable and logged
4. **Offline-First Law** — No external network dependency at runtime
5. **Human Equivalence & Agent Parity Law** — Every agent identical except identity data
6. **Founder Amplification Law** — Bounded, non-inheritable, baseline for offspring
7. **No-Mock / No-Stub Law** — No TODO, FIXME, stub, mock, fake implementations
8. **Self-Evolution Law (Bounded)** — Evolution is state, never code

## Execution Order

Phases MUST be executed in order: 0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → (10+)

Each phase depends on completion of the previous phase. No phase may be skipped or reordered.

**Blockers:** If any phase fails its CI gates, all subsequent phases are blocked until the failure is resolved.

## Structure of Each Phase Plan

Every phase plan contains these sections in exact order:

1. **Phase Objective** — What this phase achieves
2. **Governance Domains In Scope** — Which policy areas are introduced/expanded
3. **Systems & Modules Touched** — Exact engine/server/web modules affected
4. **Event Types** — All events introduced (MUST be logged)
5. **Determinism Guarantees** — Explicit determinism properties that MUST hold
6. **Enforcement Rules** — What is vetoed; where veto occurs; hard failures
7. **Audit & Replay Requirements** — Required logs, snapshots, audit tooling
8. **Tests (MANDATORY)** — Unit, integration, determinism, replay, parity tests
9. **CI / Compilation Gates** — Gates that must pass before proceeding
10. **Explicit Prohibitions** — Forbidden actions, shortcuts, implementations
11. **Phase Completion Criteria (Checklist)** — Strict checkbox list
12. **Authority Statement** — Binding statement; derivation from master roadmap
13. **Traceability** — Map each section to master roadmap source

## Determinism Guarantee Chain

Determinism is cumulative and non-negotiable:

- **Phase 0:** Baseline determinism (same seed + events → same hashes)
- **Phase 1:** Determinism hardened (DeterministicRng locked; snapshot replay proven equal)
- **Phase 2+:** Determinism maintained through all subsequent phases
- **Final:** After Phase 9, full replay of all phases produces identical `world_hash` sequence

**Violation:** Any divergence in replay → system HALT with violation report.

## Authority Boundary Enforcement

Throughout all phases:

- **Engine** owns: world state, tick advancement, RNG, all rule enforcement, all commits
- **Server** owns: auth (Keycloak), RBAC, InputEvent validation, persistence, fanout
- **Web** owns: rendering (WebGPU), inspection, operator tooling
- **Prohibited:** Server patching state, bypassing veto logic, computing outcomes directly; Web mutating state or RBAC bypass

## Governance Immutability

All governance specifications in these phase plans are IMMUTABLE:

- No phase may be weakened or reinterpreted
- No prohibition may be relaxed
- No gate may be skipped
- No enforcement rule may be modified
- No test may be removed

**Exception:** New laws may be proposed (Phase 7+) via voting, but they may NEVER contradict these phase plans or Section 1 of the master roadmap.

## File Organization

```
docs/roadmap/
├── MARKENZ_GOVERNANCE_MASTER_ROADMAP.md (source of truth)
└── phases/
    ├── README.md (this file)
    ├── MARKENZ_GOVERNANCE_PHASE_0_*.md
    ├── MARKENZ_GOVERNANCE_PHASE_1_*.md
    ├── ... (phases 2-9)
    └── MARKENZ_GOVERNANCE_PHASE_10_PLUS_*.md
```

## How to Use These Plans

### For Implementation

1. Read Phase 0 plan completely
2. Implement all requirements (modules, events, tests, CI gates)
3. Verify all completion criteria pass
4. Move to Phase 1
5. Repeat for each phase in order

### For Verification

1. Use traceability section of each phase plan to verify no loss from master roadmap
2. Check CI gates before advancing to next phase
3. Run determinism tests from Phase 1 continuously (should never fail)
4. Run audit tools to generate governance reports

### For Audit/Compliance

1. Each phase plan is self-contained and auditable
2. Traceability section maps to master roadmap
3. Execution can be verified against completion criteria
4. Replay audits verify determinism throughout

## Relationship to Master Roadmap

**Master Roadmap** (MARKENZ_GOVERNANCE_MASTER_ROADMAP.md):
- Defines constitutional principles (Section 1)
- Describes system architecture (Section 2)
- Enumerates governance domains (Section 3)
- Prescribes phase sequence (Section 4)
- Defines change protocol (Section 5)
- Limits admin authority (Section 6)
- Specifies enforcement & failure modes (Section 7)

**Phase Plans** (this directory):
- Extract each phase from master roadmap
- Expand into complete, executable specification
- Provide detailed tests and CI gates
- Maintain exact traceability to source

**One-way Dependency:**
- Phase plans depend on master roadmap (never weaken it)
- Master roadmap does not depend on phase plans (master is immutable law)
- If conflict: master roadmap wins

## Future Phases (Phase 10+)

No phases are defined beyond Phase 9. Future phases must:

1. Be proposed via law proposal InputEvent (Phase 7+)
2. Include complete specification (all 12 sections)
3. Not weaken any Phases 0-9
4. Maintain determinism guarantee
5. Be voted on by agents (democratic approval)

Template for future phases provided in Phase 10+ placeholder.

## Status Summary

| Aspect | Status |
|---|---|
| All Phases Defined | ✓ COMPLETE |
| Traceability Complete | ✓ ZERO LOSS |
| Content Preservation | ✓ EXACT |
| Phase Ordering | ✓ PRESERVED |
| Prohibition Preservation | ✓ PRESERVED |
| Gate Enforcement | ✓ MANDATORY |
| Authority Enforcement | ✓ HARDCODED |
| Determinism Guarantee | ✓ CUMULATIVE |

## Authority

All phase plans in this directory are derived from:

**MARKENZ_GOVERNANCE_MASTER_ROADMAP.md**  
**Authority:** AMP / Antigravity / KAIZA-MCP  
**Status:** BINDING  
**Effective Date:** 2026-01-11

---

**Generated:** 2026-01-11  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Signature:** [REQUIRES AMP EXECUTION AUTHORITY]
