/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic event system
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */


use serde::{Deserialize, Serialize};
use deterministic::SimTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub seq: u64,
    pub tick: SimTime,
    pub kind: EventKind,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventKind {
    Input(InputEvent),
    Tick,
    AgentSpawn { agent_id: String },
    AgentMove { agent_id: String, x: f32, y: f32, z: f32 },
    Chat { agent_id: String, message: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    Chat { user_id: String, message: String },
    Move { user_id: String, direction: (f32, f32, f32) },
    AdminCommand { command: AdminCommand },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdminCommand {
    Pause,
    Resume,
    Step,
    SetSpeed { multiplier: f32 },
    SpawnAgent { agent_id: String },
}

impl Event {
    pub fn new(seq: u64, tick: SimTime, kind: EventKind) -> Self {
        let payload = serde_json::to_string(&kind).expect("EventKind must be serializable");
        Self { seq, tick, kind, payload }
    }
    
    pub fn input(seq: u64, tick: SimTime, input: InputEvent) -> Self {
        Self::new(seq, tick, EventKind::Input(input))
    }
}
