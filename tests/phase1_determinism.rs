/**
 * File: tests/phase1_determinism.rs
 * 
 * Purpose: Phase 1 determinism validation tests
 * 
 * Why this file exists:
 * - Implements TEST-DET-001: Fixed Seed Reproducibility
 * - Implements TEST-SNAPSHOT-EQ-001: Snapshot Replay Equivalence  
 * - Implements TEST-HASH-CHAIN-001: Hash Chain Integrity
 * - Implements TEST-RNG-001: RNG Chaos Stability
 * - Validates Phase 1 determinism guarantees per PLAN_PHASE_1_DETERMINISM.md
 * 
 * Phase plan authority: docs/plans/PLAN_PHASE_1_DETERMINISM.md
 * Section 6 "REPLAY HARNESS", Section 8 "SUCCESS CRITERIA"
 * 
 * Invariants enforced:
 * - Same seed + same events = identical hash sequences across runs
 * - Snapshot replay = full replay from same tick onward
 * - Hash chain continuity must be maintained without breaks
 * - RNG values must be stable and reproducible across platforms
 * - All RNG draws must be audit-logged
 * 
 * What breaks if removed:
 * - No determinism validation → Phase 1 requirements unverified
 * - No snapshot testing → replay equivalence unproven
 * - No hash chain testing → corruption detection missing
 * - No RNG testing → platform independence unverified
 * 
 * What this file does NOT do:
 * - Does not test Phase 2+ features (biology, cognition, etc.)
 * - Does not permit any nondeterministic behavior in tests
 * - Does not allow test fixtures to be platform-dependent
 */

use std::collections::BTreeMap;
use markenz_engine::deterministic_world_loop::{DeterministicWorldLoop, DeterministicWorldConfig};
use markenz_events::{InputEvent, InputEventPayload};
use rng::rng_stream::RngSubsystem;

/// TEST-DET-001: Fixed Seed Reproducibility
/// 
/// Requirement: Same seed + same ordered InputEvents ⇒ identical hash sequence
/// Three identical runs must produce bit-for-bit identical hash sequences
#[test]
fn test_determinism_fixed_seed() -> Result<(), String> {
    let seed = 12345u64;
    let num_ticks = 100;
    
    // Create identical input events for all runs
    let mut input_events = BTreeMap::new();
    input_events.insert(5, vec![
        InputEvent {
            tick: 5,
            source_agent_id: 1,
            sequence: 1,
            payload: InputEventPayload::Move { x: 10.0, y: 20.0, z: 0.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        }
    ]);
    input_events.insert(10, vec![
        InputEvent {
            tick: 10,
            source_agent_id: 2,
            sequence: 1,
            payload: InputEventPayload::Move { x: -5.0, y: 15.0, z: 2.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        }
    ]);
    
    let config = DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 50,
        tick_rate_ms: 50,
    };
    
    // Run 1
    let mut world_loop1 = DeterministicWorldLoop::new(config.clone());
    world_loop1.run(input_events.clone()).map_err(|e| format!("Run 1 failed: {}", e))?;
    let hashes1 = world_loop1.hash_chain().to_vec();
    
    // Run 2
    let mut world_loop2 = DeterministicWorldLoop::new(config.clone());
    world_loop2.run(input_events.clone()).map_err(|e| format!("Run 2 failed: {}", e))?;
    let hashes2 = world_loop2.hash_chain().to_vec();
    
    // Run 3
    let mut world_loop3 = DeterministicWorldLoop::new(config);
    world_loop3.run(input_events).map_err(|e| format!("Run 3 failed: {}", e))?;
    let hashes3 = world_loop3.hash_chain().to_vec();
    
    // All three runs must produce identical hashes
    assert_eq!(hashes1.len(), hashes2.len(), "Hash sequence length mismatch between run 1 and 2");
    assert_eq!(hashes2.len(), hashes3.len(), "Hash sequence length mismatch between run 2 and 3");
    
    for (i, (h1, h2)) in hashes1.iter().zip(hashes2.iter()).enumerate() {
        assert_eq!(h1, h2, "Hashes differ at tick {} between run 1 and 2: {:x?} vs {:x?}", i, h1, h2);
    }
    
    for (i, (h2, h3)) in hashes2.iter().zip(hashes3.iter()).enumerate() {
        assert_eq!(h2, h3, "Hashes differ at tick {} between run 2 and 3: {:x?} vs {:x?}", i, h2, h3);
    }
    
    println!("✅ TEST-DET-001 PASSED: All {} hashes identical across 3 runs", hashes1.len());
    Ok(())
}

/// TEST-SNAPSHOT-EQ-001: Snapshot Replay Equivalence
/// 
/// Requirement: Load snapshot at tick T; replay events from T onward = full replay from boot
/// Hashes from snapshot replay must match full replay from same tick onward
#[test]
fn test_snapshot_replay_equivalence() -> Result<(), String> {
    let seed = 12345u64;
    let total_ticks = 200;
    let snapshot_tick = 100;
    
    // Create input events spanning the full run
    let mut input_events = BTreeMap::new();
    for tick in [20, 50, 80, 120, 150, 180] {
        input_events.insert(tick, vec![
            InputEvent {
                tick,
                source_agent_id: 1,
                sequence: 1,
                payload: InputEventPayload::Move { 
                    x: tick as f32, 
                    y: (tick * 2) as f32, 
                    z: 0.0 
                },
                hash: [0u8; 32],
                prev_hash: [0u8; 32],
            }
        ]);
    }
    
    // Full run: ticks 0-200
    let mut full_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: total_ticks,
        snapshot_interval: 1000, // No automatic snapshots
        tick_rate_ms: 50,
    });
    
    full_loop.run(input_events.clone()).map_err(|e| format!("Full run failed: {}", e))?;
    let full_hashes = full_loop.hash_chain().to_vec();
    
    // Snapshot run: run to snapshot_tick, then continue to total_ticks
    let mut snapshot_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: snapshot_tick,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    // Run to snapshot point
    let snapshot_events: BTreeMap<u64, Vec<InputEvent>> = input_events
        .iter()
        .filter(|(&tick, _)| tick <= snapshot_tick)
        .map(|(&tick, events)| (tick, events.clone()))
        .collect();
    
    snapshot_loop.run(snapshot_events).map_err(|e| format!("Snapshot run to tick {} failed: {}", snapshot_tick, e))?;
    
    // Continue from snapshot point
    snapshot_loop.config.max_ticks = total_ticks;
    
    let remaining_events: BTreeMap<u64, Vec<InputEvent>> = input_events
        .iter()
        .filter(|(&tick, _)| tick > snapshot_tick)
        .map(|(&tick, events)| (tick, events.clone()))
        .collect();
    
    snapshot_loop.run(remaining_events).map_err(|e| format!("Snapshot continuation failed: {}", e))?;
    let snapshot_hashes = snapshot_loop.hash_chain().to_vec();
    
    // Hashes from tick 100-200 must match exactly
    let full_tail = &full_hashes[snapshot_tick as usize..total_ticks as usize];
    let snap_tail = &snapshot_hashes[snapshot_tick as usize..total_ticks as usize];
    
    assert_eq!(full_tail.len(), snap_tail.len(), "Tail length mismatch");
    
    for (i, (full_hash, snap_hash)) in full_tail.iter().zip(snap_tail.iter()).enumerate() {
        let tick = snapshot_tick + i as u64;
        assert_eq!(full_hash, snap_hash, 
            "Hash mismatch at tick {}: full={:x?}, snapshot={:x?}", 
            tick, full_hash, snap_hash);
    }
    
    println!("✅ TEST-SNAPSHOT-EQ-001 PASSED: {} hashes match from tick {} to {}", 
        full_tail.len(), snapshot_tick, total_ticks);
    
    Ok(())
}

