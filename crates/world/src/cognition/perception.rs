use serde::{Deserialize, Serialize};
use crate::types::*;
use crate::terrain::Biome;
use crate::types::Universe;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perception {
    pub nearby_agents: Vec<(u64, f32)>,
    pub nearby_assets: Vec<(u64, String, f32)>,
    pub nearby_resources: Vec<(String, f32)>,
    pub terrain_biome: Biome,
    pub time_of_day: u32,
}

impl Perception {
    pub fn perceive(
        agent: &Agent,
        universe: &Universe,
    ) -> Result<Self, String> {
        let perception_range = 20.0;
        
        let mut nearby_agents = Vec::new();
        for (other_id, other) in &universe.agents {
            if other_id == &agent.id {
                continue;
            }
            let dist = Self::distance_to(agent.position, other.position);
            if dist < perception_range {
                nearby_agents.push((*other_id, dist));
            }
        }
        
        let mut nearby_assets = Vec::new();
        for (asset_id, asset) in &universe.assets {
            if let AssetLocation::AtPosition((x, y, z)) = asset.location {
                let dist = Self::distance_to(agent.position, (x, y, z));
                if dist < perception_range {
                    nearby_assets.push((*asset_id, asset.name.clone(), dist));
                }
            }
        }
        
        let world_x = agent.position.0 as i32;
        let world_y = agent.position.1 as i32;
        let chunk_x = world_x.div_euclid(256);
        let chunk_y = world_y.div_euclid(256);
        let biome = universe.terrain.biome_map.get(&(chunk_x, chunk_y))
            .cloned()
            .unwrap_or(Biome::Grassland);
        
        let nearby_resources = match biome {
            Biome::Forest => vec![("wood".to_string(), 1.0)],
            Biome::Grassland => vec![("food".to_string(), 1.0)],
            Biome::Mountain => vec![("stone".to_string(), 1.0)],
            Biome::Desert => vec![("sand".to_string(), 1.0)],
            Biome::Water => vec![("water".to_string(), 1.0)],
        };
        
        Ok(Self {
            nearby_agents,
            nearby_assets,
            nearby_resources,
            terrain_biome: biome,
            time_of_day: (universe.tick % 24000) as u32,
        })
    }
    
    fn distance_to(pos1: (f32, f32, f32), pos2: (f32, f32, f32)) -> f32 {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;
        let dz = pos1.2 - pos2.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
