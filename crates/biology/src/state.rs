/*!
# Biological State

**Purpose:** Unified biological state management integrating all biological systems.

**Why it exists:** Biological systems must work together as an integrated whole.
This module provides the central coordination point for all biological processes
and ensures they interact deterministically and consistently.

**Determinism guarantees:**
- All biological updates occur in fixed, deterministic order
- State transitions are atomic and auditable
- No race conditions or non-deterministic interactions
- All state changes have identifiable sources

**How it affects replay:** Same sequence of biological inputs will
produce identical integrated biological states across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

use super::{
    endocrine::{EndocrineSystem, CognitiveModulators as EndocrineModulators},
    metabolism::{MetabolicSystem, CognitiveModulators as MetabolicModulators},
    nutrition::{NutritionSystem, CognitiveModulators as NutritionalModulators},
    hydration::{HydrationSystem, CognitiveModulators as HydrationModulators},
    fatigue::{FatigueSystem, ActivityFactors, CognitiveModulators as FatigueModulators},
    injury::{InjurySystem, HealingFactors, CognitiveModulators as InjuryModulators},
    reproductive::{ReproductiveSystem, CognitiveModulators as ReproductiveModulators},
};

/// Unified biological state integrating all systems
/// This is the primary interface for other modules to interact with biology
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiologicalState {
    /// Endocrine system (hormones)
    pub endocrine: EndocrineSystem,
    /// Metabolic system (energy balance)
    pub metabolism: MetabolicSystem,
    /// Nutrition system (nutrients, hydration)
    pub nutrition: NutritionSystem,
    /// Hydration system (water balance)
    pub hydration: HydrationSystem,
    /// Fatigue system (energy, sleep)
    pub fatigue: FatigueSystem,
    /// Injury system (wounds, healing)
    pub injury: InjurySystem,
    /// Reproductive system (fertility, cycles)
    pub reproductive: ReproductiveSystem,
    /// Overall biological health (0.0 = critical, 1.0 = optimal)
    pub overall_health: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl BiologicalState {
    /// Create new biological state with human-equivalent baseline
    pub fn new(biological_sex: super::reproductive::BiologicalSex, age_years: f64) -> Self {
        Self {
            endocrine: EndocrineSystem::new(),
            metabolism: MetabolicSystem::new(),
            nutrition: NutritionSystem::new(),
            hydration: HydrationSystem::new(),
            fatigue: FatigueSystem::new(),
            injury: InjurySystem::new(),
            reproductive: ReproductiveSystem::new(biological_sex, age_years),
            overall_health: 1.0,
            tick: 0,
        }
    }

    /// Update all biological systems for one tick
    /// Returns comprehensive biological update for audit logging
    pub fn update(&mut self, activity_factors: &ActivityFactors) -> BiologicalUpdate {
        let old_health = self.overall_health;
        
        // Update all systems in deterministic order
        let endocrine_changes = self.endocrine.update();
        let metabolic_update = self.metabolism.update();
        let nutrition_update = self.nutrition.update();
        let hydration_update = self.hydration.update();
        let fatigue_update = self.fatigue.update(activity_factors);
        
        // Update injury system with healing factors from other systems
        let healing_factors = self.create_healing_factors();
        let injury_update = self.injury.update(&healing_factors);
        
        // Update reproductive system
        let reproductive_update = self.reproductive.update();
        
        // Update overall health assessment
        self.update_overall_health();
        
        self.tick += 1;
        
        trace!("Biological update {}: overall health {:.3}", 
               self.tick, self.overall_health);
        
        BiologicalUpdate {
            tick: self.tick,
            endocrine_changes,
            metabolic_update,
            nutrition_update,
            hydration_update,
            fatigue_update,
            injury_update,
            reproductive_update,
            overall_health_change: self.overall_health - old_health,
            overall_health: self.overall_health,
        }
    }

    /// Create healing factors from current biological state
    fn create_healing_factors(&self) -> HealingFactors {
        HealingFactors {
            nutrition_level: self.nutrition.get_cognitive_modulators().hunger_level, // Simplified
            hydration_level: self.hydration.get_cognitive_modulators().thirst_drive,
            energy_level: self.fatigue.get_cognitive_modulators().energy_level,
            age_factor: 1.0 + (self.reproductive.age_years - 25.0) * 0.01, // Age factor
            rest_level: if self.fatigue.energy_level > 0.8 { 1.0 } else { 0.5 },
        }
    }

    /// Update overall health assessment
    fn update_overall_health(&mut self) {
        let mut health_factors = Vec::new();
        
        // Metabolic health
        let metabolic_health = if self.metabolism.energy_balance.is_critical() {
            0.2
        } else if self.metabolism.energy_balance.is_low() {
            0.6
        } else {
            1.0
        };
        health_factors.push(metabolic_health);
        
        // Nutritional health
        let nutrition_mods = self.nutrition.get_cognitive_modulators();
        let nutritional_health = 1.0 - (nutrition_mods.nutrient_deficiency_count as f64 * 0.1);
        health_factors.push(nutritional_health);
        
        // Hydration health
        let hydration_health = self.hydration.get_cognitive_modulators().hydration_level;
        health_factors.push(hydration_health);
        
        // Fatigue health
        let fatigue_health = self.fatigue.get_cognitive_modulators().energy_level;
        health_factors.push(fatigue_health);
        
        // Injury health
        let injury_health = 1.0 - self.injury.mobility_impairment;
        health_factors.push(injury_health);
        
        // Reproductive health (simplified)
        let reproductive_health = self.reproductive.health_factor;
        health_factors.push(reproductive_health);
        
        // Calculate overall health as weighted average
        let total_health: f64 = health_factors.iter().sum();
        self.overall_health = (total_health / health_factors.len() as f64).clamp(0.0, 1.0);
    }

    /// Check if biological state supports action execution
    /// Returns BioVeto reasons if biology would prevent action
    pub fn check_action_viability(&self) -> Vec<BioVetoReason> {
        let mut all_vetoes = Vec::new();
        
        // Collect vetoes from all systems and convert to unified type
        for veto in self.endocrine.check_action_viability() {
            all_vetoes.push(match veto {
                super::endocrine::EndocrineBioVetoReason::StressOverload => BioVetoReason::StressOverload,
                super::endocrine::EndocrineBioVetoReason::Anhedonia => BioVetoReason::Anhedonia,
                super::endocrine::EndocrineBioVetoReason::ExcessiveFatigue => BioVetoReason::ExcessiveFatigue,
                super::endocrine::EndocrineBioVetoReason::LowConfidence => BioVetoReason::LowConfidence,
                super::endocrine::EndocrineBioVetoReason::HormonalImbalance => BioVetoReason::HormonalImbalance,
            });
        }
        
        for veto in self.metabolism.check_action_viability() {
            all_vetoes.push(match veto {
                super::metabolism::MetabolicBioVetoReason::EnergyExhaustion => BioVetoReason::EnergyExhaustion,
                super::metabolism::MetabolicBioVetoReason::ThermoregulatoryFailure => BioVetoReason::ThermoregulatoryFailure,
                super::metabolism::MetabolicBioVetoReason::ExtremeFatigue => BioVetoReason::ExtremeFatigue,
                super::metabolism::MetabolicBioVetoReason::MetabolicImbalance => BioVetoReason::MetabolicImbalance,
            });
        }
        
        for veto in self.nutrition.check_action_viability() {
            all_vetoes.push(match veto {
                super::nutrition::NutritionalBioVetoReason::SevereDehydration => BioVetoReason::SevereDehydration,
                super::nutrition::NutritionalBioVetoReason::SevereNutrientDeficiency(n) => BioVetoReason::SevereNutrientDeficiency(n),
                super::nutrition::NutritionalBioVetoReason::ExtremeCalorieDeficit => BioVetoReason::ExtremeCalorieDeficit,
                super::nutrition::NutritionalBioVetoReason::NutritionalImbalance => BioVetoReason::NutritionalImbalance,
            });
        }
        
        for veto in self.hydration.check_action_viability() {
            all_vetoes.push(match veto {
                super::hydration::HydrationBioVetoReason::CriticalDehydration => BioVetoReason::CriticalDehydration,
                super::hydration::HydrationBioVetoReason::UrgentWasteElimination(w) => BioVetoReason::UrgentWasteElimination(w),
                super::hydration::HydrationBioVetoReason::HydrationImbalance => BioVetoReason::HydrationImbalance,
            });
        }
        
        for veto in self.fatigue.check_action_viability() {
            all_vetoes.push(match veto {
                super::fatigue::FatigueBioVetoReason::CompleteExhaustion => BioVetoReason::CompleteExhaustion,
                super::fatigue::FatigueBioVetoReason::SevereFatigue => BioVetoReason::SevereFatigue,
                super::fatigue::FatigueBioVetoReason::CircadianSleepPressure => BioVetoReason::CircadianSleepPressure,
                super::fatigue::FatigueBioVetoReason::FatigueImbalance => BioVetoReason::FatigueImbalance,
            });
        }
        
        for veto in self.injury.check_action_viability() {
            all_vetoes.push(match veto {
                super::injury::InjuryBioVetoReason::CriticalInjury => BioVetoReason::CriticalInjury,
                super::injury::InjuryBioVetoReason::SevereMobilityImpairment => BioVetoReason::SevereMobilityImpairment,
                super::injury::InjuryBioVetoReason::SevereCognitiveImpairment => BioVetoReason::SevereCognitiveImpairment,
                super::injury::InjuryBioVetoReason::ExtremePain => BioVetoReason::ExtremePain,
                super::injury::InjuryBioVetoReason::InjuryLimitation => BioVetoReason::InjuryLimitation,
            });
        }
        
        for veto in self.reproductive.check_action_viability() {
            all_vetoes.push(match veto {
                super::reproductive::ReproductiveBioVetoReason::LatePregnancyLimitations => BioVetoReason::LatePregnancyLimitations,
                super::reproductive::ReproductiveBioVetoReason::SevereMenstrualSymptoms => BioVetoReason::SevereMenstrualSymptoms,
                super::reproductive::ReproductiveBioVetoReason::ReproductiveLimitations => BioVetoReason::ReproductiveLimitations,
            });
        }
        
        // Overall health veto
        if self.overall_health < 0.2 {
            all_vetoes.push(BioVetoReason::CriticalHealthFailure);
        }
        
        all_vetoes
    }

    /// Get integrated cognitive modulators from all systems
    /// This is the primary interface for cognition system
    pub fn get_cognitive_modulators(&self) -> IntegratedCognitiveModulators {
        let endocrine_mods = self.endocrine.get_cognitive_modulators();
        let metabolic_mods = self.metabolism.get_cognitive_modulators();
        let nutritional_mods = self.nutrition.get_cognitive_modulators();
        let hydration_mods = self.hydration.get_cognitive_modulators();
        let fatigue_mods = self.fatigue.get_cognitive_modulators();
        let injury_mods = self.injury.get_cognitive_modulators();
        let reproductive_mods = self.reproductive.get_cognitive_modulators();
        
        IntegratedCognitiveModulators {
            // Hormonal influences
            stress_level: endocrine_mods.stress_level,
            reward_sensitivity: endocrine_mods.reward_sensitivity,
            social_bonding: endocrine_mods.social_bonding,
            alertness: endocrine_mods.alertness,
            confidence: endocrine_mods.confidence,
            nurturing: endocrine_mods.nurturing,
            
            // Metabolic influences
            energy_availability: metabolic_mods.energy_availability,
            hunger_level: metabolic_mods.hunger_level,
            thermoregulatory_stress: metabolic_mods.thermoregulatory_stress,
            activity_capacity: metabolic_mods.activity_capacity,
            
            // Nutritional influences
            nutrient_deficiency_count: nutritional_mods.nutrient_deficiency_count,
            average_deficiency_severity: nutritional_mods.average_deficiency_severity,
            
            // Hydration influences
            thirst_drive: hydration_mods.thirst_drive,
            physical_performance: hydration_mods.physical_performance,
            
            // Fatigue influences
            cognitive_performance: fatigue_mods.cognitive_performance,
            reaction_time_modifier: fatigue_mods.reaction_time_modifier,
            sleep_pressure: fatigue_mods.sleep_pressure,
            
            // Injury influences
            pain_level: injury_mods.pain_level,
            mobility_impairment: injury_mods.mobility_impairment,
            
            // Reproductive influences
            fertility_level: reproductive_mods.fertility_level,
            sex_drive: reproductive_mods.sex_drive,
            hormonal_balance: reproductive_mods.hormonal_balance,
            
            // Overall state
            overall_health: self.overall_health,
        }
    }

    /// Apply external biological effects (stress, injury, etc.)
    pub fn apply_external_effect(&mut self, effect: BiologicalEffect) {
        match effect {
            BiologicalEffect::Stress(intensity) => {
                self.endocrine.apply_stress_response(intensity);
                self.metabolism.set_activity_level(super::metabolism::ActivityLevel::Heavy);
            }
            BiologicalEffect::Reward(magnitude) => {
                self.endocrine.apply_reward_response(magnitude);
            }
            BiologicalEffect::Bonding(strength) => {
                self.endocrine.apply_bonding_response(strength);
            }
            BiologicalEffect::Injury { region, severity } => {
                self.injury.inflict_injury(region, severity);
            }
            BiologicalEffect::FoodConsumption(ref food) => {
                self.nutrition.consume_food(&food);
                let mut nutrients = std::collections::BTreeMap::new();
                for (_macro_type, amount) in &food.micronutrients {
                    nutrients.insert(super::metabolism::Macronutrient::Carbohydrates, *amount);
                }
                self.metabolism.add_nutrients(&nutrients);
            }
            BiologicalEffect::WaterIntake(amount) => {
                self.hydration.drink_water(amount);
                self.nutrition.drink_water(amount);
            }
            BiologicalEffect::Rest(intensity) => {
                self.fatigue.apply_rest(intensity);
                self.metabolism.set_activity_level(super::metabolism::ActivityLevel::Rest);
            }
            BiologicalEffect::Sleep(quality) => {
                self.fatigue.start_sleep(quality);
            }
            BiologicalEffect::Wake => {
                self.fatigue.wake_up();
            }
        }
        
        debug!("Applied biological effect: {:?}", effect);
    }

    /// Get summary of current biological state
    pub fn get_state_summary(&self) -> BiologicalStateSummary {
        BiologicalStateSummary {
            overall_health: self.overall_health,
            energy_level: self.fatigue.energy_level,
            hydration_level: self.hydration.hydration_percentage,
            stress_level: self.endocrine.get_level(super::endocrine::HormoneType::Cortisol),
            pain_level: self.injury.overall_pain_level,
            hunger_level: self.metabolism.energy_balance.energy_percentage(),
            fertility_status: self.reproductive.fertility_status,
            active_wounds: self.injury.wound_count(),
            sleep_pressure: self.fatigue.circadian_phase,
        }
    }
}

/// External biological effects that can be applied to an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiologicalEffect {
    /// Stress response with intensity (0.0 = none, 1.0 = extreme)
    Stress(f64),
    /// Reward response with magnitude (0.0 = none, 1.0 = extreme)
    Reward(f64),
    /// Social bonding response with strength (0.0 = none, 1.0 = extreme)
    Bonding(f64),
    /// Physical injury
    Injury {
        region: super::injury::BodyRegion,
        severity: super::injury::InjurySeverity,
    },
    /// Food consumption
    FoodConsumption(super::nutrition::FoodItem),
    /// Water intake in liters
    WaterIntake(f64),
    /// Rest with intensity (0.0 = none, 1.0 = deep rest)
    Rest(f64),
    /// Sleep with quality level
    Sleep(super::fatigue::SleepQuality),
    /// Wake from sleep
    Wake,
}

/// Results of a comprehensive biological update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiologicalUpdate {
    /// Tick number
    pub tick: u64,
    /// Endocrine system changes
    pub endocrine_changes: BTreeMap<super::endocrine::HormoneType, f64>,
    /// Metabolic system update
    pub metabolic_update: super::metabolism::MetabolicUpdate,
    /// Nutrition system update
    pub nutrition_update: super::nutrition::NutritionUpdate,
    /// Hydration system update
    pub hydration_update: super::hydration::HydrationUpdate,
    /// Fatigue system update
    pub fatigue_update: super::fatigue::FatigueUpdate,
    /// Injury system update
    pub injury_update: super::injury::InjuryUpdate,
    /// Reproductive system update
    pub reproductive_update: super::reproductive::ReproductiveUpdate,
    /// Change in overall health
    pub overall_health_change: f64,
    /// Current overall health
    pub overall_health: f64,
}

/// Integrated cognitive modulators from all biological systems
/// This is the primary interface for the cognition system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegratedCognitiveModulators {
    // Hormonal influences
    pub stress_level: f64,
    pub reward_sensitivity: f64,
    pub social_bonding: f64,
    pub alertness: f64,
    pub confidence: f64,
    pub nurturing: f64,
    
    // Metabolic influences
    pub energy_availability: f64,
    pub hunger_level: f64,
    pub thermoregulatory_stress: f64,
    pub activity_capacity: f64,
    
    // Nutritional influences
    pub nutrient_deficiency_count: u32,
    pub average_deficiency_severity: f64,
    
    // Hydration influences
    pub thirst_drive: f64,
    pub physical_performance: f64,
    
    // Fatigue influences
    pub cognitive_performance: f64,
    pub reaction_time_modifier: f64,
    pub sleep_pressure: f64,
    
    // Injury influences
    pub pain_level: f64,
    pub mobility_impairment: f64,
    
    // Reproductive influences
    pub fertility_level: f64,
    pub sex_drive: f64,
    pub hormonal_balance: f64,
    
    // Overall state
    pub overall_health: f64,
}

/// Summary of current biological state
/// Used for quick status checks and logging
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiologicalStateSummary {
    /// Overall health (0.0 = critical, 1.0 = optimal)
    pub overall_health: f64,
    /// Energy level (0.0 = exhausted, 1.0 = fresh)
    pub energy_level: f64,
    /// Hydration level (0.0 = dehydrated, 1.0 = hydrated)
    pub hydration_level: f64,
    /// Stress level (0.0 = calm, 1.0 = extreme stress)
    pub stress_level: f64,
    /// Pain level (0.0 = no pain, 1.0 = extreme pain)
    pub pain_level: f64,
    /// Hunger level (0.0 = sated, 1.0 = starving)
    pub hunger_level: f64,
    /// Current fertility status
    pub fertility_status: super::reproductive::FertilityStatus,
    /// Number of active wounds
    pub active_wounds: usize,
    /// Sleep pressure (0.0 = alert, 1.0 = very sleepy)
    pub sleep_pressure: f64,
}

/// Reasons why biology might veto an action
/// Unified from all biological systems
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BioVetoReason {
    // Endocrine vetoes
    StressOverload,
    Anhedonia,
    ExcessiveFatigue,
    LowConfidence,
    HormonalImbalance,
    
    // Metabolic vetoes
    EnergyExhaustion,
    ThermoregulatoryFailure,
    ExtremeFatigue,
    MetabolicImbalance,
    
    // Nutritional vetoes
    SevereDehydration,
    SevereNutrientDeficiency(super::nutrition::Micronutrient),
    ExtremeCalorieDeficit,
    NutritionalImbalance,
    
    // Hydration vetoes
    CriticalDehydration,
    UrgentWasteElimination(super::hydration::WasteType),
    HydrationImbalance,
    
    // Fatigue vetoes
    CompleteExhaustion,
    SevereFatigue,
    CircadianSleepPressure,
    FatigueImbalance,
    
    // Injury vetoes
    CriticalInjury,
    SevereMobilityImpairment,
    SevereCognitiveImpairment,
    ExtremePain,
    InjuryLimitation,
    
    // Reproductive vetoes
    LatePregnancyLimitations,
    SevereMenstrualSymptoms,
    ReproductiveLimitations,
    
    // Overall health veto
    CriticalHealthFailure,
}

/// Activity factors for biological system updates
/// Re-exported from fatigue module for convenience
pub use super::fatigue::ActivityFactors;

/// Healing factors for injury system updates
/// Re-exported from injury module for convenience
pub use super::injury::HealingFactors;

#[cfg(test)]
mod tests {
    use super::*;
    use super::reproductive::BiologicalSex;

    #[test]
    fn test_biological_state_creation() {
        let state = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        assert_eq!(state.overall_health, 1.0);
        assert_eq!(state.tick, 0);
        assert_eq!(state.reproductive.biological_sex, BiologicalSex::Male);
    }

    #[test]
    fn test_biological_update() {
        let mut state = BiologicalState::new(BiologicalSex::Female, 25.0);
        let activity = ActivityFactors::moderate();
        
        let update = state.update(&activity);
        
        assert_eq!(update.tick, 1);
        assert!(update.overall_health >= 0.0);
        assert!(update.overall_health <= 1.0);
    }

    #[test]
    fn test_action_viability_check() {
        let state = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        let vetoes = state.check_action_viability();
        
        // Should have no vetoes for healthy state
        assert!(vetoes.is_empty());
    }

    #[test]
    fn test_external_effect_application() {
        let mut state = BiologicalState::new(BiologicalSex::Male, 30.0);
        let initial_stress = state.endocrine.get_level(super::endocrine::HormoneType::Cortisol);
        
        state.apply_external_effect(BiologicalEffect::Stress(0.5));
        
        assert!(state.endocrine.get_level(super::endocrine::HormoneType::Cortisol) > initial_stress);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut state1 = BiologicalState::new(BiologicalSex::Male, 30.0);
        let mut state2 = BiologicalState::new(BiologicalSex::Male, 30.0);
        
        let activity = ActivityFactors::light();
        
        // Run identical updates
        for _ in 0..100 {
            state1.update(&activity);
            state2.update(&activity);
        }
        
        // States should be identical
        assert_eq!(state1, state2);
    }
}
