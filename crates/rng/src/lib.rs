//! Deterministic Random Number Generation for MARKENZ
//! 
//! This crate provides cryptographically secure, deterministic random number generation
//! with full audit logging capabilities required for Phase 1 determinism guarantees.
//! 
//! # Features
//! 
//! - ChaCha20-based cryptographically secure RNG
//! - Subsystem-isolated RNG streams (Physics, Biology, Cognition, etc.)
//! - Complete audit logging for all random draws
//! - Deterministic replay and verification support
//! - Reality Lock compliance for Phase 0/1 requirements
//! 
//! # Usage
//! 
//! ```rust
//! use rng::{DeterministicRng, RngSubsystem};
//! 
//! let mut rng = DeterministicRng::new(1337);
//! let mut stream = rng.stream(RngSubsystem::Physics, 0);
//! let random_value = stream.next_u64();
//! ```

//! Deterministic Random Number Generation for MARKENZ
//! 
//! This crate provides cryptographically secure, deterministic random number generation
//! with full audit logging capabilities required for Phase 1 determinism guarantees.
//! 
//! # Features
//! 
//! - ChaCha20-based cryptographically secure RNG
//! - Subsystem-isolated RNG streams (Physics, Biology, Cognition, etc.)
//! - Complete audit logging for all random draws
//! - Deterministic replay and verification support
//! - Reality Lock compliance for Phase 0/1 requirements
//! 
//! # Usage
//! 
//! ```rust
//! use rng::{DeterministicRng, RngSubsystem};
//! 
//! let mut rng = DeterministicRng::new(1337);
//! let mut stream = rng.stream(RngSubsystem::Physics, 0);
//! let random_value = stream.next_u64();
//! ```

/// RFC 7539 ChaCha20 implementation
pub mod chacha20;

/// RNG stream management and subsystem isolation
pub mod rng_stream;

/// Global seed management and audit logging
pub mod global_seed;

/// Audit log for all RNG draws
pub mod audit_log;

/// Enhanced deterministic RNG with audit logging
pub mod deterministic;

/// Re-export commonly used types
pub use chacha20::ChaCha20Rng;
pub use rng_stream::{RngStream, RngSubsystem};
pub use global_seed::GlobalSeed;
pub use audit_log::{RngAuditLog, RngDrawRecord};
pub use deterministic::{DeterministicRng, RngStreamHandle};
