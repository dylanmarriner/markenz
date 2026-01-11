/// Represents a position in the world using chunk-based coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    /// X coordinate of the chunk this position is in
    pub chunk_x: i32,
    /// Y coordinate of the chunk this position is in
    pub chunk_y: i32,
    /// Local X coordinate within the chunk (0-255)
    pub local_x: u8,
    /// Local Y coordinate within the chunk (0-255)
    pub local_y: u8,
    /// Height/altitude of the position
    pub height: u8,
}

impl Position {
    /// Creates a new Position with the given coordinates
    pub fn new(chunk_x: i32, chunk_y: i32, local_x: u8, local_y: u8, height: u8) -> Self {
        Self {
            chunk_x,
            chunk_y,
            local_x,
            local_y,
            height,
        }
    }

    /// Returns the absolute world X coordinate
    pub fn world_x(&self) -> i32 {
        self.chunk_x * 256 + i32::from(self.local_x)
    }

    /// Returns the absolute world Y coordinate
    pub fn world_y(&self) -> i32 {
        self.chunk_y * 256 + i32::from(self.local_y)
    }
}

/// Checks if an agent can occupy the given position considering terrain and other agents
pub fn can_occupy(position: &Position, terrain_height: u8, agent_positions: &[Position]) -> bool {
    // Check terrain collision
    if collides_with_terrain(position, terrain_height) {
        return false;
    }

    // Check agent collision
    for agent_pos in agent_positions {
        if agent_pos == position {
            return false;
        }
    }

    true
}

/// Checks if the position collides with terrain based on height difference
pub fn collides_with_terrain(position: &Position, terrain_height: u8) -> bool {
    // Agent can occupy if height difference is within max climb (2 units)
    position.height as i32 - terrain_height as i32 > 2
}
