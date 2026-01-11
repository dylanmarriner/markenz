# Markenz Comprehensive Code Audit & Commentary Guide

**Date:** 2026-01-11  
**Authority:** ANTIGRAVITY (AMP)  
**Status:** IN PROGRESS - DETAILED AUDIT COMMENTARY  
**Purpose:** Enable full auditability and understanding of Markenz codebase

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [Component Audit Checklist](#component-audit-checklist)
4. [Commentary Standards](#commentary-standards)
5. [Critical Sections](#critical-sections)
6. [Determinism Verification](#determinism-verification)

---

## Executive Summary

The Markenz repository is a **deterministic universe simulation engine** designed to safely migrate Gem-D and Gem-K agents from the Gemini Universe. The system prioritizes:

- **Deterministic execution** (identical results from identical inputs)
- **State reproducibility** (full universe state capture and replay)
- **Agent preservation** (identity, biology, consciousness remain intact)
- **Auditability** (every decision traceable and verifiable)

### Key Design Constraints

1. **No random stdlib functions** - all randomness via seeded RNG subsystems
2. **Ordered collections** - BTreeMap/BTreeSet instead of HashMap for consistent iteration
3. **Immutable external state** - simulation independent of system time, filesystem, network
4. **Complete state snapshots** - periodic captures enable time-travel debugging

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    MARKENZ SIMULATION ENGINE                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────┐  ┌──────────────┐  ┌────────────────────┐ │
│  │  apps/engine    │  │  crates/rng  │  │ crates/deterministic│ │
│  │  (EXECUTABLE)   │  │  (BOUNDARY)  │  │   (BOUNDARY)       │ │
│  └────────┬────────┘  └──────┬───────┘  └─────────┬──────────┘ │
│           │                  │                    │             │
│           └──────────────────┼────────────────────┘             │
│                              │                                  │
│  ┌────────────────────────────▼──────────────────────────────┐ │
│  │          crates/world (EXECUTABLE)                        │ │
│  │  • universe state & agents                               │ │
│  │  • terrain generation & collision                        │ │
│  │  • biological systems (bio/)                             │ │
│  │  • cognitive systems (cognition/)                        │ │
│  │  • actions (gathering, mining, crafting, building)       │ │
│  └────────────────────────────────────────────────────────┘ │
│                              │                                │
│  ┌────────────────┬──────────┴──────────┬────────────────┐   │
│  │                │                     │                │   │
│  ▼                ▼                     ▼                ▼   │
│ events      persistence           physics            protocol │
│ (BOUNDARY)  (INFRASTRUCTURE)   (EXECUTABLE)        (BOUNDARY) │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Crate Responsibilities

| Crate | Type | Responsibility |
|-------|------|-----------------|
| **rng** | BOUNDARY | Deterministic seeded random streams for all subsystems |
| **deterministic** | BOUNDARY | Math & collection wrappers ensuring consistent ordering |
| **world** | EXECUTABLE | Complete universe state, agents, terrain, actions |
| **events** | BOUNDARY | Event definitions & serialization |
| **persistence** | INFRASTRUCTURE | Database snapshots, replay, checkpointing |
| **physics** | EXECUTABLE | Collision detection, movement physics |
| **protocol** | BOUNDARY | Network messages, serialization contracts |
| **apps/engine** | EXECUTABLE | Main simulation loop, tick advancement |
| **apps/server** | BOUNDARY | REST API gateway (NOT simulation logic) |

---

## Component Audit Checklist

### ✅ Done - With Full Commentary
- [ ] `crates/deterministic/src/lib.rs` - Core SimTime, ChaosStream
- [ ] `crates/deterministic/src/collections.rs` - Deterministic data structures
- [ ] `crates/deterministic/src/math.rs` - Floating point operations

### ⏳ In Progress - Adding Detailed Commentary
- [ ] `crates/rng/src/lib.rs` - RNG subsystem coordination
- [ ] `crates/rng/src/chacha20.rs` - ChaCha20 seeded RNG
- [ ] `crates/rng/src/rng_stream.rs` - Stream multiplexing
- [ ] `crates/rng/src/global_seed.rs` - Genesis seed management
- [ ] `crates/rng/src/audit_log.rs` - RNG audit trail

### 📋 Queued - Requiring Full Audit Pass
- [ ] `crates/world/src/universe.rs` - World state & agents
- [ ] `crates/world/src/types.rs` - Type definitions
- [ ] `crates/world/src/terrain.rs` - Terrain generation
- [ ] `crates/world/src/bio/mod.rs` - Biological systems
- [ ] `crates/world/src/cognition/mod.rs` - Cognitive systems
- [ ] `crates/world/src/action.rs` - Action processing
- [ ] `crates/physics/src/collision.rs` - Physics simulation
- [ ] `crates/persistence/src/database.rs` - Snapshot storage
- [ ] `crates/persistence/src/replay.rs` - Tick replay

---

## Commentary Standards

All source files MUST include:

### 1. File-Level Header Block
```rust
/**
 * ROLE: [EXECUTABLE | BOUNDARY | INFRASTRUCTURE | VERIFICATION]
 * REGISTERED IN: [MODULE_NAME]
 * EXECUTED VIA: [windsurf | CI/CD | manual]
 * USED BY: [comma-separated crates/modules]
 * PURPOSE: [Single sentence description]
 * FAILURE MODES: [Expected failure behaviors]
 * INVARIANTS: [What must always be true]
 *
 * Authority: antigravity
 */
```

### 2. Module-Level Documentation
```rust
/// Module purpose - what problem does this solve?
/// 
/// # Examples
/// ```
/// let example = Module::new();
/// ```
///
/// # Invariants
/// - [State invariant 1]
/// - [State invariant 2]
```

### 3. Function-Level Documentation
```rust
/// Brief description of what function does.
///
/// # Arguments
/// - `param1`: What it represents
/// - `param2`: What it represents
///
/// # Returns
/// Description of return value
///
/// # Panics
/// When does this panic?
///
/// # Examples
/// ```
/// let result = function(arg1, arg2);
/// ```
///
/// # Implementation Notes
/// Why implemented this way (performance, correctness, etc.)
```

### 4. Inline Comments for Complex Logic
```rust
// Explain WHY, not WHAT - the code shows WHAT
// Complex algorithm name and reference if applicable
// State transformations and invariant checking
// Edge cases and their handling
```

---

## Critical Sections Requiring Enhanced Commentary

### 1. Determinism Enforcement Points
- All entropy must flow through RNG subsystems
- No direct `std::random` or `thread_rng()`
- No `Instant::now()` or system time dependency
- All iterations must be deterministic (no HashMaps, HashMap-keyed loops)

### 2. State Hashing & Verification
- Universe state hash computation (must be identical across runs)
- Agent state hash computation
- Snapshot consistency verification

### 3. RNG Subsystem Boundaries
- Global seed initialization
- Per-subsystem seed derivation (using Blake3)
- Substream creation (for nested randomness)
- Audit trail recording

### 4. Biological System Constraints
- Metabolism state must be deterministic
- Hormone levels must follow exact formulas
- Reproduction genetics must use seeded RNG
- Observation (sensory input) must be deterministic

### 5. Agent Decision Making
- Cognition system must be deterministic
- Intent formation must use seeded RNG
- Memory recall must be deterministic
- Perception must be deterministic

### 6. Asset & Inventory Management
- All ownership must be tracked
- Pickup/drop actions must be deterministic
- Crafting recipes must yield consistent results
- Building placement must check collision deterministically

---

## Determinism Verification Checklist

For each module added, verify:

- [ ] **No entropy source outside RNG subsystems**
  - No `rand::thread_rng()`
  - No `std::time::Instant::now()`
  - No `std::collections::HashMap` (use BTreeMap)
  - No `std::collections::HashSet` (use BTreeSet)

- [ ] **All randomness seeded from genesis**
  - Seed derives from MARKENZ_GENESIS_SEED (1337)
  - Subsystem IDs are static, not dynamic
  - Substreams properly namespaced

- [ ] **State is fully reproducible**
  - No floating point non-determinism (use consistent formatting)
  - No system-dependent behavior
  - All operations idempotent

- [ ] **Proper audit trails**
  - RNG draws logged with timestamp, subsystem, value
  - State hashes computed and verified
  - Snapshots capture complete state at checkpoints

- [ ] **Tests verify determinism**
  - Run identical sequence twice
  - Verify output matches bit-for-bit
  - Verify state hashes match

---

## File Structure

### Each Source File Must Have:

1. **File header** (5-10 lines): Role, purpose, authority
2. **Module docs** (10-20 lines): High-level design, constraints
3. **Type definitions** with doc comments
4. **Public function docs** explaining WHY not just WHAT
5. **Complex algorithms** with inline explanation of steps
6. **Error handling** explicitly documented
7. **Test module** with determinism verification
8. **Invariants** listed at top of file and in tests

---

## Authority & Responsibility

| Role | Responsibility | Files |
|------|-----------------|-------|
| **EXECUTABLE** | Core simulation logic, state mutations | world, physics, engine |
| **BOUNDARY** | External interfaces, serialization | rng, deterministic, events, protocol |
| **INFRASTRUCTURE** | Persistence, storage, databases | persistence |
| **VERIFICATION** | Testing, validation, audit trails | tests/, determinism harness |

---

## Next Steps

1. **Phase 1:** Add detailed commentary to all boundary crates (rng, deterministic)
2. **Phase 2:** Add detailed commentary to world crate (universe, bio, cognition)
3. **Phase 3:** Add detailed commentary to executable crates (physics, action)
4. **Phase 4:** Add detailed commentary to infrastructure (persistence, events)
5. **Phase 5:** Create per-module audit checklists
6. **Phase 6:** Final review & certification

---

## References

- `ANTIGRAVITY_FORENSIC_MIGRATION_AUDIT.md` - System inventory
- `PHASE_0_1_2_COMPLETION_PLAN.md` - Phase execution roadmap
- `DETERMINISM_TEST_RESULTS.md` - Test verification results
