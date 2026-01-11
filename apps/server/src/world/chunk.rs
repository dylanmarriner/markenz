/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Chunk storage
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub data: Vec<u16>,
}
