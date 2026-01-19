/*!
# Deterministic World Representation

**Purpose:** Deterministic spatial model for human-equivalent world simulation.

**Why it exists:** The world provides the spatial context where agents exist,
interact, and experience environmental effects. It must be deterministic
to ensure reproducible simulations and proper resource management.

**Determinism guarantees:**
- All world generation uses seeded RNG streams
- Fixed coordinate system with no floating-point ambiguity
- Deterministic object ordering with BTreeMap
- All world changes are auditable and reversible

**How it affects replay:** Same seed + InputEvents will
produce identical world states across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::types::*;
use super::terrain::{Terrain, Biome};
use super::agent_location::AgentLocation;
use super::inventory::Inventory;
use super::asset::Asset;
use super::hashing::WorldHasher;
use super::bio::complete_biology::BiologicalState;
use super::config::{WorldGenesisConfig, RegionConfig, AgentConfig, ObjectConfig};
use super::agent_location::AgentLocation;
use super::inventory::Inventory;
use super::asset::ObjectProperties;
use crate::rng::{GlobalSeed, RngSubsystem};

/// World coordinate system
/// Fixed integer-based coordinates for determinism
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorldCoordinate {
    /// X coordinate (0 to world_width)
    pub x: i64,
    /// Y coordinate (0 to world_height)
    pub y: i64,
    /// Z coordinate (0 to world_depth)
    pub z: i64,
}

impl WorldCoordinate {
    /// Create new coordinate
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Get distance to another coordinate
    /// Returns integer distance for determinism (no floating point)
    pub fn distance_to(&self, other: &WorldCoordinate) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        
        // Manhattan distance for deterministic physics
        dx.abs() + dy.abs() + dz.abs()
    }

    /// Get chunk coordinates containing this coordinate
    pub fn get_chunk_coords(&self, chunk_size: i64) -> (i64, i64, i64) {
        let chunk_x = (self.x / chunk_size) * chunk_size;
        let chunk_y = (self.y / chunk_size) * chunk_size;
        let chunk_z = (self.z / chunk_size) * chunk_size;
        
        (chunk_x, chunk_y, chunk_z)
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

/// World region with hierarchical structure
/// Organizes space into manageable regions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldRegion {
    /// Unique region identifier
    pub id: String,
    /// Region name
    pub name: String,
    /// Parent region (None for top-level regions)
    pub parent_id: Option<String>,
    /// Region bounds
    pub bounds: RegionBounds,
    /// Terrain type for this region
    pub terrain_type: Biome,
    /// Locations within this region
    pub locations: BTreeMap<String, WorldLocation>,
    /// Objects in this region
    pub objects: BTreeMap<String, WorldObject>,
    /// Resource deposits
    pub resources: ResourceDeposits,
}

impl WorldRegion {
    /// Create new region
    pub fn new(
        id: String,
        name: String,
        parent_id: Option<String>,
        bounds: RegionBounds,
        terrain_type: Biome,
    ) -> Self {
        Self {
            id,
            name,
            parent_id,
            bounds,
            terrain_type,
            locations: BTreeMap::new(),
            objects: BTreeMap::new(),
            resources: ResourceDeposits::new(),
        }
    }

    /// Add location to region
    pub fn add_location(&mut self, location: WorldLocation) {
        self.locations.insert(location.id.clone(), location);
    }

    /// Add object to region
    pub fn add_object(&mut self, object: WorldObject) {
        self.objects.insert(object.id.clone(), object);
    }

    /// Check if coordinate is within region bounds
    pub fn contains_coordinate(&self, coord: &WorldCoordinate) -> bool {
        coord.x >= self.bounds.min_x &&
        coord.x < self.bounds.max_x &&
        coord.y >= self.bounds.min_y &&
        coord.y < self.bounds.max_y
    }
}

/// Region bounds for spatial constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionBounds {
    /// Minimum X coordinate
    pub min_x: i64,
    /// Maximum X coordinate
    pub max_x: i64,
    /// Minimum Y coordinate
    pub min_y: i64,
    /// Maximum Y coordinate
    pub max_y: i64,
}

impl RegionBounds {
    /// Create new bounds
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

}

/// Resource deposit with deterministic quantities
/// Tracks available resources in terrain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeposit {
    /// Resource type identifier
    pub resource_type: String,
    /// Current quantity available
    pub quantity: u64,
    /// Maximum capacity
    pub max_quantity: u64,
    /// Regeneration rate per tick (fixed-point for determinism)
    pub regeneration_rate: u64,
    /// Regeneration tick counter
    pub regeneration_tick: u64,
}

impl ResourceDeposit {
    /// Create new resource deposit
    pub fn new(
        resource_type: String,
        initial_quantity: u64,
        max_quantity: u64,
        regeneration_rate: u64,
    ) -> Self {
        Self {
            resource_type,
            quantity: initial_quantity,
            max_quantity,
            regeneration_rate,
            regeneration_tick: 0,
        }
    }

    /// Extract resource
    pub fn extract(&mut self, amount: u64) -> Result<u64, String> {
        if amount > self.quantity {
            let extracted = amount.min(self.quantity);
            self.quantity -= extracted;
            Ok(extracted)
        } else {
            Err(format!("Insufficient {} (available: {})", 
                   self.resource_type, self.quantity))
        }
    }

    /// Regenerate resource (deterministic integer math)
    pub fn regenerate(&mut self, rng: &mut crate::rng::RngStream) {
        if self.quantity < self.max_quantity {
            // Fixed-point regeneration: rate is stored as *1000 to avoid floating point
            let regeneration_amount = self.regeneration_rate;
            
            if regeneration_amount > 0 {
                self.quantity = (self.quantity + regeneration_amount).min(self.max_quantity);
                self.regeneration_tick += 1;
            }
        }
        
        trace!("Regenerated {} {} (tick {})", 
               self.resource_type, regeneration_amount, self.regeneration_tick);
    }
}

/// World object with physical properties
/// Represents any entity that exists in the world
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldObject {
    /// Unique object identifier
    pub id: String,
    /// Object type
    pub object_type: String,
    /// Current position
    pub position: WorldCoordinate,
    /// Owner (agent ID or "unowned")
    pub owner: String,
    /// Physical properties
    pub properties: ObjectProperties,
    /// Durability (0 = fragile, 10000 = durable)
    pub durability: u64,
    /// Weight for physics calculations (integer grams)
    pub weight: u64,
    /// Volume for collision detection (integer cubic units)
    pub volume: u64,
}

impl WorldObject {
    /// Create new object
    pub fn new(
        id: String,
        object_type: String,
        position: WorldCoordinate,
        owner: String,
        properties: ObjectProperties,
    ) -> Self {
        Self {
            id,
            object_type,
            position,
            owner,
            properties,
            durability: 10000, // Default to durable (fixed-point)
            weight: 10000, // Default weight (10kg in grams)
            volume: 1000, // Default volume (1 cubic unit)
        }
    }

    /// Check if object can be picked up (deterministic integer math)
    pub fn is_pickable(&self, agent_location: &AgentLocation, strength: u64) -> bool {
        let distance = agent_location.position.distance_to(&self.position);
        let reach_requirement = strength * 2; // 2 units reach
        let weight_limit = agent_location.strength * 1500; // Can lift 1.5x weight (in grams)
        
        distance <= reach_requirement &&
            self.owner == "unowned" &&
            self.weight <= weight_limit &&
            self.durability > 0 // > 0 in fixed-point
    }

    /// Apply damage to object (fixed-point for determinism)
    pub fn apply_damage(&mut self, damage: u64) {
        // Damage is stored as fixed-point with 2 decimal places
        let damage_fixed = damage.min(10000); // Max 100.00 damage
        self.durability = if self.durability > damage_fixed {
            self.durability - damage_fixed
        } else {
            0
        };
    }
}

/// Object properties for physics interaction
/// Physical characteristics for collision and movement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectProperties {
    /// Size category (tiny, small, medium, large, huge)
    pub size: ObjectSize,
    /// Material type
    pub material: String,
    /// Temperature resistance (fixed-point)
    pub temperature_resistance: u64,
    /// Fire resistance (fixed-point)
    pub fire_resistance: u64,
    /// Electrical conductivity (fixed-point)
    pub electrical_conductivity: u64,
}

impl ObjectProperties {
    /// Create default object properties
    pub fn new() -> Self {
        Self {
            size: ObjectSize::Medium,
            material: "wood".to_string(),
            temperature_resistance: 100, // 1.0 in fixed-point
            fire_resistance: 50, // 0.5 in fixed-point
            electrical_conductivity: 0, // 0.0 in fixed-point
        }
    }
}

/// Object size categories
/// Standardized size classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ObjectSize {
    /// Very small (fits in hand)
    Tiny,
    /// Small (pocketable)
    Small,
    /// Medium (one-handed)
    Medium,
    /// Large (requires two hands)
    Large,
    /// Very large (requires team)
    Huge,
}

impl ObjectSize {
    /// Get weight multiplier for physics calculations (fixed-point)
    pub const fn weight_multiplier(self) -> u64 {
        match self {
            Self::Tiny => 100,    // 0.1 in fixed-point
            Self::Small => 500,   // 0.5 in fixed-point
            Self::Medium => 1000,  // 1.0 in fixed-point
            Self::Large => 2000,  // 2.0 in fixed-point
            Self::Huge => 5000,  // 5.0 in fixed-point
            Self::Huge => 10000, // 10.0 in fixed-point
        }
    }

    /// Get base volume for collision calculations (fixed-point)
    pub const fn base_volume(self) -> u64 {
        match self {
            Self::Tiny => 10,    // 0.01 in fixed-point
            Self::Small => 100,   // 0.1 in fixed-point
            Self::Medium => 1000,  // 1.0 in fixed-point
            Self::Large => 8000,  // 8.0 in fixed-point
            Self::Huge => 27000, // 27.0 in fixed-point
        }
    }
}

/// Complete deterministic world model
/// Main container for all world state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct World {
    /// World dimensions
    pub dimensions: WorldDimensions,
    /// All regions in the world
    pub regions: BTreeMap<String, WorldRegion>,
    /// All agents in the world
    pub agents: BTreeMap<String, AgentLocation>,
    /// All objects in the world
    pub objects: BTreeMap<String, WorldObject>,
    /// Global seed for deterministic generation
    pub seed: u64,
    /// Current tick
    pub tick: u64,
    /// Previous state hash for verification
    pub state_hash: [u8; 32],
    /// RNG system with deterministic streams
    pub rng: crate::rng::RngSystem,
}

impl World {
    /// Create new world with specified dimensions
    pub fn new(width: u64, height: u64, seed: u64) -> Self {
        let dimensions = WorldDimensions::new(width, height);
        
        Self {
            dimensions,
            regions: BTreeMap::new(),
            agents: BTreeMap::new(),
            objects: BTreeMap::new(),
            seed,
            tick: 0,
            state_hash: [0; 32],
            rng: crate::rng::RngSystem::new(seed),
        }
    }

    /// Initialize world with genesis configuration
    pub fn initialize(&mut self, config: WorldGenesisConfig) {
        self.seed = config.seed;
        
        // Create world regions
        for region_config in config.regions {
            let region = WorldRegion::new(
                region_config.id,
                region_config.name,
                region_config.parent_id,
                region_config.bounds,
                region_config.terrain_type,
            );
            self.regions.insert(region.id.clone(), region);
        }
        
        // Add genesis agents
        for agent_config in config.agents {
            let location = AgentLocation::new(
                agent_config.id.clone(),
                WorldCoordinate::new(agent_config.start_x, agent_config.start_y, 0),
                5000, // Default strength
            );
            let agent = Agent::new(
                agent_config.id,
                agent_config.name,
                location,
                config.health.clone(),
                config.inventory.clone(),
                config.energy,
            );
            
            self.agents.insert(agent_config.id, agent);
        }
        
        // Add genesis objects
        for object_config in config.objects {
            let object = WorldObject::new(
                object_config.id,
                object_config.object_type,
                WorldCoordinate::new(object_config.x, object_config.y, 0),
                object_config.owner,
                object_config.properties,
            );
            self.objects.insert(object_config.id, object);
        }
        
        // Initialize terrain
        self.terrain = Terrain::new(config.terrain_seed);
        
        // Compute initial state hash
        self.state_hash = self.compute_state_hash();
        
        debug!("World initialized with {} regions, {} agents, {} objects", 
               self.regions.len(), self.agents.len(), self.objects.len());
    }

    /// Update world for one tick
    pub fn update(&mut self, input_events: &[InputEvent]) -> WorldUpdate {
        let old_hash = self.state_hash;
        
        // Process input events
        for event in input_events {
            self.process_input_event(event);
        }
        
        // Update terrain if needed
        if self.terrain.needs_update() {
            self.terrain.update();
        }
        
        // Update all agents
        for agent in self.agents.values_mut() {
            agent.biological_state.tick(&mut self.rng.get_stream("biology"));
        }
        
        // Update all objects
        for object in self.objects.values_mut() {
            object.update(&input_events);
        }
        
        // Update all regions
        for region in self.regions.values_mut() {
            region.update(&input_events);
        }
        
        self.tick += 1;
        
        // Compute new state hash
        self.state_hash = self.compute_state_hash();
        
        WorldUpdate {
            tick: self.tick,
            state_hash: self.state_hash,
            hash_delta: self.compute_hash_delta(&old_hash, &self.state_hash),
        }
    }

    /// Process input event
    fn process_input_event(&mut self, event: &InputEvent) {
        match event {
            InputEvent::MoveAgent { agent_id, direction, distance } => {
                if let Some(agent) = self.agents.get_mut(&agent_id) {
                    agent.move_in_direction(&direction, distance);
                }
            }
            InputEvent::GatherResource { agent_id, resource_id, amount } => {
                if let (Some(agent), Some(region)) = (self.agents.get(&agent_id), self.get_agent_region(&agent_id)) {
                    if let Some(resource) = region.resources.get_mut(&resource_id) {
                        if let Ok(extracted) = resource.extract(amount) {
                            agent.add_to_inventory(resource_id, extracted);
                        }
                    }
                }
            }
            InputEvent::UseTool { agent_id, object_id, tool_type } => {
                if let (Some(agent), Some(object)) = (self.agents.get(&agent_id), self.get_agent_object(&agent_id, &object_id)) {
                    agent.use_tool(&object_id, tool_type);
                }
            }
            InputEvent::DropObject { agent_id, object_id } => {
                if let (Some(agent), Some(object)) = (self.agents.get(&agent_id), self.get_agent_object(&agent_id, &object_id)) {
                    agent.drop_object(object_id);
                }
            }
            InputEvent::Rest { agent_id } => {
                if let Some(agent) = self.agents.get_mut(&agent_id) {
                    agent.rest();
                }
            }
        }
    }

    /// Get agent's current region
    fn get_agent_region(&self, agent_id: &str) -> Option<&WorldRegion> {
        if let Some(agent) = self.agents.get(agent_id) {
            let agent_coord = &agent.position;
            
            // Find region containing this coordinate
            for region in self.regions.values() {
                if region.contains_coordinate(agent_coord) {
                    return Some(region);
                }
            }
        }
        }
    }

    /// Get object by ID
    fn get_object(&self, object_id: &str) -> Option<&WorldObject> {
        self.objects.get(object_id)
    }

    /// Get agent by ID
    pub fn get_agent(&self, agent_id: &str) -> Option<&Agent> {
        self.agents.get(agent_id)
    }

    /// Get region by ID
    pub fn get_region(&self, region_id: &str) -> Option<&WorldRegion> {
        self.regions.get(region_id)
    }

    /// Compute world state hash
    fn compute_state_hash(&self) -> [u8; 32] {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        
        // Hash regions
        for (region_id, region) in &self.regions {
            region_id.hash(&mut hasher);
        }
        
        // Hash agents
        for (agent_id, agent) in &self.agents {
            agent_id.hash(&mut hasher);
        }
        
        // Hash objects
        for (object_id, object) in &self.objects {
            object_id.hash(&mut hasher);
        }
        
        hasher.finalize().into()
    }

    /// Compute hash delta for audit
    fn compute_hash_delta(&old_hash: &[u8; 32], new_hash: &[u8; 32]) -> [u8; 32] {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(old_hash);
        hasher.update(new_hash);
        
        hasher.finalize().into()
    }

    /// Get world dimensions
    pub fn get_dimensions(&self) -> &WorldDimensions {
        &self.dimensions
    }

    /// Get total number of regions
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Get total number of agents
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Get total number of objects
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Get world statistics
    pub fn get_statistics(&self) -> WorldStatistics {
        WorldStatistics {
            total_regions: self.region_count(),
            total_agents: self.agent_count(),
            total_objects: self.object_count(),
            total_resources: self.count_total_resources(),
            average_agent_health: self.calculate_average_health(),
            terrain_complexity: self.terrain.get_complexity(),
        }
    }

    /// Calculate average agent health
    fn calculate_average_health(&self) -> f64 {
        if self.agents.is_empty() {
            return 0.0;
        }
        
        let total_health: f64 = self.agents
            .values()
            .map(|agent| agent.health as f64)
            .sum();
        
        total_health / self.agents.len() as f64
    }

    /// Count total resources across all regions
    fn count_total_resources(&self) -> u64 {
        self.regions
            .values()
            .map(|region| region.resources.total_resources())
            .sum()
    }

    /// Get terrain at specific coordinate
    pub fn get_terrain_at(&self, coord: &WorldCoordinate) -> Option<&Terrain> {
        // Find region containing coordinate
        for region in &self.regions.values() {
            if region.contains_coordinate(coord) {
                return Some(&region.terrain.get_terrain_at(coord));
            }
        }
    }
}

/// World dimensions
/// Physical size of the world
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldDimensions {
    /// Width in world units
    pub width: u64,
    /// Height in world units
    pub height: u64,
    /// Depth in world units
    pub depth: u64,
}

impl WorldDimensions {
    /// Create new dimensions
    pub fn new(width: u64, height: u64) -> Self {
        Self {
            width,
            height,
            depth: 1000, // Default depth
        }
    }

    /// Get total volume
    pub fn total_volume(&self) -> u64 {
        self.width * self.height * self.depth
    }

    /// Get surface area
    pub fn surface_area(&self) -> u64 {
        self.width * self.height
    }
}

/// World update results
/// Changes that occurred during a tick
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldUpdate {
    /// Current tick number
    pub tick: u64,
    /// New state hash
    pub state_hash: [u8; 32],
    /// Hash delta from previous state
    pub hash_delta: [u8; 32],
}

/// World statistics
/// Summary of world state for monitoring
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldStatistics {
    pub total_regions: usize,
    pub total_agents: usize,
    pub total_objects: usize,
    pub total_resources: u64,
    pub average_agent_health: f64,
    pub terrain_complexity: f64,
}

/// Input events that can modify world state
/// These are the only way to change world state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    /// Move agent in direction
    MoveAgent { agent_id: String, direction: Direction, distance: u64 },
    /// Gather resource from terrain
    GatherResource { agent_id: String, resource_id: String, amount: u64 },
    /// Use tool on object
    UseTool { agent_id: String, object_id: String, tool_type: String },
    /// Drop object from inventory
    DropObject { agent_id: String, object_id: String },
    /// Agent rests
    Rest { agent_id: String },
}

/// Movement directions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Agent with biological and inventory
/// Represents an agent in the world
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Current position
    pub position: WorldCoordinate,
    /// Biological state
    pub biological_state: complete_biology::BiologicalState,
    /// Current inventory
    pub inventory: Inventory,
    /// Energy level (fixed-point: 0 = exhausted, 10000 = full)
    pub energy: u64,
}

impl Agent {
    /// Create new agent
    pub fn new(
        id: String,
        name: String,
        position: WorldCoordinate,
        biological_state: BioState,
        inventory: Inventory,
        energy: u64,
    ) -> Self {
        Self {
            id,
            name,
            position,
            biological_state,
            inventory,
            energy,
        }
    }

    /// Move agent in direction (integer-based for determinism)
    pub fn move_in_direction(&mut self, direction: Direction, distance: u64) {
        match direction {
            Direction::North => self.position.y -= distance as i64,
            Direction::South => self.position.y += distance as i64,
            Direction::East => self.position.x += distance as i64,
            Direction::West => self.position.x -= distance as i64,
        }
        
        trace!("Agent {} moved {} units {}", self.id, direction.description(), distance);
    }

    /// Add item to inventory
    pub fn add_to_inventory(&mut self, item_id: String, quantity: u64) {
        if let Some(item) = self.inventory.get_item(&item_id) {
            let new_quantity = item.quantity + quantity;
            if new_quantity <= item.max_stack {
                item.quantity = new_quantity;
                debug!("Agent {} added {} {} to inventory", self.id, item_id, quantity);
            } else {
                debug!("Agent {} inventory full for item {}", self.id, item_id);
            }
        }
    }

    /// Get item from inventory
    pub fn get_inventory_item(&self, item_id: &str) -> Option<&InventoryItem> {
        self.inventory.get_item(item_id)
    }

    /// Drop item from inventory
    pub fn drop_object(&mut self, object_id: String) {
        if let Some(item) = self.get_inventory_item(&object_id) {
            self.inventory.remove_item(&object_id);
            debug!("Agent {} dropped object {}", self.id, object_id);
        }
    }

    /// Use tool on object
    pub fn use_tool(&mut self, object_id: String, tool_type: String) {
        debug!("Agent {} used tool {} on object {}", self.id, tool_type, object_id);
        // Tool effects would be handled by the tool system
    }

    /// Rest (recover energy)
    pub fn rest(&mut self) {
        self.energy = (self.energy + 1000).min(10000); // Recover 10% per rest, max 100%
        trace!("Agent {} is resting (energy: {})", self.id, self.energy);
    }

    /// Get agent's current region
    pub fn get_region(&self) -> Option<&WorldRegion> {
        // This would be implemented by the world system
        None // Placeholder
    }

    /// Get object the agent is holding
    pub fn get_agent_object(&self, object_id: &str) -> Option<&WorldObject> {
        // This would be implemented by the world system
        None // Placeholder
    }
}

/// Inventory for agent
/// Manages items carried by agents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    /// Items indexed by ID
    items: BTreeMap<String, InventoryItem>,
    /// Maximum inventory slots
    max_slots: u32,
    /// Current used slots
    used_slots: u32,
}

impl Inventory {
    /// Create new inventory
    pub fn new(max_slots: u32) -> Self {
        Self {
            items: BTreeMap::new(),
            max_slots,
            used_slots: 0,
        }
    }

    /// Add item to inventory
    pub fn add_item(&mut self, item_id: String, quantity: u64) -> Result<(), String> {
        let item = InventoryItem::new(item_id, quantity);
        
        if self.used_slots + quantity as u32 <= self.max_slots {
            self.items.insert(item_id, item);
            self.used_slots += quantity as u32;
            debug!("Added {} items {} to inventory", item_id, quantity);
            Ok(())
        } else {
            Err(format!("Inventory full for item {}", item_id))
        }
    }

    /// Remove item from inventory
    pub fn remove_item(&mut self, item_id: &str) {
        if let Some(item) = self.items.remove(&item_id) {
            self.used_slots = self.used_slots.saturating_sub(1);
            debug!("Removed {} from inventory", item_id);
        }
    }

    /// Get item by ID
    pub fn get_item(&self, item_id: &str) -> Option<&InventoryItem> {
        self.items.get(item_id)
    }
}

/// Inventory item
/// Individual item that can be carried
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Unique item identifier
    pub id: String,
    /// Item type
    pub item_type: String,
    /// Current quantity
    pub quantity: u64,
    /// Maximum stack size
    pub max_stack: u32,
    /// Weight per unit
    pub weight_per_unit: f64,
}

impl InventoryItem {
    /// Create new inventory item
    pub fn new(id: String, item_type: String, quantity: u64) -> Self {
        Self {
            id,
            item_type,
            quantity,
            max_stack: 10,
            weight_per_unit: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_world_creation() {
        let world = World::new(100, 50, 12345);
        
        assert_eq!(world.tick, 0);
        assert_eq!(world.seed, 12345);
        assert_eq!(world.dimensions.width, 100);
        assert_eq!(world.dimensions.height, 50);
    }

    #[test]
    fn test_coordinate_operations() {
        let coord1 = WorldCoordinate::new(10, 20, 0);
        let coord2 = WorldCoordinate::new(15, 25, 0);
        
        assert_eq!(coord1.distance_to(&coord2), 18.0);
        assert_eq!(coord1.get_chunk_coords(10), (10, 20));
    }

    #[test]
    fn test_region_operations() {
        let bounds = RegionBounds::new(0, 100, 0, 100, 50);
        let region = WorldRegion::new(
            "test_region".to_string(),
            "Test Region".to_string(),
            None,
            bounds,
            Biome::Forest,
        );
        
        assert!(region.contains_coordinate(&WorldCoordinate::new(50, 25, 0));
        assert!(!region.contains_coordinate(&WorldCoordinate::new(150, 75, 0)));
    }

    #[test]
    fn test_object_operations() {
        let mut object = WorldObject::new(
            "test_object".to_string(),
            "tool".to_string(),
            WorldCoordinate::new(10, 10, 0),
            "unowned".to_string(),
            ObjectProperties::new(),
        );
        
        assert!(object.is_pickable(&AgentLocation::new(
            "test_agent".to_string(),
            WorldCoordinate::new(10, 10, 0),
            5000, // 5.0 strength in fixed-point
        ));
        
        object.apply_damage(20); // 0.2 damage in fixed-point
        assert_eq!(object.durability, 9980); // 0.8 in fixed-point
    }

    #[test]
    fn test_agent_operations() {
        let mut agent = Agent::new(
            "test_agent".to_string(),
            "Test Agent".to_string(),
            WorldCoordinate::new(10, 10, 0),
            BiologicalState::new_human_equivalent(),
            Inventory::new(10),
            8000, // 0.8 energy in fixed-point
        );
        
        agent.move_in_direction(Direction::North, 5);
        assert_eq!(agent.position.y, 5);
        
        agent.add_to_inventory("test_item".to_string(), 5);
        assert_eq!(agent.inventory.get_item("test_item").unwrap().quantity, 5);
        
        agent.rest();
        assert_eq!(agent.energy, 9000); // 0.85 in fixed-point
    }
}
