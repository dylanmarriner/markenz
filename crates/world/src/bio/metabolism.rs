use crate::types::Agent;
use crate::bio::{HealthEvent, HealthMonitor};
use rng::RngStream;

pub struct Environment {
    pub is_cold: bool,
    pub temperature: f64,
}

pub struct MetabolicProcessor;

impl MetabolicProcessor {
    pub fn process_tick(
        agent: &mut Agent,
        environment: &Environment,
        rng: &mut RngStream,
    ) -> Result<Vec<HealthEvent>, String> {
        let mut events = Vec::new();
        
        // Baseline metabolism
        agent.bio_state.tick_metabolism();
        
        // Environmental effects
        if environment.is_cold {
            agent.bio_state.metabolic_rate *= 1.2;  // Higher metabolism in cold
        }
        
        if environment.temperature > 40.0 {
            agent.bio_state.hunger += 0.5;  // Heat causes dehydration
        }
        
        // Recovery if idle
        if !agent.is_performing_action() {
            agent.bio_state.apply_recovery();
        }
        
        // Health monitoring
        if let Some(event) = HealthMonitor::check_health(agent, rng) {
            events.push(event);
        }
        
        Ok(events)
    }
}

impl Agent {
    pub fn is_performing_action(&self) -> bool {
        // Simple heuristic - in real implementation this would check action queue
        false  // Assume idle for now
    }
}
