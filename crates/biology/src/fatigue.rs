/*!
# Fatigue System

**Purpose:** Deterministic fatigue modeling and recovery cycles for human-equivalent biology.

**Why it exists:** Fatigue is a critical biological constraint that affects cognitive
performance, physical capability, and decision making. Agents must experience
realistic fatigue accumulation, sleep needs, and recovery processes.

**Determinism guarantees:**
- Fatigue accumulation follows fixed rates based on activity and stress
- Recovery follows deterministic curves based on rest and sleep
- No random fluctuations in fatigue levels
- All fatigue changes have identifiable sources

**How it affects replay:** Same sequence of activities and rest periods will
produce identical fatigue trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Fatigue levels with corresponding physiological and cognitive effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FatigueLevel {
    /// Dangerous exhaustion (<20% capacity)
    Exhausted,
    /// Severe fatigue (20-40% capacity)
    Severe,
    /// Moderate fatigue (40-60% capacity)
    Moderate,
    /// Mild fatigue (60-80% capacity)
    Mild,
    /// Well rested (80-95% capacity)
    Rested,
    /// Fully rested (95-100% capacity)
    Fresh,
}

impl FatigueLevel {
    /// Convert from energy percentage (0.0 = exhausted, 1.0 = fresh)
    pub fn from_energy_percentage(percentage: f64) -> Self {
        match percentage {
            p if p < 0.2 => Self::Exhausted,
            p if p < 0.4 => Self::Severe,
            p if p < 0.6 => Self::Moderate,
            p if p < 0.8 => Self::Mild,
            p if p < 0.95 => Self::Rested,
            _ => Self::Fresh,
        }
    }

    /// Get cognitive performance modifier (1.0 = optimal, lower = impaired)
    pub fn cognitive_modifier(self) -> f64 {
        match self {
            Self::Exhausted => 0.2,
            Self::Severe => 0.4,
            Self::Moderate => 0.6,
            Self::Mild => 0.8,
            Self::Rested => 0.95,
            Self::Fresh => 1.0,
        }
    }

    /// Get physical performance modifier (1.0 = optimal, lower = impaired)
    pub fn physical_modifier(self) -> f64 {
        match self {
            Self::Exhausted => 0.1,
            Self::Severe => 0.3,
            Self::Moderate => 0.5,
            Self::Mild => 0.7,
            Self::Rested => 0.9,
            Self::Fresh => 1.0,
        }
    }

    /// Get reaction time modifier (1.0 = optimal, higher = slower)
    pub fn reaction_time_modifier(self) -> f64 {
        match self {
            Self::Exhausted => 2.5,
            Self::Severe => 2.0,
            Self::Moderate => 1.5,
            Self::Mild => 1.2,
            Self::Rested => 1.05,
            Self::Fresh => 1.0,
        }
    }

    /// Description for logging and display
    pub const fn description(self) -> &'static str {
        match self {
            Self::Exhausted => "exhausted",
            Self::Severe => "severely fatigued",
            Self::Moderate => "moderately fatigued",
            Self::Mild => "mildly fatigued",
            Self::Rested => "well rested",
            Self::Fresh => "fully rested",
        }
    }
}

/// Types of fatigue that accumulate from different sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FatigueType {
    /// Physical fatigue from muscular exertion
    Physical,
    /// Mental fatigue from cognitive effort
    Mental,
    /// Emotional fatigue from stress and social interaction
    Emotional,
    /// Circadian fatigue from sleep deprivation
    Circadian,
}

impl FatigueType {
    /// Recovery rate multiplier during rest
    pub const fn recovery_multiplier(self) -> f64 {
        match self {
            Self::Physical => 0.02,  // Physical fatigue recovers relatively quickly
            Self::Mental => 0.015,   // Mental fatigue recovers moderately
            Self::Emotional => 0.01,   // Emotional fatigue recovers slowly
            Self::Circadian => 0.025,  // Circadian fatigue recovers with sleep
        }
    }

    /// Accumulation rate multiplier during activity
    pub const fn accumulation_multiplier(self) -> f64 {
        match self {
            Self::Physical => 0.01,   // Physical activity causes rapid accumulation
            Self::Mental => 0.008,    // Mental effort causes moderate accumulation
            Self::Emotional => 0.006,  // Emotional stress causes slow accumulation
            Self::Circadian => 0.001,  // Circadian fatigue accumulates slowly
        }
    }

    /// Name for logging and display
    pub const fn name(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Mental => "mental",
            Self::Emotional => "emotional",
            Self::Circadian => "circadian",
        }
    }
}

/// Sleep quality levels affecting recovery
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SleepQuality {
    /// No sleep or very poor quality
    None,
    /// Poor sleep (light, fragmented)
    Poor,
    /// Fair sleep (some deep sleep)
    Fair,
    /// Good sleep (adequate deep sleep)
    Good,
    /// Excellent sleep (optimal deep and REM sleep)
    Excellent,
}

impl SleepQuality {
    /// Recovery multiplier during sleep
    pub const fn recovery_multiplier(self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Poor => 0.5,
            Self::Fair => 1.0,
            Self::Good => 1.5,
            Self::Excellent => 2.0,
        }
    }

    /// Minimum sleep duration in ticks for this quality
    pub const fn min_duration_ticks(self) -> u64 {
        match self {
            Self::None => 0,
            Self::Poor => 2000,   // ~33 minutes
            Self::Fair => 4000,    // ~67 minutes
            Self::Good => 6000,     // ~100 minutes
            Self::Excellent => 8000, // ~133 minutes
        }
    }
}

/// Complete fatigue system
/// Manages all types of fatigue and recovery processes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FatigueSystem {
    /// Fatigue levels by type (0.0 = no fatigue, 1.0 = maximum fatigue)
    fatigue_levels: BTreeMap<FatigueType, f64>,
    /// Overall energy level (0.0 = exhausted, 1.0 = fresh)
    pub energy_level: f64,
    /// Current sleep state
    pub is_sleeping: bool,
    /// Sleep quality during current sleep period
    pub current_sleep_quality: SleepQuality,
    /// Ticks slept in current sleep period
    pub sleep_duration_ticks: u64,
    /// Time since last sleep (in ticks)
    pub time_since_sleep: u64,
    /// Circadian rhythm phase (0.0 = peak alertness, 1.0 = peak sleepiness)
    pub circadian_phase: f64,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl FatigueSystem {
    /// Create new fatigue system with human-equivalent baseline
    pub fn new() -> Self {
        let mut fatigue_levels = BTreeMap::new();
        fatigue_levels.insert(FatigueType::Physical, 0.1);
        fatigue_levels.insert(FatigueType::Mental, 0.1);
        fatigue_levels.insert(FatigueType::Emotional, 0.1);
        fatigue_levels.insert(FatigueType::Circadian, 0.1);
        
        Self {
            fatigue_levels,
            energy_level: 0.9, // Start well rested
            is_sleeping: false,
            current_sleep_quality: SleepQuality::None,
            sleep_duration_ticks: 0,
            time_since_sleep: 0,
            circadian_phase: 0.1, // Start in morning phase
            tick: 0,
        }
    }

    /// Update fatigue system for one tick
    /// Returns fatigue changes for audit logging
    pub fn update(&mut self, activity_factors: &ActivityFactors) -> FatigueUpdate {
        let old_energy = self.energy_level;
        
        // Update circadian rhythm
        self.update_circadian_rhythm();
        
        // Accumulate fatigue based on activity
        self.accumulate_fatigue(activity_factors);
        
        // Recover fatigue if resting or sleeping
        self.recover_fatigue();
        
        // Calculate overall energy level
        self.calculate_energy_level();
        
        // Update sleep tracking
        self.update_sleep_tracking();
        
        self.tick += 1;
        
        trace!("Fatigue update {}: energy {:.3}, sleeping: {}", 
               self.tick, self.energy_level, self.is_sleeping);
        
        FatigueUpdate {
            energy_change: self.energy_level - old_energy,
            current_level: FatigueLevel::from_energy_percentage(self.energy_level),
            is_sleeping: self.is_sleeping,
            circadian_phase: self.circadian_phase,
        }
    }

    /// Update circadian rhythm phase
    fn update_circadian_rhythm(&mut self) {
        // Circadian cycle completes in 24 hours = 86400 ticks
        let cycle_progress = (self.tick % 86400) as f64 / 86400.0;
        
        // Sleepiness peaks at night (cycle 0.75), alertness peaks in morning (cycle 0.25)
        self.circadian_phase = ((cycle_progress - 0.25).abs() * 2.0).min(1.0);
    }

    /// Accumulate fatigue based on current activities
    fn accumulate_fatigue(&mut self, activity_factors: &ActivityFactors) {
        for (fatigue_type, level) in &mut self.fatigue_levels {
            let accumulation_rate = fatigue_type.accumulation_multiplier();
            let activity_multiplier = match fatigue_type {
                FatigueType::Physical => activity_factors.physical_intensity,
                FatigueType::Mental => activity_factors.mental_intensity,
                FatigueType::Emotional => activity_factors.emotional_intensity,
                FatigueType::Circadian => self.circadian_phase, // Higher at night
            };
            
            *level = (*level + accumulation_rate * activity_multiplier).min(1.0);
        }
    }

    /// Recover fatigue during rest or sleep
    fn recover_fatigue(&mut self) {
        let recovery_multiplier = if self.is_sleeping {
            self.current_sleep_quality.recovery_multiplier()
        } else {
            0.5 // Reduced recovery during wakeful rest
        };
        
        for (fatigue_type, level) in &mut self.fatigue_levels {
            let recovery_rate = fatigue_type.recovery_multiplier();
            *level = (*level - recovery_rate * recovery_multiplier).max(0.0);
        }
    }

    /// Calculate overall energy level from fatigue components
    fn calculate_energy_level(&mut self) {
        let total_fatigue: f64 = self.fatigue_levels.values().sum();
        let average_fatigue = total_fatigue / self.fatigue_levels.len() as f64;
        
        // Energy is inverse of fatigue
        self.energy_level = (1.0 - average_fatigue).max(0.0);
    }

    /// Update sleep tracking variables
    fn update_sleep_tracking(&mut self) {
        if self.is_sleeping {
            self.sleep_duration_ticks += 1;
            self.time_since_sleep = 0;
        } else {
            self.time_since_sleep += 1;
        }
    }

    /// Start sleeping with specified quality
    pub fn start_sleep(&mut self, quality: SleepQuality) {
        self.is_sleeping = true;
        self.current_sleep_quality = quality;
        self.sleep_duration_ticks = 0;
        debug!("Started sleeping with quality: {:?}", quality);
    }

    /// Wake up from sleep
    pub fn wake_up(&mut self) {
        if self.is_sleeping {
            self.is_sleeping = false;
            self.current_sleep_quality = SleepQuality::None;
            debug!("Woke up after {} ticks of sleep", self.sleep_duration_ticks);
        }
    }

    /// Apply restorative effects (rest, meditation, etc.)
    pub fn apply_rest(&mut self, intensity: f64) {
        for (fatigue_type, level) in &mut self.fatigue_levels {
            let recovery_rate = fatigue_type.recovery_multiplier();
            let recovery = recovery_rate * intensity;
            *level = (*level - recovery).max(0.0);
        }
        
        self.calculate_energy_level();
        debug!("Applied rest with intensity {:.2}, energy: {:.3}", 
               intensity, self.energy_level);
    }

    /// Check if fatigue state supports action execution
    /// Returns FatigueBioVeto reasons if fatigue would prevent action
    pub fn check_action_viability(&self) -> Vec<FatigueBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // Complete exhaustion
        if self.energy_level < 0.1 {
            vetoes.push(FatigueBioVetoReason::CompleteExhaustion);
        }
        
        // Severe fatigue for complex tasks
        if self.energy_level < 0.3 {
            vetoes.push(FatigueBioVetoReason::SevereFatigue);
        }
        
        // Circadian sleep pressure
        if self.circadian_phase > 0.8 && self.time_since_sleep > 20000 {
            vetoes.push(FatigueBioVetoReason::CircadianSleepPressure);
        }
        
        vetoes
    }

    /// Get current fatigue state for cognitive processing
    /// Fatigue affects cognitive performance and decision making
    pub fn get_cognitive_modulators(&self) -> FatigueModulators {
        let fatigue_level = FatigueLevel::from_energy_percentage(self.energy_level);
        
        FatigueModulators {
            energy_level: self.energy_level,
            cognitive_performance: fatigue_level.cognitive_modifier(),
            physical_performance: fatigue_level.physical_modifier(),
            reaction_time_modifier: fatigue_level.reaction_time_modifier(),
            sleep_pressure: self.circadian_phase,
            time_since_sleep: self.time_since_sleep,
        }
    }

    /// Get fatigue level for specific type
    pub fn get_fatigue_level(&self, fatigue_type: FatigueType) -> f64 {
        self.fatigue_levels.get(&fatigue_type).copied().unwrap_or(0.0)
    }

    /// Get current fatigue level category
    pub fn get_fatigue_category(&self) -> FatigueLevel {
        FatigueLevel::from_energy_percentage(self.energy_level)
    }
}

/// Activity factors affecting fatigue accumulation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityFactors {
    /// Physical intensity (0.0 = rest, 1.0 = maximum exertion)
    pub physical_intensity: f64,
    /// Mental intensity (0.0 = rest, 1.0 = maximum cognitive effort)
    pub mental_intensity: f64,
    /// Emotional intensity (0.0 = calm, 1.0 = extreme emotional stress)
    pub emotional_intensity: f64,
}

impl ActivityFactors {
    /// Create activity factors for rest
    pub fn rest() -> Self {
        Self {
            physical_intensity: 0.0,
            mental_intensity: 0.0,
            emotional_intensity: 0.0,
        }
    }

    /// Create activity factors for light activity
    pub fn light() -> Self {
        Self {
            physical_intensity: 0.2,
            mental_intensity: 0.1,
            emotional_intensity: 0.05,
        }
    }

    /// Create activity factors for moderate activity
    pub fn moderate() -> Self {
        Self {
            physical_intensity: 0.5,
            mental_intensity: 0.3,
            emotional_intensity: 0.1,
        }
    }

    /// Create activity factors for heavy activity
    pub fn heavy() -> Self {
        Self {
            physical_intensity: 0.8,
            mental_intensity: 0.4,
            emotional_intensity: 0.2,
        }
    }

    /// Create activity factors for extreme exertion
    pub fn extreme() -> Self {
        Self {
            physical_intensity: 1.0,
            mental_intensity: 0.6,
            emotional_intensity: 0.4,
        }
    }
}

/// Results of a fatigue update
/// Used for audit logging and state tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FatigueUpdate {
    /// Change in overall energy level
    pub energy_change: f64,
    /// Current fatigue level category
    pub current_level: FatigueLevel,
    /// Whether currently sleeping
    pub is_sleeping: bool,
    /// Current circadian phase
    pub circadian_phase: f64,
}

/// Fatigue effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FatigueModulators {
    /// Current energy level (0.0 = exhausted, 1.0 = fresh)
    pub energy_level: f64,
    /// Cognitive performance modifier (0.0 = severely impaired, 1.0 = optimal)
    pub cognitive_performance: f64,
    /// Physical performance modifier (0.0 = severely impaired, 1.0 = optimal)
    pub physical_performance: f64,
    /// Reaction time modifier (1.0 = optimal, higher = slower)
    pub reaction_time_modifier: f64,
    /// Sleep pressure (0.0 = alert, 1.0 = very sleepy)
    pub sleep_pressure: f64,
    /// Ticks since last sleep
    pub time_since_sleep: u64,
}

/// Reasons why fatigue might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FatigueBioVetoReason {
    /// Energy completely depleted
    CompleteExhaustion,
    /// Severe fatigue impairs function
    SevereFatigue,
    /// Circadian rhythm demands sleep
    CircadianSleepPressure,
    /// General fatigue imbalance
    FatigueImbalance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fatigue_level_conversion() {
        assert_eq!(FatigueLevel::from_energy_percentage(0.1), FatigueLevel::Exhausted);
        assert_eq!(FatigueLevel::from_energy_percentage(0.3), FatigueLevel::Severe);
        assert_eq!(FatigueLevel::from_energy_percentage(0.5), FatigueLevel::Moderate);
        assert_eq!(FatigueLevel::from_energy_percentage(0.7), FatigueLevel::Mild);
        assert_eq!(FatigueLevel::from_energy_percentage(0.9), FatigueLevel::Rested);
        assert_eq!(FatigueLevel::from_energy_percentage(0.98), FatigueLevel::Fresh);
    }

    #[test]
    fn test_fatigue_accumulation() {
        let mut system = FatigueSystem::new();
        let initial_energy = system.energy_level;
        
        // Apply heavy activity for several ticks
        let activity = ActivityFactors::heavy();
        for _ in 0..100 {
            system.update(&activity);
        }
        
        assert!(system.energy_level < initial_energy);
    }

    #[test]
    fn test_sleep_recovery() {
        let mut system = FatigueSystem::new();
        
        // Accumulate fatigue
        let activity = ActivityFactors::heavy();
        for _ in 0..200 {
            system.update(&activity);
        }
        
        let fatigued_energy = system.energy_level;
        
        // Sleep with good quality
        system.start_sleep(SleepQuality::Good);
        for _ in 0..6000 {
            system.update(&ActivityFactors::rest());
        }
        system.wake_up();
        
        assert!(system.energy_level > fatigued_energy);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = FatigueSystem::new();
        let mut system2 = FatigueSystem::new();
        
        let activity = ActivityFactors::moderate();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update(&activity);
            system2.update(&activity);
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
