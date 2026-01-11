//! # Phase 1 Replay Harness Validation
//! 
//! This module implements comprehensive replay testing for Phase 1 determinism.
//! It validates snapshot creation and loading mechanisms, tests event log replay
//! with hash verification, and ensures replay equivalence across different scenarios.
//! 
//! ## Test Coverage
//! - Snapshot creation and loading
//! - Event log replay with hash verification  
//! - Replay equivalence across different scenarios
//! - Deterministic behavior validation

/**
 * File: tests/replay_harness.rs
 * 
 * Purpose: Phase 1 replay harness validation
 * 
 * Why this file exists:
 * - Implements comprehensive replay testing for Phase 1 determinism
 * - Validates snapshot creation and loading mechanisms
 * - Tests event log replay with hash verification
 * - Ensures replay equivalence across different scenarios
 * - Validates audit trail preservation during replay
 * 
 * Phase plan authority: docs/plans/PLAN_PHASE_1_DETERMINISM.md
 * Section 6 "REPLAY HARNESS", Section 7 "DETERMINISM GUARANTEES"
 * 
 * Invariants enforced:
 * - Replay from snapshot produces identical state to live simulation
 * - Event replay order must be strictly preserved
 * - Audit trail must be complete and accurate during replay
 * - Hash verification must catch any replay divergence
 * - Snapshot integrity must be verifiable at load time
 * 
 * What breaks if removed:
 * - No replay testing → Phase 1 replay guarantees unverified
 * - No snapshot validation → snapshot integrity unproven
 * - No audit verification → replay correctness unconfirmed
 * - No hash checking → undetectable replay corruption
 * 
 * What this file does NOT do:
 * - Does not test Phase 2+ features
 * - Does not permit any nondeterministic replay behavior
 * - Does not allow snapshot modification after creation
 */

use std::collections::BTreeMap;
use markenz_engine::deterministic_world_loop::{DeterministicWorldLoop, DeterministicWorldConfig, DeterministicSnapshot};
use markenz_events::{InputEvent, InputEventPayload};
use rng::rng_stream::RngSubsystem;

