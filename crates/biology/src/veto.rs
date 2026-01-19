/*!
# BioVeto System

**Purpose:** Centralized biological veto enforcement for agent actions.

**Why it exists:** Biological constraints must be able to prevent actions that
would be physically impossible, dangerous, or biologically detrimental.
This system provides the unified interface for all biological veto logic.

**Determinism guarantees:**
- Veto decisions are based on deterministic biological thresholds
- No random or unpredictable veto behavior
- All veto reasons are auditable and traceable
- Veto logic is consistent across all biological systems

**How it affects replay:** Same biological states will produce identical
veto decisions across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

use super::state::{BioVetoReason, BiologicalState};

/// Veto severity levels determining action blocking behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VetoSeverity {
    /// Warning only - action allowed but with reduced effectiveness
    Warning,
    /// Soft veto - action discouraged, agent may override with consequences
    Soft,
    /// Hard veto - action completely blocked, no override possible
    Hard,
    /// Critical veto - emergency override only, severe consequences
    Critical,
}

impl VetoSeverity {
    /// Whether this veto completely blocks the action
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Hard | Self::Critical)
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Soft => "soft veto",
            Self::Hard => "hard veto",
            Self::Critical => "critical veto",
        }
    }
}

/// Veto category for grouping related veto reasons
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VetoCategory {
    /// Energy-related vetoes (fatigue, exhaustion)
    Energy,
    /// Health-related vetoes (injury, illness)
    Health,
    /// Resource-related vetoes (hunger, thirst)
    Resources,
    /// Hormonal vetoes (stress, emotional state)
    Hormonal,
    /// Reproductive vetoes (pregnancy, cycle)
    Reproductive,
    /// Environmental vetoes (temperature, conditions)
    Environmental,
    /// Emergency vetoes (life-threatening situations)
    Emergency,
}

impl VetoCategory {
    /// Get severity for this category based on biological state
    pub fn default_severity(self) -> VetoSeverity {
        match self {
            Self::Energy => VetoSeverity::Soft,
            Self::Health => VetoSeverity::Hard,
            Self::Resources => VetoSeverity::Soft,
            Self::Hormonal => VetoSeverity::Warning,
            Self::Reproductive => VetoSeverity::Soft,
            Self::Environmental => VetoSeverity::Hard,
            Self::Emergency => VetoSeverity::Critical,
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Energy => "energy constraint",
            Self::Health => "health constraint",
            Self::Resources => "resource constraint",
            Self::Hormonal => "hormonal constraint",
            Self::Reproductive => "reproductive constraint",
            Self::Environmental => "environmental constraint",
            Self::Emergency => "emergency constraint",
        }
    }
}

/// Individual veto with complete context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BioVeto {
    /// Reason for the veto
    pub reason: BioVetoReason,
    /// Category of the veto
    pub category: VetoCategory,
    /// Severity level
    pub severity: VetoSeverity,
    /// Biological values that triggered the veto
    pub triggering_values: BTreeMap<String, f64>,
    /// Tick when veto was issued
    pub tick: u64,
    /// Whether action was actually blocked
    pub action_blocked: bool,
}

impl BioVeto {
    /// Create a new veto
    pub fn new(reason: BioVetoReason, category: VetoCategory, tick: u64) -> Self {
        let severity = category.default_severity();
        let action_blocked = severity.is_blocking();
        
        Self {
            reason,
            category,
            severity,
            triggering_values: BTreeMap::new(),
            tick,
            action_blocked,
        }
    }

    /// Add triggering value for audit logging
    pub fn with_triggering_value(mut self, name: String, value: f64) -> Self {
        self.triggering_values.insert(name, value);
        self
    }

    /// Check if this veto blocks the action
    pub fn blocks_action(&self) -> bool {
        self.action_blocked
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("{}: {} ({})", 
                self.severity.description(),
                self.category.description(),
                format!("{:?}", self.reason).to_lowercase())
    }
}

/// Complete BioVeto system
/// Evaluates biological constraints and issues vetoes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BioVetoSystem {
    /// Current active vetoes
    active_vetoes: Vec<BioVeto>,
    /// Historical veto log for audit
    veto_history: Vec<BioVeto>,
    /// Maximum history size to prevent memory issues
    max_history_size: usize,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl BioVetoSystem {
    /// Create new BioVeto system
    pub fn new() -> Self {
        Self {
            active_vetoes: Vec::new(),
            veto_history: Vec::new(),
            max_history_size: 1000,
            tick: 0,
        }
    }

    /// Evaluate biological state and return current vetoes
    pub fn evaluate_vetoes(&mut self, biological_state: &BiologicalState) -> Vec<BioVeto> {
        let veto_reasons = biological_state.check_action_viability();
        let mut new_vetoes = Vec::new();
        
        for reason in veto_reasons {
            let category = self.categorize_reason(&reason);
            let mut veto = BioVeto::new(reason, category, self.tick);
            
            // Add triggering values for audit logging
            veto = self.add_triggering_values(veto, biological_state);
            
            new_vetoes.push(veto);
        }
        
        // Update active vetoes
        self.active_vetoes = new_vetoes.clone();
        
        // Add to history
        for veto in &new_vetoes {
            self.add_to_history(veto.clone());
        }
        
        self.tick += 1;
        
        debug!("Evaluated {} biological vetoes", new_vetoes.len());
        trace!("Active vetoes: {:?}", new_vetoes);
        
        new_vetoes
    }

    /// Categorize a veto reason
    fn categorize_reason(&self, reason: &BioVetoReason) -> VetoCategory {
        match reason {
            // Energy-related vetoes
            BioVetoReason::CompleteExhaustion
            | BioVetoReason::SevereFatigue
            | BioVetoReason::CircadianSleepPressure
            | BioVetoReason::FatigueImbalance => VetoCategory::Energy,
            
            // Health-related vetoes
            BioVetoReason::CriticalInjury
            | BioVetoReason::SevereMobilityImpairment
            | BioVetoReason::SevereCognitiveImpairment
            | BioVetoReason::ExtremePain
            | BioVetoReason::InjuryLimitation
            | BioVetoReason::CriticalHealthFailure => VetoCategory::Health,
            
            // Resource-related vetoes
            BioVetoReason::SevereDehydration
            | BioVetoReason::SevereNutrientDeficiency(_)
            | BioVetoReason::ExtremeCalorieDeficit
            | BioVetoReason::NutritionalImbalance
            | BioVetoReason::CriticalDehydration
            | BioVetoReason::UrgentWasteElimination(_)
            | BioVetoReason::HydrationImbalance => VetoCategory::Resources,
            
            // Hormonal vetoes
            BioVetoReason::StressOverload
            | BioVetoReason::Anhedonia
            | BioVetoReason::ExcessiveFatigue
            | BioVetoReason::LowConfidence
            | BioVetoReason::HormonalImbalance => VetoCategory::Hormonal,
            
            // Reproductive vetoes
            BioVetoReason::LatePregnancyLimitations
            | BioVetoReason::SevereMenstrualSymptoms
            | BioVetoReason::ReproductiveLimitations => VetoCategory::Reproductive,
            
            // Environmental vetoes (categorize remaining energy/metabolic vetoes)
            BioVetoReason::EnergyExhaustion
            | BioVetoReason::ThermoregulatoryFailure
            | BioVetoReason::MetabolicImbalance => VetoCategory::Environmental,
            
            // Emergency vetoes (default for uncategorized)
            _ => VetoCategory::Emergency,
        }
    }

    /// Add relevant triggering values to a veto
    fn add_triggering_values(&self, mut veto: BioVeto, biological_state: &BiologicalState) -> BioVeto {
        match veto.reason {
            BioVetoReason::CompleteExhaustion => {
                veto = veto.with_triggering_value("energy_level".to_string(), biological_state.fatigue.energy_level);
            }
            BioVetoReason::CriticalDehydration => {
                veto = veto.with_triggering_value("hydration_level".to_string(), biological_state.hydration.hydration_percentage);
            }
            BioVetoReason::CriticalInjury => {
                veto = veto.with_triggering_value("pain_level".to_string(), biological_state.injury.overall_pain_level);
            }
            BioVetoReason::StressOverload => {
                veto = veto.with_triggering_value("cortisol_level".to_string(), 
                    biological_state.endocrine.get_level(super::endocrine::HormoneType::Cortisol));
            }
            BioVetoReason::CriticalHealthFailure => {
                veto = veto.with_triggering_value("overall_health".to_string(), biological_state.overall_health);
            }
            // Add more specific triggering values as needed
            _ => {}
        }
        
        veto
    }

    /// Add veto to history with size management
    fn add_to_history(&mut self, veto: BioVeto) {
        self.veto_history.push(veto);
        
        // Maintain maximum history size
        if self.veto_history.len() > self.max_history_size {
            self.veto_history.remove(0);
        }
    }

    /// Check if any active vetoes block the action
    pub fn has_blocking_veto(&self) -> bool {
        self.active_vetoes.iter().any(|v| v.blocks_action())
    }

    /// Get most severe active veto
    pub fn get_most_severe_veto(&self) -> Option<&BioVeto> {
        self.active_vetoes
            .iter()
            .max_by(|a, b| a.severity.cmp(&b.severity))
    }

    /// Get all blocking vetoes
    pub fn get_blocking_vetoes(&self) -> Vec<&BioVeto> {
        self.active_vetoes
            .iter()
            .filter(|v| v.blocks_action())
            .collect()
    }

    /// Get all warning vetoes
    pub fn get_warning_vetoes(&self) -> Vec<&BioVeto> {
        self.active_vetoes
            .iter()
            .filter(|v| v.severity == VetoSeverity::Warning)
            .collect()
    }

    /// Get veto statistics
    pub fn get_veto_statistics(&self) -> VetoStatistics {
        let mut category_counts = BTreeMap::new();
        let mut severity_counts = BTreeMap::new();
        
        for veto in &self.active_vetoes {
            *category_counts.entry(veto.category).or_insert(0) += 1;
            *severity_counts.entry(veto.severity).or_insert(0) += 1;
        }
        
        VetoStatistics {
            total_active_vetoes: self.active_vetoes.len(),
            has_blocking_veto: self.has_blocking_veto(),
            most_severe_veto: self.get_most_severe_veto().map(|v| v.severity),
            category_counts,
            severity_counts,
            total_historical_vetoes: self.veto_history.len(),
        }
    }

    /// Clear all active vetoes (for new action evaluation)
    pub fn clear_active_vetoes(&mut self) {
        self.active_vetoes.clear();
        debug!("Cleared all active biological vetoes");
    }

    /// Get veto history for audit
    pub fn get_veto_history(&self) -> &[BioVeto] {
        &self.veto_history
    }

    /// Get recent vetoes within specified tick range
    pub fn get_recent_vetoes(&self, tick_range: u64) -> Vec<&BioVeto> {
        let current_tick = self.tick;
        self.veto_history
            .iter()
            .filter(|v| current_tick.saturating_sub(v.tick) <= tick_range)
            .collect()
    }
}

/// Statistics about current veto state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VetoStatistics {
    /// Total number of active vetoes
    pub total_active_vetoes: usize,
    /// Whether any blocking vetoes exist
    pub has_blocking_veto: bool,
    /// Most severe veto level
    pub most_severe_veto: Option<VetoSeverity>,
    /// Count of vetoes by category
    pub category_counts: BTreeMap<VetoCategory, usize>,
    /// Count of vetoes by severity
    pub severity_counts: BTreeMap<VetoSeverity, usize>,
    /// Total vetoes in history
    pub total_historical_vetoes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::state::BiologicalState;
    use super::reproductive::BiologicalSex;

    #[test]
    fn test_veto_system_creation() {
        let system = BioVetoSystem::new();
        
        assert_eq!(system.active_vetoes.len(), 0);
        assert_eq!(system.veto_history.len(), 0);
        assert!(!system.has_blocking_veto());
    }

    #[test]
    fn test_veto_evaluation() {
        let mut veto_system = BioVetoSystem::new();
        let mut biological_state = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        // Create a state that should trigger vetoes
        biological_state.fatigue.energy_level = 0.05; // Extreme exhaustion
        
        let vetoes = veto_system.evaluate_vetoes(&biological_state);
        
        assert!(!vetoes.is_empty());
        assert!(veto_system.has_blocking_veto());
    }

    #[test]
    fn test_veto_categorization() {
        let system = BioVetoSystem::new();
        
        let energy_veto = system.categorize_reason(&BioVetoReason::CompleteExhaustion);
        assert_eq!(energy_veto, VetoCategory::Energy);
        
        let health_veto = system.categorize_reason(&BioVetoReason::CriticalInjury);
        assert_eq!(health_veto, VetoCategory::Health);
    }

    #[test]
    fn test_veto_statistics() {
        let mut veto_system = BioVetoSystem::new();
        let mut biological_state = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        // Create multiple vetoes
        biological_state.fatigue.energy_level = 0.05;
        biological_state.hydration.hydration_percentage = 0.2;
        
        veto_system.evaluate_vetoes(&biological_state);
        
        let stats = veto_system.get_veto_statistics();
        assert!(stats.total_active_vetoes > 0);
        assert!(stats.has_blocking_veto);
    }

    #[test]
    fn test_veto_history() {
        let mut veto_system = BioVetoSystem::new();
        let biological_state = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        // Generate some vetoes
        for _ in 0..5 {
            veto_system.evaluate_vetoes(&biological_state);
        }
        
        let history = veto_system.get_veto_history();
        assert!(history.len() > 0);
    }
}
