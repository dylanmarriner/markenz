# AMP DEFINITION OF DONE v2
(Merge Blocker · Phase Gate · Binding Law)

STATUS: BINDING  
ENFORCEMENT: CI + local verification scripts  
RULE: If any item fails → **NO MERGE**

---

## A. Global Invariants (Must Hold at All Times)

### Authority
- Rust engine (`apps/engine`) is the **sole** mutator of world state.
- `apps/server` and `apps/web` **cannot** mutate world state directly or indirectly.

### Determinism
- No wall clock participates in authority state evolution.
- Fixed timestep; tick index is authoritative.
- All randomness uses engine-side `DeterministicRng` streams.
- Every RNG draw is audit-logged with:
  `{ tick, subsystem, stream, callsite, value }`
- Stable ordering everywhere:
  - entity iteration
  - event ordering
  - container traversal
- `world_hash` checkpoints emitted at fixed cadence.
- Replay proofs required:
  - Same seed + same ordered InputEvents ⇒ identical hash sequence
  - Snapshot replay ⇒ identical to full replay

### Transparency
- Everything meaningful is observable and logged:
  - Per-tick state diffs
  - Per-agent bio/cog streams (vitals, hormones, emotions, somatics)
  - Thought streams, speech, intents, memory reads/writes, learning updates
  - Full causality trace:
    InputEvent → Validate/Veto → Commit → Diff → Hash
- Inner monologue is **always enabled** and streamed.

### Offline-First
- No external network dependency at runtime.
- Entire system runs locally or on LAN.
- Identity, DB, and services boot via local stack only.

### No-Mock / No-Stub Law
- No `TODO`, `FIXME`, `stub`, `mock`, `fake`, or placeholder implementations.
- No “manual-only” acceptance.
- Every feature must be exercised via:
  - Automated tests **or**
  - Automated runtime verification scripts that execute real mechanics.

### Security Baseline
- UI access requires OIDC login (Keycloak).
- RBAC enforced server-side:
  - Observer: read-only
  - Auditor: read-only + exports
  - Admin: InputEvents only (no puppeting)
- Secrets never committed to repo.
- Encryption-at-rest gates enforced when introduced.

---

## B. Phase Gate Checklist (Must Pass Per Phase)

### Build
- Engine builds and runs.
- Server builds and runs.
- Web builds and runs.
- Full stack boots via docker compose.

### Tests
- Unit tests pass (module-level).
- Integration tests pass (event pipeline end-to-end).
- Determinism tests pass:
  - Replay equality
  - Snapshot equivalence
- Integrity tests pass (hash-chain verification).

### Audit Artifacts (Required Outputs)
- Replay audit report generated (`tools/audits`).
- Hash checkpoint log captured for standard run.
- Performance metrics recorded for reference scenario.

### Verification Script
- An automated script executes the phase’s real mechanics end-to-end.
- No manual clicking or visual confirmation allowed.

**If any item above fails → Phase is NOT DONE.**

---

## C. Test Suite Requirements (Cumulative)

### Always-On Minimum
- Determinism replay test
- Snapshot equivalence test
- Hash-chain integrity test
- RBAC enforcement test:
  - Observer denied InputEvents
  - Admin allowed InputEvents

### Phase-Triggered Additions
- Physics determinism tests when physics expands
- Biology progression + BioVeto stability tests
- Cognition determinism tests (planning + NLG)
- Multi-agent drift tests when scaling
- Governance enforcement determinism tests
- Render packet hash stability tests (WebGPU)
- Tamper-detection tests when integrity hardening is enabled

---

## D. “No Mock / No Stub” Enforcement (Verification Rules)

### Static Enforcement (CI Hard Fail)
- Reject any occurrence of:
  - `TODO`
  - `FIXME`
  - `stub`
  - `mock`
  - `fake`
  - `placeholder`
- Reject global disable pragmas (e.g., `ts-nocheck`) in gated paths.
- Reject nondeterministic APIs in authority code.

### Behavioral Enforcement
- Every claimed feature must emit observable events.
- If it does not appear in:
  - event logs
  - state diffs
  - replay traces  
  then it **does not exist**.

---

## E. Performance and Scale Targets

### Phase 0–1
- 10–20 ticks/sec
- 2 agents
- Stable WS fanout to multiple observers

### Phase 2–4
- Stable ticks under real world + biology + cognition
- No tick overruns under baseline load

### Phase 5+
- Dozens of agents
- 10–20 ticks/sec on a modern desktop
- Telemetry throttling must not affect outcomes

Performance optimizations **must not** compromise determinism.

---

## F. Security Hardening Gates

### Phase 0+
- Keycloak login required.
- JWT verification via local JWKS (cached).
- No “decode without verify” allowed.

### Phase 9+
- Encryption at rest enabled:
  - Disk-level encryption
  - Envelope encryption for sensitive metadata
- Tamper-evident event log verification UI + API.
- Auth and admin actions logged immutably.

---

## G. Reproducibility Gates

- Standard seeded run produces identical `world_hash` checkpoints across repeated runs on the same architecture.
- Replay from event log matches checkpoints exactly.
- Snapshot replay matches full replay exactly.
- Any divergence:
  - Stops execution (fail-closed)
  - Produces divergence report:
    - First divergent tick
    - Subsystem RNG counters
    - Last matching hash

---

## Final Rule

A phase is **DONE** only when:
- All gates above pass,
- All artifacts are produced,
- Determinism and replay are proven,
- And no authority boundaries are violated.

Anything less is **NOT DONE**.
