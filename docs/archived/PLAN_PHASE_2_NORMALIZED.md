---
status: APPROVED
---

# PLAN_PHASE_2_NORMALIZED
## World Representation v1 (Terrain + Entities)

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 2 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.3)

---

## 1. AUTHORITY & ENTRY

Phase 1 MUST be 100% complete and all gates passing.

---

## 2. PHASE SCOPE (LOCKED)

Replace abstract world with deterministic spatial reality.

**Deliverables:**
- Chunked heightmap terrain (256x256 cells per chunk, u8 heights 0–255)
- Deterministic biome generation (Grassland, Forest, Mountain, Desert, Water)
- Agent positioning (chunk + local cell coordinates)
- Inventory system (items with durability, stacking)
- World actions: Move, Gather, Build, Mine, Craft, Transfer
- Action validation (deterministic collision, reach, resource checks)
- Deterministic mechanics (output depends only on game state, not RNG where specified)

---

## 3. EXPLICIT NON-SCOPE

- Full biology (Phase 3)
- Cognition/planning (Phase 4)
- Genetics/reproduction (Phase 5)
- Social systems (Phase 6)
- Governance (Phase 7)
- Rendering (Phase 8)
- Security (Phase 9)

---

## 4. PRESERVATION CLAUSES

Gem-D, Gem-K, House, Shed, Tools, Vehicles preserved as in Phase 1.

---

## 5. DETERMINISM REQUIREMENTS

### 5.1 Terrain Representation
- Chunks: 256x256 heightmap (65536 u8 cells per chunk)
- Generation: deterministic RNG-based (no mathematical perlin)
- Biome assignment: chunk-level, deterministic
- Same (chunk_x, chunk_y, seed) → identical terrain

### 5.2 Action Validation
- Deterministic (no probability unless explicitly noted)
- Same game state → same validation result
- All constraints: reach, tools, inventory, energy, biome

### 5.3 Mechanics
- Integer math only in authority
- Collision detection deterministic
- Gathering/mining/building outputs deterministic (same inputs → same output)
- Crafting recipes deterministic (fixed inputs → fixed outputs)

### 5.4 Ordering
- Entity iteration ordered (BTreeMap)
- Action processing in canonical tick order
- No nondeterministic container iteration

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Terrain Generation
Causal: Chunk coordinates + RNG seed  
State: Terrain chunks table  
Proof: Same inputs → identical heightmap/biome

### 6.2 Action Validation
Causal: Agent state + action + world state  
State: Action rejection log  
Proof: Same state → same validation result

### 6.3 Inventory Management
Causal: Item operations (add/remove/transfer)  
State: Inventory tables  
Proof: Operations deterministic, durability decreases predictably

### 6.4 Gathering/Mining/Building
Causal: Agent, location, terrain, tools, inventory  
State: Agent inventory, terrain (possibly modified)  
Proof: Same inputs → same output quantities

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_2_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_2_EXECUTION_REPORT.md

Must include: Terrain generation samples, action validation test results, mechanics examples (gather, mine, craft outputs)

---

## 8. EXIT CRITERIA

### Terrain & World
- [ ] Terrain generation deterministic
- [ ] Biome assignment deterministic
- [ ] Chunk coordinates correct
- [ ] Heightmap per-chunk deterministic

### Actions & Mechanics
- [ ] Movement validates correctly (collision detection)
- [ ] Gathering produces correct resources per biome
- [ ] Mining produces ore in mountains only
- [ ] Building constructs deterministically
- [ ] Crafting produces exact outputs
- [ ] Invalid actions rejected with reason

### Determinism
- [ ] Phase 1 tests still pass (no regression)
- [ ] Action validation determinism test passes
- [ ] Mechanics determinism test passes
- [ ] World hash still deterministic

### Integration
- [ ] Engine processes actions from agents
- [ ] ObservationEvents log outcomes
- [ ] WebSocket fanout includes results
- [ ] UI shows terrain viewer and inventory

### AMP Sign-Off
- [ ] AMP approval BEFORE Phase 3

---

## 9. GATES & HARD STOPS

**Gate 1:** Terrain generation deterministic (TEST-TERRAIN-001)  
**Gate 2:** Action outcome deterministic (TEST-ACTION-001)  
**Gate 3:** Phase 1 no regression (TEST-P1-REGRESS-001)

STOP if: Terrain non-deterministic, actions diverge on replay, Phase 1 tests fail, any gate fails.

---

**END OF PHASE 2 NORMALIZED PLAN**