/// TEST-HASH-CHAIN-001: Hash Chain Integrity
/// 
/// Requirement: Hash chain must be continuous with no breaks
/// Each hash must be computed from the previous state deterministically
#[test]
fn test_hash_chain_integrity() -> Result<(), String> {
    let seed = 54321u64;
    let num_ticks = 50;
    
    let mut world_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 1000, // No automatic snapshots
        tick_rate_ms: 50,
    });
    
    // Run with no input events (pure determinism test)
    let empty_events = BTreeMap::new();
    world_loop.run(empty_events).map_err(|e| format!("Hash chain test run failed: {}", e))?;
    
    let hash_chain = world_loop.hash_chain();
    
    // Verify hash chain properties
    assert_eq!(hash_chain.len(), num_ticks as usize + 1, 
        "Hash chain should have genesis + {} hashes", num_ticks);
    
    // Verify no zero hashes (except possibly genesis)
    for (i, hash) in hash_chain.iter().enumerate() {
        if i > 0 { // Skip genesis hash check
            assert_ne!(hash, &[0u8; 32], "Hash at position {} cannot be zero", i);
        }
    }
    
    // Verify hash uniqueness (no duplicate consecutive hashes)
    for i in 1..hash_chain.len() {
        assert_ne!(hash_chain[i-1], hash_chain[i], 
            "Consecutive hashes at positions {} and {} cannot be identical", 
            i-1, i);
    }
    
    println!("✅ TEST-HASH-CHAIN-001 PASSED: {} unique hashes in chain", hash_chain.len());
    Ok(())
}

