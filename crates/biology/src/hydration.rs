/*!
# Hydration System

**Purpose:** Deterministic water balance and waste management for human-equivalent biology.

**Why it exists:** Hydration is critical for cognitive function, physical performance,
and survival. Agents must experience thirst, dehydration effects, and the need for
regular water intake to maintain human equivalence.

**Determinism guarantees:**
- Water loss follows fixed rates based on activity and environment
- Thirst signals appear at deterministic dehydration thresholds
- No random fluctuations in hydration levels
- All water changes have identifiable sources

**How it affects replay:** Same sequence of water intake and activities will
produce identical hydration trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

/// Hydration levels with corresponding physiological and cognitive effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HydrationLevel {
    /// Life-threatening dehydration (<50%)
    Critical,
    /// Severe dehydration (50-60%)
    Severe,
    /// Moderate dehydration (60-75%)
    Moderate,
    /// Mild dehydration (75-85%)
    Mild,
    /// Optimal hydration (85-95%)
    Optimal,
    /// Fully hydrated (95-100%)
    Full,
}

impl HydrationLevel {
    /// Convert from hydration percentage (0.0 = empty, 1.0 = full)
    pub fn from_percentage(percentage: f64) -> Self {
        match percentage {
            p if p < 0.5 => Self::Critical,
            p if p < 0.6 => Self::Severe,
            p if p < 0.75 => Self::Moderate,
            p if p < 0.85 => Self::Mild,
            p if p < 0.95 => Self::Optimal,
            _ => Self::Full,
        }
    }

    /// Get cognitive performance modifier (1.0 = optimal, lower = impaired)
    pub fn cognitive_modifier(self) -> f64 {
        match self {
            Self::Critical => 0.3,
            Self::Severe => 0.5,
            Self::Moderate => 0.7,
            Self::Mild => 0.85,
            Self::Optimal => 1.0,
            Self::Full => 1.0,
        }
    }

    /// Get physical performance modifier (1.0 = optimal, lower = impaired)
    pub fn physical_modifier(self) -> f64 {
        match self {
            Self::Critical => 0.2,
            Self::Severe => 0.4,
            Self::Moderate => 0.6,
            Self::Mild => 0.8,
            Self::Optimal => 1.0,
            Self::Full => 1.0,
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Critical => "critically dehydrated",
            Self::Severe => "severely dehydrated",
            Self::Moderate => "moderately dehydrated",
            Self::Mild => "mildly dehydrated",
            Self::Optimal => "optimally hydrated",
            Self::Full => "fully hydrated",
        }
    }
}

/// Waste accumulation types with different urgency levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WasteType {
    /// Urinary waste (moderate urgency)
    Bladder,
    /// Bowel waste (variable urgency)
    Bowel,
}

impl WasteType {
    /// Maximum capacity before urgent need to eliminate
    pub const fn max_capacity(self) -> f32 {
        match self {
            Self::Bladder => 0.8,  // 80% of bladder capacity
            Self::Bowel => 0.9,  // 90% of bowel capacity
        }
    }

    /// Urgency threshold (percentage of capacity)
    pub const fn urgency_threshold(self) -> f32 {
        match self {
            Self::Bladder => 0.6,  // Urgent at 60%
            Self::Bowel => 0.7,    // Urgent at 70%
        }
    }

    /// Production rate per tick (varies by activity and intake)
    pub const fn base_production_rate(self) -> f64 {
        match self {
            Self::Bladder => 0.001,  // Slow accumulation
            Self::Bowel => 0.0005,   // Very slow accumulation
        }
    }

    /// Name for logging and display
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bladder => "bladder",
            Self::Bowel => "bowel",
        }
    }
}

/// Complete hydration system
/// Manages water balance, thirst, and waste elimination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydrationSystem {
    /// Current hydration level (0.0 = empty, 1.0 = full)
    pub hydration_percentage: f64,
    /// Total body water in liters
    pub total_water_liters: f64,
    /// Waste accumulation by type
    waste_levels: BTreeMap<WasteType, f64>,
    /// Thirst level (0.0 = no thirst, 1.0 = extreme thirst)
    pub thirst_level: f64,
    /// Activity modifier for water loss
    pub activity_modifier: f64,
    /// Environmental temperature modifier
    pub temperature_modifier: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl HydrationSystem {
    /// Create new hydration system with human-equivalent baseline
    pub fn new() -> Self {
        let mut waste_levels = BTreeMap::new();
        waste_levels.insert(WasteType::Bladder, 0.1);
        waste_levels.insert(WasteType::Bowel, 0.1);
        
        Self {
            hydration_percentage: 0.9, // Start well hydrated
            total_water_liters: 40.0, // ~40L in average adult
            waste_levels,
            thirst_level: 0.1,
            activity_modifier: 1.0,
            temperature_modifier: 1.0,
            tick: 0,
        }
    }

    /// Update hydration system for one tick
    /// Returns hydration changes for audit logging
    pub fn update(&mut self) -> HydrationUpdate {
        let old_hydration = self.hydration_percentage;
        let old_thirst = self.thirst_level;
        
        // Calculate water loss
        let water_loss = self.calculate_water_loss();
        
        // Apply water loss
        self.hydration_percentage = (self.hydration_percentage - water_loss).max(0.0);
        self.total_water_liters = self.hydration_percentage * 40.0; // Update total water
        
        // Update thirst based on hydration
        self.update_thirst();
        
        // Accumulate waste
        self.accumulate_waste();
        
        self.tick += 1;
        
        trace!("Hydration update {}: loss {:.6}, hydration {:.3}, thirst {:.3}", 
               self.tick, water_loss, self.hydration_percentage, self.thirst_level);
        
        HydrationUpdate {
            water_loss,
            hydration_change: self.hydration_percentage - old_hydration,
            thirst_change: self.thirst_level - old_thirst,
            current_level: HydrationLevel::from_percentage(self.hydration_percentage),
        }
    }

    /// Calculate total water loss for current tick
    fn calculate_water_loss(&self) -> f64 {
        // Base water loss rate (liters per tick)
        let base_loss = 0.0001; // ~0.36L/hour baseline
        
        // Modify by activity level
        let activity_loss = base_loss * self.activity_modifier;
        
        // Modify by temperature
        let temperature_loss = base_loss * self.temperature_modifier * 0.5;
        
        // Total loss
        activity_loss + temperature_loss
    }

    /// Update thirst level based on hydration
    fn update_thirst(&mut self) {
        // Thirst increases as hydration decreases
        let optimal_hydration = 0.9;
        let hydration_deficit = (optimal_hydration - self.hydration_percentage).max(0.0);
        
        // Thirst rises exponentially with dehydration
        self.thirst_level = (hydration_deficit / optimal_hydration).powi(2).min(1.0);
    }

    /// Accumulate waste based on intake and activity
    fn accumulate_waste(&mut self) {
        for (waste_type, level) in &mut self.waste_levels {
            let production_rate = waste_type.base_production_rate() as f64 * self.activity_modifier;
            *level = (*level + production_rate).min(waste_type.max_capacity());
        }
    }

    /// Drink water
    /// Increases hydration and reduces thirst
    pub fn drink_water(&mut self, amount_liters: f64) {
        let hydration_increase = amount_liters / self.total_water_liters;
        self.hydration_percentage = (self.hydration_percentage + hydration_increase).min(1.0);
        self.total_water_liters = self.hydration_percentage * 40.0;
        
        // Immediately reduce thirst
        self.thirst_level = (self.thirst_level - hydration_increase * 2.0).max(0.0);
        
        debug!("Drank {:.2}L water, hydration: {:.3}, thirst: {:.3}", 
               amount_liters, self.hydration_percentage, self.thirst_level);
    }

    /// Eliminate waste
    /// Reduces waste levels and provides relief
    pub fn eliminate_waste(&mut self, waste_type: WasteType) -> Result<(), WasteEliminationError> {
        if let Some(level) = self.waste_levels.get_mut(&waste_type) {
            if *level < waste_type.urgency_threshold() {
                return Err(WasteEliminationError::NoUrgency);
            }
            
            *level = 0.0; // Complete elimination
            debug!("Eliminated {} waste", waste_type.name());
            Ok(())
        } else {
            Err(WasteEliminationError::UnknownWasteType)
        }
    }

    /// Set activity modifier for water loss
    /// Higher activity increases water loss
    pub fn set_activity_modifier(&mut self, modifier: f64) {
        self.activity_modifier = modifier.clamp(0.5, 5.0);
        debug!("Set hydration activity modifier to {:.2}", self.activity_modifier);
    }

    /// Set temperature modifier for water loss
    /// Higher temperature increases water loss
    pub fn set_temperature_modifier(&mut self, modifier: f64) {
        self.temperature_modifier = modifier.clamp(0.5, 3.0);
        debug!("Set hydration temperature modifier to {:.2}", self.temperature_modifier);
    }

    /// Check if hydration state supports action execution
    /// Returns HydrationBioVeto reasons if hydration would prevent action
    pub fn check_action_viability(&self) -> Vec<HydrationBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Critical dehydration
        if self.hydration_percentage < 0.5 {
            vetoes.push(HydrationBioVetoReason::CriticalDehydration);
        }
        
        // Urgent waste elimination needs
        for (waste_type, level) in &self.waste_levels {
            if (*level as f64) > waste_type.max_capacity() as f64 * 0.9 {
                vetoes.push(HydrationBioVetoReason::UrgentWasteElimination(*waste_type));
            }
        }
        
        vetoes
    }

    /// Get current hydration state for cognitive processing
    /// Hydration affects cognitive performance and decision making
    pub fn get_cognitive_modulators(&self) -> HydrationModulators {
        let hydration_level = HydrationLevel::from_percentage(self.hydration_percentage);
        
        HydrationModulators {
            thirst_drive: self.thirst_level,
            cognitive_performance: hydration_level.cognitive_modifier(),
            physical_performance: hydration_level.physical_modifier(),
            waste_urgency: self.calculate_waste_urgency(),
        }
    }

    /// Calculate overall waste urgency
    fn calculate_waste_urgency(&self) -> f64 {
        let mut max_urgency = 0.0;
        
        for (waste_type, level) in &self.waste_levels {
            let urgency: f64 = *level / waste_type.max_capacity();
            max_urgency = max_urgency.max(urgency);
        }
        
        max_urgency
    }

    /// Get current hydration level
    pub fn get_hydration_level(&self) -> HydrationLevel {
        HydrationLevel::from_percentage(self.hydration_percentage)
    }

    /// Get waste level for specific type
    pub fn get_waste_level(&self, waste_type: WasteType) -> f64 {
        self.waste_levels.get(&waste_type).copied().unwrap_or(0.0)
    }
}

/// Results of a hydration update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydrationUpdate {
    /// Water lost this tick
    pub water_loss: f64,
    /// Change in hydration percentage
    pub hydration_change: f64,
    /// Change in thirst level
    pub thirst_change: f64,
    /// Current hydration level category
    pub current_level: HydrationLevel,
}

/// Hydration effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydrationModulators {
    /// Thirst drive (0.0 = no thirst, 1.0 = extreme thirst)
    pub thirst_drive: f64,
    /// Cognitive performance modifier (0.0 = severely impaired, 1.0 = optimal)
    pub cognitive_performance: f64,
    /// Physical performance modifier (0.0 = severely impaired, 1.0 = optimal)
    pub physical_performance: f64,
    /// Overall waste elimination urgency (0.0 = none, 1.0 = urgent)
    pub waste_urgency: f64,
}

/// Reasons why hydration might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HydrationBioVetoReason {
    /// Hydration level life-threateningly low
    CriticalDehydration,
    /// Urgent need for waste elimination
    UrgentWasteElimination(WasteType),
    /// General hydration imbalance
    HydrationImbalance,
}

/// Errors that can occur during waste elimination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasteEliminationError {
    /// No urgency to eliminate waste
    NoUrgency,
    /// Unknown waste type
    UnknownWasteType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydration_level_conversion() {
        assert_eq!(HydrationLevel::from_percentage(0.4), HydrationLevel::Critical);
        assert_eq!(HydrationLevel::from_percentage(0.55), HydrationLevel::Severe);
        assert_eq!(HydrationLevel::from_percentage(0.7), HydrationLevel::Moderate);
        assert_eq!(HydrationLevel::from_percentage(0.8), HydrationLevel::Mild);
        assert_eq!(HydrationLevel::from_percentage(0.9), HydrationLevel::Optimal);
        assert_eq!(HydrationLevel::from_percentage(0.98), HydrationLevel::Full);
    }

    #[test]
    fn test_water_drinking() {
        let mut system = HydrationSystem::new();
        let initial_hydration = system.hydration_percentage;
        let initial_thirst = system.thirst_level;
        
        system.drink_water(1.0); // Drink 1 liter
        
        assert!(system.hydration_percentage > initial_hydration);
        assert!(system.thirst_level < initial_thirst);
    }

    #[test]
    fn test_waste_accumulation() {
        let mut system = HydrationSystem::new();
        let initial_bladder = system.get_waste_level(WasteType::Bladder);
        
        // Run several ticks to accumulate waste
        for _ in 0..1000 {
            system.update();
        }
        
        assert!(system.get_waste_level(WasteType::Bladder) > initial_bladder);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = HydrationSystem::new();
        let mut system2 = HydrationSystem::new();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update();
            system2.update();
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
