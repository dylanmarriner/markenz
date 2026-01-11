use std::collections::BTreeMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use super::rng_stream::{RngStream, RngSubsystem};
use super::audit_log::RngAuditLog;

/// Global seed management for deterministic RNG
/// 
/// Centralizes all RNG streams and provides audit logging.
/// Ensures deterministic behavior across all subsystems.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSeed {
    seed: [u8; 32],
    streams: BTreeMap<(RngSubsystem, u64), RngStream>,
    audit_log: RngAuditLog,
}

impl Serialize for GlobalSeed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Only serialize the seed, streams will be reconstructed on deserialization
        serializer.serialize_bytes(&self.seed)
    }
}

impl<'de> Deserialize<'de> for GlobalSeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seed: [u8; 32] = Deserialize::deserialize(deserializer)?;
        Ok(GlobalSeed::from_seed_bytes(seed))
    }
}

impl GlobalSeed {
    /// Create GlobalSeed from genesis seed
    /// 
    /// # Arguments
    /// * `seed` - Genesis seed for the entire simulation
    /// 
    /// # Returns
    /// New GlobalSeed with derived 32-byte key
    pub fn from_genesis(seed: u64) -> Self {
        // Derive 32-byte key from genesis seed
        let mut hasher = blake3::Hasher::new();
        let _ = hasher.update(&seed.to_le_bytes());
        let derived_seed = *hasher.finalize().as_bytes();
        
        Self {
            seed: derived_seed,
            streams: BTreeMap::new(),
            audit_log: RngAuditLog::new(),
        }
    }
    
    fn from_seed_bytes(seed: [u8; 32]) -> Self {
        Self {
            seed,
            streams: BTreeMap::new(),
            audit_log: RngAuditLog::new(),
        }
    }
    
    /// Get or create stream for subsystem
    /// 
    /// Stream ID = 0 for primary stream, incrementing for secondary
    /// 
    /// # Arguments
    /// * `subsystem` - Subsystem identifier
    /// * `stream_id` - Unique stream ID within subsystem
    /// 
    /// # Returns
    /// Mutable reference to RNG stream
    pub fn stream(
        &mut self,
        subsystem: RngSubsystem,
        stream_id: u64,
    ) -> &mut RngStream {
        let key = (subsystem, stream_id);
        self.streams.entry(key).or_insert_with(|| {
            RngStream::new(self.seed, subsystem, stream_id)
        })
    }
    
    /// Get the 32-byte seed
    /// 
    /// Returns the derived seed used for all streams
    pub fn seed_bytes(&self) -> [u8; 32] {
        self.seed
    }
    
    /// Reset all streams
    /// 
    /// Clears all RNG streams and audit log.
    /// Useful for testing or restarting simulation.
    pub fn reset(&mut self) {
        self.streams.clear();
    }
    
    /// Get total number of streams
    /// 
    /// Returns count of active RNG streams
    pub fn stream_count(&self) -> usize {
        self.streams.len()
    }
    
    /// Check if stream exists
    /// 
    /// # Arguments
    /// * `subsystem` - Subsystem identifier
    /// * `stream_id` - Stream ID within subsystem
    /// 
    /// # Returns
    /// true if stream exists, false otherwise
    pub fn has_stream(&self, subsystem: RngSubsystem, stream_id: u64) -> bool {
        self.streams.contains_key(&(subsystem, stream_id))
    }
    
    /// Get reference to audit log
    /// 
    /// Returns read-only access to the audit log
    pub fn audit_log(&self) -> &RngAuditLog {
        &self.audit_log
    }
    
    /// Get mutable reference to audit log
    /// 
    /// Returns mutable access to the audit log
    pub fn audit_log_mut(&mut self) -> &mut RngAuditLog {
        &mut self.audit_log
    }
    
    /// Set current tick for audit logging
    /// 
    /// This method doesn't affect RNG generation but is used
    /// for audit trail correlation with world ticks.
    pub fn set_tick(&mut self, _tick: u64) {
        // GlobalSeed doesn't track ticks per-stream
        // This is a no-op for compatibility
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_global_seed_from_genesis() {
        let global1 = GlobalSeed::from_genesis(12345);
        let global2 = GlobalSeed::from_genesis(12345);
        let global3 = GlobalSeed::from_genesis(54321);
        
        // Same genesis seed should produce same derived seed
        assert_eq!(global1.seed_bytes(), global2.seed_bytes());
        
        // Different genesis seed should produce different derived seed
        assert_ne!(global1.seed_bytes(), global3.seed_bytes());
    }
    
    #[test]
    fn test_stream_creation() {
        let mut global = GlobalSeed::from_genesis(42);
        
        // First call should create stream
        let stream1 = global.stream(RngSubsystem::Physics, 0);
        assert_eq!(stream1.stream_id(), 0);
        assert_eq!(stream1.subsystem(), RngSubsystem::Physics);
        assert_eq!(global.stream_count(), 1);
        
        // Second call with same parameters should return existing stream
        let stream2 = global.stream(RngSubsystem::Physics, 0);
        assert_eq!(stream2.stream_id(), 0);
        assert_eq!(global.stream_count(), 1); // Still only one stream
        
        // Different stream_id should create new stream
        let stream3 = global.stream(RngSubsystem::Physics, 1);
        assert_eq!(stream3.stream_id(), 1);
        assert_eq!(global.stream_count(), 2);
    }
    
    #[test]
    fn test_stream_determinism() {
        let mut global1 = GlobalSeed::from_genesis(123);
        let mut global2 = GlobalSeed::from_genesis(123);
        
        let val1 = global1.stream(RngSubsystem::Biology, 0).next_u64();
        let val2 = global2.stream(RngSubsystem::Biology, 0).next_u64();
        
        assert_eq!(val1, val2, "Same global seed should produce same values");
    }
}
