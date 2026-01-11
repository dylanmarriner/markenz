# M1 TO GLOBAL PHASE CROSSWALK

**STATUS:** Reference Document (M1 content mapped to global phases 0–9)  
**AUTHORITY:** KAIZA-MCP  
**PURPOSE:** Preserve M1 task coverage while resolving M1's legacy phase numbering

---

## Overview

MARKENZ_M1_FOUNDATION.md defined a milestone (M1) with 6 phases focused on a "vertical slice" prototype. This crosswalk maps each M1 task to exactly one global phase (0–9) per MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md.

**Key Rule:** M1 numbering is DEPRECATED. All execution uses global phase numbering (0–9). M1 content is preserved as task specifications within those global phases.

---

## Crosswalk Table

| M1 Phase | M1 Deliverable | Global Phase | Status | Notes |
|----------|---|---|---|---|
| **M1 Phase 1: SCAFFOLD & CORE PROTOS** | | **Phase 0** | MAPPED | M1 Phase 1 work is part of Phase 0 bootstrap |
| | Directory structure (apps/, crates/, tools/, infra/) | Phase 0 | Core | PLAN_PHASE_0_NORMALIZED specifies locked structure |
| | Rust workspace (server, protocol, deterministic crates) | Phase 0 | Core | apps/engine, crates/* initialization |
| | Node/React app initialization | Phase 0 | Core | apps/server, apps/web bootstrap |
| | cargo check, npm install pass | Phase 0 | Core | Build gate criterion |
| **M1 Phase 2: DETERMINISTIC ENGINE (RUST)** | | **Phase 0 + Phase 1** | MAPPED | M1 Phase 2 spans both global phases |
| | TimeSource, ChaosStream, SimLoop | Phase 0 + Phase 1 | Core | Phase 0: basic loop; Phase 1: RNG streams |
| | Deterministic tick loop | Phase 0 | Core | Fixed timestep in Phase 0 |
| | test_determinism_simple | Phase 1 | Gate | TEST-DET-001 |
| **M1 Phase 3: AUTH & INFRA** | | **Phase 0** | MAPPED | Keycloak + OIDC in Phase 0 |
| | docker-compose with Keycloak | Phase 0 | Core | Offline-first bootstrap |
| | auth-bootstrap tool | Phase 0 | Core | Realm import configuration |
| | OIDC verification in server | Phase 0 | Core | JWT validation with local JWKS |
| | curl tests (401/200 for auth) | Phase 0 | Test | RBAC enforcement gate |
| **M1 Phase 4: PERSISTENCE & EVENT LOG** | | **Phase 0** | MAPPED | Event log + SQLite in Phase 0 |
| | SQLCipher storage setup | Phase 0 | Core | Database initialization (Phase 9: encryption at rest) |
| | EventBus → EventLog wiring | Phase 0 | Core | Append-only input_events table |
| | test_encryption_enforced | Phase 0 | Gate | DB encryption verification (Phase 9 extends this) |
| **M1 Phase 5: WEBGPU & WORLD STREAMING** | | **Phase 2 + Phase 8** | MAPPED | Terrain in Phase 2; rendering in Phase 8 |
| | Basic Voxel World (Sim) | Phase 2 | Core | Chunked heightmap terrain |
| | /ws/chunks binary stream | Phase 2 + Phase 8 | Core | Chunk streaming protocol (Phase 2); WebGPU rendering (Phase 8) |
| | WebGPU renderer (Client) | Phase 8 | Core | 3D visualization of voxel world |
| | Browser chunk rendering + camera | Phase 8 | Core | WebGPU renderer with camera controls |
| **M1 Phase 6: AGENTS & INTERACTION** | | **Phase 1 + Phase 4** | MAPPED | Genesis agents in Phase 1; cognition/chat in Phase 4 |
| | Spawn gem-d, gem-k (simple walkers) | Phase 1 | Core | Genesis snapshot with imported agents |
| | Simple wander logic | Phase 4 | Core | Minimal cognition (deterministic action selection) |
| | Chat UI | Phase 4 | Core | Chat as InputEvent; inner monologue display |
| | Chat message in event log | Phase 0 | Core | Events logged immutably |
| | Deterministic agent movement | Phase 2 | Core | Movement action validation |

---

## Mapping by Global Phase

### **Phase 0: Repo + Offline Stack**
- M1 Phase 1: Scaffold & repo structure
- M1 Phase 2: Deterministic engine (basic tick loop)
- M1 Phase 3: Auth & Keycloak infrastructure
- M1 Phase 4: Persistence & event log
- M1 Phase 6: Agent initialization (genesis)

**Key M1 tasks absorbed:** Workspace init, docker-compose, OIDC, SQLCipher setup, agent spawning.

### **Phase 1: Deterministic Kernel**
- M1 Phase 2: RNG streams & determinism proofs
- M1 Phase 6: Agent state preservation (identity fingerprints)

**Key M1 tasks absorbed:** Determinism testing, RNG audit, snapshot format.

### **Phase 2: World Representation**
- M1 Phase 5: Voxel/heightmap world
- M1 Phase 6: Deterministic movement

**Key M1 tasks absorbed:** Terrain chunks, biome generation, movement mechanics.

### **Phase 3: Biology**
- (None from M1; M1 explicitly deferred biology to M2)

### **Phase 4: Cognition**
- M1 Phase 2: Action loop (volition)
- M1 Phase 5: Chat system (language)
- M1 Phase 6: Wander logic (minimal planning)

**Key M1 tasks absorbed:** Simple action selection, chat messages, inner monologue.

### **Phases 5–7: Genetics, Social, Governance**
- (None from M1; deferred to M2/M3)

### **Phase 8: WebGPU Renderer**
- M1 Phase 5: WebGPU renderer + chunk streaming

**Key M1 tasks absorbed:** 3D visualization, chunk streaming protocol.

### **Phase 9: Security**
- (M1 Phase 4 encryption extends here; Phase 9 hardens it)

**Key M1 tasks absorbed:** Encryption at rest, audit hardening.

---

## Deferred/Out-of-Scope Notes

The following M1 items are deferred to M2/M3 per the M1 document itself:

| Item | M1 Status | Global Phase |
|---|---|---|
| Full Biology (metabolism, hormones, sleep) | Deferred to M2 | Phase 3 |
| Full Cognition (GOAP planner, language engine) | Deferred to M2 | Phase 4 |
| Culture/Governance | Deferred to M3 | Phase 6–7 |
| Large-scale (50+ agents) | Deferred to M2 | Phase 5–6 |
| Advanced physics/collision | Deferred | Phase 2 refinement |

---

## Consistency Notes

- **M1 numbering is NOT used in global plan.** Global phases 0–9 are authoritative.
- **M1 "Phase" labels are reinterpreted as "task packs" within global phases.**
- **No duplicate work.** Each M1 deliverable appears exactly once in the global phase mapping.
- **No lost content.** All M1 tasks mapped to at least one global phase.

---

## Verification Checklist

- [ ] All M1 Phase 1 tasks mapped to Phase 0
- [ ] All M1 Phase 2 tasks mapped to Phase 0 + Phase 1
- [ ] All M1 Phase 3 tasks mapped to Phase 0
- [ ] All M1 Phase 4 tasks mapped to Phase 0 (+ Phase 9 for hardening)
- [ ] All M1 Phase 5 tasks mapped to Phase 2 + Phase 8
- [ ] All M1 Phase 6 tasks mapped to Phase 1 + Phase 2 + Phase 4
- [ ] No global phase references M1 numbering
- [ ] Deferred items (M2/M3) clearly noted as out-of-scope for this normalization

---

**END OF CROSSWALK**

This document is reference-only. Execution follows PLAN_PHASE_0_THROUGH_9_NORMALIZED.
