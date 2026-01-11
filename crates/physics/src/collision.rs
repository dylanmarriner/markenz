#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub local_x: u8,
    pub local_y: u8,
    pub height: u8,
}

impl Position {
    pub fn new(chunk_x: i32, chunk_y: i32, local_x: u8, local_y: u8, height: u8) -> Self {
        Self {
            chunk_x,
            chunk_y,
            local_x,
            local_y,
            height,
        }
    }

    pub fn world_x(&self) -> i32 {
        self.chunk_x * 256 + i32::from(self.local_x)
    }

    pub fn world_y(&self) -> i32 {
        self.chunk_y * 256 + i32::from(self.local_y)
    }
}

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

pub fn collides_with_terrain(position: &Position, terrain_height: u8) -> bool {
    // Agent can occupy if height difference is within max climb (2 units)
    position.height as i32 - terrain_height as i32 > 2
}
