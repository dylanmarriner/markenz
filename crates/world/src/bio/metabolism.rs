use crate::types::Agent;
use crate::bio::complete_biology::{BiologicalState, BioFixed, HealthEvent, HealthMonitor};
use crate::rng::RngStream;

pub struct Environment {
    pub is_cold: bool,
    pub temperature: BioFixed,
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
            // In real implementation, this would affect metabolic_rate
        }
        
        if from_fixed(environment.temperature) > to_fixed(40.0) {
            // Heat causes thirst (not hunger in this implementation)
            // This would be handled by the biological state tick
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
