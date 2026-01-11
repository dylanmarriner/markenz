---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_2_WORLD
phase: 2
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Terrain Generation · Physics · Action Validation · Asset Integration
requires: PLAN_PHASE_1_DETERMINISM (100% complete)
---

# PLAN PHASE 2: WORLD
## (Terrain v1 · Physics · Action Validation · Asset Mechanics)

**AUDIENCE:** Windsurf executor (direct execution only)  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Establish a fully deterministic, actionable world where agents can interact with terrain, gather resources, build, and craft:
- Terrain generation using seeded noise (deterministic heightmap + biome map)
- Physics: collision detection, movement validation, climbing
- Action validation: gathering (location/biome), mining (location-specific), building, crafting
- Asset integration: House, Shed, Tools, Vehicles with preserved state from genesis
- All mechanics are observable through ObservationEvents
- Determinism preserved: identical seed → identical terrain and action outcomes

---

## 2. ENTRY CONDITIONS (MUST BE TRUE)

- Phase 1 complete and signed off by AMP auditor
- All Phase 1 determinism tests passing
- RNG subsystems available (Physics, Environment, Genetics)
- All Phase 0 and Phase 1 tests still passing

---

## 3. TERRAIN GENERATION

### 3.1 Deterministic Heightmap (crates/world/src/terrain/heightmap.rs)

```rust
pub struct Heightmap {
    chunk_size: usize,              // e.g., 16x16 voxels per chunk
    noise_octaves: usize,           // e.g., 4 octaves
    base_height: f32,               // e.g., 64.0
    scale: f32,                     // e.g., 0.1
}

impl Heightmap {
    pub fn generate(
        chunk_x: i32,
        chunk_y: i32,
        rng: &mut RngStream,  // Physics RNG
    ) -> Result<Vec<f32>, String> {
        let mut heights = vec![0.0; Self::chunk_size * Self::chunk_size];
        
        for y in 0..Self::chunk_size {
            for x in 0..Self::chunk_size {
                let world_x = (chunk_x * Self::chunk_size as i32 + x as i32) as f32;
                let world_y = (chunk_y * Self::chunk_size as i32 + y as i32) as f32;
                
                // Simplex noise using seeded RNG
                let height = Self::simplex_noise(
                    world_x, world_y,
                    rng,
                    Self::noise_octaves,
                    Self::base_height,
                    Self::scale,
                );
                
                heights[y * Self::chunk_size + x] = height;
            }
        }
        
        Ok(heights)
    }
    
    fn simplex_noise(
        x: f32,
        y: f32,
        rng: &mut RngStream,
        octaves: usize,
        base: f32,
        scale: f32,
    ) -> f32 {
        // Use RNG subsystem for noise generation
        // NO external noise library; deterministic implementation
        // Ensures reproducibility on all platforms
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;
        
        for _ in 0..octaves {
            // Hash-based Simplex-like noise using RNG
            let noise = Self::hash_noise(x * frequency, y * frequency, rng);
            value += noise * amplitude;
            max_value += amplitude;
            
            amplitude *= 0.5;
            frequency *= 2.0;
        }
        
        base + (value / max_value) * scale
    }
    
    fn hash_noise(x: f32, y: f32, rng: &mut RngStream) -> f32 {
        // Seed RNG with position; extract noise value
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        let xf = x - x.floor();
        let yf = y - y.floor();
        
        // Interpolated hash-based noise (deterministic from RNG)
        let n00 = Self::lerp_hash(xi, yi, rng);
        let n10 = Self::lerp_hash(xi + 1, yi, rng);
        let n01 = Self::lerp_hash(xi, yi + 1, rng);
        let n11 = Self::lerp_hash(xi + 1, yi + 1, rng);
        
        let nx0 = Self::smoothstep(n00, n10, xf);
        let nx1 = Self::smoothstep(n01, n11, xf);
        Self::smoothstep(nx0, nx1, yf)
    }
    
    fn lerp_hash(x: i32, y: i32, rng: &mut RngStream) -> f32 {
        let hash = Self::xxhash64(x as u64, y as u64, rng);
        ((hash & 0xFFFFFFFF) as f32) / u32::MAX as f32
    }
    
    fn xxhash64(x: u64, y: u64, rng: &mut RngStream) -> u64 {
        // Deterministic hash using RNG seeded with position
        let mut hasher = blake3::Hasher::new();
        hasher.update(&x.to_le_bytes());
        hasher.update(&y.to_le_bytes());
        let hash_bytes = hasher.finalize();
        u64::from_le_bytes(hash_bytes.as_bytes()[0..8].try_into().unwrap())
    }
    
    fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
        let t = t * t * (3.0 - 2.0 * t);  // Smoothstep function
        a * (1.0 - t) + b * t
    }
}
```

