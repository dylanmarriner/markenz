---
status: APPROVED
---

# MARKENZ PHASE 0–2 REMEDIATION PLAN v1

- **Status:** PLANNING ONLY
- **Authority:** KAIZA-MCP
- **Scope:** Phase 0–2 Remediation
- **Execution Mode:** Windsurf-safe (planned, not executed)

---

## 2. IMMUTABLE CONTEXT

> [!IMPORTANT]  
> **BINDING AUTHORITY DECLARATION:**  
> This plan DOES NOT replace or supersede `MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md` or any Phase 0–2 plans.  
> It serves solely as the corrective patch strategy to resolve the blocking violations identified in `AMP_NO_PLACEHOLDER_AUDIT_PHASE_0_1_2.md`.  
>
> **PRESERVATION MANDATE:**  
> All remediation actions MUST preserve the Gemini Universe world state (House, Shed, Tools) and Agent Identities (Gem-D, Gem-K) as defined in `ADDENDUM_WORLD_PRESERVATION_v1` and `ADDENDUM_IDENTITY_CONTINUITY_v1`.

---

## 3. VIOLATION MAPPING TABLE

| ID | File / Module | Root Cause | Determinism Impact | Planned Fix Strategy (DESIGN ONLY) | Phase | Acceptance Evidence |
|---|---|---|---|---|---|---|
| **V-01** | `apps/engine/src/main.rs` | Hardcoded mock `InputEvents` array | **HIGH**: Engine ignores DB; execution is fictional | **Delete mock array.** Connect to Postgres `input_events` table. Fetch strictly ordered events for Tick T. | 0 | Engine logs show events read from DB matching Tick T. |
| **V-02** | `apps/engine/src/tick_loop.rs` | Identical mock event injection | **HIGH**: Duplicates V-01 | **Use shared DB reader.** Ensure tick loop consumes the DB-sourced iterator. | 0 | `process_tick` receives `Vec<InputEvent>` from DB source. |
| **V-03** | `apps/engine/src/authority_pipeline.rs` | Placeholder `capture_state` returning strings | **LOW**: Observable state is meaningless | **Implement Reflection/Diff.** Serialize Agent/World state to JSON (canonical). Compute JSON Patch (RFC 6902) or full blob if small. | 0 | ObservationEvents contain `{"diff": {"agent_1": ...}}` not "before_change". |
| **V-04** | `apps/engine/src/authority_pipeline.rs` | Skipped Pipeline Passes (Perception/Intent) | **MED**: Logic gaps | **Implement Functional Pass-Through.** Create struct-impls for Perception/Intent that return valid (empty/identity) results rather than skipping. | 0 | Pipeline executes 10/10 steps. Logs show entry/exit of all passes. |
| **V-05** | `apps/server/src/main.rs` | Mock User Endpoint (No Auth) | **HIGH**: Security bypass | **Implement `jsonwebtoken` verify.** Check header vs local JWKS (Phase 0 Offline). Reject invalid. | 0 | `curl` without token returns 401. Valid token returns 200. |
| **V-06** | `apps/server/src/auth/oidc.rs` | Stub Token Validation | **HIGH**: Security bypass | **Full validation.** Verify signature, exp, iss. Extract claims to `UserParams`. | 0 | Unit test with expired token fails. |
| **V-07** | `crates/world/src/deterministic_rng.rs` | Re-init RNG every call | **CRITICAL**: Determinism broken | **Persist RNG State.** Move `DeterministicRng` into `Universe` struct. Pass `&mut rng` to subsystems. | 1 | `rng.next_u64()` advances internal state. Seed=X always yields sequence Y. |
| **V-08** | `crates/world/src/agent_location.rs` | `state_hash` not updated on move | **CRITICAL**: Hash chain broken | **Recalculate Hash.** On `set_position`, `self.hash = blake3(self)`. | 1 | `world_hash` changes when Agent moves. |
| **V-09** | `crates/world/src/action.rs` | `can_gather_at` returns `true` | **MED**: Mechanics broken | **Implement Biome Check.** `world.terrain.get_biome(pos)`. Check resource allow-list per biome. | 2 | Gathering Wood in Desert fails. |
| **V-10** | `crates/world/src/action.rs` | `is_at_mountain` returns `false` | **MED**: Mechanics broken | **Implement Terrain Check.** `world.terrain.get_biome(pos) == Mountain`. | 2 | Mining in Grassland fails. Mining in Mountain succeeds. |

---

## 4. FIX STRATEGY (BY PHASE)

### 4.1 PHASE 0 REMEDIATION PLAN (BOOTSTRAP & PIPELINE)

**Objective:** Establish a truthful, database-driven execution loop and secured control plane.

#### V-01 & V-02: Real Input Event Sourcing

* **Architecture:** `Engine` struct gains a `pg_client: Client`.
- **Logic:**
    1. At start of `tick(T)`: `SELECT * FROM input_events WHERE tick = T ORDER BY seq ASC`.
    2. Deserialize rows to `Vec<InputEvent>`.
    3. Pass `Vec` to `process_tick`.
- **Invariant:** No `InputEvent::new` calls in production code (only in tests).

#### V-03: Real State Diffing

* **Architecture:** Use `serde_json` for canonical serialization.
- **Logic:**
    1. `let before = serde_json::to_value(&world.agents.get(id))`.
    2. Apply mutations.
    3. `let after = serde_json::to_value(&world.agents.get(id))`.
    4. `let diff = json_patch::diff(&before, &after)`.
    5. Emit `ObservationEvent` with `payload: diff`.
