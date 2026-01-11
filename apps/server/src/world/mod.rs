/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: World state management
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

pub mod voxel;
pub mod chunk;
pub mod world;

pub use voxel::Voxel;
pub use chunk::Chunk;
pub use world::World;
