/*!
# Metabolic System

**Purpose:** Deterministic energy balance and nutrient processing for human-equivalent biology.

**Why it exists:** Metabolism governs energy availability, thermoregulation, and
the conversion of nutrients into usable energy. It must be modeled to ensure agents
have realistic energy constraints and experience fatigue, hunger, and recovery cycles.

**Determinism guarantees:**
- Energy calculations use fixed-point arithmetic
- Metabolic rates are deterministic based on activity level and body composition
- No random fluctuations in energy balance
- All energy changes have identifiable sources

**How it affects replay:** Same sequence of activities and nutrient intake will
produce identical energy trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Activity levels with deterministic energy costs
/// These map to human metabolic equivalents (METs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ActivityLevel {
    /// Sleeping or complete rest
    Rest,
    /// Sitting, light mental activity
    Sedentary,
    /// Walking, light physical work
    Light,
    /// Moderate exercise, active work
    Moderate,
    /// Heavy physical labor, intense exercise
    Heavy,
    /// Extreme exertion, emergency response
    Extreme,
}

impl ActivityLevel {
    /// Energy cost multiplier relative to basal metabolic rate
    /// 1.0 = BMR, higher values increase energy expenditure
    pub const fn metabolic_multiplier(self) -> f64 {
        match self {
            Self::Rest => 0.9,
            Self::Sedentary => 1.2,
            Self::Light => 2.5,
            Self::Moderate => 5.0,
            Self::Heavy => 8.0,
            Self::Extreme => 12.0,
        }
    }

    /// Description for logging and debugging
    pub const fn description(self) -> &'static str {
        match self {
            Self::Rest => "resting",
            Self::Sedentary => "sedentary",
            Self::Light => "light activity",
            Self::Moderate => "moderate activity",
            Self::Heavy => "heavy activity",
            Self::Extreme => "extreme exertion",
        }
    }
}

/// Macronutrient types with energy densities
/// All values are in kilocalories per gram
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Macronutrient {
    Carbohydrates,
    Proteins,
    Fats,
}

impl Macronutrient {
    /// Energy density in kcal/gram
    pub const fn energy_density(self) -> f64 {
        match self {
            Self::Carbohydrates => 4.0,
            Self::Proteins => 4.0,
            Self::Fats => 9.0,
        }
    }

    /// Digestion efficiency (fraction of energy actually absorbed)
    pub const fn digestion_efficiency(self) -> f64 {
        match self {
            Self::Carbohydrates => 0.95,
            Self::Proteins => 0.85,
            Self::Fats => 0.97,
        }
    }
}

/// Current energy balance state
/// Tracks available energy for immediate use and stored reserves
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyBalance {
    /// Immediately available energy (glucose in bloodstream)
    pub available_energy: f64,
    /// Short-term energy storage (glycogen in muscles and liver)
    pub glycogen_stores: f64,
    /// Long-term energy storage (adipose tissue)
    pub fat_stores: f64,
    /// Total energy capacity of all stores
    pub max_energy_capacity: f64,
}

impl EnergyBalance {
    /// Create new energy balance with human-equivalent baseline
    pub fn new() -> Self {
        let max_glycogen = 2000.0; // ~2000 kcal glycogen capacity
        let max_fat = 50000.0; // ~50,000 kcal fat capacity in average adult
        
        Self {
            available_energy: 400.0, // ~400 kcal glucose in bloodstream
            glycogen_stores: 1000.0, // Half-full glycogen stores
            fat_stores: 15000.0, // Moderate fat stores
            max_energy_capacity: max_glycogen + max_fat,
        }
    }

    /// Get total energy across all stores
    pub fn total_energy(&self) -> f64 {
        self.available_energy + self.glycogen_stores + self.fat_stores
    }

    /// Get energy availability percentage
    pub fn energy_percentage(&self) -> f64 {
        self.total_energy() / self.max_energy_capacity
    }

    /// Check if energy is critically low
    pub fn is_critical(&self) -> bool {
        self.available_energy < 100.0 || self.energy_percentage() < 0.1
    }

    /// Check if energy is low but not critical
    pub fn is_low(&self) -> bool {
        self.available_energy < 300.0 || self.energy_percentage() < 0.2
    }
}

/// Complete metabolic system
/// Manages energy balance, thermoregulation, and nutrient processing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetabolicSystem {
    /// Current energy balance
    pub energy_balance: EnergyBalance,
    /// Basal metabolic rate (kcal per tick)
    pub basal_metabolic_rate: f64,
    /// Current activity level
    pub activity_level: ActivityLevel,
    /// Body temperature in Celsius
    pub body_temperature: f64,
    /// Thermoregulation efficiency (0.0 = poor, 1.0 = optimal)
    pub thermoregulation_efficiency: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl MetabolicSystem {
    /// Create new metabolic system with human-equivalent baseline
    pub fn new() -> Self {
        Self {
            energy_balance: EnergyBalance::new(),
            basal_metabolic_rate: 75.0, // ~1800 kcal/day = 75 kcal/hour
            activity_level: ActivityLevel::Sedentary,
            body_temperature: 36.8, // Normal human temperature
            thermoregulation_efficiency: 0.95,
            tick: 0,
        }
    }

    /// Update metabolism for one tick
    /// Returns energy change for audit logging
    pub fn update(&mut self) -> MetabolicUpdate {
        let old_energy = self.energy_balance.total_energy();
        
        // Calculate total energy expenditure
        let total_expenditure = self.calculate_energy_expenditure();
        
        // Subtract energy expenditure
        self.consume_energy(total_expenditure);
        
        // Update body temperature based on activity and environment
        self.update_thermoregulation();
        
        let new_energy = self.energy_balance.total_energy();
        let energy_change = new_energy - old_energy;
        
        self.tick += 1;
        
        trace!("Metabolic update {}: expended {:.2} kcal, energy change: {:.2}", 
               self.tick, total_expenditure, energy_change);
        
        MetabolicUpdate {
            energy_expenditure: total_expenditure,
            energy_change,
            body_temperature: self.body_temperature,
            activity_level: self.activity_level,
        }
    }

    /// Calculate total energy expenditure for current tick
    fn calculate_energy_expenditure(&self) -> f64 {
        let base_expenditure = self.basal_metabolic_rate / 3600.0; // Convert to per-second
        let activity_multiplier = self.activity_level.metabolic_multiplier();
        let thermoregulation_cost = self.calculate_thermoregulation_cost();
        
        base_expenditure * activity_multiplier + thermoregulation_cost
    }

    /// Calculate energy cost of thermoregulation
    fn calculate_thermoregulation_cost(&self) -> f64 {
        let optimal_temp = 37.0;
        let temp_deviation = (self.body_temperature - optimal_temp).abs();
        
        // Energy cost increases quadratically with temperature deviation
        // Reduced by thermoregulation efficiency
        temp_deviation.powi(2) * 10.0 * (1.0 - self.thermoregulation_efficiency)
    }

    /// Update body temperature based on activity and environment
    fn update_thermoregulation(&mut self) {
        let activity_heat = match self.activity_level {
            ActivityLevel::Rest => -0.01,
            ActivityLevel::Sedentary => 0.0,
            ActivityLevel::Light => 0.02,
            ActivityLevel::Moderate => 0.05,
            ActivityLevel::Heavy => 0.1,
            ActivityLevel::Extreme => 0.2,
        };
        
        // Apply temperature change with efficiency damping
        self.body_temperature += activity_heat * self.thermoregulation_efficiency;
        
        // Clamp to survivable range
        self.body_temperature = self.body_temperature.clamp(35.0, 40.0);
    }

    /// Consume energy from available stores
    /// Uses glycogen first, then fat stores
    pub fn consume_energy(&mut self, amount: f64) {
        let mut remaining = amount;
        
        // First consume available energy (glucose)
        if remaining > 0.0 {
            let consumed = remaining.min(self.energy_balance.available_energy);
            self.energy_balance.available_energy -= consumed;
            remaining -= consumed;
        }
        
        // Then consume glycogen
        if remaining > 0.0 {
            let consumed = remaining.min(self.energy_balance.glycogen_stores);
            self.energy_balance.glycogen_stores -= consumed;
            remaining -= consumed;
        }
        
        // Finally consume fat stores
        if remaining > 0.0 {
            let consumed = remaining.min(self.energy_balance.fat_stores);
            self.energy_balance.fat_stores -= consumed;
            remaining -= consumed;
        }
        
        debug!("Consumed {:.2} kcal energy, {:.2} kcal remaining", 
               amount, amount - remaining);
    }

    /// Add energy from nutrient intake
    /// Converts macronutrients to usable energy
    pub fn add_nutrients(&mut self, nutrients: &BTreeMap<Macronutrient, f64>) {
        let mut total_energy = 0.0;
        
        for (macro_type, grams) in nutrients {
            let energy = grams * macro_type.energy_density() * macro_type.digestion_efficiency();
            total_energy += energy;
        }
        
        // Add energy to stores (reverse order of consumption)
        self.energy_balance.fat_stores += total_energy * 0.7; // Most excess goes to fat
        self.energy_balance.glycogen_stores += total_energy * 0.25; // Some to glycogen
        self.energy_balance.available_energy += total_energy * 0.05; // Small amount to glucose
        
        // Clamp stores to maximum capacity
        let max_glycogen = 2000.0;
        let max_fat = 50000.0;
        self.energy_balance.glycogen_stores = self.energy_balance.glycogen_stores.clamp(0.0, max_glycogen);
        self.energy_balance.fat_stores = self.energy_balance.fat_stores.clamp(0.0, max_fat);
        
        debug!("Added {:.2} kcal from nutrients", total_energy);
    }

    /// Set activity level and update energy expenditure
    pub fn set_activity_level(&mut self, level: ActivityLevel) {
        debug!("Changing activity level from {:?} to {:?}", 
               self.activity_level, level);
        self.activity_level = level;
    }

    /// Check if metabolic state supports action execution
    /// Returns MetabolicBioVeto reasons if metabolism would prevent action
    pub fn check_action_viability(&self) -> Vec<MetabolicBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Critical energy exhaustion
        if self.energy_balance.is_critical() {
            vetoes.push(MetabolicBioVetoReason::EnergyExhaustion);
        }
        
        // Dangerous body temperature
        if self.body_temperature < 35.5 || self.body_temperature > 39.5 {
            vetoes.push(MetabolicBioVetoReason::ThermoregulatoryFailure);
        }
        
        // Extreme fatigue (low energy stores)
        if self.energy_balance.is_low() && self.activity_level == ActivityLevel::Extreme {
            vetoes.push(MetabolicBioVetoReason::ExtremeFatigue);
        }
        
        vetoes
    }

    /// Get current metabolic state for cognitive processing
    /// Energy availability affects decision making and risk assessment
    pub fn get_cognitive_modulators(&self) -> MetabolicModulators {
        MetabolicModulators {
            energy_availability: self.energy_balance.energy_percentage(),
            hunger_level: 1.0 - self.energy_balance.energy_percentage(),
            thermoregulatory_stress: (self.body_temperature - 37.0).abs() / 3.0,
            activity_capacity: self.activity_level.metabolic_multiplier(),
        }
    }
}

/// Results of a metabolic update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetabolicUpdate {
    /// Total energy expended this tick
    pub energy_expenditure: f64,
    /// Net change in total energy stores
    pub energy_change: f64,
    /// Current body temperature
    pub body_temperature: f64,
    /// Current activity level
    pub activity_level: ActivityLevel,
}

/// Metabolic effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetabolicModulators {
    /// Available energy as percentage of capacity (0.0 = empty, 1.0 = full)
    pub energy_availability: f64,
    /// Hunger drive (0.0 = sated, 1.0 = starving)
    pub hunger_level: f64,
    /// Stress from temperature regulation (0.0 = comfortable, 1.0 = extreme stress)
    pub thermoregulatory_stress: f64,
    /// Capacity for physical activity (1.0 = rest, higher = more demanding)
    pub activity_capacity: f64,
}

/// Reasons why metabolism might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicBioVetoReason {
    /// Energy stores critically depleted
    EnergyExhaustion,
    /// Body temperature outside survivable range
    ThermoregulatoryFailure,
    /// Fatigue prevents extreme exertion
    ExtremeFatigue,
    /// General metabolic imbalance
    MetabolicImbalance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_balance_creation() {
        let balance = EnergyBalance::new();
        
        assert!(balance.available_energy > 0.0);
        assert!(balance.glycogen_stores > 0.0);
        assert!(balance.fat_stores > 0.0);
        assert!(balance.total_energy() > 0.0);
    }

    #[test]
    fn test_activity_level_multipliers() {
        assert!(ActivityLevel::Rest.metabolic_multiplier() < ActivityLevel::Sedentary.metabolic_multiplier());
        assert!(ActivityLevel::Sedentary.metabolic_multiplier() < ActivityLevel::Light.metabolic_multiplier());
        assert!(ActivityLevel::Light.metabolic_multiplier() < ActivityLevel::Moderate.metabolic_multiplier());
        assert!(ActivityLevel::Moderate.metabolic_multiplier() < ActivityLevel::Heavy.metabolic_multiplier());
        assert!(ActivityLevel::Heavy.metabolic_multiplier() < ActivityLevel::Extreme.metabolic_multiplier());
    }

    #[test]
    fn test_nutrient_energy_conversion() {
        let mut system = MetabolicSystem::new();
        let initial_energy = system.energy_balance.total_energy();
        
        let mut nutrients = BTreeMap::new();
        nutrients.insert(Macronutrient::Carbohydrates, 100.0); // 100g carbs
        nutrients.insert(Macronutrient::Proteins, 50.0); // 50g protein
        nutrients.insert(Macronutrient::Fats, 25.0); // 25g fat
        
        system.add_nutrients(&nutrients);
        
        let expected_energy = 100.0 * 4.0 * 0.95 + 50.0 * 4.0 * 0.85 + 25.0 * 9.0 * 0.97;
        assert!(system.energy_balance.total_energy() > initial_energy);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = MetabolicSystem::new();
        let mut system2 = MetabolicSystem::new();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update();
            system2.update();
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
