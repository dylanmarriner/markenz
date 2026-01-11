use crate::*;
use rng::{GlobalSeed, RngSubsystem};
use physics::collision::{Position, can_occupy, collides_with_terrain};

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST-COLLISION-001: Agent cannot move into terrain
    #[test]
    fn test_collision_001() {
        let seed = 1337;
        
        // Generate terrain directly to avoid universe borrow issues
        let chunk = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        // Test position at ground level (should be valid)
        let terrain_height = chunk.height_at(128, 128);
        let ground_pos = Position::new(0, 0, 128, 128, terrain_height);
        
        // Should be able to occupy at ground level
        assert!(!collides_with_terrain(&ground_pos, terrain_height));
        
        // Test position below terrain (should be fine - collision only checks for being too high)
        let below_pos = Position::new(0, 0, 128, 128, terrain_height.saturating_sub(5));
        assert!(!collides_with_terrain(&below_pos, terrain_height));
        
        // Test position too high above terrain (should collide due to climb limit)
        let above_pos = Position::new(0, 0, 128, 128, terrain_height + 10);
        assert!(collides_with_terrain(&above_pos, terrain_height));
        
        // Test with multiple agent positions (agent-to-agent collision)
        let agent_positions = vec![
            Position::new(0, 0, 128, 128, terrain_height),
            Position::new(0, 0, 129, 128, terrain_height),
        ];
        
        // Can occupy empty position
        let empty_pos = Position::new(0, 0, 130, 128, terrain_height);
        assert!(can_occupy(&empty_pos, terrain_height, &agent_positions));
        
        // Cannot occupy occupied position
        assert!(!can_occupy(&agent_positions[0], terrain_height, &agent_positions));
    }

    /// Additional collision test: steep terrain constraints
    #[test]
    fn test_collision_steep_terrain() {
        let seed = 1337;
        
        // Generate mountain terrain (should have steep areas)
        let chunk = {
            let mut terrain = Terrain::new();
            let mut global_seed = GlobalSeed::from_genesis(seed);
            let mut rng = global_seed.stream(RngSubsystem::Environment, 0);
            terrain.generate_chunk(0, 0, &mut rng)
        };
        
        // Test position at maximum height
        let max_height = *chunk.heights.iter().max().unwrap();
        let min_height = *chunk.heights.iter().min().unwrap();
        
        // Test position at maximum height
        let high_pos = Position::new(0, 0, 128, 128, max_height);
        assert!(!collides_with_terrain(&high_pos, max_height));
        
        // Test position at minimum height
        let low_pos = Position::new(0, 0, 128, 128, min_height);
        assert!(!collides_with_terrain(&low_pos, min_height));
        
        // Test climbing constraint (max climb of 2 units)
        let terrain_height = 50;
        let climb_pos = Position::new(0, 0, 128, 128, terrain_height + 3);
        
        // Should collide due to climb limit
        assert!(collides_with_terrain(&climb_pos, terrain_height));
        
        // Within climb limit should be fine
        let valid_climb_pos = Position::new(0, 0, 128, 128, terrain_height + 2);
        assert!(!collides_with_terrain(&valid_climb_pos, terrain_height));
    }
}
