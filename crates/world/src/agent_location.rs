use crate::types::Agent;
use physics::Position;
use blake3;
use bincode;

impl Agent {
    pub fn position(&self) -> Position {
        // Convert from (f32, f32, f32) to Position
        let world_x = self.position.0;
        let world_y = self.position.1;
        let height = self.position.2 as u8;
        
        let chunk_x = (world_x as i32).div_euclid(256);
        let chunk_y = (world_y as i32).div_euclid(256);
        let local_x = (world_x as i32).rem_euclid(256) as u8;
        let local_y = (world_y as i32).rem_euclid(256) as u8;
        
        Position::new(chunk_x, chunk_y, local_x, local_y, height)
    }
    
    pub fn move_to(&mut self, new_position: Position) -> Result<(), String> {
        // Update the agent's position
        self.position = (
            new_position.world_x() as f32,
            new_position.world_y() as f32,
            new_position.height as f32
        );
        
        // Update state hash to reflect position change
        // Serialize agent state and compute new hash
        let _temp_hash = self.state_hash;
        self.state_hash = [0u8; 32]; // Clear hash during computation
        
        let state_bytes = bincode::serialize(self)
            .expect("Failed to serialize agent for hash computation");
        
        let mut hasher = blake3::Hasher::new();
        let _ = hasher.update(&self.name.as_bytes());
        let _ = hasher.update(&state_bytes);
        self.state_hash = hasher.finalize().into();
        
        Ok(())
    }
}