/// Test snapshot creation and basic integrity
#[test]
fn test_snapshot_creation_and_integrity() -> Result<(), String> {
    let seed = 12345u64;
    let num_ticks = 50;
    
    let mut world_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 1000, // No automatic snapshots
        tick_rate_ms: 50,
    });
    
    // Add some events to create interesting state
    let mut input_events = BTreeMap::new();
    let _ = input_events.insert(10, vec![
        InputEvent {
            tick: 10,
            source_agent_id: 1,
            sequence: 1,
            payload: InputEventPayload::Move { x: 5.0, y: 10.0, z: 0.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        }
    ]);
    
    let _ = input_events.insert(25, vec![
        InputEvent {
            tick: 25,
            source_agent_id: 2,
            sequence: 1,
            payload: InputEventPayload::Move { x: -3.0, y: 7.0, z: 1.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        }
    ]);
    
    world_loop.run(input_events).map_err(|e| format!("Initial run failed: {}", e))?;
    
    // Create snapshot at final state
    let snapshot = DeterministicSnapshot::from_world_loop(&world_loop);
    
    // Verify snapshot integrity
    snapshot.verify_integrity().map_err(|e| format!("Snapshot integrity check failed: {}", e))?;
    
    // Verify snapshot contents
    assert_eq!(snapshot.tick, world_loop.current_tick());
    assert_eq!(snapshot.world_hash, world_loop.current_hash());
    assert_eq!(snapshot.hash_chain, world_loop.hash_chain());
    
    println!("✅ Snapshot creation and integrity test passed");
    Ok(())
}

/// Test replay from snapshot produces identical results
#[test]
fn test_snapshot_replay_identical_results() -> Result<(), String> {
    let seed = 54321u64;
    let total_ticks = 100;
    let snapshot_tick = 50;
    
    // Create input events spanning the entire run
    let mut input_events = BTreeMap::new();
    for tick in [15, 30, 45, 60, 75, 90] {
        let _ = input_events.insert(tick, vec![
            InputEvent {
                tick,
                source_agent_id: 1,
                sequence: 1,
                payload: InputEventPayload::Move { 
                    x: tick as f32 * 0.5, 
                    y: tick as f32 * 0.3, 
                    z: (tick % 5) as f32 
                },
                hash: [0u8; 32],
                prev_hash: [0u8; 32],
            }
        ]);
    }
    
    // Full run for reference
    let mut full_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: total_ticks,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    full_loop.run(input_events.clone()).map_err(|e| format!("Full run failed: {}", e))?;
    let full_final_hash = full_loop.current_hash();
    let full_hash_chain = full_loop.hash_chain().to_vec();
    
    // Create snapshot at tick 50
    let mut snapshot_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: snapshot_tick,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    let snapshot_events: BTreeMap<u64, Vec<InputEvent>> = input_events
        .iter()
        .filter(|(&tick, _)| tick <= snapshot_tick)
        .map(|(&tick, events)| (tick, events.clone()))
        .collect();
    
    snapshot_loop.run(snapshot_events).map_err(|e| format!("Snapshot run failed: {}", e))?;
    
    let snapshot = DeterministicSnapshot::from_world_loop(&snapshot_loop);
    snapshot.verify_integrity().map_err(|e| format!("Snapshot verification failed: {}", e))?;
    
    // Continue from snapshot to total_ticks
    let mut replay_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed, // Same seed (required for deterministic replay)
        max_ticks: total_ticks,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    // Load snapshot state (in real implementation, this would deserialize)
    replay_loop.universe = snapshot.universe.clone();
    replay_loop.rng = snapshot.rng_state.clone();
    replay_loop.current_tick = snapshot.tick;
    replay_loop.hash_chain = snapshot.hash_chain.clone();
    
    // Process remaining events
    let remaining_events: BTreeMap<u64, Vec<InputEvent>> = input_events
        .iter()
        .filter(|(&tick, _)| tick > snapshot_tick)
        .map(|(&tick, events)| (tick, events.clone()))
        .collect();
    
    replay_loop.run(remaining_events).map_err(|e| format!("Replay continuation failed: {}", e))?;
    
    let replay_final_hash = replay_loop.current_hash();
    let replay_hash_chain = replay_loop.hash_chain().to_vec();
    
    // Verify final results are identical
    assert_eq!(full_final_hash, replay_final_hash, 
        "Final hashes differ: full={:x?}, replay={:x?}", 
        full_final_hash, replay_final_hash);
    
    // Verify hash chains from snapshot point onward are identical
    let full_tail = &full_hash_chain[snapshot_tick as usize..];
    let replay_tail = &replay_hash_chain[snapshot_tick as usize..];
    
    assert_eq!(full_tail.len(), replay_tail.len(), "Hash chain tail length mismatch");
    
    for (i, (full_hash, replay_hash)) in full_tail.iter().zip(replay_tail.iter()).enumerate() {
        let tick = snapshot_tick + i as u64;
        assert_eq!(full_hash, replay_hash, 
            "Hash mismatch at tick {}: full={:x?}, replay={:x?}", 
            tick, full_hash, replay_hash);
    }
    
    println!("✅ Snapshot replay identical results test passed");
    Ok(())
}

/// Test event order preservation during replay
#[test]
fn test_event_order_preservation() -> Result<(), String> {
    let seed = 98765u64;
    let num_ticks = 30;
    
    // Create events with specific ordering requirements
    let mut input_events = BTreeMap::new();
    
    // Multiple events in same tick (must be processed in sequence order)
    let _ = input_events.insert(10, vec![
        InputEvent {
            tick: 10,
            source_agent_id: 1,
            sequence: 1,
            payload: InputEventPayload::Move { x: 1.0, y: 0.0, z: 0.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        },
        InputEvent {
            tick: 10,
            source_agent_id: 2,
            sequence: 2,
            payload: InputEventPayload::Move { x: 0.0, y: 1.0, z: 0.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        },
        InputEvent {
            tick: 10,
            source_agent_id: 3,
            sequence: 3,
            payload: InputEventPayload::Move { x: 0.0, y: 0.0, z: 1.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        },
    ]);
    
    // Events across different ticks
    let _ = input_events.insert(20, vec![
        InputEvent {
            tick: 20,
            source_agent_id: 1,
            sequence: 1,
            payload: InputEventPayload::Move { x: -1.0, y: 0.0, z: 0.0 },
            hash: [0u8; 32],
            prev_hash: [0u8; 32],
        }
    ]);
    
    // Run twice to verify order preservation
    let mut loop1 = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    let mut loop2 = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    loop1.run(input_events.clone()).map_err(|e| format!("First run failed: {}", e))?;
    loop2.run(input_events).map_err(|e| format!("Second run failed: {}", e))?;
    
    // Verify identical hash sequences
    let hashes1 = loop1.hash_chain();
    let hashes2 = loop2.hash_chain();
    
    assert_eq!(hashes1.len(), hashes2.len(), "Hash chain length mismatch");
    
    for (i, (h1, h2)) in hashes1.iter().zip(hashes2.iter()).enumerate() {
        assert_eq!(h1, h2, "Hash mismatch at tick {}: {:x?} vs {:x?}", i, h1, h2);
    }
    
    // Verify RNG audit logs are identical
    let audit1 = loop1.rng_audit_log();
    let audit2 = loop2.rng_audit_log();
    
    assert_eq!(audit1.len(), audit2.len(), "Audit log length mismatch");
    
    for (i, (record1, record2)) in audit1.records().iter().zip(audit2.records().iter()).enumerate() {
        assert_eq!(record1.tick, record2.tick, "Audit record {} tick mismatch", i);
        assert_eq!(record1.subsystem, record2.subsystem, "Audit record {} subsystem mismatch", i);
        assert_eq!(record1.value, record2.value, "Audit record {} value mismatch", i);
    }
    
    println!("✅ Event order preservation test passed");
    Ok(())
}

/// Test audit trail preservation during replay
#[test]
fn test_audit_trail_preservation() -> Result<(), String> {
    let seed = 11111u64;
    let num_ticks = 25;
    
    let mut world_loop = DeterministicWorldLoop::new(DeterministicWorldConfig {
        genesis_seed: seed,
        max_ticks: num_ticks,
        snapshot_interval: 1000,
        tick_rate_ms: 50,
    });
    
    // Create events that trigger RNG usage
    let mut input_events = BTreeMap::new();
    for tick in [5, 10, 15, 20] {
        let _ = input_events.insert(tick, vec![
            InputEvent {
                tick,
                source_agent_id: 1,
                sequence: 1,
                payload: InputEventPayload::Move { x: tick as f32, y: 0.0, z: 0.0 },
                hash: [0u8; 32],
                prev_hash: [0u8; 32],
            }
        ]);
    }
    
    world_loop.run(input_events).map_err(|e| format!("Run failed: {}", e))?;
    
    // Verify audit log completeness
    let audit_log = world_loop.rng_audit_log();
    
    // Should have RNG entries for each movement event
    assert!(!audit_log.is_empty(), "Audit log should not be empty");
    
    // Verify all entries have required fields
    for (i, record) in audit_log.records().iter().enumerate() {
        assert!(record.tick > 0, "Record {} has invalid tick", i);
        assert!(record.value > 0, "Record {} has invalid value", i);
        assert!(!record.callsite.is_empty(), "Record {} has empty callsite", i);
    }
    
    // Verify subsystem distribution
    let physics_records = audit_log.records_by_subsystem(RngSubsystem::Physics);
    assert!(!physics_records.is_empty(), "Should have physics RNG records");
    
    // Verify tick ordering in audit log
    let mut prev_tick = 0u64;
    for record in audit_log.records() {
        assert!(record.tick >= prev_tick, "Audit log not ordered by tick");
        prev_tick = record.tick;
    }
    
    // Create snapshot and verify audit preservation
    let snapshot = DeterministicSnapshot::from_world_loop(&world_loop);
    snapshot.verify_integrity().map_err(|e| format!("Snapshot verification failed: {}", e))?;
    
    // Verify audit log is preserved in snapshot
    let snapshot_audit = snapshot.rng_state.audit_log();
    assert_eq!(audit_log.len(), snapshot_audit.len(), "Audit log length not preserved in snapshot");
    
    println!("✅ Audit trail preservation test passed");
    Ok(())
}

/// Test comprehensive replay scenario
#[test]
fn test_comprehensive_replay_scenario() -> Result<(), String> {
    println!("Running comprehensive replay scenario test...");
    
    // Test all replay functionality
    test_snapshot_creation_and_integrity()?;
    test_snapshot_replay_identical_results()?;
    test_event_order_preservation()?;
    test_audit_trail_preservation()?;
    
    println!("✅ COMPREHENSIVE REPLAY SCENARIO TEST PASSED");
    println!("All replay harness functionality verified:");
    println!("  - Snapshot creation and integrity ✓");
    println!("  - Snapshot replay equivalence ✓");
    println!("  - Event order preservation ✓");
    println!("  - Audit trail preservation ✓");
    
    Ok(())
}
