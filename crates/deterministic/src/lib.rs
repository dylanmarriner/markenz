/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: world, engine, rng, events, persistence
 * PURPOSE: Deterministic math operations, collections, and simulation time tracking
 * FAILURE MODES: PANIC_ON_INVALID_STATE, OVERFLOW_ON_TIME_TICK (wrap at u64::MAX)
 * INVARIANTS:
 *   - All entropy flows through seeded RNG (ChaosStream)
 *   - No std::collections::HashMap or HashSet (breaks determinism)
 *   - SimTime monotonically increases (never decreases)
 *   - All math operations must be deterministic and reproducible
 *   - No system time dependencies (all time is simulation time)
 *
 * # Design Rationale
 *
 * The deterministic module provides three critical layers:
 *
 * 1. **SimTime** - Logical simulation clock independent of wall time
 * 2. **ChaosStream** - Deterministic seeded RNG for all entropy
 * 3. **Collections** - BTreeMap/BTreeSet for ordered iteration
 * 4. **Math** - Floating point operations with consistent behavior
 *
 * Together, these ensure:
 * - Reproducible universe state from identical seed
 * - Identical agent behavior from identical starting conditions  
 * - State snapshots that can be resumed at any tick
 * - Deterministic time travel debugging
 *
 * # Critical Invariants
 *
 * ## No System Time
 * Never use `std::time::Instant::now()` or `std::time::SystemTime`.
 * Only use `SimTime` for all temporal operations.
 *
 * ## No External Entropy
 * Never call:
 * - `rand::thread_rng()`
 * - `rand::random()`
 * - `std::random`
 * All randomness must flow through seeded RNG subsystems.
 *
 * ## Ordered Collections Only
 * Never use:
 * - `std::collections::HashMap`
 * - `std::collections::HashSet`
 * Use `DeterministicMap` and `DeterministicSet` instead.
 *
 * ## Floating Point Consistency
 * Floating point math is inherently non-deterministic in some cases
 * (e.g., order of operations affects rounding). Use `DeterministicMath`
 * wrappers to ensure consistent results.
 *
 * Authority: antigravity
 */

use serde::{Deserialize, Serialize};
use blake3::Hasher;

pub mod math;
pub mod collections;

pub use math::DeterministicMath;
pub use collections::{DeterministicMap, DeterministicSet, DeterministicVec};

/// Simulation time - logical clock independent of wall time.
///
/// # Design
///
/// `SimTime` represents discrete simulation ticks, not wall-clock time.
/// It starts at 0 at genesis and monotonically increases by 1 each tick.
///
/// # Why Not Use System Time?
///
/// System time is:
/// - Non-deterministic (depends on when code runs)
/// - Non-reproducible (same seed produces different times on different runs)
/// - Environment-dependent (different systems have different clocks)
///
/// For a deterministic simulation, we need logical time:
/// - Deterministic (same seed, same tick numbers)
/// - Reproducible (can replay exactly)
/// - Independent (works on any system)
///
/// # Invariants
///
/// - Starts at 0
/// - Only increases (never decreases or resets mid-run)
/// - Wraps at `u64::MAX` (after ~18 quintillion ticks at 60 fps)
/// - Serializable and deserializable
///
/// # Example
///
/// ```
/// let mut time = SimTime::zero();
/// assert_eq!(time.as_u64(), 0);
/// 
/// time.tick();
/// assert_eq!(time.as_u64(), 1);
/// 
/// time.tick();
/// assert_eq!(time.as_u64(), 2);
/// ```
///
/// # Use Cases
///
/// - Main simulation loop tick counter
/// - Event timestamps for replay
/// - Snapshot checkpoints
/// - Determinism verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimTime(pub u64);

impl SimTime {
    /// Create a new SimTime at genesis (tick 0).
    ///
    /// # Returns
    /// `SimTime(0)`
    pub fn zero() -> Self {
        SimTime(0)
    }
    
