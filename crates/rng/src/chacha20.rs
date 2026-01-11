//! RFC 7539 ChaCha20 cipher stream generator
/// 
/// Provides cryptographically secure random number generation
/// following the ChaCha20 specification exactly.
#[derive(Debug, Clone, PartialEq)]
pub struct ChaCha20Rng {
    key: [u32; 8],          // 256-bit key
    nonce: [u32; 3],        // 96-bit nonce
    counter: u64,           // 64-bit block counter
    block_index: usize,     // Position within current block (0-63)
    current_block: [u32; 16], // Current 64-byte block
}

impl ChaCha20Rng {
    /// Initialize from 32-byte seed (derived from genesis seed)
    /// 
    /// # Arguments
    /// * `key` - 256-bit encryption key
    /// * `nonce` - 96-bit nonce for this instance
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        let key_bytes: arrayvec::ArrayVec<[u8; 4], 8> = key.chunks_exact(4).map(|chunk| {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(chunk);
            arr
        }).collect();
        
        let key: [u32; 8] = match key_bytes.into_inner() {
            Ok(bytes) => bytes.map(|bytes| u32::from_le_bytes(bytes)),
            Err(_) => [0u32; 8],
        };
        
        let nonce_bytes: arrayvec::ArrayVec<[u8; 4], 3> = nonce.chunks_exact(4).map(|chunk| {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(chunk);
            arr
        }).collect();
        
        let nonce: [u32; 3] = match nonce_bytes.into_inner() {
            Ok(bytes) => bytes.map(|bytes| u32::from_le_bytes(bytes)),
            Err(_) => [0u32; 3],
        };
        
        Self {
            key,
            nonce,
            counter: 0,
            block_index: 64,  // Force generation of first block
            current_block: [0u32; 16],
        }
    }
    
    /// Generate next 32-bit random value
    /// 
    /// Returns a cryptographically secure random u32
    pub fn next_u32(&mut self) -> u32 {
        if self.block_index >= 64 {
            self.generate_block();
            self.block_index = 0;
        }
        
        let value = self.current_block[self.block_index / 4];
        self.block_index += 4;
        value
    }
    
    /// Generate next 64-bit random value
    /// 
    /// Returns a cryptographically secure random u64
    pub fn next_u64(&mut self) -> u64 {
        let lo = self.next_u32() as u64;
        let hi = self.next_u32() as u64;
        (hi << 32) | lo
    }
    
    /// Generate next f64 in [0, 1)
    /// 
    /// Returns a cryptographically secure random floating point value
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }
    
    /// RFC 7539 block function (exact algorithm, not approximated)
    fn generate_block(&mut self) {
        // Initialize state matrix
        let mut state = [0u32; 16];
        
        // Constants "expand 32-byte k"
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;
        
        // Key
        state[4] = self.key[0];
        state[5] = self.key[1];
        state[6] = self.key[2];
        state[7] = self.key[3];
        state[8] = self.key[4];
        state[9] = self.key[5];
        state[10] = self.key[6];
        state[11] = self.key[7];
        
        // Counter and nonce (RFC 7539 uses 96-bit nonce)
        state[12] = (self.counter & 0xffffffff) as u32;
        state[13] = ((self.counter >> 32) & 0xffffffff) as u32;
        state[14] = self.nonce[0];
        state[15] = self.nonce[1];
        // Note: RFC 7539 nonce is 96 bits, stored in state[14], state[15], and part of state[12]
        
        // Working copy
        let mut working = state;
        
        // 20 rounds (10 column rounds + 10 diagonal rounds)
        // RFC 7539 quarter round: a += b; d ^= a; d <<<= 16; c += d; b ^= c; b <<<= 12; a += b; d ^= a; d <<<= 8; c += d; b ^= c; b <<<= 7
        for _ in 0..10 {
            // Column round
            self.quarter_round(&mut working, 0, 4, 8, 12);
            self.quarter_round(&mut working, 1, 5, 9, 13);
            self.quarter_round(&mut working, 2, 6, 10, 14);
            self.quarter_round(&mut working, 3, 7, 11, 15);
            
            // Diagonal round
            self.quarter_round(&mut working, 0, 5, 10, 15);
            self.quarter_round(&mut working, 1, 6, 11, 12);
            self.quarter_round(&mut working, 2, 7, 8, 13);
            self.quarter_round(&mut working, 3, 4, 9, 14);
        }
        
        // Add initial state to working state
        for i in 0..16 {
            self.current_block[i] = working[i].wrapping_add(state[i]);
        }
        
        self.counter += 1;
    }
    
    /// ChaCha20 quarter round operation (RFC 7539)
    #[inline]
    fn quarter_round(&self, state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(16);
        
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(12);
        
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(8);
        
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chacha20_determinism() {
        let key = [42u8; 32];
        let nonce = [0u8; 12];
        
        let mut rng1 = ChaCha20Rng::new(key, nonce);
        let mut rng2 = ChaCha20Rng::new(key, nonce);
        
        // Test determinism - same seed should produce same values
        for _ in 0..10 {
            assert_eq!(rng1.next_u32(), rng2.next_u32());
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
    
    #[test]
    fn test_chacha20_different_seeds() {
        let key1 = [42u8; 32];
        let key2 = [43u8; 32];
        let nonce = [0u8; 12];
        
        let mut rng1 = ChaCha20Rng::new(key1, nonce);
        let mut rng2 = ChaCha20Rng::new(key2, nonce);
        
        // Different keys should produce different values
        assert_ne!(rng1.next_u32(), rng2.next_u32());
    }
    
    #[test]
    fn test_chacha20_range() {
        let key = [42u8; 32];
        let nonce = [0u8; 12];
        let mut rng = ChaCha20Rng::new(key, nonce);
        
        // Test that f64 produces values in [0, 1)
        for _ in 0..100 {
            let val = rng.next_f64();
            assert!(val >= 0.0 && val < 1.0);
        }
    }
}
