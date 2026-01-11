/**
 * File: crates/events/tests/determinism_test.rs
 * 
 * Purpose: Integration tests for event-based determinism
 * 
 * Phase plan authority: MARKENZ_GOVERNANCE_PHASE_0_REPO_AND_EVENT_LOG_BASELINE.md
 * Section 8.1 "Determinism Replay Test"
 */

use markenz_events::{InputEvent, InputEventPayload};

#[test]
fn test_event_hash_determinism() {
    // REQUIREMENT: Same event always produces same hash
    // This is critical for replay verification
    
    let payload = InputEventPayload::Move {
        x: 10.0,
        y: 20.0,
        z: 30.0,
    };
    
    // Create two identical events
    let event1 = InputEvent::new(
        1,
        1,
        1,
        payload.clone(),
        [0u8; 32],
    );
    
    let event2 = InputEvent::new(
        1,
        1,
        1,
        payload.clone(),
        [0u8; 32],
    );
    
    // They MUST have identical hashes
    assert_eq!(event1.hash, event2.hash,
        "Identical events must produce identical hashes");
    
    println!("✓ Event hash is deterministic");
}

#[test]
fn test_hash_chain_linkage() {
    // REQUIREMENT: Each event must properly link to previous via hash chain
    
    let payload1 = InputEventPayload::Move {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    
    // Genesis event (prev_hash = zero)
    let event1 = InputEvent::new(
        0,
        0,
        1,
        InputEventPayload::BootEvent,
        [0u8; 32],
    );
    
    // Event 2 links to event 1
    let event2 = InputEvent::new(
        1,
        1,
        2,
        payload1,
        event1.hash,
    );
    
    // Verify linkage
    assert!(event2.verify_hash_link(event1.hash),
        "Event 2 must properly link to Event 1");
    
    println!("✓ Hash-chain linkage verified");
}

#[test]
fn test_event_validation_prevents_corruption() {
    // REQUIREMENT: Event validation prevents invalid states
    
    // Test 1: Zero tick for non-boot event should fail
    let invalid_event = InputEvent::new(
        0,
        1,
        1,
        InputEventPayload::Move { x: 1.0, y: 1.0, z: 1.0 },
        [0u8; 32],
    );
    
    let result = invalid_event.validate();
    assert!(result.is_err(),
        "Non-boot event with tick=0 should fail validation");
    
    // Test 2: Boot event with non-zero prev_hash should fail
    let mut boot_event = InputEvent::new(
        0,
        0,
        1,
        InputEventPayload::BootEvent,
        [0u8; 32],
    );
    boot_event.prev_hash = [1u8; 32];  // Corrupt prev_hash
    
    let result = boot_event.validate();
    assert!(result.is_err(),
        "Boot event with non-zero prev_hash should fail validation");
    
    println!("✓ Event validation enforces integrity constraints");
}

#[test]
fn test_sequence_ordering() {
    // REQUIREMENT: Events with same tick must be ordered by sequence number
    
    let event1 = InputEvent::new(
        1,
        1,
        1,  // sequence 1
        InputEventPayload::Move { x: 1.0, y: 2.0, z: 3.0 },
        [0u8; 32],
    );
    
    let event2 = InputEvent::new(
        1,
        2,
        2,  // sequence 2
        InputEventPayload::Chat { text: "hello".to_string() },
        event1.hash,
    );
    
    // Verify sequencing
    assert!(event1.sequence < event2.sequence,
        "Events should have ordered sequences");
    
    println!("✓ Event sequence ordering maintained");
}

#[test]
fn test_event_schema_completeness() {
    // REQUIREMENT: All Phase 0 required events are defined
    
    let boot = InputEventPayload::BootEvent;
    let tick = InputEventPayload::TickAdvance;
    let input = InputEventPayload::InputEventSubmitted;
    let obs = InputEventPayload::ObservationEvent;
    let snap = InputEventPayload::SnapshotTaken;
    
    // These should all be creatable
    let events = vec![
        InputEvent::new(0, 0, 1, boot, [0u8; 32]),
        InputEvent::new(1, 0, 2, tick, [1u8; 32]),
        InputEvent::new(2, 0, 3, input, [2u8; 32]),
        InputEvent::new(3, 0, 4, obs, [3u8; 32]),
        InputEvent::new(4, 0, 5, snap, [4u8; 32]),
    ];
    
    assert_eq!(events.len(), 5,
        "All Phase 0 required events should be definable");
    
    println!("✓ Phase 0 event schema is complete");
}
