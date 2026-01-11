//! Events for the MARKENZ deterministic world
//! 
//! This crate provides the canonical event system for Phase 0/1.
//! All state changes must flow through InputEvents with hash-chain verification.
//! 
//! # Features
//! 
//! - Deterministic InputEvent schema with hash chaining
//! - ObservationEvent generation for state change notifications
//! - Immutable event storage with audit trails
//! - Event validation and business rule enforcement
//! - serde serialization for persistence and replay

//! # Usage
//! 
//! ```rust
//! use markenz_events::{InputEvent, InputEventPayload};
//! 
//! let event = InputEvent {
//!     tick: 100,
//!     source_agent_id: 1,
//!     sequence: 1,
//!     payload: InputEventPayload::Move { x: 10.0, y: 20.0, z: 0.0 },
//!     hash: [0u8; 32],
//!     prev_hash: [0u8; 32],
//! };
//! ```

/// Input event module
pub mod input_event;

/// Observation event module  
pub mod observation_event;

/// Re-export commonly used types
pub use input_event::*;
pub use observation_event::*;
