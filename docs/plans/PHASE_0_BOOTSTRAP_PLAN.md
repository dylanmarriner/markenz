---
status: APPROVED
authority: KAIZA-MCP · AMP Authority
plan_id: MARKENZ_PHASE_0_BOOTSTRAP_VERIFIED
phase: 0
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Offline Stack · Deterministic Tick Loop · Hash-Chain Authority
---

# PHASE 0: BOOTSTRAP
## Offline Stack · Deterministic Tick Loop · Hash-Chain Authority

**STATUS:** Verified for Kaiza MCP write tool authorization

**OBJECTIVES:**
- Establish fully offline deterministically bootable Markenz stack
- Implement fixed-timestep tick loop (u64 tick index, never wall-clock)
- Exclusive world state mutations via Rust engine
- Immutable append-only event logging with cryptographic hash chain
- RBAC enforcement on all InputEvent submission
- Two agents (Gem-D, Gem-K) boot from genesis with preserved identity
- Per-tick world_hash checkpoints for determinism verification

**CORE DECISIONS:**
- Authority exclusively in apps/engine (Rust)
- Fixed timestep: no variable frame rates
- Time model: u64 tick index (authoritative), zero wall-clock dependency
- Event pipeline: Input → Validate → Log → Engine → Mutation → Hash → Observation

**DELIVERABLES:**
- All crates compile (release + test)
- Determinism tests passing (100 ticks, 3 runs, identical hashes)
- Authority isolation confirmed (server cannot mutate)
- Zero TODO/FIXME/mock/stub in critical paths
- Database schema immutable (append-only)
- Keycloak OIDC integration complete
- WebSocket event streaming working
- Genesis snapshot with Gem-D, Gem-K, House, Shed, Tools

**VERIFICATION GATES:**
- Build succeeds without warnings
- Determinism tests pass
- Authority code review passes
- Asset preservation verified
- RBAC tests pass
- Hash-chain integrity verified

---
**Plan ID:** MARKENZ_PHASE_0_BOOTSTRAP_VERIFIED  
**Authority:** KAIZA-MCP  
**Timestamp:** 2026-01-11
