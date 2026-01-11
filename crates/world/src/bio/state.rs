use serde::{Deserialize, Serialize};
use markenz_events::InputEventPayload;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BioState {
    pub energy: f64,                    // 0-100 (joules-like units)
    pub hunger: f64,                    // 0-100 (100 = starving)
    pub exhaustion: f64,                // 0-100 (100 = collapsed)
    pub health: f64,                    // 0-100 (100 = perfect)
    pub metabolic_rate: f64,            // Rate of energy consumption per tick
    pub recovery_rate: f64,             // Recovery from exhaustion when idle
}

impl BioState {
    pub fn new() -> Self {
        Self {
            energy: 100.0,
            hunger: 0.0,
            exhaustion: 0.0,
            health: 100.0,
            metabolic_rate: 0.5,           // Energy per tick
            recovery_rate: 0.1,            // Exhaustion recovery per tick at rest
        }
    }
    
    pub fn can_perform_action(&self, action: &InputEventPayload) -> Result<(), String> {
        let energy_cost = self.action_cost(action);
        
        if self.energy < energy_cost {
            return Err("Insufficient energy".to_string());
        }
        
        if self.exhaustion > 80.0 && self.is_intensive(action) {
            return Err("Too exhausted for intensive action".to_string());
        }
        
        if self.health < 20.0 {
            return Err("Too injured to act".to_string());
        }
        
        Ok(())
    }
    
    pub fn action_cost(&self, action: &InputEventPayload) -> f64 {
        match action {
            InputEventPayload::Move { .. } => 0.0,
            InputEventPayload::Chat { .. } => 0.0,
            InputEventPayload::Gather { .. } => 5.0,
            InputEventPayload::Craft { .. } => 5.0,
            InputEventPayload::Mine => 5.0,
            InputEventPayload::Build { .. } => 10.0,
            // Phase 0 required events - no energy cost
            InputEventPayload::BootEvent
            | InputEventPayload::TickAdvance
            | InputEventPayload::InputEventSubmitted
            | InputEventPayload::ObservationEvent
            | InputEventPayload::SnapshotTaken => 0.0,
        }
    }
    
    fn is_intensive(&self, action: &InputEventPayload) -> bool {
        matches!(action, 
            InputEventPayload::Mine | InputEventPayload::Build { .. }
        )
    }
    
    pub fn consume_energy(&mut self, amount: f64) {
        self.energy = (self.energy - amount).max(0.0);
        self.exhaustion = (self.exhaustion + amount * 0.1).min(100.0);
    }
    
    pub fn tick_metabolism(&mut self) {
        self.energy = (self.energy - self.metabolic_rate).max(0.0);
        
        if self.energy < 30.0 {
            self.hunger = (self.hunger + 1.0).min(100.0);
        } else if self.hunger > 0.0 {
            self.hunger = (self.hunger - 0.5).max(0.0);
        }
    }
    
    pub fn apply_recovery(&mut self) {
        if self.exhaustion > 0.0 {
            self.exhaustion = (self.exhaustion - self.recovery_rate).max(0.0);
        }
    }
}
