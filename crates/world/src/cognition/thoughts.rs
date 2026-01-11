use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::types::*;
use markenz_events::ObservationEvent;
use super::perception::Perception;
use super::intent::Intent;
use super::memory::AgentMemory;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thought {
    pub tick: u64,
    pub agent_id: u64,
    pub content: String,
    pub emotion: Option<String>,
}

pub fn emit_thought_event(
    tick: u64,
    agent: &Agent,
    perception: &Perception,
    intent: &Intent,
    memory: &AgentMemory,
) -> ObservationEvent {
    let thought_content = format!(
        "Agent {} perceives: {} agents, {} resources. Intent: {:?}. Recall: {} known locations.",
        agent.name,
        perception.nearby_agents.len(),
        perception.nearby_resources.len(),
        intent,
        memory.known_locations.len()
    );
    
    let payload = json!({
        "type": "inner_monologue",
        "agent_id": agent.id,
        "content": thought_content,
        "intent": format!("{:?}", intent),
        "bio_state": {
            "energy": agent.bio_state.energy,
            "hunger": agent.bio_state.hunger,
        },
    });
    
    ObservationEvent {
        tick,
        event_type: "inner_monologue".to_string(),
        payload,
        hash: [0u8; 32],
    }
}
