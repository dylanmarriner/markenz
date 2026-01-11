---
status: APPROVED
---

# AMP DEFINITION OF DONE v2
(Merge Blocker · Phase Gate · Binding Law)

STATUS: BINDING  
ENFORCEMENT: CI + local verification scripts  
RULE: If any item fails → **NO MERGE**

---

## A. Global Invariants (Must Hold at All Times)

### Authority
- Rust engine is the sole mutator of world state.
- TypeScript server and React web cannot mutate world state.

### Determinism
- No wall clock in authority state evolution.
- Fixed timestep; tick index authoritative.
- All randomness uses engine-side DeterministicRng streams.
- Every RNG draw audit-logged.
- Stable ordering everywhere.
- world_hash checkpoints at fixed cadence.
- Replay proofs: seed + ordered InputEvents = identical hash sequence

### Transparency
- Everything meaningful is observable and logged.
- Inner monologue always enabled and streamed.

### Offline-First
- No external network dependency at runtime.
- Entire system runs locally or on LAN.

### No-Mock / No-Stub Law
- No stubbed, mocked, or placeholder implementations.
- Every feature exercised via automated tests or verification scripts.

### Security Baseline
- UI requires OIDC login (Keycloak).
- RBAC enforced server-side.
- Secrets never committed to repo.

---

## B. Phase Gate Checklist

### Build
- Engine builds and runs.
- Server builds and runs.
- Web builds and runs.
- Full stack boots via docker compose.

### Tests
- Unit tests pass.
- Integration tests pass.
- Determinism tests pass.
- Integrity tests pass.

### Audit Artifacts
- Replay audit report generated.
- Hash checkpoint log captured.
- Performance metrics recorded.

### Verification Script
- Automated script executes phase's real mechanics end-to-end.
- No manual clicking allowed.

**If any item fails → Phase is NOT DONE.**

---

## C. Test Suite Requirements (Cumulative)

### Always-On Minimum
- Determinism replay test
- Snapshot equivalence test
- Hash-chain integrity test
- RBAC enforcement test

### Phase-Triggered Additions
- Physics determinism tests
- Biology + BioVeto stability tests
- Cognition determinism tests
- Multi-agent drift tests
- Governance enforcement tests
- Render packet hash stability tests
- Tamper-detection tests

---

## D. Enforcement Verification Rules

### Static Enforcement (CI Hard Fail)
- Reject any implementation shortcuts or incomplete code
- Reject global disable pragmas
- Reject nondeterministic APIs in authority code.

### Behavioral Enforcement
- Every claimed feature must emit observable events.

---

## E. Performance and Scale Targets

### Phase 0–1
- 10–20 ticks/sec
- 2 agents
- Stable WebSocket fanout

### Phase 2–4
- Stable ticks under real world + biology + cognition
- No tick overruns

### Phase 5+
- Dozens of agents
- 10–20 ticks/sec on modern desktop
- Telemetry throttling must not affect outcomes

---

## F. Security Hardening Gates

### Phase 0+
- Keycloak login required.
- JWT verification via local JWKS (cached).

### Phase 9+
- Encryption at rest enabled.
- Tamper-evident event log verification.
- Auth and admin actions logged immutably.

---

## G. Reproducibility Gates

- Standard seeded run produces identical world_hash checkpoints.
- Replay from event log matches checkpoints exactly.
- Snapshot replay matches full replay exactly.

---

## Final Rule

A phase is **DONE** only when: all gates pass, all artifacts produced, determinism and replay proven, no authority boundaries violated.

Anything less is **NOT DONE**.
