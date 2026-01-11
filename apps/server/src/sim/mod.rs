/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic simulation engine
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

pub mod loop_engine;
pub mod events;

#[cfg(test)]
mod tests;

// Re-export from deterministic crate
pub use deterministic::{SimTime, ChaosStream, DeterministicMath, DeterministicMap, DeterministicSet, DeterministicVec};

// Local exports
pub use loop_engine::{SimLoop, TickResult};
pub use events::{Event, EventKind, InputEvent};