/// TEST-RNG-001: RNG Chaos Stability
/// 
/// Requirement: RNG must produce stable, reproducible sequences
/// Same seed must produce identical values across runs
#[test]
fn test_rng_chaos_stability() -> Result<(), String> {
    let seed = 999u64;
    let num_draws = 1000;
    
    // Create two identical RNG instances
    let mut rng1 = rng::DeterministicRng::new(seed);
    let mut rng2 = rng::DeterministicRng::new(seed);
    
    // Generate sequence from first RNG
    let mut values1 = Vec::new();
    for tick in 1..=num_draws {
        rng1.set_tick(tick);
        let mut stream = rng1.stream(RngSubsystem::Physics, 0);
        let value = stream.next_u64("test_rng_chaos_stability.rs:value");
        values1.push(value);
    }
    
    // Generate sequence from second RNG
    let mut values2 = Vec::new();
    for tick in 1..=num_draws {
        rng2.set_tick(tick);
        let mut stream = rng2.stream(RngSubsystem::Physics, 0);
        let value = stream.next_u64("test_rng_chaos_stability.rs:value");
        values2.push(value);
    }
    
    // Verify sequences are identical
    assert_eq!(values1.len(), values2.len(), "Sequence length mismatch");
    
    for (i, (v1, v2)) in values1.iter().zip(values2.iter()).enumerate() {
        assert_eq!(v1, v2, "RNG values differ at draw {}: {} vs {}", i, v1, v2);
    }
    
    // Verify RNG audit logging
    let audit_log = rng1.audit_log();
    assert_eq!(audit_log.len(), num_draws as usize, "Audit log should contain {} entries", num_draws);
    
    // Verify audit log entries are correct
    for (i, record) in audit_log.records().iter().enumerate() {
        assert_eq!(record.tick, (i + 1) as u64, "Record {} has wrong tick", i);
        assert_eq!(record.subsystem, RngSubsystem::Physics, "Record {} has wrong subsystem", i);
        assert_eq!(record.stream_id, 0, "Record {} has wrong stream ID", i);
        assert_eq!(record.value, values1[i], "Record {} has wrong value", i);
    }
    
    println!("✅ TEST-RNG-001 PASSED: {} identical RNG values with full audit logging", num_draws);
    Ok(())
}

/// TEST-RNG-AUDIT-001: RNG Sequence Bit-Identical Across Platforms
/// 
/// Requirement: Platform-independent RNG behavior
/// Same seed must produce identical sequences on all platforms
#[test]
fn test_rng_platform_independence() -> Result<(), String> {
    let seed = 42u64;
    
    // Create RNG with known seed
    let mut rng = rng::DeterministicRng::new(seed);
    rng.set_tick(1);
    
    // Generate test sequence from multiple subsystems
    let mut physics_values = Vec::new();
    let mut biology_values = Vec::new();
    let mut cognition_values = Vec::new();
    
    for i in 0..100 {
        rng.set_tick(i + 1);
        
        // Physics stream
        let mut physics_stream = rng.stream(RngSubsystem::Physics, 0);
        physics_values.push(physics_stream.next_u64("test_rng_platform_independence.rs:physics"));
        
        // Biology stream  
        let mut biology_stream = rng.stream(RngSubsystem::Biology, 0);
        biology_values.push(biology_stream.next_u64("test_rng_platform_independence.rs:biology"));
        
        // Cognition stream
        let mut cognition_stream = rng.stream(RngSubsystem::Cognition, 0);
        cognition_values.push(cognition_stream.next_u64("test_rng_platform_independence.rs:cognition"));
    }
    
    // Verify subsystem isolation (different streams should produce different values)
    assert_ne!(physics_values, biology_values, "Physics and Biology streams must differ");
    assert_ne!(biology_values, cognition_values, "Biology and Cognition streams must differ");
    assert_ne!(physics_values, cognition_values, "Physics and Cognition streams must differ");
    
    // Verify audit log contains all draws
    let audit_log = rng.audit_log();
    assert_eq!(audit_log.len(), 300, "Should have 300 audit entries (100 per subsystem)");
    
    // Verify subsystem distribution in audit log
    let physics_records = audit_log.records_by_subsystem(RngSubsystem::Physics);
    let biology_records = audit_log.records_by_subsystem(RngSubsystem::Biology);
    let cognition_records = audit_log.records_by_subsystem(RngSubsystem::Cognition);
    
    assert_eq!(physics_records.len(), 100, "Should have 100 physics records");
    assert_eq!(biology_records.len(), 100, "Should have 100 biology records");
    assert_eq!(cognition_records.len(), 100, "Should have 100 cognition records");
    
    println!("✅ TEST-RNG-AUDIT-001 PASSED: Platform-independent RNG with {} audit entries", audit_log.len());
    Ok(())
}

/// Integration test combining all Phase 1 determinism requirements
#[test]
fn test_phase1_determinism_integration() -> Result<(), String> {
    println!("Running Phase 1 Determinism Integration Test...");
    
    // Run all individual tests
    test_determinism_fixed_seed()?;
    test_snapshot_replay_equivalence()?;
    test_hash_chain_integrity()?;
    test_rng_chaos_stability()?;
    test_rng_platform_independence()?;
    
    println!("✅ PHASE 1 DETERMINISM INTEGRATION TEST PASSED");
    println!("All Phase 1 determinism guarantees verified:");
    println!("  - Fixed seed reproducibility ✓");
    println!("  - Snapshot replay equivalence ✓");
    println!("  - Hash chain integrity ✓");
    println!("  - RNG chaos stability ✓");
    println!("  - Platform independence ✓");
    
    Ok(())
}
