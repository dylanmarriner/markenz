use markenz_world::Universe;
use rng::GlobalSeed;
use serde::{Deserialize, Serialize};

/// Snapshot version 1 for Phase 1 compatibility
/// 
/// This struct captures universe and RNG state at specific tick boundaries.
/// Used for deterministic replay and state recovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotV1 {
    /// Tick number when snapshot was taken
    pub tick: u64,
    /// Serialized universe state
    pub world_state: Vec<u8>,
    /// Serialized RNG state for deterministic continuation
    pub rng_state: Vec<u8>,
    /// World hash at snapshot time for verification
    pub world_hash: [u8; 32],
}

/// Write universe and RNG state to snapshot format
/// 
/// Serializes current universe and RNG state into deterministic snapshot format.
/// Used for creating replay checkpoints and state recovery.
pub fn snapshot_write(universe: &Universe, rng: &GlobalSeed, tick: u64) -> Vec<u8> {
    // Serialize universe state (includes world_hash)
    let world_state = bincode::serialize(universe)
        .expect("Failed to serialize universe");
    
    // Serialize RNG state
    let rng_state = bincode::serialize(rng)
        .expect("Failed to serialize RNG");
    
    let snapshot = SnapshotV1 {
        tick,
        world_state,
        rng_state,
        world_hash: universe.state_hash,
    };
    
    bincode::serialize(&snapshot)
        .expect("Failed to serialize snapshot")
}

/// Read universe and RNG state from snapshot format
/// 
/// Deserializes snapshot data back into universe and RNG state.
/// Validates snapshot format and returns reconstructed state.
pub fn snapshot_read(data: &[u8]) -> Result<(Universe, GlobalSeed), String> {
    let snapshot: SnapshotV1 = bincode::deserialize(data)
        .map_err(|e| format!("Failed to deserialize snapshot: {}", e))?;
    
    // Deserialize universe
    let universe: Universe = bincode::deserialize(&snapshot.world_state)
        .map_err(|e| format!("Failed to deserialize universe: {}", e))?;
    
    // Deserialize RNG
    let rng: GlobalSeed = bincode::deserialize(&snapshot.rng_state)
        .map_err(|e| format!("Failed to deserialize RNG: {}", e))?;
    
    Ok((universe, rng))
}
