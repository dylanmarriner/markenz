use std::collections::BTreeMap;
use std::collections::BTreeSet;
use rng::RngStream;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Biome {
    Grassland,
    Forest,
    Mountain,
    Desert,
    Water,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub biome: Biome,
    pub heights: Vec<u8>, // 256×256 heightmap as Vec for serialization
    pub entities: BTreeSet<u64>,
    pub assets: BTreeSet<u64>,
}

impl Chunk {
    pub fn new(x: i32, y: i32, biome: Biome) -> Self {
        Self {
            x,
            y,
            biome,
            heights: vec![0; 65536], // Initialize 256×256 heightmap
            entities: BTreeSet::new(),
            assets: BTreeSet::new(),
        }
    }

    pub fn height_at(&self, local_x: u8, local_y: u8) -> u8 {
        let index = usize::from(local_y) * 256 + usize::from(local_x);
        self.heights[index]
    }

    pub fn set_height(&mut self, local_x: u8, local_y: u8, height: u8) {
        let index = usize::from(local_y) * 256 + usize::from(local_x);
        self.heights[index] = height;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Terrain {
    pub chunks: BTreeMap<(i32, i32), Chunk>,
    pub biome_map: BTreeMap<(i32, i32), Biome>,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            chunks: BTreeMap::new(),
            biome_map: BTreeMap::new(),
        }
    }

    pub fn get_chunk(&self, chunk_x: i32, chunk_y: i32) -> Option<&Chunk> {
        self.chunks.get(&(chunk_x, chunk_y))
    }

    pub fn get_chunk_mut(&mut self, chunk_x: i32, chunk_y: i32) -> Option<&mut Chunk> {
        self.chunks.get_mut(&(chunk_x, chunk_y))
    }

    pub fn height_at_world(&self, world_x: i32, world_y: i32) -> u8 {
        let chunk_x = world_x.div_euclid(256);
        let chunk_y = world_y.div_euclid(256);
        let local_x = (world_x.rem_euclid(256)) as u8;
        let local_y = (world_y.rem_euclid(256)) as u8;

        if let Some(chunk) = self.get_chunk(chunk_x, chunk_y) {
            chunk.height_at(local_x, local_y)
        } else {
            0 // Default height for ungenerated chunks
        }
    }

    pub fn generate_chunk(&mut self, chunk_x: i32, chunk_y: i32, rng: &mut RngStream) -> Chunk {
        // Generate biome first
        let biome = crate::biome_generator::generate_biome(chunk_x, chunk_y, rng);
        
        // Store biome in biome map
        let _ = self.biome_map.insert((chunk_x, chunk_y), biome.clone());
        
        // Generate heightmap
        let heights = crate::biome_generator::generate_heightmap(chunk_x, chunk_y, &biome, rng);
        
        let mut chunk = Chunk::new(chunk_x, chunk_y, biome);
        chunk.heights = heights;
        
        // Store chunk
        let _ = self.chunks.insert((chunk_x, chunk_y), chunk.clone());
        
        chunk
    }
}
