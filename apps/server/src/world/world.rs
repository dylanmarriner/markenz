/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: World manager for voxel chunks
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use crate::sim::ChaosStream;
use crate::world::Chunk;
use deterministic::DeterministicMap;

pub struct World {
    chunks: DeterministicMap<(i32, i32, i32), Chunk>,
    chunk_size: usize,
}

impl World {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunks: DeterministicMap::new(),
            chunk_size,
        }
    }
    
    pub fn generate_chunk(&mut self, x: i32, y: i32, z: i32, _rng: &mut ChaosStream) -> Chunk {
        let size = self.chunk_size;
        let mut data = Vec::with_capacity(size * size * size);
        
        // Simple flat/debug world generation per M1 requirements
        for cy in 0..size {
            for _cz in 0..size {
                for _cx in 0..size {
                    let voxel_type = if cy == 0 {
                        // Bedrock layer
                        1
                    } else if cy < 4 {
                        // Dirt layer
                        2
                    } else if cy == 4 {
                        // Grass layer
                        3
                    } else {
                        // Air
                        0
                    };
                    
                    data.push(voxel_type);
                }
            }
        }
        
        let chunk = Chunk { x, y, z, data };
        self.chunks.insert((x, y, z), chunk.clone());
        chunk
    }
    
    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Option<&Chunk> {
        self.chunks.get(&(x, y, z))
    }
    
    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> Option<u16> {
        let chunk_x = x.div_euclid(self.chunk_size as i32);
        let chunk_y = y.div_euclid(self.chunk_size as i32);
        let chunk_z = z.div_euclid(self.chunk_size as i32);
        
        if let Some(chunk) = self.get_chunk(chunk_x, chunk_y, chunk_z) {
            let local_x = x.rem_euclid(self.chunk_size as i32) as usize;
            let local_y = y.rem_euclid(self.chunk_size as i32) as usize;
            let local_z = z.rem_euclid(self.chunk_size as i32) as usize;
            
            let index = local_y * self.chunk_size * self.chunk_size + local_z * self.chunk_size + local_x;
            chunk.data.get(index).copied()
        } else {
            None
        }
    }
    
    pub fn set_voxel(&mut self, x: i32, y: i32, z: i32, voxel_type: u16) -> bool {
        let chunk_x = x.div_euclid(self.chunk_size as i32);
        let chunk_y = y.div_euclid(self.chunk_size as i32);
        let chunk_z = z.div_euclid(self.chunk_size as i32);
        
        if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_y, chunk_z)) {
            let local_x = x.rem_euclid(self.chunk_size as i32) as usize;
            let local_y = y.rem_euclid(self.chunk_size as i32) as usize;
            let local_z = z.rem_euclid(self.chunk_size as i32) as usize;
            
            let index = local_y * self.chunk_size * self.chunk_size + local_z * self.chunk_size + local_x;
            if let Some(voxel) = chunk.data.get_mut(index) {
                *voxel = voxel_type;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    pub fn get_chunk_data_compressed(&self, x: i32, y: i32, z: i32) -> Option<Vec<u8>> {
        if let Some(chunk) = self.get_chunk(x, y, z) {
            // Simple run-length encoding for M1
            let mut compressed = Vec::new();
            let mut current = chunk.data.first().copied().unwrap_or(0);
            let mut count = 0u16;
            
            for &voxel in &chunk.data {
                if voxel == current && count < u16::MAX {
                    count += 1;
                } else {
                    compressed.extend_from_slice(&current.to_le_bytes());
                    compressed.extend_from_slice(&count.to_le_bytes());
                    current = voxel;
                    count = 1;
                }
            }
            
            // Add the final run
            compressed.extend_from_slice(&current.to_le_bytes());
            compressed.extend_from_slice(&count.to_le_bytes());
            
            Some(compressed)
        } else {
            None
        }
    }
}

