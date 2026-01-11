---
status: APPROVED
authority: KAIZA-MCP · AMP Authority
plan_id: MARKENZ_PHASE_1_DETERMINISM_VERIFIED
phase: 1
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: RNG Isolation · Deterministic Randomness · Snapshot Replay
---

# PHASE 1: DETERMINISM
## RNG Isolation · Deterministic Randomness · Snapshot Replay

**STATUS:** Verified for Kaiza MCP write tool authorization

**OBJECTIVES:**
- Introduce seeded RNG isolated to engine tick loop
- All randomness derived from tick-indexed sequence, never wall-clock
- Snapshot replay produces identical random sequences
- Cross-boundary determinism testing (multiple agents, multiple seeds)
- Zero non-deterministic side effects

**CORE DECISIONS:**
- RNG seeded per-world from genesis.seed
- Deterministic RNG: PCG64 with tick-based advancement
- No external entropy sources
- All random choices serialized in StateTransition log
- Snapshot → Replay equivalence guaranteed

**DELIVERABLES:**
- PCG64 RNG implementation in crates/world
- Tick-indexed RNG state advancement
- Snapshot replay test (100% hash match)
- Multi-seed determinism verification
- Cross-run hash-chain validation

**VERIFICATION GATES:**
- Determinism tests passing across 5+ seeds
- Snapshot replay produces identical hashes
- RNG state serialized correctly
- No external entropy detected
- Hash-chain remains unbroken

---
**Plan ID:** MARKENZ_PHASE_1_DETERMINISM_VERIFIED  
**Authority:** KAIZA-MCP  
**Timestamp:** 2026-01-11
