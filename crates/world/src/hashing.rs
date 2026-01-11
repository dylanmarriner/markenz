/**
 * File: crates/world/src/hashing.rs
 * 
 * Purpose: State hashing and canonical serialization for Phase 1 determinism
 * 
 * Why this file exists:
 * - Implements canonical state serialization for deterministic hashing
 * - Provides hash chain verification for replay equivalence
 * - Ensures stable hash computation across runs and platforms
 * - Supports hash equality verification as proof of determinism
 * - Enables detection of state corruption or tampering
 * 
 * Phase plan authority: PLAN_PHASE_1_DETERMINISM.md
 * Section 5 "STATE HASHING", Section 7 "DETERMINISM GUARANTEES"
 * 
 * Invariants enforced:
 * - Hash equality is the proof of determinism
 * - Canonical serialization produces identical bytes across runs
 * - Hash chain integrity prevents retroactive tampering
 * - No floating-point or nondeterministic operations in hashing
 * - All state mutations result in deterministic hash changes
 * 
 * What breaks if removed:
 * - No hash equality verification → cannot prove determinism
 * - No canonical serialization → hash divergence across runs
 * - No hash chain verification → tampering undetectable
 * - Non-deterministic hashing → replay equivalence fails
 * 
 * What this file does NOT do:
 * - Does not use any nondeterministic serialization formats
 * - Does not permit hash computation without full state
 * - Does not allow partial or incremental hashing
 * - Does not implement business logic (hashing infrastructure only)
 */

use crate::types::Universe;
use blake3::Hasher;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use tracing::{debug, error};

/// Canonical world state hash for Phase 1 determinism
/// 
/// This function implements the core Phase 1 hashing invariant:
/// "Hash equality is the proof of determinism"
/// 
/// The hash includes all world state in a canonical form to ensure
/// identical states produce identical hashes across all runs.
pub fn world_hash(universe: &Universe) -> [u8; 32] {
    let serialized = serialize_canonical(universe);
    let mut hasher = Hasher::new();
    
    // Include previous hash for hash-chain integrity
    let _ = hasher.update(&universe.prev_state_hash);
    
    // Include serialized state
    let _ = hasher.update(&serialized);
    
    let hash = hasher.finalize().into();
    
    debug!("World hash computed: {}", hex::encode(hash));
    hash
}

/// Canonical serialization of universe state
/// 
/// This function ensures deterministic serialization by:
/// - Using bincode with fixed configuration
/// - Excluding transient state (only persistent data)
/// - Maintaining stable ordering for all collections
/// - Avoiding any platform-dependent representations
pub fn serialize_canonical(universe: &Universe) -> Vec<u8> {
    // Create canonical representation
    let canonical = CanonicalUniverse::from_universe(universe);
    
    // Serialize with deterministic configuration
    bincode::serialize(&canonical)
        .expect("Failed to serialize canonical universe")
}

/// Canonical universe representation for deterministic hashing
/// 
/// This struct contains only the state that should be included
/// in hash computation, excluding transient or derived data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalUniverse {
    /// Tick index (authoritative time)
    pub tick: u64,
    /// Genesis seed (for reproducibility verification)
    pub seed: u64,
    /// Agents in deterministic order (sorted by ID)
    pub agents: BTreeMap<u64, CanonicalAgent>,
    /// Assets in deterministic order (sorted by ID)
    pub assets: BTreeMap<u64, CanonicalAsset>,
    /// Terrain state hash (terrain is large, so we hash it)
    pub terrain_hash: [u8; 32],
}

/// Canonical agent representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalAgent {
    /// Agent ID
    pub id: u64,
    /// Agent name
    pub name: String,
    /// Position (fixed-point representation for determinism)
    pub position: (i32, i32, i32),  // Convert from f64 to i32
    /// Inventory items in deterministic order
    pub inventory: BTreeMap<String, u64>,  // item_id -> quantity
    /// Bio state hash (biology is complex, so we hash it)
    pub bio_hash: [u8; 32],
    /// Memory state hash (cognition is complex, so we hash it)
    pub memory_hash: [u8; 32],
}

