/*!
# Reproductive System

**Purpose:** Deterministic reproductive biology for human-equivalent agents.

**Why it exists:** Reproduction is fundamental to species survival and genetic
continuity. Agents must have realistic reproductive cycles, fertility mechanics,
and the biological constraints that govern reproduction to maintain human equivalence.

**Determinism guarantees:**
- Reproductive cycles follow fixed biological patterns
- Fertility calculations are deterministic based on biological state
- No random fluctuations in reproductive hormones
- All reproductive changes have identifiable sources

**How it affects replay:** Same sequence of biological states and time will
produce identical reproductive trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Biological sex with corresponding reproductive characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

impl BiologicalSex {
    /// Base fertility level (0.0 = infertile, 1.0 = peak fertility)
    pub const fn base_fertility(self) -> f64 {
        match self {
            Self::Male => 0.8,    // Generally high fertility
            Self::Female => 0.7,   // Variable fertility with cycle
        }
    }

    /// Reproductive hormone production rate
    pub const fn hormone_production_rate(self) -> f64 {
        match self {
            Self::Male => 0.02,    // Testosterone production
            Self::Female => 0.015,  // Estrogen/progesterone production
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
        }
    }
}

/// Female reproductive cycle phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReproductiveCyclePhase {
    /// Menstrual phase (bleeding)
    Menstrual,
    /// Follicular phase (estrogen rising)
    Follicular,
    /// Ovulation (peak fertility)
    Ovulation,
    /// Luteal phase (progesterone dominant)
    Luteal,
}

impl ReproductiveCyclePhase {
    /// Duration in ticks (assuming 28-day cycle)
    pub const fn duration_ticks(self) -> u64 {
        match self {
            Self::Menstrual => 1440,   // ~4 days
            Self::Follicular => 7200,   // ~10 days
            Self::Ovulation => 720,      // ~1 day
            Self::Luteal => 10080,      // ~14 days
        }
    }

    /// Fertility multiplier for this phase
    pub const fn fertility_multiplier(self) -> f64 {
        match self {
            Self::Menstrual => 0.0,     // Infertile
            Self::Follicular => 0.3,     // Low fertility
            Self::Ovulation => 1.0,       // Peak fertility
            Self::Luteal => 0.1,        // Very low fertility
        }
    }

    /// Hormonal profile for this phase
    pub const fn hormonal_profile(self) -> (f64, f64) {
        // (estrogen_level, progesterone_level)
        match self {
            Self::Menstrual => (0.2, 0.1),
            Self::Follicular => (0.7, 0.2),
            Self::Ovulation => (1.0, 0.3),
            Self::Luteal => (0.5, 0.8),
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Menstrual => "menstrual phase",
            Self::Follicular => "follicular phase",
            Self::Ovulation => "ovulation",
            Self::Luteal => "luteal phase",
        }
    }
}

/// Fertility status with corresponding reproductive capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FertilityStatus {
    /// Infertile (cannot reproduce)
    Infertile,
    /// Low fertility (reproduction unlikely)
    Low,
    /// Moderate fertility (reproduction possible)
    Moderate,
    /// High fertility (reproduction likely)
    High,
    /// Peak fertility (optimal reproduction conditions)
    Peak,
}

impl FertilityStatus {
    /// Convert from fertility percentage (0.0 = infertile, 1.0 = peak)
    pub fn from_percentage(percentage: f64) -> Self {
        match percentage {
            p if p < 0.1 => Self::Infertile,
            p if p < 0.3 => Self::Low,
            p if p < 0.6 => Self::Moderate,
            p if p < 0.9 => Self::High,
            _ => Self::Peak,
        }
    }

    /// Get fertility percentage
    pub fn percentage(self) -> f64 {
        match self {
            Self::Infertile => 0.0,
            Self::Low => 0.2,
            Self::Moderate => 0.5,
            Self::High => 0.8,
            Self::Peak => 1.0,
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Infertile => "infertile",
            Self::Low => "low fertility",
            Self::Moderate => "moderate fertility",
            Self::High => "high fertility",
            Self::Peak => "peak fertility",
        }
    }
}

/// Pregnancy stages with corresponding biological changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PregnancyStage {
    /// Not pregnant
    NotPregnant,
    /// First trimester (conception to 12 weeks)
    FirstTrimester,
    /// Second trimester (13 to 27 weeks)
    SecondTrimester,
    /// Third trimester (28 weeks to birth)
    ThirdTrimester,
}

impl PregnancyStage {
    /// Duration in ticks
    pub const fn duration_ticks(self) -> u64 {
        match self {
            Self::NotPregnant => 0,
            Self::FirstTrimester => 604800,    // ~7 weeks
            Self::SecondTrimester => 907200,   // ~10.5 weeks
            Self::ThirdTrimester => 907200,    // ~10.5 weeks
        }
    }

    /// Fertility during this stage (always 0.0 when pregnant)
    pub const fn fertility(self) -> f64 {
        match self {
            Self::NotPregnant => 1.0,  // Normal fertility
            _ => 0.0,                // Infertile when pregnant
        }
    }

    /// Energy cost multiplier for this stage
    pub const fn energy_cost_multiplier(self) -> f64 {
        match self {
            Self::NotPregnant => 1.0,
            Self::FirstTrimester => 1.1,
            Self::SecondTrimester => 1.2,
            Self::ThirdTrimester => 1.3,
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::NotPregnant => "not pregnant",
            Self::FirstTrimester => "first trimester",
            Self::SecondTrimester => "second trimester",
            Self::ThirdTrimester => "third trimester",
        }
    }
}

/// Complete reproductive system
/// Manages fertility, cycles, and pregnancy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductiveSystem {
    /// Biological sex
    pub biological_sex: BiologicalSex,
    /// Current fertility status
    pub fertility_status: FertilityStatus,
    /// Current reproductive cycle phase (female only)
    pub cycle_phase: Option<ReproductiveCyclePhase>,
    /// Progress through current cycle phase (0.0 = start, 1.0 = complete)
    pub cycle_progress: f64,
    /// Current pregnancy stage
    pub pregnancy_stage: PregnancyStage,
    /// Progress through current pregnancy stage (0.0 = start, 1.0 = complete)
    pub pregnancy_progress: f64,
    /// Age in years (affects fertility)
    pub age_years: f64,
    /// Overall health factor (0.0 = poor health, 1.0 = optimal health)
    pub health_factor: f64,
    /// Tick when last cycle phase started
    pub phase_start_tick: u64,
    /// Tick when pregnancy started (if pregnant)
    pub pregnancy_start_tick: Option<u64>,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl ReproductiveSystem {
    /// Create new reproductive system with specified biological sex
    pub fn new(biological_sex: BiologicalSex, age_years: f64) -> Self {
        let base_fertility = biological_sex.base_fertility();
        let fertility_status = FertilityStatus::from_percentage(base_fertility);
        
        Self {
            biological_sex,
            fertility_status,
            cycle_phase: None, // Will be set for females
            cycle_progress: 0.0,
            pregnancy_stage: PregnancyStage::NotPregnant,
            pregnancy_progress: 0.0,
            age_years,
            health_factor: 1.0,
            phase_start_tick: 0,
            pregnancy_start_tick: None,
            tick: 0,
        }
    }

    /// Update reproductive system for one tick
    /// Returns reproductive changes for audit logging
    pub fn update(&mut self) -> ReproductiveUpdate {
        let old_fertility = self.fertility_status.percentage();
        let old_cycle_phase = self.cycle_phase;
        let old_pregnancy_stage = self.pregnancy_stage;
        
        // Update cycle phase for females
        if self.biological_sex == BiologicalSex::Female {
            self.update_cycle_phase();
        }
        
        // Update pregnancy if pregnant
        if self.pregnancy_stage != PregnancyStage::NotPregnant {
            self.update_pregnancy();
        }
        
        // Update fertility based on current state
        self.update_fertility();
        
        self.tick += 1;
        
        trace!("Reproductive update {}: fertility {:.3}, cycle: {:?}, pregnancy: {:?}", 
               self.tick, self.fertility_status.percentage(), self.cycle_phase, self.pregnancy_stage);
        
        ReproductiveUpdate {
            fertility_change: self.fertility_status.percentage() - old_fertility,
            cycle_phase_changed: old_cycle_phase != self.cycle_phase,
            pregnancy_stage_changed: old_pregnancy_stage != self.pregnancy_stage,
            current_fertility: self.fertility_status,
            cycle_phase: self.cycle_phase,
            pregnancy_stage: self.pregnancy_stage,
        }
    }

    /// Update female reproductive cycle phase
    fn update_cycle_phase(&mut self) {
        if self.pregnancy_stage != PregnancyStage::NotPregnant {
            // No cycle when pregnant
            self.cycle_phase = None;
            return;
        }
        
        let current_phase = self.cycle_phase.unwrap_or(ReproductiveCyclePhase::Menstrual);
        let phase_duration = current_phase.duration_ticks();
        let ticks_in_phase = self.tick - self.phase_start_tick;
        
        if ticks_in_phase >= phase_duration {
            // Advance to next phase
            let next_phase = match current_phase {
                ReproductiveCyclePhase::Menstrual => ReproductiveCyclePhase::Follicular,
                ReproductiveCyclePhase::Follicular => ReproductiveCyclePhase::Ovulation,
                ReproductiveCyclePhase::Ovulation => ReproductiveCyclePhase::Luteal,
                ReproductiveCyclePhase::Luteal => ReproductiveCyclePhase::Menstrual,
            };
            
            self.cycle_phase = Some(next_phase);
            self.phase_start_tick = self.tick;
            self.cycle_progress = 0.0;
            
            debug!("Advanced to reproductive phase: {:?}", next_phase);
        } else {
            // Update progress within current phase
            self.cycle_progress = ticks_in_phase as f64 / phase_duration as f64;
        }
    }

    /// Update pregnancy progression
    fn update_pregnancy(&mut self) {
        let current_stage = self.pregnancy_stage;
        let stage_duration = current_stage.duration_ticks();
        let ticks_in_stage = self.tick - self.pregnancy_start_tick.unwrap_or(self.tick);
        
        if ticks_in_stage >= stage_duration {
            // Advance to next stage
            let next_stage = match current_stage {
                PregnancyStage::NotPregnant => PregnancyStage::NotPregnant,
                PregnancyStage::FirstTrimester => PregnancyStage::SecondTrimester,
                PregnancyStage::SecondTrimester => PregnancyStage::ThirdTrimester,
                PregnancyStage::ThirdTrimester => {
                    // Pregnancy complete - would trigger birth in full system
                    PregnancyStage::NotPregnant
                }
            };
            
            self.pregnancy_stage = next_stage;
            self.pregnancy_progress = 0.0;
            
            if next_stage == PregnancyStage::NotPregnant {
                self.pregnancy_start_tick = None;
                debug!("Pregnancy completed");
            } else {
                debug!("Advanced to pregnancy stage: {:?}", next_stage);
            }
        } else {
            // Update progress within current stage
            self.pregnancy_progress = ticks_in_stage as f64 / stage_duration as f64;
        }
    }

    /// Update fertility based on current state
    fn update_fertility(&mut self) {
        let base_fertility = self.biological_sex.base_fertility();
        
        // Age-related fertility decline
        let age_factor = self.calculate_age_fertility_factor();
        
        // Health-related fertility factor
        let health_factor = self.health_factor;
        
        // Cycle-related fertility factor (female only)
        let cycle_factor = if self.biological_sex == BiologicalSex::Female {
            self.cycle_phase
                .map(|phase| phase.fertility_multiplier())
                .unwrap_or(0.0)
        } else {
            1.0 // Males have relatively constant fertility
        };
        
        // Pregnancy-related fertility factor
        let pregnancy_factor = self.pregnancy_stage.fertility();
        
        // Calculate overall fertility
        let overall_fertility = base_fertility * age_factor * health_factor * cycle_factor * pregnancy_factor;
        
        self.fertility_status = FertilityStatus::from_percentage(overall_fertility);
    }

    /// Calculate age-related fertility factor
    fn calculate_age_fertility_factor(&self) -> f64 {
        match self.biological_sex {
            BiologicalSex::Male => {
                // Male fertility declines gradually after 40
                if self.age_years < 40.0 {
                    1.0
                } else {
                    (1.0 - (self.age_years - 40.0) * 0.02).max(0.1)
                }
            }
            BiologicalSex::Female => {
                // Female fertility has more complex pattern
                if self.age_years < 15.0 {
                    0.1 // Before puberty
                } else if self.age_years < 25.0 {
                    1.0 // Peak fertility
                } else if self.age_years < 35.0 {
                    0.9 // Slight decline
                } else if self.age_years < 45.0 {
                    (1.0 - (self.age_years - 35.0) * 0.05).max(0.1)
                } else {
                    0.01 // Near menopause
                }
            }
        }
    }

    /// Attempt to initiate pregnancy
    /// Returns success/failure based on fertility
    pub fn attempt_conception(&mut self, partner_fertility: f64) -> bool {
        if self.biological_sex != BiologicalSex::Female {
            return false; // Only females can become pregnant
        }
        
        if self.pregnancy_stage != PregnancyStage::NotPregnant {
            return false; // Already pregnant
        }
        
        let my_fertility = self.fertility_status.percentage();
        let conception_probability = (my_fertility * partner_fertility) / 2.0;
        
        // In a full implementation, this would use deterministic RNG
        // For now, deterministic threshold at 50% combined fertility
        let success = conception_probability > 0.5;
        
        if success {
            self.pregnancy_stage = PregnancyStage::FirstTrimester;
            self.pregnancy_start_tick = Some(self.tick);
            self.pregnancy_progress = 0.0;
            debug!("Conception successful, pregnancy started");
        }
        
        success
    }

    /// Set health factor (affects fertility)
    pub fn set_health_factor(&mut self, health_factor: f64) {
        self.health_factor = health_factor.clamp(0.0, 1.0);
        debug!("Set reproductive health factor to {:.3}", self.health_factor);
    }

    /// Check if reproductive state supports action execution
    /// Returns ReproductiveBioVeto reasons if reproduction would prevent action
    pub fn check_action_viability(&self) -> Vec<ReproductiveBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Late pregnancy complications
        if self.pregnancy_stage == PregnancyStage::ThirdTrimester {
            vetoes.push(ReproductiveBioVetoReason::LatePregnancyLimitations);
        }
        
        // Severe menstrual symptoms (simplified)
        if let Some(ReproductiveCyclePhase::Menstrual) = self.cycle_phase {
            if self.health_factor < 0.5 {
                vetoes.push(ReproductiveBioVetoReason::SevereMenstrualSymptoms);
            }
        }
        
        vetoes
    }

    /// Get current reproductive state for cognitive processing
    /// Reproductive state affects mood, decision making, and social behavior
    pub fn get_cognitive_modulators(&self) -> ReproductiveModulators {
        let hormonal_profile = if self.biological_sex == BiologicalSex::Female {
            self.cycle_phase
                .map(|phase| phase.hormonal_profile())
                .unwrap_or((0.5, 0.5))
        } else {
            // Male hormonal profile (simplified)
            (0.7, 0.1) // (testosterone_dominant, progesterone_equivalent)
        };
        
        ReproductiveModulators {
            fertility_level: self.fertility_status.percentage(),
            sex_drive: self.calculate_sex_drive(),
            hormonal_balance: (hormonal_profile.0 + hormonal_profile.1) / 2.0,
            reproductive_stress: self.calculate_reproductive_stress(),
            cycle_phase: self.cycle_phase,
            pregnancy_stage: self.pregnancy_stage,
        }
    }

    /// Calculate sex drive based on hormones and health
    fn calculate_sex_drive(&self) -> f64 {
        let base_drive = match self.biological_sex {
            BiologicalSex::Male => 0.7,
            BiologicalSex::Female => 0.5,
        };
        
        let fertility_factor = self.fertility_status.percentage();
        let health_factor = self.health_factor;
        
        // Reduce drive during pregnancy
        let pregnancy_factor = if self.pregnancy_stage != PregnancyStage::NotPregnant {
            0.3
        } else {
            1.0
        };
        
        base_drive * fertility_factor * health_factor * pregnancy_factor
    }

    /// Calculate reproductive stress level
    fn calculate_reproductive_stress(&self) -> f64 {
        let mut stress = 0.0;
        
        // Age-related stress
        if self.biological_sex == BiologicalSex::Female && self.age_years > 35.0 {
            stress += (self.age_years - 35.0) * 0.02;
        }
        
        // Infertility stress
        if self.fertility_status == FertilityStatus::Infertile && self.age_years > 25.0 {
            stress += 0.3;
        }
        
        // Pregnancy stress
        if self.pregnancy_stage != PregnancyStage::NotPregnant {
            stress += match self.pregnancy_stage {
                PregnancyStage::FirstTrimester => 0.2,
                PregnancyStage::SecondTrimester => 0.1,
                PregnancyStage::ThirdTrimester => 0.4,
                PregnancyStage::NotPregnant => 0.0,
            };
        }
        
        stress.min(1.0)
    }
}

/// Results of a reproductive update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductiveUpdate {
    /// Change in fertility percentage
    pub fertility_change: f64,
    /// Whether cycle phase changed this tick
    pub cycle_phase_changed: bool,
    /// Whether pregnancy stage changed this tick
    pub pregnancy_stage_changed: bool,
    /// Current fertility status
    pub current_fertility: FertilityStatus,
    /// Current cycle phase (if female)
    pub cycle_phase: Option<ReproductiveCyclePhase>,
    /// Current pregnancy stage
    pub pregnancy_stage: PregnancyStage,
}

/// Reproductive effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductiveModulators {
    /// Current fertility level (0.0 = infertile, 1.0 = peak fertility)
    pub fertility_level: f64,
    /// Sex drive (0.0 = no drive, 1.0 = high drive)
    pub sex_drive: f64,
    /// Hormonal balance (0.0 = imbalanced, 1.0 = balanced)
    pub hormonal_balance: f64,
    /// Reproductive-related stress (0.0 = none, 1.0 = high stress)
    pub reproductive_stress: f64,
    /// Current cycle phase (female only)
    pub cycle_phase: Option<ReproductiveCyclePhase>,
    /// Current pregnancy stage
    pub pregnancy_stage: PregnancyStage,
}

/// Reasons why reproductive state might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReproductiveBioVetoReason {
    /// Late pregnancy limits physical activity
    LatePregnancyLimitations,
    /// Severe menstrual symptoms impair function
    SevereMenstrualSymptoms,
    /// General reproductive limitations
    ReproductiveLimitations,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproductive_system_creation() {
        let system = ReproductiveSystem::new(BiologicalSex::Female, 25.0);
        
        assert_eq!(system.biological_sex, BiologicalSex::Female);
        assert_eq!(system.age_years, 25.0);
        assert!(system.fertility_status.percentage() > 0.0);
    }

    #[test]
    fn test_cycle_progression() {
        let mut system = ReproductiveSystem::new(BiologicalSex::Female, 25.0);
        
        // Update for many ticks to progress through cycle
        for _ in 0..20000 {
            system.update();
        }
        
        // Should have progressed through at least one phase
        assert!(system.cycle_phase.is_some());
    }

    #[test]
    fn test_fertility_calculation() {
        let mut system = ReproductiveSystem::new(BiologicalSex::Male, 30.0);
        
        // Set poor health
        system.set_health_factor(0.3);
        system.update();
        
        // Fertility should be reduced
        assert!(system.fertility_status.percentage() < 0.8);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = ReproductiveSystem::new(BiologicalSex::Female, 25.0);
        let mut system2 = ReproductiveSystem::new(BiologicalSex::Female, 25.0);
        
        // Run identical updates
        for _ in 0..100 {
            system1.update();
            system2.update();
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
