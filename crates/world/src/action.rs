use std::collections::BTreeMap;
use crate::types::{Agent, Universe};
use crate::inventory::ItemType;
use crate::terrain::Biome;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Move(Direction),
    Gather(ResourceType),
    Build(BuildingType),
    Mine(MineType),
    Craft(CraftRecipe),
    Transfer(ItemType, u32, TargetId),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResourceType {
    Wood,
    Stone,
    Water,
    Food,
    Berry,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BuildingType {
    Wall,
    Door,
    Floor,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MineType {
    Coal,
    Metal,
    Gem,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetId {
    Agent(u64),
    Asset(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftRecipe {
    pub inputs: BTreeMap<ItemType, u32>,
    pub output: ItemType,
    pub ticks_required: u32,
}

impl CraftRecipe {
    pub fn new(inputs: BTreeMap<ItemType, u32>, output: ItemType, ticks_required: u32) -> Self {
        Self {
            inputs,
            output,
            ticks_required,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Conditional(u64),
}

impl Action {
    pub fn validate(&self, agent: &Agent, world: &Universe) -> ValidationResult {
        match self {
            Action::Move(direction) => {
                // Check if agent has enough energy
                if agent.bio_state.energy < 1.0 {
                    return ValidationResult::Invalid("Insufficient energy".to_string());
                }
                
                // Check if movement direction is valid
                let new_position = self.calculate_new_position(agent, direction);
                let chunk_key = (new_position.0.div_euclid(256), new_position.1.div_euclid(256));
                if let Some(chunk) = world.terrain.chunks.get(&chunk_key) {
                    let local_x = (new_position.0.rem_euclid(256)) as u8;
                    let local_y = (new_position.1.rem_euclid(256)) as u8;
                    let terrain_height = chunk.height_at(local_x, local_y);
                    let height_diff = (agent.position.2 as u8).abs_diff(terrain_height);
                    if height_diff > 2 {
                        return ValidationResult::Invalid("Terrain too steep".to_string());
                    }
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Invalid position".to_string())
                }
            },
            Action::Gather(resource_type) => {
                // Check if agent has appropriate tool
                let has_tool = agent.inventory.values().any(|asset| {
                    // Check if this asset is a tool
                    asset.name == "Tool"
                });
                if !has_tool {
                    return ValidationResult::Invalid("No tool available".to_string());
                }
                
                // Check if resource is available at current location
                if self.can_gather_at(agent, resource_type, world) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Resource not available".to_string())
                }
            },
            Action::Build(building_type) => {
                // Check if agent has building materials
                let required_materials = self.get_required_materials(building_type);
                for (material_type, required_qty) in &required_materials {
                    use crate::inventory::ItemType;
                    let available_qty = agent.inventory.values()
                        .filter(|asset| match material_type {
                            ItemType::Resource => asset.name == "Resource",
                            _ => false,
                        })
                        .count() as u32;
                    if available_qty < *required_qty {
                        return ValidationResult::Invalid(format!("Insufficient {:?}", material_type));
                    }
                }
                ValidationResult::Valid
            },
            Action::Mine(_mine_type) => {
                // Check if agent has pickaxe
                let has_pickaxe = agent.inventory.values().any(|asset| {
                    asset.name == "Tool"
                });
                if !has_pickaxe {
                    return ValidationResult::Invalid("No pickaxe available".to_string());
                }
                
                // Check if at mountain location
                if self.is_at_mountain(agent, world) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid("Not at mountain location".to_string())
                }
            },
            Action::Craft(recipe) => {
                // Check if agent has all required inputs
                for (input_type, required_qty) in &recipe.inputs {
                    let available_qty = agent.inventory.values()
                        .filter(|asset| match input_type {
                            ItemType::Resource => asset.name == "Resource",
                            ItemType::Tool => asset.name == "Tool",
                            _ => false,
                        })
                        .count() as u32;
                    if available_qty < *required_qty {
                        return ValidationResult::Invalid(format!("Insufficient {:?}", input_type));
                    }
                }
                ValidationResult::Conditional(recipe.ticks_required.into())
            },
            Action::Transfer(item_type, quantity, target_id) => {
                // Check if agent has sufficient quantity
                let available_qty = agent.inventory.values()
                    .filter(|asset| match item_type {
                        ItemType::Resource => asset.name == "Resource",
                        ItemType::Tool => asset.name == "Tool",
                        _ => false,
                    })
                    .count() as u32;
                if available_qty < *quantity {
                    return ValidationResult::Invalid("Insufficient quantity".to_string());
                }
                
                // Check if target exists and is reachable
                match target_id {
                    TargetId::Agent(target_agent_id) => {
                        if world.agents.contains_key(target_agent_id) {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid("Target agent not found".to_string())
                        }
                    },
                    TargetId::Asset(target_asset_id) => {
                        if world.assets.contains_key(target_asset_id) {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid("Target asset not found".to_string())
                        }
                    },
                }
            },
        }
    }
    
    fn calculate_new_position(&self, agent: &Agent, direction: &Direction) -> (i32, i32) {
        let (x, y, _) = agent.position;
        let x_i = x as i32;
        let y_i = y as i32;
        match direction {
            Direction::North => (x_i, y_i - 1),
            Direction::South => (x_i, y_i + 1),
            Direction::East => (x_i + 1, y_i),
            Direction::West => (x_i - 1, y_i),
            Direction::Up | Direction::Down => (x_i, y_i), // Vertical movement handled separately
        }
    }
    
    fn can_gather_at(&self, agent: &Agent, resource_type: &ResourceType, world: &Universe) -> bool {
        // Get agent's position
        let (x, y, _z) = agent.position;
        
        // Get chunk coordinates
        let chunk_x = (x as i32).div_euclid(256);
        let chunk_y = (y as i32).div_euclid(256);
        
        // Check if chunk exists and get its biome
        if let Some(chunk) = world.terrain.get_chunk(chunk_x, chunk_y) {
            // Validate resource availability based on biome
            match (resource_type, &chunk.biome) {
                (ResourceType::Wood, Biome::Forest) => true,
                (ResourceType::Wood, Biome::Grassland) => true, // Some trees in grassland
                (ResourceType::Berry, Biome::Grassland) => true,
                (ResourceType::Berry, Biome::Forest) => true,
                (ResourceType::Stone, Biome::Mountain) => true,
                (ResourceType::Stone, Biome::Desert) => true,
                (ResourceType::Water, Biome::Water) => true,
                (ResourceType::Water, Biome::Grassland) => true, // Rivers/lakes
                (ResourceType::Food, Biome::Grassland) => true,
                (ResourceType::Food, Biome::Forest) => true,
                _ => false, // Resource not available in this biome
            }
        } else {
            false // No chunk at this location
        }
    }
    
    fn get_required_materials(&self, building_type: &BuildingType) -> BTreeMap<ItemType, u32> {
        match building_type {
            BuildingType::Wall => {
                let mut materials = BTreeMap::new();
                let _ = materials.insert(ItemType::Resource, 5); // 5 stone
                materials
            },
            BuildingType::Door => {
                let mut materials = BTreeMap::new();
                let _ = materials.insert(ItemType::Resource, 3); // 3 wood
                materials
            },
            BuildingType::Floor => {
                let mut materials = BTreeMap::new();
                let _ = materials.insert(ItemType::Resource, 2); // 2 wood
                materials
            },
        }
    }
    
    fn is_at_mountain(&self, agent: &Agent, world: &Universe) -> bool {
        // Get agent's position
        let (x, y, _z) = agent.position;
        
        // Get chunk coordinates
        let chunk_x = (x as i32).div_euclid(256);
        let chunk_y = (y as i32).div_euclid(256);
        
        // Check if chunk exists and validate it's a mountain biome
        if let Some(chunk) = world.terrain.get_chunk(chunk_x, chunk_y) {
            matches!(chunk.biome, Biome::Mountain)
        } else {
            false // No chunk at this location
        }
    }
}
