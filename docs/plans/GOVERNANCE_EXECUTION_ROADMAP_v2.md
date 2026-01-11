---
status: APPROVED
---

# MARKENZ EXECUTION ROADMAP (Phase 0 → Phase 9)

STATUS: BINDING  
UNIVERSE: Markenz  
DETERMINISM: Hardcore (seed + ordered inputs + fixed timestep = identical replay)  
AUTHORITY: Rust engine only (`apps/engine`)

---

## Global Invariants (Apply to Every Phase)

- **Rust owns truth:** Single-writer, fixed-timestep loop is ONLY component allowed to mutate world state.
- **TypeScript control plane:** May authenticate, authorize, validate, order, persist. MUST NEVER mutate world state.
- **React UI:** Read-only by default. Any action becomes InputEvent.
- **PostgreSQL:** Append-only event log is truth.
- **All external influence is InputEvent:** Tick-indexed and deterministically ordered.
- **Deterministic RNG only:** Engine-side streams. Every draw audit-logged.
- **No wall-clock influence:** Wall clock may schedule ticks but never enters state evolution.
- **Stable ordering everywhere:** No nondeterministic maps/sets in authority state.
- **Cognition cannot mutate:** Pipeline: Perception → Intent → Volition → BioVeto → PhysicsValidate → Commit.
- **Replay is law:** Replay tests must prove equality.

---

## Phase 0 — Repo + Offline Stack Baseline Closure

Boot full stack offline. Establish immutable event sourcing, hash-chain integrity, deterministic tick progression.

Deliverables: Offline stack, OIDC/RBAC, Schema, Engine loop, Genesis snapshot
Exit: Works offline, RBAC enforced, Events logged, Engine advances, Replay identical

---

## Phase 1 — Deterministic Kernel + Replay Harness

Prove determinism formally via replay and snapshot equivalence.

Deliverables: Deterministic scheduler, RNG streams, canonical hashing, snapshot/replay, genesis world
Exit: Seed + events identical hashes, Snapshot replay equals full replay, No authority leakage

---

## Phase 2 — World Representation v1 (Terrain + Entities)

Replace abstract world with deterministic spatial reality.

Deliverables: Chunked terrain, biomes, structures/tools/vehicles/inventories, mechanics (Move/Gather/Build/Mine)
Exit: Actions deterministic, causality visible, replay identical

---

## Phase 3 — Embodied Biology v1

Enforce biological reality and veto unsafe actions.

Deliverables: Metabolism, hydration, thermoregulation, sleep, nutrients, injury/healing, immune, endocrine, BioVeto
Exit: Agents starve/fatigue/heal/sleep, Unsafe actions vetoed, Biology deterministic

---

## Phase 4 — Cognition Engine (No LLM)

Deterministic minds and language, fully offline.

Deliverables: Perception→Drives→Intent→Queue, Planner, Skills/habits, Deterministic English, Inner monologue, Learning/memory
Exit: Identical thoughts/speech, No LLM, Replay identical

---

## Phase 5 — Social Dynamics + Scaling

Emergent society without determinism drift.

Deliverables: Relationship graph, Attachment styles, Trust/conflict/bonding, Gossip/reputation, Culture, Multi-agent scaling
Exit: Social state replay-identical, Stable tick rate

---

## Phase 6 — Genetics + Reproduction

True population growth with lineage.

Deliverables: Double-helix genome, Recombination/mutation, Phenotype, Reproduction pipeline, Lineage trees
Exit: Same parents + seed identical genome, Lineage deterministic

---

## Phase 7 — Economy + Governance

Deterministic rules governing society and resources.

Deliverables: Property/ownership, Markets, Farming/animals, Elections, Laws/policies, Courts/penalties
Exit: Laws constrain deterministically, Governance replay-identical

---

## Phase 8 — WebGPU Renderer + Transparency UI

Professional visualization without authority leakage.

Deliverables: WebGPU renderer, Render packets, Multi-monitor, Diff heatmaps, Causality graph, Time-travel debugger
Exit: Renderer hash-stable, UI never mutates

---

## Phase 9 — Security + Integrity Hardening

Lock security without breaking determinism or offline mode.

Deliverables: Keycloak primary, Authentik backup, Encryption at rest, Tamper-evident logs, Integrity UI
Exit: Tampering detected, Passkeys work offline, Replay passes