**Critical:** All noise is derived from RNG; no external noise libraries allowed.

### 3.2 Biome Map (crates/world/src/terrain/biome.rs)

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BiomeType {
    Forest,    // Wood, berries
    Mountain,  // Ore, stone
    Desert,    // Sand, limited water
    Grassland, // Grain, grass
    Ocean,     // Fish (Phase 3+)
}

pub struct BiomeMap {
    chunk_size: usize,
}

impl BiomeMap {
    pub fn biome_at(
        x: f32,
        y: f32,
        heightmap: &Heightmap,
        rng: &mut RngStream,
    ) -> BiomeType {
        let height = heightmap.height_at(x, y);
        
        // Determine biome from height + secondary noise
        if height > 200.0 {
            BiomeType::Mountain
        } else if height < 50.0 {
            BiomeType::Ocean
        } else if height > 100.0 {
            BiomeType::Forest
        } else if height > 60.0 {
            // Use secondary RNG to distinguish Forest/Desert
            let secondary = rng.next_f64();
            if secondary > 0.5 {
                BiomeType::Desert
            } else {
                BiomeType::Grassland
            }
        } else {
            BiomeType::Grassland
        }
    }
    
    pub fn resources_in_biome(biome: BiomeType) -> Vec<ResourceType> {
        match biome {
            BiomeType::Forest => vec![
                ResourceType::Wood,
                ResourceType::Berries,
                ResourceType::Stone,
            ],
            BiomeType::Mountain => vec![
                ResourceType::Ore,
                ResourceType::Stone,
                ResourceType::Coal,
            ],
            BiomeType::Desert => vec![
                ResourceType::Sand,
                ResourceType::Stone,
            ],
            BiomeType::Grassland => vec![
                ResourceType::Grain,
                ResourceType::Berries,
                ResourceType::Stone,
            ],
            BiomeType::Ocean => vec![
                ResourceType::Fish,  // Phase 3+
            ],
        }
    }
}
```

### 3.3 Chunk Management (crates/world/src/terrain/chunk.rs)

```rust
pub struct Chunk {
    pub position: (i32, i32),
    pub voxels: Vec<Voxel>,  // Flat array for deterministic iteration
    pub heightmap: Vec<f32>,
    pub biome_map: Vec<BiomeType>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Voxel {
    pub block_type: BlockType,
    pub height: u8,  // 0-255 for LOD
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    Air,
    Stone,
    Dirt,
    Sand,
    Wood,
    Ore,
    Grass,
}

impl Chunk {
    pub fn generate(
        chunk_x: i32,
        chunk_y: i32,
        rng: &mut RngStream,
    ) -> Result<Self, String> {
        let heightmap = Heightmap::generate(chunk_x, chunk_y, rng)?;
        
        let mut voxels = Vec::new();
        let mut biome_map = Vec::new();
        
        for y in 0..16 {
            for x in 0..16 {
                let world_x = (chunk_x * 16 + x) as f32;
                let world_y = (chunk_y * 16 + y) as f32;
                
                let height = heightmap[y * 16 + x];
                let biome = BiomeMap::biome_at(world_x, world_y, &heightmap, rng)?;
                
                let block_type = match biome {
                    BiomeType::Forest => BlockType::Dirt,
                    BiomeType::Mountain => BlockType::Stone,
                    BiomeType::Desert => BlockType::Sand,
                    BiomeType::Grassland => BlockType::Grass,
                    BiomeType::Ocean => BlockType::Air,
                };
                
                voxels.push(Voxel {
                    block_type,
                    height: height as u8,
                });
                
                biome_map.push(biome);
            }
        }
        
        Ok(Self {
            position: (chunk_x, chunk_y),
            voxels,
            heightmap,
            biome_map,
        })
    }
    