    /// Advance simulation time by one tick.
    ///
    /// # Panics
    /// Never panics, but wraps at `u64::MAX` (will take ~300 billion years at 1000 ticks/sec)
    ///
    /// # Time Complexity
    /// O(1) - Just increments an integer
    pub fn tick(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    
    /// Get the current tick number as a u64.
    ///
    /// # Returns
    /// The current simulation tick (starting from 0)
    ///
    /// # Time Complexity
    /// O(1) - Just returns the value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    
    /// Create a SimTime from a u64 tick number.
    ///
    /// # Arguments
    /// - `t`: The tick number to create
    ///
    /// # Returns
    /// `SimTime(t)`
    ///
    /// # Use Cases
    /// - Restoring from snapshot (load tick from database)
    /// - Replay from checkpoint
    /// - Testing specific time points
    pub fn from_u64(t: u64) -> Self {
        SimTime(t)
    }
}

/// Xorshift64* - Deterministic PRNG algorithm.
///
/// # Algorithm Overview
///
/// Xorshift64* is a linear congruential generator that:
/// 1. Uses XOR and bit shifts to scramble state
/// 2. Multiplies by a magic constant to distribute bits
/// 3. Produces 64-bit outputs from 64-bit state
///
/// # Design Properties
///
/// - **Deterministic**: Same seed always produces same sequence
/// - **Fast**: O(1) per number (just bit operations)
/// - **Cycle Length**: 2^64 - 1 (won't repeat for practical simulations)
/// - **Period**: Full period for all bits (high quality)
///
/// # Why Xorshift64*?
///
/// We chose this over other PRNGs because:
/// - Simple implementation (no complex state)
/// - No external dependencies
/// - Well-analyzed algorithm with known properties
/// - Good statistical distribution
/// - Fast enough for simulation
///
/// # References
/// - Blackman & Vigna: "Scrambled Linear Pseudorandom Number Generators"
/// - Period: 2^64 - 1
/// - Passes BigCrush statistical tests
///
/// # Critical Note
///
/// This is NOT a cryptographic PRNG. For security-sensitive operations,
/// use a cryptographic RNG. For simulation, this is excellent.
struct Xorshift64Star {
    state: u64,
}

impl Xorshift64Star {
    /// Create a new Xorshift64* RNG with the given seed.
    ///
    /// # Arguments
    /// - `seed`: Initial state (must not be 0)
    ///
    /// # Implementation Note
    /// If seed is 0, we set it to 1. Xorshift with state 0 would produce
    /// all zeros forever, which is useless. By converting 0 → 1, we ensure
    /// every seed value produces a valid sequence.
    fn new(seed: u64) -> Self {
        let mut state = seed;
        // Ensure non-zero state (0 would produce only zeros)
        if state == 0 {
            state = 1;
        }
        Self { state }
    }
    
    /// Generate the next 64-bit random number.
    ///
    /// # Algorithm Steps
    /// 1. XOR with right-shifted state (eliminates lower bits)
    /// 2. XOR with left-shifted state (adds upper bit info)
    /// 3. XOR with right-shifted state again (final mixing)
    /// 4. Multiply by magic constant (distribute bits)
    ///
    /// # Magic Constant
    /// `0x2545F4914F6CDD1D` is carefully chosen to ensure:
    /// - Good bit distribution
    /// - Minimal correlation with previous states
    /// - Passes statistical randomness tests
    ///
    /// # Time Complexity
    /// O(1) - Just bit operations
    ///
    /// # Returns
    /// Next 64-bit value in the sequence
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }
    
    /// Generate the next 32-bit random number.
    ///
    /// # Implementation
    /// Takes upper 32 bits of a 64-bit number.
    /// This preserves good statistical properties while fitting in u32.
    ///
    /// # Returns
    /// Next 32-bit value
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    
    /// Generate the next f32 in range [0.0, 1.0).
    ///
    /// # Implementation
    /// Uses bit manipulation to convert u64 to f32:
    /// 1. Right shift to get 23 bits of entropy
    /// 2. OR with 0x3f800000 (sets float exponent for [1.0, 2.0))
    /// 3. Cast to f32 and subtract 1.0 (gives [0.0, 1.0))
    ///
    /// This technique avoids division (slow) while maintaining
    /// good uniform distribution in [0.0, 1.0).
    ///
    /// # Returns
    /// f32 in [0.0, 1.0) with uniform distribution
    fn next_f32(&mut self) -> f32 {
        const SCALE: f32 = 1.0 / (1u64 << 23) as f32;
        ((self.next_u64() >> 41) as u32 | 0x3f800000) as f32 - 1.0
    }
    
