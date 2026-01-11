use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentMemory {
    pub known_locations: BTreeMap<String, (f32, f32)>,
    pub known_agents: BTreeMap<u64, AgentKnowledge>,
    pub experience_log: Vec<MemoryTrace>,
    pub skill_levels: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentKnowledge {
    pub agent_id: u64,
    pub name: String,
    pub last_seen_tick: u64,
    pub trust_level: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemoryTrace {
    pub tick: u64,
    pub event: String,
    pub location: (f32, f32),
    pub outcome: bool,
}

impl AgentMemory {
    pub fn new() -> Self {
        Self {
            known_locations: BTreeMap::new(),
            known_agents: BTreeMap::new(),
            experience_log: Vec::new(),
            skill_levels: BTreeMap::new(),
        }
    }
    
    pub fn record_event(
        &mut self,
        tick: u64,
        event: String,
        location: (f32, f32),
        outcome: bool,
    ) {
        self.experience_log.push(MemoryTrace {
            tick,
            event,
            location,
            outcome,
        });
    }
    
    pub fn recall_location(&self, location_name: &str) -> Option<(f32, f32)> {
        self.known_locations.get(location_name).copied()
    }
}
