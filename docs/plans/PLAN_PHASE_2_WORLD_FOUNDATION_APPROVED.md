---
status: APPROVED
---

# PLAN_PHASE_2_WORLD_FOUNDATION
## World Representation v1 (Terrain + Entities)

**Phase Number:** 2  
**Authority:** KAIZA-MCP  
**Executor:** Windsurf  
**Parent Plan:** MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md  
**Status:** EXECUTABLE · FILE-LEVEL DECOMPOSITION

---

## Objective

Replace abstract world with deterministic spatial reality. Implement chunked terrain, deterministic biome generation, and real-world mechanics (move, gather, build, mine) with deterministic physics validation. Windsurf must create/modify exact files specified below with no architectural choices.

---

## Terrain Representation (LOCKED CHOICE: HEIGHTMAP + CHUNKS)

This plan resolves the blocked "Heightmap vs Voxel" decision:

**DECIDED: Chunked Heightmap (256×256 cells per chunk, height values 0–255 u8)**

- Each chunk: 256×256 heightmap (65536 u8 values)
- Chunks are 256-unit squares in world space
- Height range: 0–255 meters
- Biomes are assigned per chunk (not per cell)
- Deterministically seeded: terrain generation uses Environment RNG stream

---

## File-Level Write Map

### crates/world/src/terrain.rs (new)
- struct Chunk { x: i32, y: i32, biome: Biome, heights: [u8; 65536], entities: BTreeSet<u64>, assets: BTreeSet<u64> }
- struct Terrain { chunks: BTreeMap<(i32, i32), Chunk>, biome_map: BTreeMap<(i32, i32), Biome> }
- enum Biome { Grassland, Forest, Mountain, Desert, Water }
- Chunks are 256×256, deterministic generation per (chunk_x, chunk_y, seed)
- Forbidden: Floating-point heights, non-deterministic generation, unordered collections

### crates/world/src/biome_generator.rs (new)
- fn generate_biome(chunk_x: i32, chunk_y: i32, rng: &mut RngStream) -> Biome
- fn generate_heightmap(chunk_x: i32, chunk_y: i32, biome: Biome, rng: &mut RngStream) -> [u8; 65536]
- Biome clustering via RNG, heightmap varies by biome
- Grassland: 10–50, Forest: 20–60, Mountain: 50–200, Desert: 5–40, Water: 0–10
- Forbidden: Floating-point noise libraries

### crates/physics/src/collision.rs (new)
- struct Position { chunk_x: i32, chunk_y: i32, local_x: u8, local_y: u8, height: u8 }
- fn can_occupy, fn move_to, fn collides_with_terrain
- Height-based collision (max_climb e.g. 2 units), agent-agent prevention
- Forbidden: Floating-point distance calculations

### crates/world/src/agent_location.rs (modify)
- impl Agent { pub fn position(), pub fn move_to() }
- Movement range limited, collision validated
- Forbidden: Floating-point positions

### crates/world/src/inventory.rs (new)
- struct Inventory { max_slots: u16, items: BTreeMap<u64, Item> }
- struct Item { id: u64, item_type: ItemType, quantity: u32, durability: u16 }
- Item durability tracked, tools degrade, BTreeMap ensures deterministic iteration
- Forbidden: Unordered collections

### crates/world/src/asset.rs (modify)
- impl Asset { pub fn position(), pub fn inventory(), pub fn inventory_mut() }
- Assets have fixed world position with inventory
- House/Shed immovable, vehicles movable
- Forbidden: Implicit location, untracked changes

### crates/world/src/action.rs (new)
- enum Action { Move(Direction), Gather(ResourceType), Build(BuildingType), Mine(MineType), Craft(CraftRecipe), Transfer(...) }
- Deterministic validation, all outputs deterministic
- Forbidden: Non-deterministic outcomes, hidden RNG

### apps/engine/src/world_loop.rs (modify)
- fn process_world_actions(agents: &mut [Agent], terrain: &Terrain, world: &mut Universe) -> Vec<ObservationEvent>
- For each agent, validate and apply actions, track outcomes in ObservationEvents
- Forbidden: Unordered processing, skipped validation

### crates/world/src/gathering.rs (new)
- fn gather, fn get_gatherable_resources
- Grassland (berries, wood), Forest (wood, berries, mushrooms), Mountain (stone, coal, ore), Desert (sand, rocks), Water (water, algae, fish)
- Tool requirements, deterministic output
- Forbidden: Random outputs, non-biome gathering

### crates/world/src/building.rs (new)
- fn build, fn can_build_at
- Wall, door, floor structures requiring materials and tool
- Building progress tracked across multiple ticks
- Forbidden: Floating-point height calculations, non-deterministic build times

