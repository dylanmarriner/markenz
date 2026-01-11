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
- **Purpose:** Terrain representation and chunk management
- **Required exports:**
  - struct Chunk { x: i32, y: i32, biome: Biome, heights: [u8; 65536], entities: BTreeSet<u64>, assets: BTreeSet<u64> }
  - struct Terrain { chunks: BTreeMap<(i32, i32), Chunk>, biome_map: BTreeMap<(i32, i32), Biome> }
  - enum Biome { Grassland, Forest, Mountain, Desert, Water }
  - impl Chunk { pub fn height_at(&self, local_x: u8, local_y: u8) -> u8 { ... } }
  - impl Terrain {
      pub fn get_chunk(&self, chunk_x: i32, chunk_y: i32) -> Option<&Chunk>
      pub fn height_at_world(&self, world_x: i32, world_y: i32) -> u8
      pub fn generate_chunk(chunk_x: i32, chunk_y: i32, rng: &mut RngStream) -> Chunk
    }
- **Required behaviors:**
  - Chunks are 256×256 cells, world coordinates in multiples of 256
  - Heightmap uses u8 (0–255), no floating-point in authority state
  - Biome determined by chunk-level seed + position
  - Entities and assets tracked per chunk (for spatial queries)
  - Chunk generation is deterministic: chunk_x, chunk_y + RNG seed → identical terrain
- **Determinism constraints:**
  - Heightmap generation uses Environment RNG stream (stream_id=1)
  - Same (chunk_x, chunk_y, seed) → identical terrain
  - Biome assignment is deterministic (perlin-like via RNG)
- **Forbidden:** Floating-point heights; non-deterministic generation; unordered collections

### crates/world/src/biome_generator.rs (new)
- **Purpose:** Deterministic biome and heightmap generation
- **Required exports:**
  - fn generate_biome(chunk_x: i32, chunk_y: i32, rng: &mut RngStream) -> Biome
  - fn generate_heightmap(chunk_x: i32, chunk_y: i32, biome: Biome, rng: &mut RngStream) -> [u8; 65536]
- **Required behaviors:**
  - Biome selection: use RNG to assign biome type to chunk
    - Perlin-like noise simulation via RNG (iterative refinement)
    - Biome clustering: nearby chunks tend to same biome
  - Heightmap generation depends on biome:
    - Grassland: smooth rolling hills (height 10–50)
    - Forest: moderate variation (height 20–60)
    - Mountain: steep peaks and valleys (height 50–200)
    - Desert: flat with dunes (height 5–40)
    - Water: low elevation (height 0–10, often underwater designation)
- **Determinism constraints:**
  - Noise simulation must be deterministic RNG-based (no mathematical perlin)
  - Same inputs → identical heightmap
- **Forbidden:** Floating-point noise libraries; library-based terrain generation

### crates/physics/src/collision.rs (new)
- **Purpose:** Deterministic collision detection and resolution
- **Required exports:**
  - struct Position { chunk_x: i32, chunk_y: i32, local_x: u8, local_y: u8, height: u8 }
  - fn can_occupy(position: &Position, terrain: &Terrain, agents: &[Agent]) -> bool
  - fn move_to(agent: &mut Agent, new_position: Position, terrain: &Terrain, agents: &[Agent]) -> Result<()>
  - fn collides_with_terrain(position: &Position, terrain: &Terrain) -> bool
- **Required behaviors:**
  - Agent occupies single cell (256×256 grid per chunk)
  - Height-based collision: agent can occupy cell if height difference ≤ max_climb (e.g., 2 units)
  - Agent-agent collision: agents cannot occupy same cell
  - Movement validation: deterministically rejects illegal moves
- **Determinism constraints:**
  - Collision checks are deterministic (no floating-point; integer math only)
  - Same position sequence → identical collision outcomes
- **Forbidden:** Floating-point distance calculations; approximations in collision

### crates/world/src/agent_location.rs (modify)
- **Purpose:** Track agent positions in world
- **Required exports:**
  - impl Agent {
      pub fn position(&self) -> Position { ... }
      pub fn move_to(&mut self, new_position: Position) -> Result<()> { ... }
    }
- **Required behaviors:**
  - Agents have Position struct (chunk coordinates + local cell + height)
  - Move validation via collision check
  - Agents cannot occupy same cell as other agents
  - Movement range per action is limited (e.g., 1 cell adjacent, or 3 cells with vehicle)
