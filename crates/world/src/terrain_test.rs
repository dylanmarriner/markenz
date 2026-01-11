use crate::*;
use rng::{GlobalSeed, RngSubsystem};

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST-TERRAIN-DETERMINISM-001: Same seed → identical heightmap and biomes
    #[test]
    fn test_terrain_determinism_001() {
        let seed = 1337;
        
        // Test direct terrain generation without universe borrow issues
        let chunk1 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        let chunk2 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        // Assert chunks are identical
        assert_eq!(chunk1.x, chunk2.x);
        assert_eq!(chunk1.y, chunk2.y);
        assert_eq!(chunk1.biome, chunk2.biome);
        assert_eq!(chunk1.heights, chunk2.heights);
        
        // Test different coordinates produce different but deterministic results
        let chunk3 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(1, 0, &mut rng)
        };
        
        let chunk4 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(1, 0, &mut rng)
        };
        
        assert_eq!(chunk3.x, chunk4.x);
        assert_eq!(chunk3.y, chunk4.y);
        assert_eq!(chunk3.biome, chunk4.biome);
        assert_eq!(chunk3.heights, chunk4.heights);
        
        // Ensure (0,0) and (1,0) are different
        assert_ne!(chunk1.heights, chunk3.heights);
    }

    /// TEST-TERRAIN-HASH-001: Terrain hash stable across runs
    #[test]
    fn test_terrain_hash_001() {
        let seed = 1337;
        
        // Generate universe and compute hash - direct terrain approach
        let hash1 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            let _ = terrain.generate_chunk(0, 0, &mut rng);
            
            // Create universe with terrain for hash computation
            let mut universe = Universe::new(seed);
            universe.terrain = terrain;
            hashing::world_hash(&universe)
        };
        
        let hash2 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            let _ = terrain.generate_chunk(0, 0, &mut rng);
            
            // Create universe with terrain for hash computation
            let mut universe = Universe::new(seed);
            universe.terrain = terrain;
            hashing::world_hash(&universe)
        };
        
        // Hashes must be identical
        assert_eq!(hash1, hash2, "Terrain hash must be deterministic across runs");
        
        // Verify hash is not all zeros (indicates real computation)
        assert_ne!(hash1, [0u8; 32], "Hash should not be all zeros");
    }

    /// TEST-BIOME-DETERMINISM-001: Biomes are deterministic and match height patterns
    #[test]
    fn test_biome_determinism_001() {
        let seed = 1337;
        
        // Generate test chunks - direct terrain generation
        let chunk1 = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        let chunk1_b = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        // Biomes must match exactly
        assert_eq!(chunk1.biome, chunk1_b.biome);
        
        // Test biome height patterns - only test if we have the expected biome
        let avg_height: u32 = chunk1.heights.iter().map(|&h| h as u32).sum::<u32>() / chunk1.heights.len() as u32;
        
        match chunk1.biome {
            Biome::Water => {
                assert!(avg_height < 100, "Water biomes should have low average height, got {}", avg_height);
            },
            Biome::Mountain => {
                assert!(avg_height > 80, "Mountain biomes should have high average height, got {}", avg_height);
            },
            Biome::Desert => {
                assert!(avg_height < 80, "Desert biomes should have low average height, got {}", avg_height);
            },
            Biome::Forest | Biome::Grassland => {
                // Forest and grassland can have moderate heights
                assert!(avg_height >= 20 && avg_height <= 120, "Forest/Grassland biomes should have moderate height, got {}", avg_height);
            }
        }
    }
}
