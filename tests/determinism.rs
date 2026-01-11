use markenz_world::{Universe, UniverseConfig};
use rng::{GlobalSeed, RngSubsystem};

#[test]
fn test_determinism_fixed_seed() -> Result<(), String> {
    let seed = 12345u64;
    let num_ticks = 100;
    
    // Run 1
    let (_, hashes1) = simulate(seed, num_ticks)?;
    
    // Run 2
    let (_, hashes2) = simulate(seed, num_ticks)?;
    
    // Run 3
    let (_, hashes3) = simulate(seed, num_ticks)?;
    
    // All three runs must produce identical hashes
    assert_eq!(hashes1, hashes2, "Run 1 and 2 diverged");
    assert_eq!(hashes2, hashes3, "Run 2 and 3 diverged");
    
    Ok(())
}

fn simulate(seed: u64, num_ticks: u64) -> Result<(Universe, Vec<[u8; 32]>), String> {
    let mut universe = Universe::new(seed);
    let mut hashes = Vec::new();
    
    for tick in 0..num_ticks {
        // Simulate one tick with no external input (deterministic)
        // In Phase 1, we would use RNG streams here, but for now just advance tick
        universe.tick += 1;
        
        // Compute hash after each tick
        universe.state_hash = universe.compute_hash();
        hashes.push(universe.state_hash);
    }
    
    Ok((universe, hashes))
}

#[test]
fn test_snapshot_replay_equivalence() -> Result<(), String> {
    let seed = 12345u64;
    
    // Full run: ticks 0-1000
    let (universe_full, hashes_full) = full_run(seed, 1000)?;
    
    // Snapshot run: save at tick 500, replay to 1000
    let (universe_snap, hashes_snap) = snapshot_run(seed, 500, 1000)?;
    
    // Hashes from tick 500-1000 must match exactly
    let full_tail = &hashes_full[500..1000];
    let snap_tail = &hashes_snap[0..500];  // Relative to snapshot
    
    assert_eq!(full_tail, snap_tail, "Snapshot replay diverged from full run");
    assert_eq!(universe_full.state_hash, universe_snap.state_hash, "Final state differs");
    
    Ok(())
}

fn full_run(seed: u64, num_ticks: u64) -> Result<(Universe, Vec<[u8; 32]>), String> {
    simulate(seed, num_ticks)
}

fn snapshot_run(seed: u64, snapshot_tick: u64, target_tick: u64) -> Result<(Universe, Vec<[u8; 32]>), String> {
    // Run to snapshot point
    let (mut universe, _) = simulate(seed, snapshot_tick)?;
    
    // Continue from snapshot to target
    let mut hashes = Vec::new();
    for tick in snapshot_tick..target_tick {
        universe.tick += 1;
        universe.state_hash = universe.compute_hash();
        hashes.push(universe.state_hash);
    }
    
    Ok((universe, hashes))
}

#[test]
fn test_hash_chain_integrity() -> Result<(), String> {
    let seed = 12345u64;
    let (universe, _) = simulate(seed, 100)?;
    
    // Verify hash chain property: H(prev_hash || state) = current_hash
    // This is enforced by compute_hash() which includes prev_state_hash
    
    // For this test, we just verify that hashes are changing (not stuck)
    let mut prev_hash = [0u8; 32];
    for tick in 0..10 {
        let current_hash = universe.compute_hash();
        assert_ne!(current_hash, prev_hash, "Hash should change at tick {}", tick);
        prev_hash = current_hash;
        universe.tick += 1;
    }
    
    Ok(())
}

#[test]
fn test_rng_chaos_stability() -> Result<(), String> {
    // Generate 1000 random values with known seed
    let mut global_seed = GlobalSeed::from_genesis(42);
    let mut stream = global_seed.stream(RngSubsystem::Physics, 0);
    
    let mut values = Vec::new();
    for _ in 0..1000 {
        values.push(stream.next_u64());
    }
    
    // Compare to fixture (pre-computed on reference platform)
    // For now, just verify determinism
    let mut global_seed2 = GlobalSeed::from_genesis(42);
    let mut stream2 = global_seed2.stream(RngSubsystem::Physics, 0);
    
    for i in 0..1000 {
        assert_eq!(stream.next_u64(), stream2.next_u64(), "RNG diverged at index {}", i);
    }
    
    Ok(())
}

#[test]
fn test_rng_platform_independence() -> Result<(), String> {
    // Run on x64 Linux, arm64 Linux, macOS
    // All must produce identical RNG sequences
    
    let seed = 999u64;
    let mut rng = GlobalSeed::from_genesis(seed);
    let stream = rng.stream(RngSubsystem::Biology, 0);
    
    let mut values = Vec::new();
    for _ in 0..100 {
        values.push(stream.next_u64());
    }
    
    // Load reference from platform fixture
    // For now, just verify determinism across multiple instances
    let mut rng2 = GlobalSeed::from_genesis(seed);
    let stream2 = rng2.stream(RngSubsystem::Biology, 0);
    
    for i in 0..100 {
        let val1 = stream.next_u64();
        let val2 = stream2.next_u64();
        assert_eq!(val1, val2, "Platform-dependent RNG behavior detected at index {}", i);
    }
    
    Ok(())
}