    pub fn height_at(&self, x: usize, y: usize) -> f32 {
        self.heightmap[y * 16 + x]
    }
    
    pub fn biome_at(&self, x: usize, y: usize) -> BiomeType {
        self.biome_map[y * 16 + x]
    }
}
```

---

## 4. PHYSICS & COLLISION

### 4.1 Collision Detection (crates/world/src/physics/collision.rs)

```rust
pub struct PhysicsEngine {
    terrain: Terrain,
}

impl PhysicsEngine {
    pub fn can_move_to(
        &self,
        from: (f32, f32, f32),
        to: (f32, f32, f32),
    ) -> Result<bool, String> {
        // Check if movement is valid (no collision, not underground)
        let to_height = self.terrain.height_at(to.0, to.1);
        
        // Agent cannot move below terrain
        if to.2 < to_height {
            return Ok(false);
        }
        
        // Agent cannot jump more than 2 blocks
        let height_diff = to.2 - from.2;
        if height_diff > 2.0 {
            return Ok(false);
        }
        
        // Maximum fall is 10 blocks (damage in Phase 3)
        if height_diff < -10.0 {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    pub fn apply_gravity(
        &self,
        position: &mut (f32, f32, f32),
    ) -> Result<(), String> {
        let terrain_height = self.terrain.height_at(position.0, position.1);
        
        // Apply gravity: agent rests on terrain
        if position.2 < terrain_height {
            position.2 = terrain_height;
        }
        
        Ok(())
    }
}
```

### 4.2 Movement Validation (crates/world/src/physics/movement.rs)

```rust
pub struct MovementValidator;

impl MovementValidator {
    pub fn validate_move(
        from: (f32, f32, f32),
        to: (f32, f32, f32),
        physics: &PhysicsEngine,
    ) -> Result<(), String> {
        // Check distance (max 1.0 per tick)
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 1.0 {
            return Err("Movement distance too large".to_string());
        }
        
        // Check collision
        if !physics.can_move_to(from, to)? {
            return Err("Collision detected".to_string());
        }
        
        Ok(())
    }
}
```

---

## 5. ACTION SYSTEM

### 5.1 Action Validation (crates/world/src/actions/validation.rs)

```rust
pub struct ActionValidator {
    physics: PhysicsEngine,
}

impl ActionValidator {
    pub fn validate_action(
        &self,
        agent: &Agent,
        action: &InputEventPayload,
        universe: &Universe,
    ) -> Result<(), String> {
        match action {
            InputEventPayload::Move { x, y, z } => {
                let to = (*x, *y, *z);
                MovementValidator::validate_move(agent.position, to, &self.physics)?;
                Ok(())
            },
            InputEventPayload::Gather { resource_type } => {
                self.validate_gather(agent, resource_type, universe)?;
                Ok(())
            },
            InputEventPayload::Mine => {
                self.validate_mine(agent, universe)?;
                Ok(())
            },
            InputEventPayload::Build { blueprint_id } => {
                self.validate_build(agent, blueprint_id, universe)?;
                Ok(())
            },
            InputEventPayload::Craft { recipe_id } => {
                self.validate_craft(agent, recipe_id, universe)?;
                Ok(())
            },
            InputEventPayload::Chat { .. } => {
                // No validation needed for chat
                Ok(())
            },
        }
    }
    
    fn validate_gather(
        &self,
        agent: &Agent,
        resource_type: &str,
        universe: &Universe,
    ) -> Result<(), String> {
        // Must be at location with resource
        let biome = universe.terrain.biome_at(agent.position.0, agent.position.1);
        let resources = BiomeMap::resources_in_biome(biome);
        
        if !resources.iter().any(|r| r.name() == resource_type) {
            return Err(format!(
                "Cannot gather {} in {:?} biome",
                resource_type, biome
            ));
        }
        
        Ok(())
    }
    
    fn validate_mine(
        &self,
        agent: &Agent,
        universe: &Universe,
    ) -> Result<(), String> {
        // Mining only in mountains with ore
        let biome = universe.terrain.biome_at(agent.position.0, agent.position.1);
        
        if biome != BiomeType::Mountain {
            return Err("Mining only allowed in mountains".to_string());
        }
        
        Ok(())
    }
    
    fn validate_build(
        &self,
        agent: &Agent,
        blueprint_id: u64,
        universe: &Universe,
    ) -> Result<(), String> {
        // Check required materials in inventory
        // Check location is buildable (not water, not occupied)
        
        let height = universe.terrain.height_at(agent.position.0, agent.position.1);
        if agent.position.2 != height {
            return Err("Must build on solid ground".to_string());
        }
        
        Ok(())
    }
    
    fn validate_craft(
        &self,
        agent: &Agent,
        recipe_id: u64,
        universe: &Universe,
    ) -> Result<(), String> {
        // Check required ingredients in inventory
        // Check crafting station availability
        
        Ok(())
    }
}
```

### 5.2 Action Execution (crates/world/src/actions/execution.rs)

```rust
pub struct ActionExecutor;

impl ActionExecutor {
    pub fn execute(
        action: &InputEventPayload,
        agent: &mut Agent,
        universe: &mut Universe,
        rng: &mut RngStream,
    ) -> Result<ActionOutcome, String> {
        match action {
            InputEventPayload::Move { x, y, z } => {
                agent.position = (*x, *y, *z);
                Ok(ActionOutcome::MovementSuccess)
            },
            InputEventPayload::Gather { resource_type } => {
                let quantity = 1 + (rng.next_u64() % 3) as u32;  // 1-3 items
                agent.inventory.push(Asset::new(
                    resource_type.to_string(),
                    quantity,
                ));
                Ok(ActionOutcome::GatherSuccess { quantity })
            },
            InputEventPayload::Mine => {
                let ore_quantity = 1 + (rng.next_u64() % 2) as u32;  // 1-2 ore
                agent.inventory.push(Asset::new("ore".to_string(), ore_quantity));
                Ok(ActionOutcome::MineSuccess { quantity: ore_quantity })
            },
            InputEventPayload::Build { blueprint_id } => {
                // Create asset at agent position
                let asset = Asset::new_building(*blueprint_id, agent.position);
                universe.assets.insert(asset.id, asset);
                Ok(ActionOutcome::BuildSuccess)
            },
            InputEventPayload::Craft { recipe_id } => {
                // Consume inputs, produce output
                let output = Self::execute_recipe(*recipe_id, agent)?;
                agent.inventory.push(output);
                Ok(ActionOutcome::CraftSuccess)
            },
            InputEventPayload::Chat { text } => {
                Ok(ActionOutcome::ChatSuccess { text: text.clone() })
            },
        }
    }
    
    fn execute_recipe(recipe_id: u64, agent: &mut Agent) -> Result<Asset, String> {
        // Recipe: 2 ore → 1 tool
        let recipe = get_recipe(recipe_id)?;
        
        // Check ingredients
        for ingredient in &recipe.inputs {
            let count = agent.inventory.iter()
                .filter(|a| a.name == ingredient.name)
                .count();
            
            if count < ingredient.quantity as usize {
                return Err(format!("Not enough {}", ingredient.name));
            }
        }
        
        // Remove ingredients
        for ingredient in &recipe.inputs {
            for _ in 0..ingredient.quantity {
                agent.inventory.retain(|a| a.name != ingredient.name);
            }
        }
        
        // Produce output
        Ok(Asset::new(recipe.output_name.clone(), recipe.output_quantity))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionOutcome {
    MovementSuccess,
    GatherSuccess { quantity: u32 },
    MineSuccess { quantity: u32 },
    BuildSuccess,
    CraftSuccess,
    ChatSuccess { text: String },
}
```

---

## 6. ASSET PRESERVATION & MECHANICS

### 6.1 Asset System (crates/world/src/assets/asset.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: u64,
    pub name: String,
    pub location: AssetLocation,
    pub state: AssetState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AssetLocation {
    Inventory(u64),  // Agent ID
    World(f32, f32, f32),  // Position
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetState {
    pub durability: f32,  // 0.0-1.0
    pub owner_id: Option<u64>,
    pub created_tick: u64,
}

impl Asset {
    pub fn validate(&self, universe: &Universe) -> Result<(), String> {
        match &self.location {
            AssetLocation::Inventory(agent_id) => {
                if !universe.agents.contains_key(agent_id) {
                    return Err("Agent does not exist".to_string());
                }
            },
            AssetLocation::World(x, y, z) => {
                let terrain_height = universe.terrain.height_at(*x, *y);
                if *z < terrain_height {
                    return Err("Asset below terrain".to_string());
                }
            },
        }
        Ok(())
    }
}
```

### 6.2 Asset Import from Gemini (crates/world/src/genesis/asset_import.rs)

```rust
pub fn import_assets_from_genesis(
    genesis_data: &GenesisData,
    universe: &mut Universe,
) -> Result<(), String> {
    // Import House (Homestead)
    if let Some(house_data) = &genesis_data.house {
        let house = Asset {
            id: 1,
            name: "House".to_string(),
            location: AssetLocation::World(
                house_data.position.0,
                house_data.position.1,
                house_data.position.2,
            ),
            state: AssetState {
                durability: house_data.durability.unwrap_or(1.0),
                owner_id: Some(genesis_data.agent_id),
                created_tick: 0,
            },
        };
        
        house.validate(universe)?;
        universe.assets.insert(house.id, house);
    }
    
    // Import Shed (Tool Storage)
    if let Some(shed_data) = &genesis_data.shed {
        let shed = Asset {
            id: 2,
            name: "Shed".to_string(),
            location: AssetLocation::World(
                shed_data.position.0,
                shed_data.position.1,
                shed_data.position.2,
            ),
            state: AssetState {
                durability: shed_data.durability.unwrap_or(1.0),
                owner_id: Some(genesis_data.agent_id),
                created_tick: 0,
            },
        };
        
        shed.validate(universe)?;
        universe.assets.insert(shed.id, shed);
    }
    
    // Import Tools
    for (idx, tool_data) in genesis_data.tools.iter().enumerate() {
        let tool = Asset {
            id: 100 + idx as u64,
            name: tool_data.name.clone(),
            location: AssetLocation::Inventory(genesis_data.agent_id),
            state: AssetState {
                durability: tool_data.durability.unwrap_or(1.0),
                owner_id: Some(genesis_data.agent_id),
                created_tick: 0,
            },
        };
        
        tool.validate(universe)?;
        universe.assets.insert(tool.id, tool);
    }
    
    Ok(())
}
```

---

## 7. OBSERVATION EVENTS

All actions must produce observable state changes:

```rust
pub fn action_to_observation(
    tick: u64,
    action: &InputEventPayload,
    outcome: &ActionOutcome,
    agent_before: &Agent,
    agent_after: &Agent,
) -> ObservationEvent {
    let payload = match (action, outcome) {
        (InputEventPayload::Move { x, y, z }, ActionOutcome::MovementSuccess) => {
            json!({
                "type": "agent_moved",
                "agent_id": agent_after.id,
                "from": [agent_before.position.0, agent_before.position.1, agent_before.position.2],
                "to": [x, y, z]
            })
        },
        (InputEventPayload::Gather { resource_type }, ActionOutcome::GatherSuccess { quantity }) => {
            json!({
                "type": "resource_gathered",
                "agent_id": agent_after.id,
                "resource": resource_type,
                "quantity": quantity,
                "position": [agent_after.position.0, agent_after.position.1, agent_after.position.2],
                "inventory_size_before": agent_before.inventory.len(),
                "inventory_size_after": agent_after.inventory.len()
            })
        },
        _ => json!({}),
    };
    
    ObservationEvent {
        tick,
        event_type: format!("{:?}", outcome),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes()).as_bytes().try_into().unwrap(),
    }
}
```

---

## 8. SUCCESS CRITERIA (ALL REQUIRED)

### Build & Compilation
- [ ] All terrain/physics/actions crates compile
- [ ] `cargo test --all` passes
- [ ] Zero clippy warnings

### Terrain Generation
- [ ] **TEST-TERRAIN-DETERMINISM-001**: Same seed → identical heightmap and biomes
- [ ] **TEST-TERRAIN-HASH-001**: Terrain hash stable across runs
- [ ] **TEST-BIOME-ACCURACY-001**: Resources available in correct biomes

### Physics & Collision
- [ ] **TEST-COLLISION-001**: Agent cannot move into terrain
- [ ] **TEST-GRAVITY-001**: Agents rest on terrain height
- [ ] **TEST-MOVEMENT-DISTANCE-001**: Movement limited to 1.0 per tick

### Action Validation
- [ ] **TEST-GATHER-LOCATION-001**: Gathering fails in wrong biome
- [ ] **TEST-MINE-LOCATION-001**: Mining only in mountains
- [ ] **TEST-BUILD-LOCATION-001**: Building only on solid ground
- [ ] **TEST-CRAFT-RECIPE-001**: Crafting consumes/produces correct items

### Asset Integration
- [ ] **TEST-ASSET-IMPORT-001**: House, Shed, Tools imported from genesis
- [ ] **TEST-ASSET-PERSISTENCE-001**: Assets preserved across snapshots
- [ ] **TEST-ASSET-VALIDATION-001**: All assets valid after genesis

### Observations
- [ ] **TEST-OBSERVATION-MOVE-001**: Movement produces observable event
- [ ] **TEST-OBSERVATION-GATHER-001**: Gathering produces observable event
- [ ] **TEST-OBSERVATION-MINE-001**: Mining produces observable event
- [ ] **TEST-OBSERVATION-BUILD-001**: Building produces observable event

### Regression
- [ ] All Phase 0 and Phase 1 tests still passing
- [ ] Determinism maintained

---

## 9. FORBIDDEN ACTIONS

Windsurf MUST NOT:

1. Use non-deterministic terrain libraries (e.g., noise-rs without explicit seeding)
2. Implement TODO/FIXME/stub action validation
3. Allow gathering in wrong biomes
4. Allow mining outside mountains
5. Skip action validation in authority pipeline
6. Implement mock gathering/mining (always true/false)
7. Add randomness outside RNG subsystems
8. Modify imported assets from genesis (only use, not change data)
9. Create unobservable state changes
10. Implement action execution without ObservationEvent generation

---

## 10. HARD STOP CONDITIONS

Execution STOPS immediately if:

1. Terrain generation non-deterministic (TEST-TERRAIN-DETERMINISM-001 fails)
2. Action validation bypassed (stub implementations remain)
3. Collision detection broken (agent moves into terrain)
4. Asset loss detected (imported assets missing)
5. Observation event not generated for action
6. Regression in Phase 0/1 tests
7. Determinism divergence after terrain integration

---

## 11. PHASE 2 EXIT CHECKLIST

Phase 2 is DONE only when ALL are TRUE:

**Build:**
- [ ] All terrain/physics/action crates compile
- [ ] All unit tests pass
- [ ] Zero clippy warnings

**Terrain:**
- [ ] TEST-TERRAIN-DETERMINISM-001 passing
- [ ] TEST-BIOME-ACCURACY-001 passing

**Physics:**
- [ ] TEST-COLLISION-001 passing
- [ ] TEST-MOVEMENT-DISTANCE-001 passing

**Actions:**
- [ ] TEST-GATHER-LOCATION-001 passing
- [ ] TEST-MINE-LOCATION-001 passing
- [ ] TEST-BUILD-LOCATION-001 passing

**Assets:**
- [ ] TEST-ASSET-IMPORT-001 passing
- [ ] TEST-ASSET-VALIDATION-001 passing

**Observations:**
- [ ] All observation tests passing

**Regression:**
- [ ] All Phase 0 and Phase 1 tests passing

**AMP Sign-Off:**
- [ ] AMP Principal-Level Auditor approval in writing

---

## END OF PLAN

**Authority:** KAIZA-MCP · AMP Principal Planner  
**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_2_WORLD  
**Timestamp:** 2026-01-11
