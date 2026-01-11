use crate::types::Agent;
use crate::terrain::{Terrain, Biome};
use crate::action::BuildingType;
use crate::inventory::ItemType;
use physics::Position;

pub fn build(agent: &Agent, building_type: BuildingType, terrain: &mut Terrain) -> Result<(), String> {
    let position = agent.position();
    
    if !can_build_at(&position, terrain, building_type) {
        return Err("Cannot build at this location".to_string());
    }
    
    // Check if agent has required materials
    let required_materials = get_required_materials(&building_type);
    for (material_type, required_qty) in &required_materials {
        let available_qty = agent.inventory.values()
            .filter(|asset| asset.name == "Resource")
            .map(|_asset| 1) // Each asset counts as 1 unit
            .sum::<u32>();
        if available_qty < *required_qty {
            return Err(format!("Insufficient {:?}: need {}, have {}", material_type, required_qty, available_qty));
        }
    }
    
    // Apply building effect to terrain
    apply_building_effect(&position, terrain, building_type);
    
    Ok(())
}

pub fn can_build_at(position: &Position, terrain: &Terrain, building_type: BuildingType) -> bool {
    let chunk_x = position.chunk_x;
    let chunk_y = position.chunk_y;
    
    // Check if chunk exists
    let chunk = match terrain.get_chunk(chunk_x, chunk_y) {
        Some(chunk) => chunk,
        None => return false,
    };
    
    // Check biome restrictions
    let biome_allowed = match chunk.biome {
        Biome::Water => false, // Cannot build on water
        Biome::Mountain => {
            // Can only build certain structures on mountains
            match building_type {
                BuildingType::Wall => true,
                BuildingType::Door | BuildingType::Floor => false,
            }
        },
        _ => true, // Can build on other biomes
    };
    
    if !biome_allowed {
        return false;
    }
    
    // Check terrain slope
    let current_height = chunk.height_at(position.local_x, position.local_y);
    let adjacent_heights = get_adjacent_heights(position, terrain);
    
    for adjacent_height in adjacent_heights {
        if (current_height as i32 - adjacent_height as i32).abs() > 3 {
            return false; // Too steep
        }
    }
    
    true
}

fn get_required_materials(building_type: &BuildingType) -> std::collections::BTreeMap<ItemType, u32> {
    match building_type {
        BuildingType::Wall => {
            let mut materials = std::collections::BTreeMap::new();
            let _ = materials.insert(ItemType::Resource, 5); // 5 stone
            materials
        },
        BuildingType::Door => {
            let mut materials = std::collections::BTreeMap::new();
            let _ = materials.insert(ItemType::Resource, 3); // 3 wood
            materials
        },
        BuildingType::Floor => {
            let mut materials = std::collections::BTreeMap::new();
            let _ = materials.insert(ItemType::Resource, 2); // 2 wood
            materials
        },
    }
}

fn apply_building_effect(position: &Position, terrain: &mut Terrain, building_type: BuildingType) {
    let chunk_x = position.chunk_x;
    let chunk_y = position.chunk_y;
    
    if let Some(chunk) = terrain.get_chunk_mut(chunk_x, chunk_y) {
        match building_type {
            BuildingType::Wall => {
                // Increase height to represent wall
                let current_height = chunk.height_at(position.local_x, position.local_y);
                let new_height = (current_height + 3).min(255);
                chunk.set_height(position.local_x, position.local_y, new_height);
            },
            BuildingType::Floor => {
                // Flatten terrain for floor
                chunk.set_height(position.local_x, position.local_y, 20);
            },
            BuildingType::Door => {
                // Create doorway (lower height)
                let current_height = chunk.height_at(position.local_x, position.local_y);
                let new_height = current_height.saturating_sub(1);
                chunk.set_height(position.local_x, position.local_y, new_height);
            },
        }
    }
}

fn get_adjacent_heights(position: &Position, terrain: &Terrain) -> Vec<u8> {
    let mut heights = Vec::new();
    
    // Check adjacent cells (north, south, east, west)
    let adjacent_positions = [
        (position.local_x.saturating_sub(1), position.local_y),
        (position.local_x, position.local_y.saturating_sub(1)),
        (position.local_x.saturating_add(1), position.local_y),
        (position.local_x, position.local_y.saturating_add(1)),
    ];
    
    for (local_x, local_y) in adjacent_positions {
        if let Some(chunk) = terrain.get_chunk(position.chunk_x, position.chunk_y) {
            heights.push(chunk.height_at(local_x, local_y));
        }
    }
    
    heights
}
