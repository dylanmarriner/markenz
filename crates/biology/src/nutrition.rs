/*!
# Nutrition System

**Purpose:** Deterministic nutrient tracking and deficiency management for human-equivalent biology.

**Why it exists:** Nutrition governs long-term health, cognitive function, and physical
performance. Agents must experience hunger, nutrient deficiencies, and the effects of
diet on their capabilities to maintain human equivalence.

**Determinism guarantees:**
- Nutrient absorption follows fixed percentages
- Deficiency symptoms appear at deterministic thresholds
- No random fluctuations in nutrient levels
- All nutrient changes have identifiable sources

**How it affects replay:** Same sequence of food intake will produce identical
nutrient trajectories and deficiency states across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Essential micronutrients with human physiological requirements
/// All values are in milligrams per day unless otherwise specified
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Micronutrient {
    /// Fat-soluble vitamins
    VitaminA,
    VitaminD,
    VitaminE,
    VitaminK,
    /// Water-soluble vitamins
    VitaminC,
    Thiamine(B1),
    Riboflavin(B2),
    Niacin(B3),
    VitaminB6,
    Folate(B9),
    VitaminB12,
    /// Major minerals
    Calcium,
    Iron,
    Magnesium,
    Phosphorus,
    Potassium,
    Sodium,
    Zinc,
    Copper,
    Manganese,
    Selenium,
    Iodine,
}

impl Micronutrient {
    /// Daily requirement for average adult
    pub const fn daily_requirement(self) -> f64 {
        match self {
            Self::VitaminA => 900.0,      // μg RAE
            Self::VitaminD => 2000.0,     // IU
            Self::VitaminE => 15.0,        // mg
            Self::VitaminK => 120.0,       // μg
            Self::VitaminC => 90.0,        // mg
            Self::Thiamine(_) => 1.2,      // mg
            Self::Riboflavin(_) => 1.3,    // mg
            Self::Niacin(_) => 16.0,       // mg NE
            Self::VitaminB6 => 1.3,       // mg
            Self::Folate(_) => 400.0,      // μg DFE
            Self::VitaminB12 => 2.4,      // μg
            Self::Calcium => 1000.0,       // mg
            Self::Iron => 8.0,             // mg
            Self::Magnesium => 420.0,       // mg
            Self::Phosphorus => 700.0,      // mg
            Self::Potassium => 3400.0,     // mg
            Self::Sodium => 2300.0,        // mg
            Self::Zinc => 11.0,            // mg
            Self::Copper => 0.9,           // mg
            Self::Manganese => 2.3,        // mg
            Self::Selenium => 55.0,        // μg
            Self::Iodine => 150.0,         // μg
        }
    }

    /// Absorption efficiency (fraction of intake actually used)
    pub const fn absorption_efficiency(self) -> f64 {
        match self {
            Self::Iron => 0.1,  // Iron has poor absorption
            Self::Calcium => 0.3,  // Calcium absorption is limited
            Self::Zinc => 0.2,  // Zinc absorption is moderate
            Self::VitaminD => 0.8,  // Vitamin D absorption is good
            _ => 0.7,  // Most nutrients have ~70% absorption
        }
    }

    /// Storage capacity in body (days of requirement)
    /// Determines how quickly deficiencies develop
    pub const fn storage_days(self) -> f64 {
        match self {
            Self::VitaminA => 90.0,   // Stored in liver
            Self::VitaminD => 60.0,   // Stored in fat
            Self::VitaminB12 => 365.0, // Stored for years
            Self::Iron => 120.0,       // Stored in ferritin
            Self::Calcium => 365.0,    // Stored in bones
            _ => 30.0,  // Most water-soluble vitamins have minimal storage
        }
    }

    /// Name for logging and display
    pub const fn name(self) -> &'static str {
        match self {
            Self::VitaminA => "Vitamin A",
            Self::VitaminD => "Vitamin D",
            Self::VitaminE => "Vitamin E",
            Self::VitaminK => "Vitamin K",
            Self::VitaminC => "Vitamin C",
            Self::Thiamine(_) => "Thiamine (B1)",
            Self::Riboflavin(_) => "Riboflavin (B2)",
            Self::Niacin(_) => "Niacin (B3)",
            Self::VitaminB6 => "Vitamin B6",
            Self::Folate(_) => "Folate (B9)",
            Self::VitaminB12 => "Vitamin B12",
            Self::Calcium => "Calcium",
            Self::Iron => "Iron",
            Self::Magnesium => "Magnesium",
            Self::Phosphorus => "Phosphorus",
            Self::Potassium => "Potassium",
            Self::Sodium => "Sodium",
            Self::Zinc => "Zinc",
            Self::Copper => "Copper",
            Self::Manganese => "Manganese",
            Self::Selenium => "Selenium",
            Self::Iodine => "Iodine",
        }
    }
}

/// Nutrient store tracking current levels and deficits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutrientStore {
    /// Current amount stored in body
    pub current_level: f64,
    /// Daily requirement for optimal function
    pub daily_requirement: f64,
    /// Absorption efficiency from food
    pub absorption_efficiency: f64,
    /// Storage capacity in days
    pub storage_days: f64,
}

impl NutrientStore {
    /// Create new nutrient store with human-equivalent baseline
    pub fn new(micronutrient: Micronutrient) -> Self {
        let daily_req = micronutrient.daily_requirement();
        let storage_capacity = daily_req * micronutrient.storage_days();
        
        Self {
            current_level: storage_capacity * 0.7, // Start at 70% capacity
            daily_requirement: daily_req,
            absorption_efficiency: micronutrient.absorption_efficiency(),
            storage_days: micronutrient.storage_days(),
        }
    }

    /// Add nutrients from food intake
    pub fn add(&mut self, amount: f64) {
        let absorbed = amount * self.absorption_efficiency;
        self.current_level += absorbed;
        
        trace!("Added {:.2} {} (absorbed: {:.2}), current level: {:.2}", 
               amount, "units", absorbed, self.current_level);
    }

    /// Consume nutrients for daily requirements
    /// Returns amount actually consumed
    pub fn consume_daily(&mut self) -> f64 {
        let consumed = self.current_level.min(self.daily_requirement);
        self.current_level -= consumed;
        consumed
    }

    /// Check if nutrient level indicates deficiency
    pub fn is_deficient(&self) -> bool {
        let days_of_supply = self.current_level / self.daily_requirement;
        days_of_supply < 7.0 // Less than 7 days supply = deficiency
    }

    /// Check if nutrient level indicates severe deficiency
    pub fn is_severely_deficient(&self) -> bool {
        let days_of_supply = self.current_level / self.daily_requirement;
        days_of_supply < 2.0 // Less than 2 days supply = severe deficiency
    }

    /// Get deficiency severity (0.0 = optimal, 1.0 = severely deficient)
    pub fn deficiency_severity(&self) -> f64 {
        let days_of_supply = self.current_level / self.daily_requirement;
        if days_of_supply >= 30.0 {
            0.0
        } else if days_of_supply <= 2.0 {
            1.0
        } else {
            1.0 - (days_of_supply - 2.0) / 28.0
        }
    }
}

/// Complete nutrition system
/// Manages all macro and micronutrient tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutritionSystem {
    /// All micronutrients indexed by type for deterministic iteration
    micronutrients: BTreeMap<Micronutrient, NutrientStore>,
    /// Hydration level (0.0 = dehydrated, 1.0 = fully hydrated)
    pub hydration_level: f64,
    /// Total calories consumed today
    pub daily_calories: f64,
    /// Daily calorie requirement
    pub daily_calorie_requirement: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl NutritionSystem {
    /// Create new nutrition system with human-equivalent baseline
    pub fn new() -> Self {
        let mut micronutrients = BTreeMap::new();
        
        // Initialize all micronutrients
        for micronutrient in [
            Micronutrient::VitaminA,
            Micronutrient::VitaminD,
            Micronutrient::VitaminE,
            Micronutrient::VitaminK,
            Micronutrient::VitaminC,
            Micronutrient::Thiamine(B1),
            Micronutrient::Riboflavin(B2),
            Micronutrient::Niacin(B3),
            Micronutrient::VitaminB6,
            Micronutrient::Folate(B9),
            Micronutrient::VitaminB12,
            Micronutrient::Calcium,
            Micronutrient::Iron,
            Micronutrient::Magnesium,
            Micronutrient::Phosphorus,
            Micronutrient::Potassium,
            Micronutrient::Sodium,
            Micronutrient::Zinc,
            Micronutrient::Copper,
            Micronutrient::Manganese,
            Micronutrient::Selenium,
            Micronutrient::Iodine,
        ] {
            micronutrients.insert(micronutrient, NutrientStore::new(micronutrient));
        }
        
        Self {
            micronutrients,
            hydration_level: 0.9, // Start well hydrated
            daily_calories: 0.0,
            daily_calorie_requirement: 2000.0, // Average adult requirement
            tick: 0,
        }
    }

    /// Update nutrition system for one tick
    /// Returns nutrition changes for audit logging
    pub fn update(&mut self) -> NutritionUpdate {
        let mut consumed_nutrients = BTreeMap::new();
        
        // Consume daily requirements for all micronutrients
        for (micronutrient, store) in &mut self.micronutrients {
            let consumed = store.consume_daily();
            consumed_nutrients.insert(*micronutrient, consumed);
        }
        
        // Update hydration (small decrease each tick)
        self.hydration_level = (self.hydration_level - 0.0001).max(0.0);
        
        self.tick += 1;
        
        trace!("Nutrition update {}: hydration {:.3}, daily calories {:.0}", 
               self.tick, self.hydration_level, self.daily_calories);
        
        NutritionUpdate {
            consumed_nutrients,
            hydration_change: -0.0001,
            daily_calories: self.daily_calories,
        }
    }

    /// Process food intake
    /// Adds nutrients and calories from consumed food
    pub fn consume_food(&mut self, food: &FoodItem) {
        // Add micronutrients
        for (micronutrient, amount) in &food.micronutrients {
            if let Some(store) = self.micronutrients.get_mut(micronutrient) {
                store.add(*amount);
            }
        }
        
        // Add calories
        self.daily_calories += food.calories;
        
        // Add hydration
        self.hydration_level = (self.hydration_level + food.hydration).min(1.0);
        
        debug!("Consumed food: {} calories, {:.3} hydration", 
               food.calories, food.hydration);
    }

    /// Drink water directly
    pub fn drink_water(&mut self, amount: f64) {
        self.hydration_level = (self.hydration_level + amount).min(1.0);
        debug!("Drank water, hydration level: {:.3}", self.hydration_level);
    }

    /// Reset daily counters (called at start of each day)
    pub fn reset_daily(&mut self) {
        self.daily_calories = 0.0;
        debug!("Reset daily nutrition counters");
    }

    /// Check if nutritional state supports action execution
    /// Returns NutritionalBioVeto reasons if nutrition would prevent action
    pub fn check_action_viability(&self) -> Vec<NutritionalBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Severe dehydration
        if self.hydration_level < 0.3 {
            vetoes.push(NutritionalBioVetoReason::SevereDehydration);
        }
        
        // Severe nutrient deficiencies
        for (micronutrient, store) in &self.micronutrients {
            if store.is_severely_deficient() {
                vetoes.push(NutritionalBioVetoReason::SevereNutrientDeficiency(*micronutrient));
            }
        }
        
        // Extreme calorie deficit
        if self.daily_calories < self.daily_calorie_requirement * 0.3 {
            vetoes.push(NutritionalBioVetoReason::ExtremeCalorieDeficit);
        }
        
        vetoes
    }

    /// Get current nutritional state for cognitive processing
    /// Nutrition affects cognitive performance and decision making
    pub fn get_cognitive_modulators(&self) -> NutritionalModulators {
        let mut deficiency_count = 0;
        let mut total_deficiency_severity = 0.0;
        
        for store in self.micronutrients.values() {
            if store.is_deficient() {
                deficiency_count += 1;
                total_deficiency_severity += store.deficiency_severity();
            }
        }
        
        NutritionalModulators {
            hunger_level: 1.0 - (self.daily_calories / self.daily_calorie_requirement).min(1.0),
            hydration_level: self.hydration_level,
            nutrient_deficiency_count: deficiency_count,
            average_deficiency_severity: if deficiency_count > 0 {
                total_deficiency_severity / deficiency_count as f64
            } else {
                0.0
            },
        }
    }

    /// Get list of current deficiencies
    pub fn get_deficiencies(&self) -> Vec<(Micronutrient, f64)> {
        let mut deficiencies = Vec::new();
        
        for (micronutrient, store) in &self.micronutrients {
            if store.is_deficient() {
                deficiencies.push((*micronutrient, store.deficiency_severity()));
            }
        }
        
        deficiencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        deficiencies
    }
}

/// Food item with nutritional composition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodItem {
    /// Name of the food
    pub name: String,
    /// Caloric content
    pub calories: f64,
    /// Hydration contribution (0.0 = dry, 1.0 = pure water)
    pub hydration: f64,
    /// Micronutrient content
    pub micronutrients: BTreeMap<Micronutrient, f64>,
}

/// Results of a nutrition update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutritionUpdate {
    /// Nutrients consumed for daily requirements
    pub consumed_nutrients: BTreeMap<Micronutrient, f64>,
    /// Change in hydration level
    pub hydration_change: f64,
    /// Total calories consumed today
    pub daily_calories: f64,
}

/// Nutritional effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutritionalModulators {
    /// Hunger drive (0.0 = sated, 1.0 = starving)
    pub hunger_level: f64,
    /// Hydration level (0.0 = dehydrated, 1.0 = fully hydrated)
    pub hydration_level: f64,
    /// Number of nutrient deficiencies
    pub nutrient_deficiency_count: u32,
    /// Average severity of all deficiencies
    pub average_deficiency_severity: f64,
}

/// Reasons why nutrition might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NutritionalBioVetoReason {
    /// Hydration level critically low
    SevereDehydration,
    /// Specific nutrient severely deficient
    SevereNutrientDeficiency(Micronutrient),
    /// Calorie intake extremely low
    ExtremeCalorieDeficit,
    /// General nutritional imbalance
    NutritionalImbalance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutrient_store_creation() {
        let store = NutrientStore::new(Micronutrient::Iron);
        
        assert!(store.current_level > 0.0);
        assert!(store.daily_requirement > 0.0);
        assert!(store.absorption_efficiency > 0.0);
        assert!(store.absorption_efficiency <= 1.0);
    }

    #[test]
    fn test_nutrient_deficiency_detection() {
        let mut store = NutrientStore::new(Micronutrient::VitaminC);
        
        // Should not be deficient initially
        assert!(!store.is_deficient());
        
        // Consume all nutrients
        store.current_level = 0.0;
        
        // Should be severely deficient
        assert!(store.is_severely_deficient());
        assert_eq!(store.deficiency_severity(), 1.0);
    }

    #[test]
    fn test_food_consumption() {
        let mut system = NutritionSystem::new();
        let initial_hydration = system.hydration_level;
        
        let mut food = FoodItem {
            name: "Apple".to_string(),
            calories: 95.0,
            hydration: 0.15,
            micronutrients: BTreeMap::new(),
        };
        food.micronutrients.insert(Micronutrient::VitaminC, 8.0);
        
        system.consume_food(&food);
        
        assert!(system.daily_calories > 0.0);
        assert!(system.hydration_level > initial_hydration);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = NutritionSystem::new();
        let mut system2 = NutritionSystem::new();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update();
            system2.update();
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
