/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic time source
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimTime(pub u64);

impl SimTime {
    pub fn zero() -> Self {
        SimTime(0)
    }
    
    pub fn tick(&mut self) {
        self.0 += 1;
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    
    pub fn from_u64(t: u64) -> Self {
        SimTime(t)
    }
}