- **Determinism constraints:** Movement outcomes deterministic
- **Forbidden:** Floating-point positions

### crates/world/src/inventory.rs (new)
- **Purpose:** Track items owned by agents or stored in containers
- **Required exports:**
  - struct Inventory { max_slots: u16, items: BTreeMap<u64, Item> }
  - struct Item { id: u64, item_type: ItemType, quantity: u32, durability: u16 }
  - enum ItemType { Tool, Resource, Food, Vehicle, BuildingBlock }
  - impl Inventory {
      pub fn add_item(&mut self, item: Item) -> Result<()> { ... }
      pub fn remove_item(&mut self, item_id: u64) -> Option<Item> { ... }
      pub fn find_items(&self, item_type: ItemType) -> Vec<&Item> { ... }
    }
- **Required behaviors:**
  - Max inventory slots per agent (e.g., 20 slots, stackable items count differently)
  - Items have unique IDs and tracked durability
  - Tools degrade with use (durability decreases)
  - Resources can stack (wood, stone, etc.)
  - BTreeMap ensures deterministic iteration
- **Determinism constraints:** Item ordering is deterministic
- **Forbidden:** Unordered collections; non-deterministic item generation

### crates/world/src/asset.rs (modify)
- **Purpose:** Extend Asset definition with world position and container support
- **Required exports:**
  - impl Asset {
      pub fn position(&self) -> Position { ... }
      pub fn inventory(&self) -> &Inventory { ... }
      pub fn inventory_mut(&mut self) -> &mut Inventory { ... }
    }
- **Required behaviors:**
  - Assets (House, Shed, Tools, Vehicles) now have fixed world position
  - Assets can contain items (inventory)
  - House and Shed are immovable (fixed position)
  - Vehicles can move (position updated by movement action)
- **Determinism constraints:** Position and inventory changes are logged as ObservationEvents
- **Forbidden:** Implicit location; untracked position changes

### crates/world/src/action.rs (new)
- **Purpose:** Define deterministic world actions
- **Required exports:**
  - enum Action {
      Move(Direction),
      Gather(ResourceType),
      Build(BuildingType),
      Mine(MineType),
      Craft(CraftRecipe),
      Transfer(ItemType, u32, TargetId),
    }
  - enum Direction { North, South, East, West, Up, Down }
  - enum ResourceType { Wood, Stone, Water, Food, Berry }
  - enum BuildingType { Wall, Door, Floor }
  - enum MineType { Coal, Metal, Gem }
  - struct CraftRecipe { inputs: BTreeMap<ItemType, u32>, output: ItemType, ticks_required: u32 }
  - impl Action { pub fn validate(&self, agent: &Agent, world: &Universe) -> bool { ... } }
- **Required behaviors:**
  - Each action has deterministic validation rules
  - Validation checks: agent has required tools, sufficient energy, proper location
  - Actions produce side effects (item acquisition, state change, energy cost)
  - All recipe outputs are deterministic (same inputs → same output quantity)
- **Determinism constraints:** Action outcomes depend only on game state, not RNG (except where explicitly probabilistic)
- **Forbidden:** Non-deterministic action outcomes; hidden RNG in mechanics

### apps/engine/src/world_loop.rs (modify)
- **Purpose:** Extend tick loop to process world actions
- **Required exports:**
  - fn process_world_actions(agents: &mut [Agent], terrain: &Terrain, world: &mut Universe) -> Vec<ObservationEvent>
- **Required behaviors:**
  - For each agent, check action queue (built from cognition phase)
  - Validate each action (movement, gathering, building, mining, crafting)
  - Apply valid actions to world state
  - Track action outcomes in ObservationEvents
  - Invalid actions produce rejection ObservationEvent (reason logged)
- **Determinism constraints:** Action ordering is deterministic (tick, agent_id, action_index)
- **Forbidden:** Unordered action processing; skipping validation

### crates/world/src/gathering.rs (new)
- **Purpose:** Gather action mechanics
- **Required exports:**
  - fn gather(agent: &Agent, resource_type: ResourceType, terrain: &Terrain) -> Result<Item>
  - fn get_gatherable_resources(position: &Position, terrain: &Terrain) -> Vec<ResourceType>
