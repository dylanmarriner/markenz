/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server,web
 * PURPOSE: Shared protocol types and events
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub seq: u64,
    pub tick: u64,
    pub kind: EventKind,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    Chat,
    Move,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickUpdate {
    pub tick: u64,
    pub events: Vec<InputEvent>,
}

