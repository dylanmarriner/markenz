use serde::{Deserialize, Serialize};
use crate::types::Agent;
use rng::RngStream;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HealthEvent {
    EnergyDepleted,
    Exhausted,
    Starving,
    Injured(f64),  // damage amount
    Recovered,
}

pub struct HealthMonitor;

impl HealthMonitor {
    pub fn check_health(
        agent: &Agent,
        rng: &mut RngStream,
    ) -> Option<HealthEvent> {
        if agent.bio_state.energy == 0.0 {
            return Some(HealthEvent::EnergyDepleted);
        }
        
        if agent.bio_state.exhaustion > 95.0 {
            return Some(HealthEvent::Exhausted);
        }
        
        if agent.bio_state.hunger > 90.0 {
            // Starving: take health damage
            let damage = rng.next_f64() * 5.0;
            if agent.bio_state.health - damage <= 0.0 {
                return Some(HealthEvent::Injured(damage));
            }
        }
        
        None
    }
}
