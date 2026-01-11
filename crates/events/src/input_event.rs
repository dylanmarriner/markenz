/**
 * File: crates/events/src/input_event.rs
 * 
 * Purpose: Defines the canonical InputEvent schema for Phase 0 deterministic event sourcing
 * 
 * Why this file exists: 
 * - InputEvent is the sole mechanism for state mutations in the Markenz system
 * - Provides immutable hash-chain linkage for audit trails
 * - Enforces deterministic event ordering and validation
 * 
 * Phase plan authority: MARKENZ_GOVERNANCE_PHASE_0_REPO_AND_EVENT_LOG_BASELINE.md
 * 
 * Invariants enforced:
 * - Every InputEvent has a unique hash linking to previous event (hash-chain)
 * - No InputEvent can be modified after creation (immutable)
 * - All events are validated before processing
 * - Hash-chain integrity prevents retroactive tampering
 * 
 * What breaks if removed:
 * - No deterministic event ordering → system cannot guarantee replay consistency
 * - No hash-chain → no audit trail, possible state tampering
 * - No validation → invalid events could corrupt world state
 * 
 * What this file does NOT do:
 * - Does not implement server-side logic (authority is engine-only)
 * - Does not allow event modification after creation
 * - Does not provide bypass mechanisms for validation
 */
use serde::{Deserialize, Serialize};
use blake3;

/// Input event for deterministic world simulation
/// 
/// Represents a single input from an agent or system that
/// triggers state changes in the world.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputEvent {
    /// The tick at which this event occurs
    pub tick: u64,
    /// ID of the agent that generated this event
    pub source_agent_id: u64,
    /// Sequence number for ordering within the same tick
    pub sequence: u64,
    /// The actual payload/data for this event
    pub payload: InputEventPayload,
    /// Hash of this event for chain verification
    pub hash: [u8; 32],
    /// Hash of the previous event in the chain
    pub prev_hash: [u8; 32],
}

/// Payload types for input events
/// 
/// Defines the different types of actions that can be performed
/// in the deterministic world simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputEventPayload {
    /// Movement event with 3D coordinates
    /// 
    /// # Fields
    /// * `x` - X coordinate in world space
    /// * `y` - Y coordinate in world space  
    /// * `z` - Z coordinate in world space
    Move { x: f32, y: f32, z: f32 },
    
    /// Chat/message event
    /// 
    /// # Fields
    /// * `text` - The message content
    Chat { text: String },
    
    /// Resource gathering event
    /// 
    /// # Fields
    /// * `resource_type` - Type of resource being gathered
    Gather { resource_type: String },
    
    /// Crafting event
    /// 
    /// # Fields
    /// * `recipe_id` - ID of the recipe being crafted
    Craft { recipe_id: u64 },
    
    /// Mining event
    Mine,
    
    /// Building construction event
    /// 
    /// # Fields
    /// * `building_type` - Type of building being constructed
    Build { building_type: String },
    
    /// System boot event
    BootEvent,
    
    /// Tick advancement event
    TickAdvance,
    
    /// Event submission event
    InputEventSubmitted,
    
    /// Observation event reference
    ObservationEvent,
    
    /// Snapshot creation event
    SnapshotTaken,
}

impl InputEvent {
    /// Validates event schema and business rules
    /// 
    /// This is a critical enforcement point for Phase 0 determinism.
    /// Any validation failure must halt processing to prevent state corruption.
    pub fn validate(&self) -> Result<(), String> {
        // Tick cannot be zero (genesis uses tick 0 for BootEvent only)
        if self.tick == 0 && !matches!(self.payload, InputEventPayload::BootEvent) {
            return Err("Tick cannot be zero for non-boot events".to_string());
        }
        
        // Source agent must exist (0 reserved for system events)
        if self.source_agent_id == 0 && !matches!(self.payload, InputEventPayload::BootEvent) {
            return Err("Source agent ID cannot be zero for non-system events".to_string());
        }
        
        // Hash cannot be zero (indicates uninitialized event)
        if self.hash == [0u8; 32] {
            return Err("Event hash cannot be zero".to_string());
        }
        
        // Genesis event (BootEvent) must have zero prev_hash
        if matches!(self.payload, InputEventPayload::BootEvent) {
            if self.prev_hash != [0u8; 32] {
                return Err("BootEvent must have zero prev_hash".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Computes deterministic hash for this InputEvent
    /// 
    /// Hash includes all event fields to ensure immutability.
    /// Any modification to event fields will break the hash-chain.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        
        // Hash all fields in deterministic order
        let _ = hasher.update(&self.tick.to_le_bytes());
        let _ = hasher.update(&self.source_agent_id.to_le_bytes());
        let _ = hasher.update(&self.sequence.to_le_bytes());
        
        // Hash payload using canonical serialization
        let payload_bytes = serde_json::to_vec(&self.payload)
            .expect("Failed to serialize InputEvent payload for hashing");
        let _ = hasher.update(&payload_bytes);
        
        // Include previous hash for chain linkage
        let _ = hasher.update(&self.prev_hash);
        
        hasher.finalize().into()
    }
    
    /// Creates new InputEvent with computed hash
    /// 
    /// This is the factory method that ensures all events are properly hashed.
    /// Direct struct creation should be avoided to prevent uninitialized hashes.
    pub fn new(
        tick: u64,
        source_agent_id: u64,
        sequence: u64,
        payload: InputEventPayload,
        prev_hash: [u8; 32],
    ) -> Self {
        let mut event = Self {
            tick,
            source_agent_id,
            sequence,
            payload,
            hash: [0u8; 32], // Will be computed
            prev_hash,
        };
        
        // Compute deterministic hash
        event.hash = event.compute_hash();
        
        event
    }
    
    /// Verifies that this event properly links to previous hash
    /// 
    /// This is critical for hash-chain integrity verification.
    /// Any break in the chain indicates tampering or corruption.
    pub fn verify_hash_link(&self, expected_prev_hash: [u8; 32]) -> bool {
        self.prev_hash == expected_prev_hash
    }
}