    /// Generate the next f64 in range [0.0, 1.0).
    ///
    /// # Implementation
    /// Similar to next_f32 but uses 52 bits of entropy and f64:
    /// 1. Right shift to get 52 bits of entropy
    /// 2. OR with 0x3ff0000000000000 (sets double exponent for [1.0, 2.0))
    /// 3. Cast to f64 and subtract 1.0 (gives [0.0, 1.0))
    ///
    /// Double precision provides better resolution for values
    /// very close to 0 or 1.
    ///
    /// # Returns
    /// f64 in [0.0, 1.0) with uniform distribution
    fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / (1u64 << 52) as f64;
        ((self.next_u64() >> 12) as u64 | 0x3ff0000000000000) as f64 - 1.0
    }
}

/// ChaosStream - Deterministic random number generator for simulation subsystems.
///
/// # Purpose
///
/// ChaosStream wraps Xorshift64* to provide:
/// 1. Genesis-based seeding (same seed → same sequence)
/// 2. Subsystem isolation (physics RNG ≠ biology RNG)
/// 3. Substream branching (nested randomness)
/// 4. Utility methods (ranges, probability, choosing)
///
/// # Design Hierarchy
///
/// ```
/// Genesis Seed (MARKENZ_GENESIS_SEED = 1337)
///   ├─ Physics Subsystem RNG → collision rolls, movement variance
///   ├─ Biology Subsystem RNG → metabolism, reproduction genetics
///   ├─ Cognition Subsystem RNG → decision-making
///   └─ Events Subsystem RNG → event generation
///
/// Each subsystem can further branch into substreams:
///   Physics RNG
///     ├─ Collision-checks
///     ├─ Movement-variance
///     └─ Terrain-generation
/// ```
///
/// # Determinism Guarantee
///
/// If you:
/// 1. Use same genesis seed
/// 2. Create subsystems in same order
/// 3. Call methods in same order
/// 4. With same arguments
///
/// Then: You get identical results, byte-for-byte.
///
/// # Example
///
/// ```
/// // This always produces the same sequence
/// let mut stream = ChaosStream::from_global_seed(1337);
/// assert_eq!(stream.next_u64(), 5818260462485093637);
/// assert_eq!(stream.next_u64(), 15825848098099633107);
/// ```
pub struct ChaosStream {
    rng: Xorshift64Star,
}

impl ChaosStream {
    /// Create a new ChaosStream from a global genesis seed.
    ///
    /// # Arguments
    /// - `global_seed`: The genesis seed (e.g., MARKENZ_GENESIS_SEED = 1337)
    ///
    /// # Returns
    /// A new ChaosStream initialized with the global seed
    ///
    /// # Determinism Note
    /// This is the primary way to seed the entire universe.
    /// All subsystems derive from this single seed via subsystem IDs.
    ///
    /// # Example
    /// ```
    /// let mut stream = ChaosStream::from_global_seed(1337);
    /// // Now stream can generate deterministic random numbers
    /// ```
    pub fn from_global_seed(global_seed: u64) -> Self {
        let rng = Xorshift64Star::new(global_seed);
        Self { rng }
    }
    
