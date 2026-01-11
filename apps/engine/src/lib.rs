//! MARKENZ Engine - Deterministic World Loop and Authority Pipeline
//! 
//! This crate provides the core engine components for Phase 0/1 deterministic execution:
//! 
//! - Authority Pipeline: 13-pass validation and commit system for InputEvents
//! - Deterministic World Loop: Fixed-timestep execution with hash-chain verification
//! - RNG Audit Integration: Complete logging of all random number generation
//! 
//! # Authority
//! 
//! All world state mutations must pass through the authority pipeline to ensure
//! deterministic replay and Phase 0 compliance.
//! 
//! # Usage
//! 
//! ```rust
//! use markenz_engine::{DeterministicWorldLoop, DeterministicWorldConfig};
//! 
//! let config = DeterministicWorldConfig::default();
//! let mut world_loop = DeterministicWorldLoop::new(config);
//! world_loop.run(input_events)?;
//! ```

//! Authority Pipeline for deterministic InputEvent processing
pub mod authority_pipeline;

/// Deterministic World Loop with hash-chain verification
pub mod deterministic_world_loop;

// Re-export commonly used types
pub use markenz_world::Universe;
pub use markenz_events::{InputEvent, ObservationEvent};
pub use markenz_persistence::{snapshot_write, snapshot_read};
