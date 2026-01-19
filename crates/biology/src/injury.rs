/*!
# Injury System

**Purpose:** Deterministic injury modeling and healing processes for human-equivalent biology.

**Why it exists:** Injuries are critical biological events that affect mobility,
cognitive function, and survival. Agents must experience realistic wound mechanics,
pain responses, and healing timelines to maintain human equivalence.

**Determinism guarantees:**
- Injury effects follow fixed physiological models
- Healing rates are deterministic based on biological state
- No random fluctuations in injury progression
- All injury changes have identifiable sources

**How it affects replay:** Same sequence of injuries and biological conditions will
produce identical healing trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Injury severity levels with corresponding physiological effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InjurySeverity {
    /// Minor injury (scratch, bruise)
    Minor,
    /// Moderate injury (deep cut, sprain)
    Moderate,
    /// Severe injury (fracture, deep wound)
    Severe,
    /// Critical injury (life-threatening)
    Critical,
}

impl InjurySeverity {
    /// Pain level (0.0 = no pain, 1.0 = extreme pain)
    pub const fn pain_level(self) -> f64 {
        match self {
            Self::Minor => 0.2,
            Self::Moderate => 0.5,
            Self::Severe => 0.8,
            Self::Critical => 1.0,
        }
    }

    /// Mobility impairment (0.0 = no impairment, 1.0 = completely immobilized)
    pub const fn mobility_impairment(self) -> f64 {
        match self {
            Self::Minor => 0.05,
            Self::Moderate => 0.2,
            Self::Severe => 0.6,
            Self::Critical => 0.9,
        }
    }

    /// Cognitive impairment (0.0 = no impairment, 1.0 = severely impaired)
    pub const fn cognitive_impairment(self) -> f64 {
        match self {
            Self::Minor => 0.0,
            Self::Moderate => 0.1,
            Self::Severe => 0.3,
            Self::Critical => 0.7,
        }
    }

    /// Base healing time in ticks (assuming optimal conditions)
    pub const fn base_healing_ticks(self) -> u64 {
        match self {
            Self::Minor => 5000,     // ~83 minutes
            Self::Moderate => 20000,  // ~5.5 hours
            Self::Severe => 80000,    // ~22 hours
            Self::Critical => 200000,  // ~55 hours
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Minor => "minor injury",
            Self::Moderate => "moderate injury",
            Self::Severe => "severe injury",
            Self::Critical => "critical injury",
        }
    }
}

/// Body regions that can be injured
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BodyRegion {
    /// Head and brain
    Head,
    /// Torso and vital organs
    Torso,
    /// Arms and hands
    Arms,
    /// Legs and feet
    Legs,
    /// Internal systems
    Internal,
}

impl BodyRegion {
    /// Criticality factor for survival (higher = more dangerous)
    pub const fn criticality_factor(self) -> f64 {
        match self {
            Self::Head => 1.0,    // Most critical
            Self::Torso => 0.8,    // Very critical
            Self::Internal => 0.9,  // Critical
            Self::Arms => 0.3,     // Less critical
            Self::Legs => 0.4,     // Less critical
        }
    }

    /// Mobility impact factor
    pub const fn mobility_impact_factor(self) -> f64 {
        match self {
            Self::Head => 0.1,     // Minimal mobility impact
            Self::Torso => 0.3,    // Moderate impact
            Self::Internal => 0.2,   // Low impact
            Self::Arms => 0.6,     // High impact
            Self::Legs => 1.0,     // Maximum impact
        }
    }

    /// Name for logging and display
    pub const fn name(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Torso => "torso",
            Self::Arms => "arms",
            Self::Legs => "legs",
            Self::Internal => "internal",
        }
    }
}

/// Healing stages with different recovery characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HealingStage {
    /// Fresh injury with bleeding
    Inflammatory,
    /// Cleaning and preparation
    Debridement,
    /// Tissue rebuilding
    Proliferation,
    /// Strengthening and remodeling
    Maturation,
    /// Fully healed
    Healed,
}

impl HealingStage {
    /// Healing rate multiplier for this stage
    pub const fn healing_multiplier(self) -> f64 {
        match self {
            Self::Inflammatory => 0.5,   // Slow initial healing
            Self::Debridement => 0.7,    // Moderate healing
            Self::Proliferation => 1.2,   // Rapid tissue growth
            Self::Maturation => 0.8,     // Slower strengthening
            Self::Healed => 0.0,         // No healing needed
        }
    }

    /// Pain level for this stage
    pub const fn pain_level(self) -> f64 {
        match self {
            Self::Inflammatory => 0.8,    // High pain
            Self::Debridement => 0.6,     // Moderate pain
            Self::Proliferation => 0.3,    // Low pain
            Self::Maturation => 0.1,       // Minimal pain
            Self::Healed => 0.0,          // No pain
        }
    }

    /// Infection risk for this stage
    pub const fn infection_risk(self) -> f64 {
        match self {
            Self::Inflammatory => 0.3,    // High risk
            Self::Debridement => 0.2,     // Moderate risk
            Self::Proliferation => 0.1,    // Low risk
            Self::Maturation => 0.05,     // Very low risk
            Self::Healed => 0.0,          // No risk
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Inflammatory => "inflammatory stage",
            Self::Debridement => "debridement stage",
            Self::Proliferation => "proliferation stage",
            Self::Maturation => "maturation stage",
            Self::Healed => "healed",
        }
    }
}

/// Individual injury with complete tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wound {
    /// Unique identifier for this injury
    pub id: u64,
    /// Body region affected
    pub region: BodyRegion,
    /// Injury severity
    pub severity: InjurySeverity,
    /// Current healing stage
    pub healing_stage: HealingStage,
    /// Progress through current stage (0.0 = start, 1.0 = complete)
    pub stage_progress: f64,
    /// Overall healing progress (0.0 = fresh, 1.0 = fully healed)
    pub healing_progress: f64,
    /// Pain level (0.0 = no pain, 1.0 = extreme pain)
    pub pain_level: f64,
    /// Tick when injury occurred
    pub injury_tick: u64,
    /// Ticks since injury
    pub age_ticks: u64,
    /// Whether injury is infected
    pub is_infected: bool,
}

impl Wound {
    /// Create a new injury
    pub fn new(id: u64, region: BodyRegion, severity: InjurySeverity, tick: u64) -> Self {
        Self {
            id,
            region,
            severity,
            healing_stage: HealingStage::Inflammatory,
            stage_progress: 0.0,
            healing_progress: 0.0,
            pain_level: severity.pain_level(),
            injury_tick: tick,
            age_ticks: 0,
            is_infected: false,
        }
    }

    /// Update injury healing for one tick
    /// Returns whether injury has progressed to next stage
    pub fn update(&mut self, healing_factors: &HealingFactors) -> bool {
        self.age_ticks += 1;
        
        // Calculate healing rate
        let base_rate = 1.0 / self.severity.base_healing_ticks() as f64;
        let stage_multiplier = self.healing_stage.healing_multiplier();
        let biological_multiplier = healing_factors.get_biological_multiplier();
        
        let healing_rate = base_rate * stage_multiplier * biological_multiplier;
        
        // Update stage progress
        self.stage_progress += healing_rate;
        
        // Check if stage is complete
        if self.stage_progress >= 1.0 {
            self.advance_stage();
            return true;
        }
        
        // Update overall healing progress
        self.update_healing_progress();
        
        // Update pain level
        self.update_pain_level();
        
        false
    }

    /// Advance to next healing stage
    fn advance_stage(&mut self) {
        self.healing_stage = match self.healing_stage {
            HealingStage::Inflammatory => HealingStage::Debridement,
            HealingStage::Debridement => HealingStage::Proliferation,
            HealingStage::Proliferation => HealingStage::Maturation,
            HealingStage::Maturation => HealingStage::Healed,
            HealingStage::Healed => HealingStage::Healed, // Already healed
        };
        
        self.stage_progress = 0.0;
        debug!("Wound {} advanced to {:?}", self.id, self.healing_stage);
    }

    /// Update overall healing progress
    fn update_healing_progress(&mut self) {
        let stage_weight = match self.healing_stage {
            HealingStage::Inflammatory => 0.1,
            HealingStage::Debridement => 0.2,
            HealingStage::Proliferation => 0.4,
            HealingStage::Maturation => 0.2,
            HealingStage::Healed => 1.0,
        };
        
        let base_progress = match self.healing_stage {
            HealingStage::Inflammatory => 0.0,
            HealingStage::Debridement => 0.1,
            HealingStage::Proliferation => 0.3,
            HealingStage::Maturation => 0.7,
            HealingStage::Healed => 1.0,
        };
        
        self.healing_progress = base_progress + (stage_weight * self.stage_progress);
    }

    /// Update pain level based on healing stage and infection
    fn update_pain_level(&mut self) {
        let base_pain = self.healing_stage.pain_level();
        let infection_pain = if self.is_infected { 0.3 } else { 0.0 };
        
        self.pain_level = (base_pain + infection_pain).min(1.0);
    }

    /// Check if wound is fully healed
    pub fn is_healed(&self) -> bool {
        self.healing_stage == HealingStage::Healed
    }

    /// Get mobility impairment from this wound
    pub fn get_mobility_impairment(&self) -> f64 {
        let severity_impairment = self.severity.mobility_impairment();
        let region_factor = self.region.mobility_impact_factor();
        let healing_reduction = 1.0 - self.healing_progress;
        
        severity_impairment * region_factor * healing_reduction
    }

    /// Get cognitive impairment from this wound
    pub fn get_cognitive_impairment(&self) -> f64 {
        let severity_impairment = self.severity.cognitive_impairment();
        let criticality_factor = self.region.criticality_factor();
        let healing_reduction = 1.0 - self.healing_progress;
        let pain_impairment = self.pain_level * 0.3;
        
        (severity_impairment * criticality_factor * healing_reduction) + pain_impairment
    }
}

/// Factors affecting healing rate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealingFactors {
    /// Nutrition level (0.0 = malnourished, 1.0 = optimal)
    pub nutrition_level: f64,
    /// Hydration level (0.0 = dehydrated, 1.0 = optimal)
    pub hydration_level: f64,
    /// Energy level (0.0 = exhausted, 1.0 = optimal)
    pub energy_level: f64,
    /// Age factor (1.0 = young adult, higher = older)
    pub age_factor: f64,
    /// Rest level (0.0 = active, 1.0 = complete rest)
    pub rest_level: f64,
}

impl HealingFactors {
    /// Create optimal healing factors
    pub fn optimal() -> Self {
        Self {
            nutrition_level: 1.0,
            hydration_level: 1.0,
            energy_level: 1.0,
            age_factor: 1.0,
            rest_level: 1.0,
        }
    }

    /// Get overall biological healing multiplier
    pub fn get_biological_multiplier(&self) -> f64 {
        let nutrition_multiplier = 0.5 + (self.nutrition_level * 0.5);
        let hydration_multiplier = 0.7 + (self.hydration_level * 0.3);
        let energy_multiplier = 0.6 + (self.energy_level * 0.4);
        let age_multiplier = 1.0 / self.age_factor; // Older = slower healing
        let rest_multiplier = 0.5 + (self.rest_level * 0.5);
        
        nutrition_multiplier * hydration_multiplier * energy_multiplier * age_multiplier * rest_multiplier
    }
}

/// Complete injury system
/// Manages all wounds and healing processes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjurySystem {
    /// All current injuries indexed by ID for deterministic iteration
    wounds: BTreeMap<u64, Wound>,
    /// Next wound ID to assign
    next_wound_id: u64,
    /// Overall pain level (0.0 = no pain, 1.0 = extreme pain)
    pub overall_pain_level: f64,
    /// Overall mobility impairment (0.0 = no impairment, 1.0 = completely immobilized)
    pub mobility_impairment: f64,
    /// Overall cognitive impairment (0.0 = no impairment, 1.0 = severely impaired)
    pub cognitive_impairment: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl InjurySystem {
    /// Create new injury system with no injuries
    pub fn new() -> Self {
        Self {
            wounds: BTreeMap::new(),
            next_wound_id: 1,
            overall_pain_level: 0.0,
            mobility_impairment: 0.0,
            cognitive_impairment: 0.0,
            tick: 0,
        }
    }

    /// Update injury system for one tick
    /// Returns injury changes for audit logging
    pub fn update(&mut self, healing_factors: &HealingFactors) -> InjuryUpdate {
        let mut advanced_wounds = Vec::new();
        let mut healed_wounds = Vec::new();
        
        // Update all wounds
        for wound in self.wounds.values_mut() {
            let was_healed = wound.is_healed();
            let advanced = wound.update(healing_factors);
            
            if advanced {
                advanced_wounds.push(wound.id);
            }
            
            if !was_healed && wound.is_healed() {
                healed_wounds.push(wound.id);
            }
        }
        
        // Remove fully healed wounds
        for wound_id in &healed_wounds {
            self.wounds.remove(wound_id);
            debug!("Wound {} fully healed and removed", wound_id);
        }
        
        // Update overall impairments
        self.update_overall_impairments();
        
        self.tick += 1;
        
        trace!("Injury update {}: {} active wounds, pain: {:.3}", 
               self.tick, self.wounds.len(), self.overall_pain_level);
        
        InjuryUpdate {
            advanced_wounds,
            healed_wounds,
            overall_pain: self.overall_pain_level,
            mobility_impairment: self.mobility_impairment,
            cognitive_impairment: self.cognitive_impairment,
        }
    }

    /// Inflict a new injury
    pub fn inflict_injury(&mut self, region: BodyRegion, severity: InjurySeverity) -> u64 {
        let wound_id = self.next_wound_id;
        self.next_wound_id += 1;
        
        let wound = Wound::new(wound_id, region, severity, self.tick);
        self.wounds.insert(wound_id, wound);
        
        self.update_overall_impairments();
        
        debug!("Inflicted {:?} injury to {}, ID: {}", 
               severity, region.name(), wound_id);
        
        wound_id
    }

    /// Update overall impairment levels
    fn update_overall_impairments(&mut self) {
        if self.wounds.is_empty() {
            self.overall_pain_level = 0.0;
            self.mobility_impairment = 0.0;
            self.cognitive_impairment = 0.0;
            return;
        }
        
        let mut total_pain = 0.0;
        let mut total_mobility = 0.0;
        let mut total_cognitive = 0.0;
        
        for wound in self.wounds.values() {
            total_pain += wound.pain_level;
            total_mobility += wound.get_mobility_impairment();
            total_cognitive += wound.get_cognitive_impairment();
        }
        
        let count = self.wounds.len() as f64;
        self.overall_pain_level = (total_pain / count).min(1.0);
        self.mobility_impairment = (total_mobility / count).min(1.0);
        self.cognitive_impairment = (total_cognitive / count).min(1.0);
    }

    /// Check if injury state supports action execution
    /// Returns InjuryBioVeto reasons if injuries would prevent action
    pub fn check_action_viability(&self) -> Vec<InjuryBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Critical injuries
        for wound in self.wounds.values() {
            if wound.severity == InjurySeverity::Critical {
                vetoes.push(InjuryBioVetoReason::CriticalInjury);
            }
        }
        
        // Severe mobility impairment
        if self.mobility_impairment > 0.8 {
            vetoes.push(InjuryBioVetoReason::SevereMobilityImpairment);
        }
        
        // Severe cognitive impairment
        if self.cognitive_impairment > 0.7 {
            vetoes.push(InjuryBioVetoReason::SevereCognitiveImpairment);
        }
        
        // Extreme pain
        if self.overall_pain_level > 0.8 {
            vetoes.push(InjuryBioVetoReason::ExtremePain);
        }
        
        vetoes
    }

    /// Get current injury state for cognitive processing
    /// Injuries affect cognitive performance and decision making
    pub fn get_cognitive_modulators(&self) -> InjuryModulators {
        InjuryModulators {
            pain_level: self.overall_pain_level,
            mobility_impairment: self.mobility_impairment,
            cognitive_impairment: self.cognitive_impairment,
            wound_count: self.wounds.len(),
            has_critical_injury: self.wounds.values().any(|w| w.severity == InjurySeverity::Critical),
        }
    }

    /// Get wound by ID
    pub fn get_wound(&self, wound_id: u64) -> Option<&Wound> {
        self.wounds.get(&wound_id)
    }

    /// Get all wounds
    pub fn get_all_wounds(&self) -> impl Iterator<Item = &Wound> {
        self.wounds.values()
    }

    /// Get number of active wounds
    pub fn wound_count(&self) -> usize {
        self.wounds.len()
    }
}

/// Results of an injury update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryUpdate {
    /// Wounds that advanced to next healing stage
    pub advanced_wounds: Vec<u64>,
    /// Wounds that fully healed
    pub healed_wounds: Vec<u64>,
    /// Current overall pain level
    pub overall_pain: f64,
    /// Current mobility impairment
    pub mobility_impairment: f64,
    /// Current cognitive impairment
    pub cognitive_impairment: f64,
}

/// Injury effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryModulators {
    /// Overall pain level (0.0 = no pain, 1.0 = extreme pain)
    pub pain_level: f64,
    /// Mobility impairment (0.0 = no impairment, 1.0 = completely immobilized)
    pub mobility_impairment: f64,
    /// Cognitive impairment (0.0 = no impairment, 1.0 = severely impaired)
    pub cognitive_impairment: f64,
    /// Number of active wounds
    pub wound_count: usize,
    /// Whether any critical injuries exist
    pub has_critical_injury: bool,
}

/// Reasons why injuries might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryBioVetoReason {
    /// Life-threatening injury
    CriticalInjury,
    /// Severe mobility impairment prevents movement
    SevereMobilityImpairment,
    /// Severe cognitive impairment prevents complex tasks
    SevereCognitiveImpairment,
    /// Extreme pain prevents concentration
    ExtremePain,
    /// General injury-related limitation
    InjuryLimitation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wound_creation() {
        let wound = Wound::new(1, BodyRegion::Arms, InjurySeverity::Moderate, 1000);
        
        assert_eq!(wound.id, 1);
        assert_eq!(wound.region, BodyRegion::Arms);
        assert_eq!(wound.severity, InjurySeverity::Moderate);
        assert_eq!(wound.healing_stage, HealingStage::Inflammatory);
        assert!(!wound.is_healed());
    }

    #[test]
    fn test_injury_infliction() {
        let mut system = InjurySystem::new();
        
        let wound_id = system.inflict_injury(BodyRegion::Legs, InjurySeverity::Severe);
        
        assert_eq!(system.wound_count(), 1);
        assert!(system.get_wound(wound_id).is_some());
        assert!(system.mobility_impairment > 0.0);
    }

    #[test]
    fn test_healing_progression() {
        let mut system = InjurySystem::new();
        let wound_id = system.inflict_injury(BodyRegion::Arms, InjurySeverity::Moderate);
        
        let healing_factors = HealingFactors::optimal();
        
        // Update for many ticks to allow healing
        for _ in 0..5000 {
            system.update(&healing_factors);
        }
        
        let wound = system.get_wound(wound_id).unwrap();
        assert!(wound.healing_progress > 0.0);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = InjurySystem::new();
        let mut system2 = InjurySystem::new();
        
        // Inflict identical injuries
        system1.inflict_injury(BodyRegion::Arms, InjurySeverity::Moderate);
        system2.inflict_injury(BodyRegion::Arms, InjurySeverity::Moderate);
        
        let healing_factors = HealingFactors::optimal();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update(&healing_factors);
            system2.update(&healing_factors);
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
