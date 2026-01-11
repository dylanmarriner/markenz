/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic RNG stream
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use blake3::Hasher;

// Xorshift64* deterministic PRNG
struct Xorshift64Star {
    state: u64,
}

impl Xorshift64Star {
    fn new(seed: u64) -> Self {
        let mut state = seed;
        // Ensure non-zero state
        if state == 0 {
            state = 1;
        }
        Self { state }
    }
    
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }
    
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    
    fn next_f32(&mut self) -> f32 {
        const SCALE: f32 = 1.0 / (1u64 << 23) as f32;
        ((self.next_u64() >> 41) as u32 | 0x3f800000) as f32 - 1.0
    }
}

pub struct ChaosStream {
    rng: Xorshift64Star,
}

impl ChaosStream {
    pub fn new(seed: u64) -> Self {
        let rng = Xorshift64Star::new(seed);
        Self { rng }
    }
    
    pub fn from_global_seed(global_seed: u64, system_id: &str) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&global_seed.to_le_bytes());
        hasher.update(system_id.as_bytes());
        let system_seed = hasher.finalize();
        let seed = u64::from_le_bytes(system_seed.as_bytes()[..8].try_into().unwrap());
        Self { rng: Xorshift64Star::new(seed) }
    }
    
    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    pub fn next_f32(&mut self) -> f32 {
        self.rng.next_f32()
    }
    
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    
    pub fn range(&mut self, range: std::ops::Range<u32>) -> u32 {
        let range_size = range.end - range.start;
        range.start + (self.next_u32() % range_size)
    }
}

pub struct RngStream {
    stream: ChaosStream,
}

impl RngStream {
    pub fn new(seed: u64, stream_id: &str) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&seed.to_le_bytes());
        hasher.update(stream_id.as_bytes());
        let stream_seed = hasher.finalize();
        let seed_bytes: [u8; 32] = stream_seed.into();
        let seed = u64::from_le_bytes(seed_bytes[..8].try_into().unwrap());
        Self {
            stream: ChaosStream::new(seed),
        }
    }
    
    pub fn next_f32(&mut self) -> f32 {
        self.stream.next_f32()
    }
    
    pub fn next_u32(&mut self) -> u32 {
        self.stream.next_u32()
    }
    
    pub fn next_in_range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min + 1;
        min + (self.next_u32() % range)
    }
}
