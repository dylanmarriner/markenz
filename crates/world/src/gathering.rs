use crate::types::Agent;
use crate::terrain::{Terrain, Biome};
use crate::inventory::{Item, ItemType};
use crate::action::ResourceType;
use physics::Position;

pub fn gather(agent: &Agent, resource_type: ResourceType, terrain: &Terrain) -> Result<Item, String> {
    let position = agent.position();
    let resources = get_gatherable_resources(&position, terrain);
    
    if !resources.contains(&resource_type) {
        return Err("Resource not available at this location".to_string());
    }
    
    // Check if agent has appropriate tool
    let has_tool = agent.inventory.values().any(|asset| {
        asset.name == "Tool"
    });
    if !has_tool {
        return Err("No tool available".to_string());
    }
    
    // Generate resource item with deterministic quantity
    let quantity = get_gather_quantity(&resource_type, &position, terrain);
    let item_id = generate_item_id(agent.id, resource_type);
    
    let item = match resource_type {
        ResourceType::Wood => Item::resource(item_id, ItemType::Resource, quantity),
        ResourceType::Stone => Item::resource(item_id, ItemType::Resource, quantity),
        ResourceType::Water => Item::resource(item_id, ItemType::Resource, quantity),
        ResourceType::Food => Item::resource(item_id, ItemType::Food, quantity),
        ResourceType::Berry => Item::resource(item_id, ItemType::Food, quantity),
    };
    
    Ok(item)
}

pub fn get_gatherable_resources(position: &Position, terrain: &Terrain) -> Vec<ResourceType> {
    let chunk_x = position.chunk_x;
    let chunk_y = position.chunk_y;
    
    if let Some(chunk) = terrain.get_chunk(chunk_x, chunk_y) {
        match chunk.biome {
            Biome::Grassland => vec![ResourceType::Berry, ResourceType::Wood],
            Biome::Forest => vec![ResourceType::Wood, ResourceType::Berry],
            Biome::Mountain => vec![ResourceType::Stone],
            Biome::Desert => vec![ResourceType::Stone],
            Biome::Water => vec![ResourceType::Water],
        }
    } else {
        vec![]
    }
}

fn get_gather_quantity(resource_type: &ResourceType, position: &Position, _terrain: &Terrain) -> u32 {
    // Deterministic quantity based on resource type and location
    let base_quantity = match resource_type {
        ResourceType::Wood => 3,
        ResourceType::Stone => 2,
        ResourceType::Water => 1,
        ResourceType::Food => 2,
        ResourceType::Berry => 4,
    };
    
    // Add location-based variation (deterministic)
    let variation = ((position.chunk_x + position.chunk_y) % 3) as u32;
    base_quantity + variation
}

fn generate_item_id(agent_id: u64, resource_type: ResourceType) -> u64 {
    // Generate deterministic item ID
    let type_seed = match resource_type {
        ResourceType::Wood => 1000,
        ResourceType::Stone => 2000,
        ResourceType::Water => 3000,
        ResourceType::Food => 4000,
        ResourceType::Berry => 5000,
    };
    
    agent_id * 10000 + type_seed
}
