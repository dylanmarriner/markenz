use serde::{Serialize, Deserialize};
use super::chacha20::ChaCha20Rng;

/// Subsystem identifier for RNG isolation
/// 
/// Each subsystem gets its own isolated RNG stream to prevent
/// cross-contamination and ensure deterministic replay.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RngSubsystem {
    /// Physics subsystem (movement, collision, forces)
    Physics = 0,
    /// Biology subsystem (metabolism, hormones, immune response)
    Biology = 1,
    /// Cognition subsystem (perception, planning, decision making)
    Cognition = 2,
    /// Genetics subsystem (inheritance, mutation, recombination)
    Genetics = 3,
    /// Governance subsystem (policy enforcement, voting)
    Governance = 4,
    /// Environment subsystem (weather, resource distribution)
    Environment = 5,
}

impl RngSubsystem {
    /// Get string representation of subsystem
    /// 
    /// Returns human-readable name for logging and debugging
    pub fn as_str(&self) -> &'static str {
        match self {
            RngSubsystem::Physics => "Physics",
            RngSubsystem::Biology => "Biology",
            RngSubsystem::Cognition => "Cognition",
            RngSubsystem::Genetics => "Genetics",
            RngSubsystem::Governance => "Governance",
            RngSubsystem::Environment => "Environment",
        }
    }
}

/// Isolated RNG stream per subsystem
/// 
/// Each stream maintains its own state and nonce to ensure
/// isolation between different subsystems.
#[derive(Debug, Clone, PartialEq)]
pub struct RngStream {
    subsystem: RngSubsystem,
    stream_id: u64,        // Stream identifier within subsystem
    rng: ChaCha20Rng,
}

impl RngStream {
    /// Create new stream for subsystem
    /// 
    /// # Arguments
    /// * `global_seed` - 32-byte global seed derived from genesis
    /// * `subsystem` - Subsystem identifier for stream isolation
    /// * `stream_id` - Unique ID within subsystem (0 for primary)
    /// 
    /// # Returns
    /// New isolated RNG stream
    /// 
    /// # Implementation
    /// Nonce = blake3(global_seed || subsystem_id || stream_id) first 12 bytes
    pub fn new(
        global_seed: [u8; 32],
        subsystem: RngSubsystem,
        stream_id: u64,
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        let _ = hasher.update(&global_seed);
        let _ = hasher.update(&(subsystem as u64).to_le_bytes());
        let _ = hasher.update(&stream_id.to_le_bytes());
        let nonce_bytes = hasher.finalize();
        
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&nonce_bytes.as_bytes()[0..12]);
        
        Self {
            subsystem,
            stream_id,
            rng: ChaCha20Rng::new(global_seed, nonce),
        }
    }
    
    /// Generate next 32-bit random value
    /// 
    /// Returns cryptographically secure random u32 from this stream
    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    /// Generate next 64-bit random value
    /// 
    /// Returns cryptographically secure random u64 from this stream
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    
    /// Generate next f64 in [0, 1)
    /// 
    /// Returns cryptographically secure random f64 from this stream
    pub fn next_f64(&mut self) -> f64 {
        self.rng.next_f64()
    }
    
    /// Get the subsystem identifier for this stream
    /// 
    /// Returns which subsystem this stream belongs to
    pub fn subsystem(&self) -> RngSubsystem {
        self.subsystem
    }
    
    /// Get the stream ID within the subsystem
    /// 
    /// Returns unique identifier for this stream within its subsystem
    pub fn stream_id(&self) -> u64 {
        self.stream_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rng_stream_determinism() {
        let seed = [42u8; 32];
        
        let mut stream1 = RngStream::new(seed, RngSubsystem::Physics, 0);
        let mut stream2 = RngStream::new(seed, RngSubsystem::Physics, 0);
        
        // Same seed, same subsystem, same stream_id should produce same values
        assert_eq!(stream1.next_u64(), stream2.next_u64());
        assert_eq!(stream1.next_u64(), stream2.next_u64());
    }
    
    #[test]
    fn test_different_streams_different_values() {
        let seed = [42u8; 32];
        
        let mut physics_stream = RngStream::new(seed, RngSubsystem::Physics, 0);
        let mut biology_stream = RngStream::new(seed, RngSubsystem::Biology, 0);
        
        // Different subsystems should produce different values
        assert_ne!(physics_stream.next_u64(), biology_stream.next_u64());
    }
    
    #[test]
    fn test_different_stream_ids_different_values() {
        let seed = [42u8; 32];
        
        let mut stream1 = RngStream::new(seed, RngSubsystem::Physics, 0);
        let mut stream2 = RngStream::new(seed, RngSubsystem::Physics, 1);
        
        // Same subsystem, different stream_id should produce different values
        assert_ne!(stream1.next_u64(), stream2.next_u64());
    }
}
