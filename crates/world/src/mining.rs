use crate::types::Agent;
use crate::terrain::{Terrain, Biome};
use crate::action::MineType;
use crate::inventory::{Item, ItemType};
use physics::Position;

pub fn mine(agent: &Agent, mine_type: MineType, terrain: &Terrain) -> Result<Item, String> {
    let position = agent.position();
    
    if !can_mine_at(&position, terrain, mine_type) {
        return Err("Cannot mine at this location".to_string());
    }
    
    // Check if agent has pickaxe
    let has_pickaxe = agent.inventory.values().any(|asset| {
        asset.name == "Tool"
    });
    if !has_pickaxe {
        return Err("No pickaxe available".to_string());
    }
    
    // Generate ore item with deterministic quantity
    let quantity = get_mine_quantity(mine_type, &position);
    let item_id = generate_mine_item_id(agent.id, mine_type);
    
    let item = match mine_type {
        MineType::Coal => Item::resource(item_id, ItemType::Resource, quantity),
        MineType::Metal => Item::resource(item_id, ItemType::Resource, quantity),
        MineType::Gem => Item::resource(item_id, ItemType::Resource, quantity),
    };
    
    Ok(item)
}

pub fn can_mine_at(position: &Position, terrain: &Terrain, mine_type: MineType) -> bool {
    let chunk_x = position.chunk_x;
    let chunk_y = position.chunk_y;
    
    // Check if chunk exists
    let chunk = match terrain.get_chunk(chunk_x, chunk_y) {
        Some(chunk) => chunk,
        None => return false,
    };
    
    // Must be in mountain biome
    if !matches!(chunk.biome, Biome::Mountain) {
        return false;
    }
    
    // Check if specific ore type is available at this location
    is_ore_available_at(mine_type, position, terrain)
}

fn get_mine_quantity(mine_type: MineType, position: &Position) -> u32 {
    // Base quantity by ore type
    let base_quantity = match mine_type {
        MineType::Coal => 4,
        MineType::Metal => 2,
        MineType::Gem => 1,
    };
    
    // Add location-based variation (deterministic)
    let variation = ((position.chunk_x * position.chunk_y) % 3) as u32;
    base_quantity + variation
}

fn generate_mine_item_id(agent_id: u64, mine_type: MineType) -> u64 {
    // Generate deterministic item ID
    let type_seed = match mine_type {
        MineType::Coal => 6000,
        MineType::Metal => 7000,
        MineType::Gem => 8000,
    };
    
    agent_id * 10000 + type_seed
}

fn is_ore_available_at(mine_type: MineType, position: &Position, _terrain: &Terrain) -> bool {
    // Deterministic ore availability based on position
    let chunk_x = position.chunk_x;
    let chunk_y = position.chunk_y;
    
    // Use chunk coordinates to determine ore availability
    let position_hash = ((chunk_x.abs() as u64).wrapping_mul(31).wrapping_add(chunk_y.abs() as u64 * 37)) % 100;
    
    match mine_type {
        MineType::Coal => position_hash < 40, // 40% chance
        MineType::Metal => position_hash < 25, // 25% chance
        MineType::Gem => position_hash < 10,   // 10% chance
    }
}