- **Required behaviors:**
  - Agent can gather resources at current location based on biome and terrain
  - Grassland: berries, wood (if trees)
  - Forest: wood, berries, mushrooms
  - Mountain: stone, coal, metal ore
  - Desert: sand, rocks
  - Water: water, algae, fish (if adjacent)
  - Gathering requires correct tool (berry picking vs wood chopping)
  - Output quantity is deterministic (resource_type + tool_quality → fixed quantity)
- **Determinism constraints:** Same location + tool + biome → identical output
- **Forbidden:** Random output quantities; non-biome-based gathering

### crates/world/src/building.rs (new)
- **Purpose:** Build action mechanics
- **Required exports:**
  - fn build(agent: &Agent, building_type: BuildingType, terrain: &mut Terrain) -> Result<()>
  - fn can_build_at(position: &Position, terrain: &Terrain, building_type: BuildingType) -> bool
- **Required behaviors:**
  - Agent can build structures (wall, door, floor) at adjacent location
  - Building requires materials (stone, wood) and tool (hammer)
  - Building modifies terrain (increases height or adds collision)
  - Building progress is tracked (multiple ticks required for complex structures)
  - Only valid on buildable terrain (not on water, not on steep slopes)
- **Determinism constraints:** Build outcome depends only on inputs, materials, location
- **Forbidden:** Floating-point height calculations; non-deterministic build times

### crates/world/src/mining.rs (new)
- **Purpose:** Mine action mechanics
- **Required exports:**
  - fn mine(agent: &Agent, mine_type: MineType, terrain: &Terrain) -> Result<Item>
  - fn can_mine_at(position: &Position, terrain: &Terrain, mine_type: MineType) -> bool
- **Required behaviors:**
  - Agent can mine (coal, metal, gems) at mountain locations
  - Mining requires pickaxe tool
  - Output quantity depends on tool quality and location
  - Mining depletes resource (terrain height may decrease)
  - Location must be in mountain biome
- **Determinism constraints:** Same tool + location → identical ore yield
- **Forbidden:** Random ore quantities; mining outside mountains

### crates/world/src/crafting.rs (new)
- **Purpose:** Craft action mechanics
- **Required exports:**
  - fn craft(agent: &Agent, recipe: &CraftRecipe, inventory: &mut Inventory) -> Result<Item>
  - fn get_available_recipes(agent: &Agent) -> Vec<CraftRecipe>
- **Required behaviors:**
  - Agent can craft items (tools, building blocks, etc.) from inventory resources
  - Crafting consumes input items and produces output
  - Recipe validation: agent has all required inputs in correct quantity
  - Output is deterministic (no RNG in crafting)
  - Crafting takes multiple ticks (recipe.ticks_required)
  - Progress is tracked in agent crafting queue
- **Determinism constraints:** Recipes produce fixed outputs
- **Forbidden:** Randomized recipes; non-linear crafting times

### apps/engine/src/action_validator.rs (modify)
- **Purpose:** Comprehensive action validation pipeline
- **Required exports:**
  - fn validate_action(action: &Action, agent: &Agent, world: &Universe) -> ValidationResult
  - enum ValidationResult { Valid, Invalid(String), Conditional(u64) }
- **Required behaviors:**
  - Movement: path is clear, destination is reachable, not blocked by terrain/agent
  - Gathering: location has resource, agent has tool, agent has inventory space
  - Building: location is valid, agent has materials, agent has tool
  - Mining: location is mountain, agent has pickaxe, agent has inventory space
  - Crafting: agent has recipe, inventory has all inputs
  - All validations are deterministic (no probability; either valid or invalid)
- **Determinism constraints:** Validation result is repeatable
- **Forbidden:** Probabilistic validation; side effects in validation

### apps/web/src/components/TerrainViewer.tsx (new)
- **Purpose:** WebGPU-based terrain visualization
- **Required exports:** default export TerrainViewer component
- **Required behaviors:**
  - Display terrain chunks around agent position
  - Show heightmap as colored mesh (height → color)
  - Show biome coloring (grassland green, forest dark green, mountain gray, desert yellow, water blue)
  - Show agent position and other entities
  - Allow zoom and pan
  - Click to inspect cell (show height, biome, resources, entities)
- **Determinism constraints:** Visualization only; no state mutation
- **Forbidden:** Client-side terrain generation; server-dependent rendering

