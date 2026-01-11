---
status: APPROVED
---

# PLAN_PHASE_8_NORMALIZED
## WebGPU Renderer + Transparency UI

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 8 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.9)

---

## 1. ENTRY CONDITION
Phase 7 complete with all gates passing.

---

## 2. SCOPE (LOCKED)

Professional visualization without authority leakage.

**Deliverables:**
- WebGPU renderer (3D terrain, agents, assets visualization)
- Render packet generation from snapshots (deterministic, hash-stable)
- Multi-monitor layouts
- Diff heatmaps (state changes per tick)
- Causality graph (event dependency visualization)
- Time-travel debugger (replay navigation)
- Timeline scrubber (tick seeking)

---

## 3. NON-SCOPE

- Security hardening (Phase 9)

---

## 4. PRESERVATION

All prior phase guarantees preserved. Renderer is observer-only.

---

## 5. DETERMINISM

### 5.1 Render Packets
- Deterministically generated from snapshots
- Same snapshot → same packet
- Hash-stable (packet hash verifiable)

### 5.2 Floating-Point Allowed
- Renderer-only floating-point is permitted
- Derivation from integer world state is deterministic
- Derivation repeatable (same state → same floats)

### 5.3 No Mutations
- Renderer never mutates state
- Renderer never feeds back to engine
- Read-only visualization

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Render Packets
Causal: Snapshot at tick T  
State: RenderPacket (derived, non-authoritative)  
Proof: Same snapshot → same packet hash

### 6.2 WebGPU Rendering
Causal: RenderPacket  
State: Visual output (GPU framebuffer)  
Proof: No feedback to engine; visualization deterministic

### 6.3 Time-Travel
Causal: User seeks to tick T  
State: Snapshot loaded, events replayed (temporary)  
Proof: Replay deterministic (verified by hash comparison)

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_8_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_8_EXECUTION_REPORT.md

Must include: Render packet sample, WebGPU performance metrics, time-travel test results.

---

## 8. EXIT CRITERIA

### WebGPU Renderer
- [ ] Renderer boots without errors
- [ ] Terrain renders correctly
- [ ] Agents render as distinct objects
- [ ] Assets (house, shed, vehicles) render
- [ ] Camera controls functional
- [ ] Performance acceptable (60+ fps on modern GPU)

### Render Packets
- [ ] Packets generated from snapshots
- [ ] Hash-stability verified (same snapshot → same hash)
- [ ] Floating-point derivation deterministic

### UI Features
- [ ] Timeline scrubber functional
- [ ] Diff heatmap displays state changes
- [ ] Causality graph traces dependencies
- [ ] Time-travel debugger allows replay navigation
- [ ] Multi-monitor layouts supported

### Determinism
- [ ] Renderer does not mutate state
- [ ] No feedback loop to engine
- [ ] Replay + visualization deterministic
- [ ] Hash comparison matches checkpoint

### AMP Sign-Off
- [ ] AMP approval BEFORE Phase 9

---

## 9. GATES

**Gate 1: Render Determinism (TEST-RENDER-001)**  
**Gate 2: Time-Travel Correctness (TEST-TTIME-001)**

STOP if any fail.

---

**END OF PHASE 8 NORMALIZED PLAN**