### crates/world/src/mining.rs (new)
- fn mine, fn can_mine_at
- Coal, metal, gems at mountain locations only
- Pickaxe required, quantity depends on tool quality and location
- Forbidden: Random ore quantities, mining outside mountains

### crates/world/src/crafting.rs (new)
- fn craft, fn get_available_recipes
- Tools/blocks from inventory resources
- Deterministic recipes, no RNG in crafting
- Crafting queue with progress tracking
- Forbidden: Randomized recipes, non-linear crafting times

### apps/engine/src/action_validator.rs (modify)
- fn validate_action, enum ValidationResult
- Deterministic validation for all action types
- Forbidden: Probabilistic validation, side effects in validation

### apps/web/src/components/TerrainViewer.tsx (new)
- WebGPU terrain visualization
- Display chunks, heightmap coloring, biome colors, zoom/pan, cell inspection
- Forbidden: Client-side terrain generation, server-dependent rendering

### apps/web/src/components/InventoryPanel.tsx (modify)
- Show agent and container inventories
- Drag-drop transfers (admin only), item details on hover
- Forbidden: Client-side inventory modifications

### apps/web/src/components/ActionPanel.tsx (new)
- Show available actions, action queue
- Admin can submit new action via InputEvent, show action result
- Forbidden: Client-side action validation that differs from server

### tools/test/terrain_generation_determinism_test.sh (new)
- Generate chunks (0,0)–(10,10) with seed S twice
- Compare: all heightmaps must be bit-identical
- Forbidden: Approximations in terrain comparison

### tools/test/action_validation_test.sh (new)
- Create scenario with agent, inventory, terrain state
- Validate and replay action sequence
- Compare: all validation results and state changes must match
- Forbidden: Randomized validation

### tools/test/gathering_mining_crafting_test.sh (new)
- Test gather (grassland → berries), mine (mountain → ore), craft (resources → tool)
- Replay with same inputs, compare outputs
- Forbidden: Randomized yields

---

## Forbidden Actions

Windsurf **MUST NOT**:

1. Use voxel grid instead of heightmap
2. Use floating-point for terrain heights
3. Generate terrain with non-deterministic libraries
4. Implement action validation with RNG probability
5. Create unordered collections for terrain/inventory
6. Skip action validation
7. Allow agents to move through terrain/other agents
8. Implement gathering/mining/crafting with random outputs
9. Use chunk sizes other than 256×256
10. Proceed to Phase 3 without Phase 2 gates passing

---

## Entry Conditions

- Phase 1 complete and all gates passing
- crates/world, crates/physics from Phase 1 present
- apps/engine capable of ticking with RNG streams
- agents (Gem-D, Gem-K) present in genesis snapshot

---

## Exit Criteria (ALL REQUIRED)

- `cargo build --release` succeeds with zero warnings
- All Phase 2 tests compile and pass
- Terrain generation deterministic
- Chunks generated correctly for seed
- Biome assignment deterministic
- Agent movement validates correctly
- Gathering produces correct resources for biome
- Mining produces ore in mountains
- Building constructs structures deterministically
- Crafting produces exact outputs from recipes
- Invalid actions rejected with logged reason
- Action outcomes deterministic
- Phase 1 determinism tests still pass (no regression)
- Action validation determinism test passes
- Gathering/mining/crafting determinism test passes
- World hash still deterministic
- Engine processes actions from agents
- ObservationEvents log all action outcomes
- WebSocket fanout includes action results
- Web UI shows terrain viewer
- Web UI shows inventory and action panel
- AMP Principal-Level Auditor approval obtained in writing before Phase 3 begins

---

## Determinism & Replay Gates

**Gate 1: Terrain Generation Determinism**
- Condition: Generate chunks with seed S twice
- Expected: Identical heightmaps and biomes
- Failure action: Stop Phase 2; escalate with divergent chunk

**Gate 2: Action Outcome Determinism**
- Condition: Replay action sequence (move, gather, build, mine, craft)
- Expected: Identical action results and state changes
- Failure action: Stop Phase 2; escalate with divergent action

**Gate 3: Phase 1 Regression Check**
- Condition: Verify Phase 1 determinism tests still pass
- Expected: world_hash and snapshot equivalence unaffected
- Failure action: Stop Phase 2; escalate with regression report

---

## Hard Stop Conditions

Execution **STOPS IMMEDIATELY** if:

1. Terrain generation is non-deterministic
2. Action validation produces different results on replay
3. Phase 1 determinism tests fail (regression)
4. Build fails
5. Any action outputs randomized data
6. Unordered collections introduced in critical paths
7. Any gate test fails
8. AMP auditor directs halt

Upon hard stop: Do not proceed to Phase 3; escalate with full diagnostic.

---

**END OF PHASE 2 PLAN**
