use serde::{Deserialize, Serialize};
use crate::types::*;
use super::perception::Perception;
use super::memory::AgentMemory;
use rng::RngStream;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Intent {
    Forage,
    Explore,
    Rest,
    Build,
    Craft,
    Socialize,
    Hunt,
}

pub struct IntentPlanner;

impl IntentPlanner {
    pub fn form_intent(
        agent: &Agent,
        perception: &Perception,
        _memory: &AgentMemory,
        rng: &mut RngStream,
    ) -> Intent {
        if agent.bio_state.hunger > 70.0 {
            if perception.nearby_resources.len() > 0 {
                return Intent::Forage;
            } else {
                return Intent::Explore;
            }
        }
        
        if agent.bio_state.exhaustion > 60.0 {
            return Intent::Rest;
        }
        
        if perception.nearby_agents.len() > 0 && rng.next_f64() > 0.5 {
            return Intent::Socialize;
        }
        
        Intent::Explore
    }
}
