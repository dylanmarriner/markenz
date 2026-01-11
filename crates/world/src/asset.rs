use crate::types::{Asset, AssetLocation};
use physics::Position;

impl Asset {
    pub fn get_position(&self) -> Position {
        match &self.location {
            AssetLocation::AtPosition((x, y, z)) => {
                let world_x = *x;
                let world_y = *y;
                let height = *z as u8;
                
                let chunk_x = world_x.div_euclid(256.0) as i32;
                let chunk_y = world_y.div_euclid(256.0) as i32;
                let local_x = (world_x.rem_euclid(256.0)) as u8;
                let local_y = (world_y.rem_euclid(256.0)) as u8;
                
                Position::new(chunk_x, chunk_y, local_x, local_y, height)
            }
            AssetLocation::OnAgent(_) => {
                // Assets on agents don't have world position
                Position::new(0, 0, 0, 0, 0)
            }
        }
    }

    pub fn is_movable(&self) -> bool {
        match self.name.as_str() {
            "Vehicle" => true,
            "House" | "Shed" | "Tool" => false,
            _ => false,
        }
    }
}
