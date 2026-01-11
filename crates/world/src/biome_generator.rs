use crate::terrain::Biome;
use rng::RngStream;

pub fn generate_biome(chunk_x: i32, chunk_y: i32, rng: &mut RngStream) -> Biome {
    // Use deterministic RNG to simulate Perlin-like noise for biome selection
    let base_noise = rng.next_u64();
    
    // Create clustering effect by using chunk coordinates in the calculation
    let coord_influence = ((chunk_x as u64).wrapping_mul(31).wrapping_add(chunk_y as u64 * 37)) % 100;
    let combined_value = (base_noise % 100).wrapping_add(coord_influence) % 100;
    
    // Biome selection based on combined value
    match combined_value {
        0..=19 => Biome::Grassland,
        20..=39 => Biome::Forest,
        40..=59 => Biome::Mountain,
        60..=79 => Biome::Desert,
        80..=99 => Biome::Water,
        _ => Biome::Grassland, // Fallback
    }
}

pub fn generate_heightmap(_chunk_x: i32, _chunk_y: i32, biome: &Biome, rng: &mut RngStream) -> Vec<u8> {
    let mut heights = vec![0u8; 65536];
    
    // Generate heightmap based on biome
    let (base_height, variation): (u8, u8) = match biome {
        Biome::Grassland => (30, 20),    // Smooth rolling hills (height 10–50)
        Biome::Forest => (40, 20),       // Moderate variation (height 20–60)
        Biome::Mountain => (125, 75),    // Steep peaks and valleys (height 50–200)
        Biome::Desert => (22, 18),       // Flat with dunes (height 5–40)
        Biome::Water => (5, 5),          // Low elevation (height 0–10)
    };
    
    // Generate heightmap using deterministic RNG
    for y in 0..256 {
        for x in 0..256 {
            let index = y * 256 + x;
            
            // Create smooth variation using multiple RNG calls
            let noise1 = rng.next_u64() % 100;
            let noise2 = rng.next_u64() % 100;
            let noise3 = rng.next_u64() % 100;
            
            // Combine noises for smoother terrain
            let combined_noise = (noise1 + noise2 * 2 + noise3) / 4;
            
            // Apply biome-specific height constraints
            let height = base_height.saturating_add(
                ((combined_noise as i32 - 50) * variation as i32 / 50) as u8
            );
            
            heights[index] = height.clamp(0, 255);
        }
    }
    
    heights
}