- **Invariant:** Every mutation produces a computable, serializable diff.

#### V-04: Complete Authority Pipeline

* **Architecture:** Reify missing passes as traits/structs.
- **Logic:**
  - `PerceptionPass::run(&world) -> PerceptionMap`: Returns empty map if no sensors yet (valid, not stub).
  - `IntentPass::run(&perceptions) -> IntentQueue`: Returns empty queue if no drives yet (valid, not stub).
  - `VolitionPass::run(&intents) -> ActionPlan`: Returns wait/idle action if no intents (valid, not stub).
- **Invariant:** Pipeline structure is 10/10 complete. Logic inside can be minimal (e.g., identity function) but must be **real code**, not `// TODO`.

#### V-05 & V-06: Verified Offline OIDC

* **Architecture:** `jsonwebtoken` crate (or Node equivalent in Server).
- **Logic:**
    1. Load `certs/jwks.json` (local file, Phase 0 constraint) on startup.
    2. On request: Parse `Authorization: Bearer <token>`.
    3. Verify signature against JWKS. Validate `exp` > `now`.
    4. Reject if invalid.
- **Invariant:** No user identity exists without cryptographic proof.

---

### 4.2 PHASE 1 REMEDIATION PLAN (DETERMINISM)

**Objective:** Ensure bit-perfect determinism and hash integrity.

#### V-07: Persistent RNG State

* **Architecture:** `struct Universe { ..., rng: DeterministicRng, ... }`.
- **Logic:**
  - `DeterministicRng` initialized ONCE at genesis (Tick 0) with Master Seed.
  - Subsystems accept `&mut DeterministicRng`.
  - State (internal counter) advances with every call.
- **Invariant:** Calling `rng_stream` twice returns DIFFERENT values.

#### V-08: Reactive State Hashing

* **Architecture:** `Agent` struct method encapsulation.
- **Logic:**
  - Make fields private (e.g., `position`).
  - Expose `fn move_to(&mut self, new_pos: Vec3)`.
  - Inside `move_to`:
        1. Update position.
        2. `self.recalculate_hash()`.
- **Invariant:** `agent.hash` is **always** consistent with `agent.position` (verified by `debug_assert!` in tests).

---

### 4.3 PHASE 2 REMEDIATION PLAN (WORLD FOUNDATION)

**Objective:** Enable functional mechanics backed by spatial reality.

#### V-09 & V-10: Biome-Aware Action Validation

* **Architecture:** `Terrain` struct with `get_biome(x, y)` method.
- **Logic:**
  - `can_gather_at(agent, resource)`:
        1. Get `biome` at `agent.position`.
        2. Check `biome.resources.contains(resource)`.
        3. Return result.
  - `is_at_mountain(agent)`:
        1. Get `biome` at `agent.position`.
        2. Return `biome == Biome::Mountain`.
- **Invariant:** Physical constraints are enforced by data lookup, not hardcoded bools.

---

## 5. NO-PLACEHOLDER ENFORCEMENT STRATEGY

### 5.1 Static Analysis (Pre-Commit / Pre-Build)

Windsurf must run a `grep` check before declaring completion:

```bash
# REJECTION PATTERNS
grep -rE "TODO|FIXME|stub|mock|placeholder|fake|// For now" src/
if [ $? -eq 0 ]; then
    echo "FAIL: Placeholders detected."
    exit 1
fi
```

### 5.2 Hard Failure Conditions

Any of the following constitutes an immediate failure of the remediation:
- Use of `unimplemented!()` or `todo!()` macros.
- Commenting out tests to make them pass.
- Hardcoding return values (e.g., `return true;`) without logic.
- Using `unsafe` blocks to bypass borrow checker implies incomplete architecture.

---

## 6. WINDSURF EXECUTION PRECONDITIONS

**Windsurf is AUTHORIZED to begin remediation ONLY when:**

1. **Environment:**
    - `docker compose ps` shows Postgres and Keycloak healthy.
    - `cargo check` passes (even with warnings, as long as it compiles).
2. **Inputs:**
    - `MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md` is available.
    - This Remediation Plan is available.
3. **Guards:**
    - Windsurf MUST use `write_file` to modify code.
    - Windsurf MUST NOT delete `crates/world`, `crates/physics` (refactor/fix in place).
4. **Stop Condition:**
    - If fixing V-01 causes build errors in V-02 (cascade break), **STOP** and request architectural guidance. Do not "patch" with more mocks.

---

## 7. ACCEPTANCE GATES

### Phase 0 Gate

* [ ] **Clean Logs:** Engine log shows "Connected to DB", "Read N events".
- [ ] **Diffs Valid:** `ObservationEvent` payload is valid JSON Patch.
- [ ] **Security:** Auth endpoint rejects empty/bad tokens.

### Phase 1 Gate

* [ ] **RNG Drift:** Two runs with same seed print exact same sequence of RNG values in logs.
- [ ] **Hash Sensitivity:** Moving Agent A by 1 unit changes `world_hash`.

### Phase 2 Gate

* [ ] **Biome Logic:** Agent cannot mine in default spawn (Grassland). Agent teleported to Mountain CAN mine.

---

## 8. NON-GOALS

- **Refactoring unrelated code:** Do not clean up code that isn't a violation.
- **Feature Expansion:** Do not add "Health" or "Damage" in this pass (Phase 3).
- **UI Polish:** Do not touch `apps/web` CSS/Components unless they break due to API changes.

---

**APPROVED FOR PLANNING ONLY.**  
**Signed:** AMP (Antigravity) Audit Authority  
**Date:** 2026-01-11
