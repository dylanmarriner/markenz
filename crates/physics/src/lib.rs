//! Physics simulation for MARKENZ
//! 
//! This crate provides physics simulation capabilities for the Markenz system,
//! including collision detection and spatial positioning.
//! 
//! # Features
//! 
//! - 3D collision detection with terrain awareness
//! - Agent positioning and movement validation
//! - Spatial queries and raycasting
//! - Terrain interaction and physics properties
//! - Deterministic physics calculations
//! 
//! # Usage
//! 
//! ```rust
//! use markenz_physics::{Position, Collision};
//! 
//! let pos = Position::new(10.0, 20.0, 0.0);
//! let collision = Collision::check_terrain(&pos, terrain_height)?;
//! ```

/// Collision detection and spatial management
pub mod collision;

/// Re-export public types
pub use collision::*;