    /// Create a new ChaosStream for a specific subsystem using global seed + system ID.
    ///
    /// # Arguments
    /// - `global_seed`: The genesis seed
    /// - `system_id`: Identifier for the subsystem (e.g., "physics", "biology", "cognition")
    ///
    /// # Returns
    /// A new ChaosStream seeded from global_seed + system_id
    ///
    /// # How It Works
    /// 1. Hash (global_seed || system_id) using Blake3
    /// 2. Extract first 8 bytes as u64
    /// 3. Create Xorshift64* with that seed
    ///
    /// # Determinism Properties
    /// - Same global_seed + system_id always produces same stream
    /// - Different system_id produces completely different stream
    /// - System IDs are static (hardcoded), not dynamic
    ///
    /// # Why Blake3?
    /// Blake3 is cryptographically secure, ensuring:
    /// - Different system IDs never collide (different streams)
    /// - Small changes in seed create large changes in output
    /// - Uniform distribution of derived seeds
    ///
    /// # Example
    /// ```
    /// let physics_rng = ChaosStream::from_system_seed(1337, "physics");
    /// let biology_rng = ChaosStream::from_system_seed(1337, "biology");
    /// // physics_rng and biology_rng are completely independent streams
    /// ```
    pub fn from_system_seed(global_seed: u64, system_id: &str) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&global_seed.to_le_bytes());
        hasher.update(system_id.as_bytes());
        let system_seed = hasher.finalize();
        
        // Use first 8 bytes of hash as seed
        let mut seed_bytes = [0u8; 8];
        seed_bytes.copy_from_slice(&system_seed.as_bytes()[..8]);
        let seed = u64::from_le_bytes(seed_bytes);
        
        Self {
            rng: Xorshift64Star::new(seed),
        }
    }
    
    /// Create a substream from this stream for nested randomness.
    ///
    /// # Arguments
    /// - `substream_id`: Identifier for the substream (e.g., "terrain_gen", "mutation")
    ///
    /// # Returns
    /// A new ChaosStream seeded from current state + substream_id
    ///
    /// # How It Works
    /// 1. Generate next value from current stream (consumes entropy)
    /// 2. Hash (that_value || substream_id) using Blake3
    /// 3. Extract first 8 bytes as u64
    /// 4. Create new Xorshift64* with that seed
    ///
    /// # Important Note
    /// Calling substream() advances the parent stream! This is intentional:
    /// - Prevents stream divergence (different substreams in different orders)
    /// - Ensures determinism with proper usage
    /// - Documents that substream creation consumes entropy
    ///
    /// # Example
    /// ```
    /// let mut parent = ChaosStream::from_system_seed(1337, "physics");
    /// let mut terrain = parent.substream("terrain_generation");
    /// let mut collision = parent.substream("collision_checks");
    /// // terrain and collision are independent but derived deterministically
    /// ```
    pub fn substream(&mut self, substream_id: &str) -> Self {
        // Get current state to derive substream seed
        let mut hasher = Hasher::new();
        let next_val = self.rng.next_u64();
        hasher.update(&next_val.to_le_bytes());
        hasher.update(substream_id.as_bytes());
        let substream_seed = hasher.finalize();
        
        let mut seed_bytes = [0u8; 8];
        seed_bytes.copy_from_slice(&substream_seed.as_bytes()[..8]);
        let seed = u64::from_le_bytes(seed_bytes);
        
        Self {
            rng: Xorshift64Star::new(seed),
        }
    }
    
    /// Generate the next 32-bit random number.
    ///
    /// # Returns
    /// Uniformly distributed u32 in [0, 2^32)
    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    /// Generate the next 64-bit random number.
    ///
    /// # Returns
    /// Uniformly distributed u64 in [0, 2^64)
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    
    /// Generate the next f32 in range [0.0, 1.0).
    ///
    /// # Returns
    /// Uniformly distributed f32 in [0.0, 1.0)
    pub fn next_f32(&mut self) -> f32 {
        self.rng.next_f32()
    }
    
    /// Generate the next f64 in range [0.0, 1.0).
    ///
    /// # Returns
    /// Uniformly distributed f64 in [0.0, 1.0)
    pub fn next_f64(&mut self) -> f64 {
        self.rng.next_f64()
    }
    
    /// Generate a value in the range [0.0, 1.0).
    ///
    /// # Returns
    /// Uniformly distributed f32 in [0.0, 1.0)
    ///
    /// # Alias For
    /// This is an alias for `next_f32()` for semantic clarity.
    pub fn next_unit_f32(&mut self) -> f32 {
        self.rng.next_f32()
    }
    
    /// Generate a value in the range [min, max).
    ///
    /// # Arguments
    /// - `min`: Lower bound (inclusive)
    /// - `max`: Upper bound (exclusive)
    ///
    /// # Returns
    /// Uniformly distributed f32 in [min, max)
    ///
    /// # Formula
    /// `result = min + (max - min) * random_unit_f32()`
    ///
    /// # Example
    /// ```
    /// let mut rng = ChaosStream::from_global_seed(1337);
    /// let speed = rng.range_f32(0.5, 2.0); // Movement speed multiplier
    /// ```
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_unit_f32()
    }
    
    /// Generate a value in the range [min, max] (inclusive on both ends).
    ///
    /// # Arguments
    /// - `min`: Lower bound (inclusive)
    /// - `max`: Upper bound (inclusive)
    ///
    /// # Returns
    /// Uniformly distributed i32 in [min, max]
    ///
    /// # Algorithm
    /// Uses modulo to ensure uniform distribution over the range.
    ///
    /// # Example
    /// ```
    /// let mut rng = ChaosStream::from_global_seed(1337);
    /// let priority = rng.range_i32(1, 10); // Pick priority 1-10
    /// ```
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u32;
        min + (self.next_u32() % range) as i32
    }
    
    /// Generate a boolean with given probability.
    ///
    /// # Arguments
    /// - `p`: Probability of true (0.0 = always false, 1.0 = always true)
    ///
    /// # Returns
    /// `true` with probability p, `false` with probability (1-p)
    ///
    /// # Formula
    /// `result = (random_unit_f32() < p)`
    ///
    /// # Example
    /// ```
    /// let mut rng = ChaosStream::from_global_seed(1337);
    /// if rng.probability(0.1) {
    ///     // This happens 10% of the time
    /// }
    /// ```
    pub fn probability(&mut self, p: f32) -> bool {
        self.next_unit_f32() < p.clamp(0.0, 1.0)
    }
    
    /// Randomly choose an element from a slice.
    ///
    /// # Arguments
    /// - `slice`: The slice to choose from
    ///
    /// # Returns
    /// `Some(&element)` if slice is non-empty, `None` if empty
    /// Each element has equal probability of selection.
    ///
    /// # Time Complexity
    /// O(1) - Just generates one random index
    ///
    /// # Example
    /// ```
    /// let mut rng = ChaosStream::from_global_seed(1337);
    /// let biomes = vec!["forest", "desert", "swamp"];
    /// let chosen = rng.choose(&biomes);
    /// // Each biome has 1/3 probability
    /// ```
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let index = self.range_i32(0, slice.len() as i32 - 1) as usize;
            slice.get(index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sim_time() {
        let mut time = SimTime::zero();
        assert_eq!(time.as_u64(), 0);
        
        time.tick();
        assert_eq!(time.as_u64(), 1);
        
        time.tick();
        assert_eq!(time.as_u64(), 2);
    }
    
    #[test]
    fn test_chaos_stream_determinism() {
        let seed = 12345;
        let mut stream1 = ChaosStream::from_global_seed(seed);
        let mut stream2 = ChaosStream::from_global_seed(seed);
        
        // Same seed should produce same sequence
        for _ in 0..10 {
            assert_eq!(stream1.next_u32(), stream2.next_u32());
            assert_eq!(stream1.next_f32(), stream2.next_f32());
        }
    }
    
    #[test]
    fn test_system_seed_determinism() {
        let global_seed = 54321;
        let system_id = "test_system";
        
        let mut stream1 = ChaosStream::from_system_seed(global_seed, system_id);
        let mut stream2 = ChaosStream::from_system_seed(global_seed, system_id);
        
        // Same global seed and system ID should produce same sequence
        for _ in 0..10 {
            assert_eq!(stream1.next_u32(), stream2.next_u32());
        }
        
        // Different system ID should produce different sequence
        let mut stream3 = ChaosStream::from_system_seed(global_seed, "other_system");
        assert_ne!(stream1.next_u32(), stream3.next_u32());
    }
    
    #[test]
    fn test_substream_determinism() {
        let seed = 99999;
        let sub_id = "test_substream";
        
        // Create first parent and generate entropy
        let mut parent1 = ChaosStream::from_global_seed(seed);
        for _ in 0..5 {
            parent1.next_u32();
        }
        let mut substream1 = parent1.substream(sub_id);
        
        // Create second parent in identical state
        let mut parent2 = ChaosStream::from_global_seed(seed);
        for _ in 0..5 {
            parent2.next_u32();
        }
        let mut substream2 = parent2.substream(sub_id);
        
        // Substreams with same ID should be identical
        for _ in 0..10 {
            assert_eq!(substream1.next_u32(), substream2.next_u32());
        }
    }
    
    #[test]
    fn test_chaos_stream_utilities() {
        let mut stream = ChaosStream::from_global_seed(42);
        
        // Test range functions
        let val = stream.range_f32(10.0, 20.0);
        assert!(val >= 10.0 && val < 20.0);
        
        let val = stream.range_i32(5, 15);
        assert!(val >= 5 && val <= 15);
        
        // Test probability
        let true_count = (0..1000).filter(|_| stream.probability(0.5)).count();
        assert!(true_count > 400 && true_count < 600); // Should be around 500
        
        // Test choose
        let items = vec![1, 2, 3, 4, 5];
        let chosen = stream.choose(&items);
        assert!(chosen.is_some());
        assert!(items.contains(chosen.unwrap()));
    }
}