### apps/web/src/components/InventoryPanel.tsx (modify)
- **Purpose:** Display agent and container inventories
- **Required exports:** default export InventoryPanel component
- **Required behaviors:**
  - Show agent current inventory (items, quantities, durability)
  - Show nearby container inventory (if adjacent to House/Shed/Vehicle)
  - Allow drag-drop to transfer items between inventories (admin only)
  - Show item details on hover (type, durability, value)
- **Determinism constraints:** Visualization only
- **Forbidden:** Client-side inventory modifications

### apps/web/src/components/ActionPanel.tsx (new)
- **Purpose:** Display available actions and action queue
- **Required exports:** default export ActionPanel component
- **Required behaviors:**
  - Show available actions (gather, build, mine, craft)
  - Show action queue (planned actions for next N ticks)
  - Allow admin to submit new action via InputEvent
  - Show action result (success/failure + reason)
- **Determinism constraints:** Input only
- **Forbidden:** Client-side action validation that differs from server

### tools/test/terrain_generation_determinism_test.sh (new)
- **Purpose:** Verify terrain generation is deterministic
- **Required exports:** Exit 0 if pass, 1 if fail
- **Required behaviors:**
  - Generate chunks (0,0)–(10,10) with seed S
  - Generate same chunks with seed S again
  - Compare: all heightmaps must be bit-identical
- **Determinism constraints:** Terrain generation is repeatable
- **Forbidden:** Approximations in terrain comparison

### tools/test/action_validation_test.sh (new)
- **Purpose:** Verify action validation is deterministic
- **Required exports:** Exit 0 if pass, 1 if fail
- **Required behaviors:**
  - Create scenario with agent, inventory, terrain state
  - Validate sequence of actions (move, gather, build, mine)
  - Replay scenario with identical inputs
  - Compare: all validation results and state changes must match
- **Determinism constraints:** Action outcomes are repeatable
- **Forbidden:** Randomized validation

### tools/test/gathering_mining_crafting_test.sh (new)
- **Purpose:** Verify mechanics of gathering, mining, crafting
- **Required exports:** Exit 0 if pass, 1 if fail
- **Required behaviors:**
  - Test gather in grassland → produces berries
  - Test mine in mountain → produces ore
  - Test craft recipe → produces tool from resources
  - Replay with same inputs
  - Compare: outputs must match exactly
- **Determinism constraints:** Mechanics are deterministic
- **Forbidden:** Randomized yields

---

## Forbidden Actions

Windsurf **MUST NOT**:

1. Use voxel grid instead of heightmap
2. Use floating-point for terrain heights (must be u8 0–255)
3. Generate terrain with non-deterministic libraries
4. Implement action validation with RNG-based probability
5. Create unordered collections for terrain/inventory/actions
6. Skip action validation; all actions must be validated
7. Allow agents to move through terrain or other agents
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

## Exit Criteria

**Build & Compilation (ALL REQUIRED):**
- [ ] `cargo build --release` succeeds with zero warnings
- [ ] All Phase 2 tests compile and pass
- [ ] crates/world, crates/physics extended without errors

**Terrain & World (ALL REQUIRED):**
- [ ] Terrain generation deterministic (TEST-TERRAIN-DETERMINISM-001)
- [ ] Chunks generated correctly for seed
- [ ] Biome assignment deterministic
- [ ] Heightmap per-chunk deterministic
- [ ] Chunk coordinates and spatial indexing correct

**Actions & Mechanics (ALL REQUIRED):**
- [ ] Agent movement validates correctly (collision detection works)
- [ ] Gathering produces correct resources for biome
- [ ] Mining produces ore in mountains
- [ ] Building constructs structures deterministically
- [ ] Crafting produces exact outputs from recipes
- [ ] Invalid actions rejected with logged reason
- [ ] Action outcomes are deterministic (same inputs → same output)

**Determinism (ALL REQUIRED):**
- [ ] Phase 1 determinism tests still pass (no regression)
- [ ] Action validation determinism test passes
- [ ] Gathering/mining/crafting determinism test passes
- [ ] World hash still deterministic (terrain doesn't break hashing)

**Integration (ALL REQUIRED):**
- [ ] Engine processes actions from agents
- [ ] Observations events log all action outcomes
- [ ] WebSocket fanout includes action results
- [ ] Web UI shows terrain viewer
- [ ] Web UI shows inventory and action panel

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval obtained in writing before Phase 3 begins

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
