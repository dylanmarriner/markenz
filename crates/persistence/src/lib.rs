//! Persistence layer for Markenz deterministic event sourcing
//! 
//! This crate provides immutable storage and replay capabilities for the Markenz system,
//! ensuring deterministic execution and audit trails.
//! 
//! # Features
//! 
//! - Immutable event storage with hash-chain verification
//! - Snapshot creation and loading for state persistence
//! - Deterministic replay from any point in time
//! - Audit trail preservation and verification
//! - serde serialization for long-term storage
//! 
//! # Usage
//! 
//! ```rust
//! use markenz_persistence::{Database, Snapshot};
//! 
//! let db = Database::new("path/to/events.db")?;
//! let snapshot = db.create_snapshot(tick)?;
//! ```

/// Database module for event storage
pub mod database;

/// Snapshot module for state persistence
pub mod snapshot;

/// Replay module for deterministic execution
pub mod replay;

/// Re-export public API
pub use database::*;
pub use snapshot::*;
pub use replay::*;
