/*!
# World Configuration Types

**Purpose:** Provide configuration structures for world initialization.

**Why it exists:** World needs structured configuration for genesis
state including regions, agents, and objects.

**Determinism guarantees:**
- All configuration uses deterministic types
- No floating-point in configuration
- All values are fixed-point integers
- Configuration is serializable and auditable

**How it affects replay:** Same configuration produces identical
world initialization across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::terrain::Biome;
use super::agent_location::AgentLocation;
use super::inventory::Inventory;
use super::asset::ObjectProperties;

/// World genesis configuration
/// Defines initial world state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldGenesisConfig {
    /// World seed for deterministic generation
    pub seed: u64,
    /// Terrain generation seed
    pub terrain_seed: u64,
    /// Initial regions
    pub regions: Vec<RegionConfig>,
    /// Initial agents
    pub agents: Vec<AgentConfig>,
    /// Initial objects
    pub objects: Vec<ObjectConfig>,
    /// Initial health for agents
    pub health: super::bio::complete_biology::BiologicalState,
    /// Initial energy for agents
    pub energy: u64,
    /// Initial inventory for agents
    pub inventory: Inventory,
}

impl WorldGenesisConfig {
    /// Create default genesis configuration
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            terrain_seed: seed + 1, // Different seed for terrain
            regions: vec![
                RegionConfig {
                    id: "spawn_region".to_string(),
                    name: "Spawn Region".to_string(),
                    parent_id: None,
                    bounds: RegionBounds::new(0, 100, 0, 100),
                    terrain_type: Biome::Forest,
                },
            ],
            agents: vec![
                AgentConfig {
                    id: "gem-d".to_string(),
                    name: "Gem-D".to_string(),
                    start_x: 50,
                    start_y: 50,
                },
                AgentConfig {
                    id: "gem-k".to_string(),
                    name: "Gem-K".to_string(),
                    start_x: 60,
                    start_y: 50,
                },
            ],
            objects: vec![
                ObjectConfig {
                    id: "tool_1".to_string(),
                    object_type: "tool".to_string(),
                    x: 45,
                    y: 45,
                    owner: "unowned".to_string(),
                    properties: ObjectProperties::new(),
                },
                ObjectConfig {
                    id: "resource_1".to_string(),
                    object_type: "resource".to_string(),
                    x: 55,
                    y: 55,
                    owner: "unowned".to_string(),
                    properties: ObjectProperties::new(),
                },
            ],
            health: super::bio::complete_biology::BiologicalState::new_human_equivalent(),
            energy: 8000, // 80% energy in fixed-point
            inventory: Inventory::new(10),
        }
    }
}

/// Region configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionConfig {
    /// Region ID
    pub id: String,
    /// Region name
    pub name: String,
    /// Parent region ID
    pub parent_id: Option<String>,
    /// Region bounds
    pub bounds: RegionBounds,
    /// Terrain type
    pub terrain_type: Biome,
}

/// Agent configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Starting X coordinate
    pub start_x: i64,
    /// Starting Y coordinate
    pub start_y: i64,
}

/// Object configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectConfig {
    /// Object ID
    pub id: String,
    /// Object type
    pub object_type: String,
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
    /// Owner
    pub owner: String,
    /// Object properties
    pub properties: ObjectProperties,
}

/// Region bounds
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
    /// Create new region bounds
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

/// Resource deposits container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeposits {
    /// Resources indexed by ID
    pub resources: BTreeMap<String, ResourceDeposit>,
}

impl ResourceDeposits {
    /// Create new resource deposits
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }
    
    /// Get total resources
    pub fn total_resources(&self) -> u64 {
        self.resources.values().map(|r| r.quantity).sum()
    }
    
    /// Get resource by ID
    pub fn get(&self, resource_id: &str) -> Option<&ResourceDeposit> {
        self.resources.get(resource_id)
    }
    
    /// Get mutable resource by ID
    pub fn get_mut(&mut self, resource_id: &str) -> Option<&mut ResourceDeposit> {
        self.resources.get_mut(resource_id)
    }
    
    /// Add resource deposit
    pub fn insert(&mut self, resource_id: String, deposit: ResourceDeposit) {
        self.resources.insert(resource_id, deposit);
    }
}

/// Resource deposit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeposit {
    /// Resource type
    pub resource_type: String,
    /// Current quantity
    pub quantity: u64,
    /// Maximum quantity
    pub max_quantity: u64,
    /// Regeneration rate per tick
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
            Err(format!("Insufficient {} (available: {})", 
                   self.resource_type, self.quantity))
        } else {
            let extracted = amount;
            self.quantity -= extracted;
            Ok(extracted)
        }
    }
    
    /// Regenerate resource
    pub fn regenerate(&mut self) {
        if self.quantity < self.max_quantity && self.regeneration_rate > 0 {
            self.quantity = (self.quantity + self.regeneration_rate).min(self.max_quantity);
            self.regeneration_tick += 1;
        }
    }
}
