use markenz_world::Universe;
use std::path::Path;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct SnapshotMeta {
    pub tick: u64,
    pub path: String,
    pub size_bytes: usize,
}

pub fn write_snapshot(universe: &Universe, tick: u64, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create directory if it doesn't exist
    let dir = Path::new(path).parent().unwrap();
    fs::create_dir_all(dir)?;
    
    // Serialize universe state
    let universe_bytes = bincode::serialize(universe)?;
    
    // Compute checksum before moving universe_bytes
    let checksum: [u8; 32] = blake3::hash(&universe_bytes).into();
    
    // Create snapshot with header
    let snapshot = SnapshotV1 {
        version: 1,
        tick,
        world_state: universe_bytes,
        checksum,
    };
    
    // Serialize snapshot
    let snapshot_bytes = bincode::serialize(&snapshot)?;
    
    // Write to file
    let filename = format!("snapshot_{:010}.bin", tick);
    let full_path = Path::new(path).join(filename);
    let mut file = fs::File::create(&full_path)?;
    file.write_all(&snapshot_bytes)?;
    
    Ok(())
}

pub fn read_snapshot(path: &str) -> Result<Universe, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    
    // Deserialize snapshot
    let snapshot: SnapshotV1 = bincode::deserialize(&bytes)?;
    
    // Verify checksum
    let computed_checksum: [u8; 32] = blake3::hash(&snapshot.world_state).into();
    if computed_checksum != snapshot.checksum {
        return Err("Snapshot checksum verification failed".into());
    }
    
    // Deserialize universe
    let universe: Universe = bincode::deserialize(&snapshot.world_state)?;
    
    Ok(universe)
}

pub fn list_snapshots(dir: &str) -> Vec<SnapshotMeta> {
    let mut snapshots = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("snapshot_") && filename.ends_with(".bin") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Some(tick_str) = filename.strip_prefix("snapshot_").and_then(|s| s.strip_suffix(".bin")) {
                            if let Ok(tick) = tick_str.parse::<u64>() {
                                snapshots.push(SnapshotMeta {
                                    tick,
                                    path: path.to_string_lossy().to_string(),
                                    size_bytes: metadata.len() as usize,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    snapshots.sort_by_key(|s| s.tick);
    snapshots
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SnapshotV1 {
    version: u8,
    tick: u64,
    world_state: Vec<u8>,
    checksum: [u8; 32],
}