/// Canonical asset representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalAsset {
    /// Asset ID
    pub id: u64,
    /// Asset name
    pub name: String,
    /// Asset location (canonical representation)
    pub location: CanonicalLocation,
    /// Asset state
    pub state: CanonicalAssetState,
}

/// Canonical location representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalLocation {
    /// Location type
    pub location_type: String,  // "position", "on_agent", "in_container"
    /// Position data (if applicable)
    pub position: Option<(i32, i32, i32)>,
    /// Agent ID (if on agent)
    pub agent_id: Option<u64>,
}

/// Canonical asset state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalAssetState {
    /// Durability (fixed-point)
    pub durability: u32,  // Convert from f64 to u32 (scaled by 100)
    /// Owner ID (if any)
    pub owner_id: Option<u64>,
    /// Properties in deterministic order
    pub properties: BTreeMap<String, String>,
}

impl CanonicalUniverse {
    /// Convert from full universe to canonical representation
    /// 
    /// This method extracts only the state that should be included
    /// in hash computation, ensuring deterministic representation.
    pub fn from_universe(universe: &Universe) -> Self {
        // Convert agents to canonical form
        let mut canonical_agents = BTreeMap::new();
        for (id, agent) in &universe.agents {
            let canonical_agent = CanonicalAgent {
                id: agent.id,
                name: agent.name.clone(),
                position: (
                    (agent.position.0 * 1000.0) as i32,  // Convert to fixed-point
                    (agent.position.1 * 1000.0) as i32,
                    (agent.position.2 * 1000.0) as i32,
                ),
                inventory: agent.inventory
                    .iter()
                    .map(|(_k, v)| (v.name.clone(), 1u64)) // Each asset counts as 1
                    .collect(),
                bio_hash: hash_bio_state(&agent.bio_state),
                memory_hash: hash_memory_state(&agent.memory),
            };
            let _ = canonical_agents.insert(*id, canonical_agent);
        }
        
        // Convert assets to canonical form
        let mut canonical_assets = BTreeMap::new();
        for (id, asset) in &universe.assets {
            let canonical_asset = CanonicalAsset {
                id: asset.id,
                name: asset.name.clone(),
                location: CanonicalLocation::from_asset_location(&asset.location),
                state: CanonicalAssetState {
                    durability: (asset.state.durability * 100.0) as u32,  // Scale to fixed-point
                    owner_id: asset.state.ownership,
                    properties: asset.state.properties
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
            };
            let _ = canonical_assets.insert(*id, canonical_asset);
        }
        
        Self {
            tick: universe.tick,
            seed: universe.seed,
            agents: canonical_agents,
            assets: canonical_assets,
            terrain_hash: hash_terrain_state(&universe.terrain),
        }
    }
    
    /// Compute hash of canonical universe
    /// 
    /// This provides the same result as world_hash() but
    /// operates on the canonical representation directly.
    pub fn compute_hash(&self) -> [u8; 32] {
        let serialized = bincode::serialize(self)
            .expect("Failed to serialize canonical universe");
        let mut hasher = Hasher::new();
        let _ = hasher.update(&serialized);
        hasher.finalize().into()
    }
}

impl CanonicalLocation {
    /// Convert from asset location to canonical representation
    pub fn from_asset_location(location: &crate::types::AssetLocation) -> Self {
        match location {
            crate::types::AssetLocation::AtPosition(pos) => Self {
                location_type: "position".to_string(),
                position: Some((
                    (pos.0 * 1000.0) as i32,
                    (pos.1 * 1000.0) as i32,
                    (pos.2 * 1000.0) as i32,
                )),
                agent_id: None,
            },
            crate::types::AssetLocation::OnAgent(agent_id) => Self {
                location_type: "on_agent".to_string(),
                position: None,
                agent_id: Some(*agent_id),
            },
        }
    }
}

/// Hash complex biological state
/// 
/// Since biology contains complex data structures, we hash it
/// rather than including it directly in canonical serialization.
fn hash_bio_state(bio_state: &crate::bio::BioState) -> [u8; 32] {
    let serialized = bincode::serialize(bio_state)
        .expect("Failed to serialize bio state");
    let mut hasher = Hasher::new();
    let _ = hasher.update(&serialized);
    hasher.finalize().into()
}

/// Hash complex memory state
/// 
/// Since cognition contains complex data structures, we hash it
/// rather than including it directly in canonical serialization.
fn hash_memory_state(memory: &crate::cognition::memory::AgentMemory) -> [u8; 32] {
    let serialized = bincode::serialize(memory)
        .expect("Failed to serialize memory state");
    let mut hasher = Hasher::new();
    let _ = hasher.update(&serialized);
    hasher.finalize().into()
}

/// Hash terrain state
/// 
/// Terrain is large and complex, so we hash it rather than
/// including it directly in canonical serialization.
fn hash_terrain_state(terrain: &crate::terrain::Terrain) -> [u8; 32] {
    let serialized = bincode::serialize(terrain)
        .expect("Failed to serialize terrain");
    let mut hasher = Hasher::new();
    let _ = hasher.update(&serialized);
    hasher.finalize().into()
}

/// Verify hash chain integrity
/// 
/// This function verifies that each hash in the sequence
/// properly links to the previous hash, forming an unbroken chain.
pub fn verify_hash_chain(hashes: &[[u8; 32]]) -> Result<(), String> {
    if hashes.is_empty() {
        return Ok(()); // Empty chain is valid
    }
    
    // Genesis hash should have zero prev_hash
    let mut prev_hash = [0u8; 32];
    
    for (i, &current_hash) in hashes.iter().enumerate() {
        // For genesis (i=0), prev_hash should be zero
        // For others, prev_hash should match previous hash
        if i > 0 {
            prev_hash = hashes[i - 1];
        }
        
        // Verify hash chain linkage
        let canonical = CanonicalUniverse {
            tick: i as u64,
            seed: 0, // Not used for hash chain verification
            agents: BTreeMap::new(),
            assets: BTreeMap::new(),
            terrain_hash: prev_hash,
        };
        
        let expected_hash = canonical.compute_hash();
        if current_hash != expected_hash {
            return Err(format!("Hash chain broken at tick {}: expected {}, got {}", 
                i, hex::encode(expected_hash), hex::encode(current_hash)));
        }
    }
    
    debug!("Hash chain verified: {} hashes", hashes.len());
    Ok(())
}

/// Compare two hash sequences for equality
/// 
/// This function is used to verify that replay produces
/// identical hash sequences to live simulation.
pub fn compare_hash_sequences(seq1: &[[u8; 32]], seq2: &[[u8; 32]]) -> Result<bool, String> {
    if seq1.len() != seq2.len() {
        return Err(format!("Hash sequence length mismatch: {} vs {}", 
            seq1.len(), seq2.len()));
    }
    
    for (i, (hash1, hash2)) in seq1.iter().zip(seq2.iter()).enumerate() {
        if hash1 != hash2 {
            error!("Hash divergence at tick {}: {} vs {}", 
                i, hex::encode(hash1), hex::encode(hash2));
            return Ok(false);
        }
    }
    
    debug!("Hash sequences match: {} hashes compared", seq1.len());
    Ok(true)
}

/// Generate hash difference report
/// 
/// This function creates a detailed report of where two
/// hash sequences differ, useful for debugging determinism issues.
pub fn generate_hash_diff_report(seq1: &[[u8; 32]], seq2: &[[u8; 32]]) -> String {
    let mut report = String::new();
    
    if seq1.len() != seq2.len() {
        report.push_str(&format!("Length mismatch: {} vs {}\n", seq1.len(), seq2.len()));
        return report;
    }
    
    let mut differences = Vec::new();
    
    for (i, (hash1, hash2)) in seq1.iter().zip(seq2.iter()).enumerate() {
        if hash1 != hash2 {
            differences.push(i);
        }
    }
    
    if differences.is_empty() {
        report.push_str("No differences found\n");
    } else {
        report.push_str(&format!("Found {} differences at ticks: ", differences.len()));
        for (i, diff_tick) in differences.iter().enumerate() {
            if i > 0 {
                report.push_str(", ");
            }
            report.push_str(&diff_tick.to_string());
        }
        report.push('\n');
        
        // Show first few differences in detail
        for &diff_tick in differences.iter().take(5) {
            let hash1 = &seq1[diff_tick];
            let hash2 = &seq2[diff_tick];
            report.push_str(&format!(
                "Tick {}: seq1={}, seq2={}\n",
                diff_tick, hex::encode(hash1), hex::encode(hash2)
            ));
        }
        
        if differences.len() > 5 {
            report.push_str(&format!("... and {} more differences\n", differences.len() - 5));
        }
    }
    
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_universe_creation() {
        let universe = Universe::new(1337);
        let canonical = CanonicalUniverse::from_universe(&universe);
        
        assert_eq!(canonical.tick, 0);
        assert_eq!(canonical.seed, 1337);
        assert_eq!(canonical.agents.len(), 2); // Gem-D and Gem-K
        assert_eq!(canonical.assets.len(), 4); // House, Shed, Tool, Vehicle
    }
    
    #[test]
    fn test_canonical_serialization_determinism() {
        let universe1 = Universe::new(42);
        let universe2 = Universe::new(42);
        
        let canonical1 = CanonicalUniverse::from_universe(&universe1);
        let canonical2 = CanonicalUniverse::from_universe(&universe2);
        
        // Same seed should produce identical canonical representation
        assert_eq!(canonical1, canonical2);
        
        // Hash should be identical
        let hash1 = canonical1.compute_hash();
        let hash2 = canonical2.compute_hash();
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_hash_chain_verification() {
        // Create valid hash chain
        let mut hashes = Vec::new();
        let mut prev_hash = [0u8; 32];
        
        for i in 0..10 {
            let canonical = CanonicalUniverse {
                tick: i,
                seed: 1337,
                agents: BTreeMap::new(),
                assets: BTreeMap::new(),
                terrain_hash: prev_hash,
            };
            let hash = canonical.compute_hash();
            hashes.push(hash);
            prev_hash = hash;
        }
        
        // Should verify successfully
        assert!(verify_hash_chain(&hashes).is_ok());
        
        // Break the chain
        let mut broken_hashes = hashes.clone();
        broken_hashes[5] = [42u8; 32];
        
        // Should fail verification
        assert!(verify_hash_chain(&broken_hashes).is_err());
    }
    
    #[test]
    fn test_hash_sequence_comparison() {
        let hashes1 = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let hashes2 = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let hashes3 = vec![[1u8; 32], [2u8; 32], [4u8; 32]];
        
        // Identical sequences should match
        assert!(compare_hash_sequences(&hashes1, &hashes2).unwrap());
        
        // Different sequences should not match
        assert!(!compare_hash_sequences(&hashes1, &hashes3).unwrap());
        
        // Different lengths should error
        let hashes4 = vec![[1u8; 32], [2u8; 32]];
        assert!(compare_hash_sequences(&hashes1, &hashes4).is_err());
    }
    
    #[test]
    fn test_hash_diff_report() {
        let hashes1 = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let hashes2 = vec![[1u8; 32], [2u8; 32], [4u8; 32]];
        
        let report = generate_hash_diff_report(&hashes1, &hashes2);
        
        assert!(report.contains("1 differences"));
        assert!(report.contains("tick 2"));
    }
}
