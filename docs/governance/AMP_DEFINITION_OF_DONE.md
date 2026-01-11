# AMP Definition of Done (Merge Blocker)
STATUS: BINDING
ENFORCEMENT: CI + local scripts. If any fails: NO MERGE.

## A. Global invariants (must hold always)
Authority:
- Rust engine (apps/engine) is the sole world-state mutator.
- apps/server and apps/web cannot mutate world state directly.

Determinism:
- No wall clock in authority state evolution.
- All RNG is engine-side DeterministicRng streams; every draw audit-logged.
- Stable ordering for entity/event/container iteration in authority code.
- world_hash checkpoints are produced at a fixed cadence and persisted.
- Replay tests prove:
  - same seed + same ordered InputEvents => identical world_hash sequence
  - snapshot replay equals full replay hashes

Transparency:
- Everything meaningful is observable:
  - per-tick diffs
  - per-agent bio/cog streams (vitals/hormones/emotions/somatics/thoughts/speech/intents/memory/learning)
  - full causality trace (event → validation/veto → commit → diff → hash)
- Inner monologue always enabled and streamed.

Offline-first:
- No external network dependency at runtime.
- All required services run locally/LAN via docker compose.

No-mock / no-stub:
- No TODO/FIXME placeholders.
- No “fake mechanics” used to satisfy a gate.
- Each delivered feature has:
  - automated tests OR
  - an automated runtime verification script that exercises real mechanics end-to-end (not manual).

Security:
- UI access requires OIDC login (Keycloak).
- Observer role cannot submit InputEvents.
- Admin role can submit InputEvents but cannot puppet agents directly; only lawful interventions via events.
- Secrets never committed to repo; env/example + local secrets only.
- Encryption at rest gates apply when introduced (Phase 9).

## B. Phase gate checklist template (must pass per phase)
Build:
- Engine builds and runs.
- Server builds and runs.
- Web builds and runs.
- Compose boots the full stack.

Tests:
- Unit tests pass (module-level).
- Integration tests pass (event pipeline).
- Determinism tests pass (replay + snapshot equivalence).
- Integrity tests pass (hash-chain verification).

Audit artifacts:
- Replay audit report generated (tools/audits).
- Hash checkpoint log captured for a standard run.
- Performance metrics captured for standard scenario.

Verification script:
- An automated script proves the phase feature end-to-end (no manual clicking required).

## C. Test suite requirements per phase
Minimum always-on suite:
- Determinism replay test (same seed + same events => same hashes).
- Snapshot equivalence test.
- Hash-chain integrity test.
- RBAC test (observer denied InputEvents; admin allowed).

Phase-dependent additions:
- Physics determinism tests when physics expanded.
- Biology progression + veto stability tests when bio expanded.
- Cognition determinism tests (NLG/plan selection) when cognition expanded.
- Multi-agent scaling drift test when scaling expanded.
- Governance enforcement determinism tests when laws introduced.
- Render packet stability test when WebGPU renderer introduced.
- Tamper detection tests when integrity hardening introduced.

## D. “No mock/stub” enforcement (how verified)
- CI rejects:
  - TODO/FIXME in tracked source and docs gated by phase
  - “stub”, “mock”, “fake”, “placeholder” implementations in authority path
- Runtime verification scripts must touch real code paths and assert real outcomes.

## E. Performance/scale targets (per milestone)
Phase 0–1:
- 10–20 ticks/sec with 2 agents and baseline telemetry.
- WS fanout stable to multiple observers.

Phase 2–4:
- Stable tick scheduling under real mechanics and biology/cognition with a small agent count (2–10).

Phase 5+:
- Dozens of agents at 10–20 ticks/sec on a modern desktop, with outcome-invariant telemetry throttling.

## F. Security hardening gates
Phase 0+:
- Keycloak login required; RBAC enforced on server.
- JWT verification via JWKS from local Keycloak (cached). No “decode without verify” allowed in gated phases.

Phase 9:
- Encryption at rest enabled (disk-level + envelope encryption for sensitive metadata).
- Tamper-evident event log verification UI + endpoint.
- Auth events audited immutably.

## G. Reproducibility gates
- Standard seeded run produces identical world_hash checkpoints across repeated runs on the same architecture.
- Replay from event log matches checkpoints exactly.
- Snapshot replay matches full replay exactly.
