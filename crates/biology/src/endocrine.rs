/*!
# Endocrine System

**Purpose:** Deterministic hormone modeling for human-equivalent biology.

**Why it exists:** Hormones are the primary chemical messengers that regulate
mood, energy, stress, social bonding, and physiological processes. They must
be modeled explicitly to ensure human-equivalent behavior and provide BioVeto
constraints on agent actions.

**Determinism guarantees:**
- Hormone production follows fixed mathematical models
- Decay rates are constant per tick
- Interactions between hormones are deterministic
- No random fluctuations - all changes have identifiable causes

**How it affects replay:** Same sequence of physiological events and psychological
stimuli will produce identical hormone trajectories across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Explicitly enumerated hormones with biological analogues
/// Each hormone has deterministic production and decay mechanics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HormoneType {
    /// Stress hormone - increases during threats, decreases recovery
    Cortisol,
    /// Bonding hormone - increases during social connection, decreases isolation
    Oxytocin,
    /// Reward hormone - increases during achievement, decreases with repetition
    Dopamine,
    /// Sleep hormone - increases in darkness, decreases with light exposure
    Melatonin,
    /// Male sex hormone - affects aggression, confidence, muscle mass
    Testosterone,
    /// Female sex hormone - affects nurturing, social bonding, verbal fluency
    Estrogen,
    /// Blood sugar regulation - increases with glucose, decreases with insulin
    Insulin,
    /// Metabolism regulator - affects energy expenditure, temperature
    Thyroid,
}

impl HormoneType {
    /// Baseline production rate per tick (micrograms per tick)
    /// These values are calibrated to human physiological ranges
    pub const fn baseline_production(self) -> f64 {
        match self {
            Self::Cortisol => 0.05,      // ~15 μg/day baseline
            Self::Oxytocin => 0.08,      // ~8 μg/day baseline  
            Self::Dopamine => 0.42,      // ~1050 μg/day baseline
            Self::Melatonin => 0.02,     // ~50 μg/day baseline
            Self::Testosterone => 2.5,    // ~600 μg/day baseline
            Self::Estrogen => 0.17,      // ~40 μg/day baseline
            Self::Insulin => 0.5,        // ~12 μIU/day baseline
            Self::Thyroid => 0.08,       // ~2.0 μIU/day baseline
        }
    }

    /// Decay rate per tick (fraction of current level)
    /// Represents biological half-life converted to tick-based decay
    pub const fn decay_rate(self) -> f64 {
        match self {
            Self::Cortisol => 0.001,      // ~90 minute half-life
            Self::Oxytocin => 0.003,      // ~3 minute half-life
            Self::Dopamine => 0.002,      // ~2 minute half-life
            Self::Melatonin => 0.001,      // ~30 minute half-life
            Self::Testosterone => 0.0001,   // ~10 day half-life
            Self::Estrogen => 0.0002,      // ~24 day half-life
            Self::Insulin => 0.005,        // ~5 minute half-life
            Self::Thyroid => 0.00005,     // ~7 day half-life
        }
    }

    /// Minimum viable level for basic physiological function
    /// Below this level, severe dysfunction occurs
    pub const fn minimum_level(self) -> f64 {
        match self {
            Self::Cortisol => 0.1,
            Self::Oxytocin => 0.05,
            Self::Dopamine => 0.2,
            Self::Melatonin => 0.01,
            Self::Testosterone => 0.5,
            Self::Estrogen => 0.1,
            Self::Insulin => 0.1,
            Self::Thyroid => 0.05,
        }
    }

    /// Maximum sustainable level before toxicity
    /// Above this level, pathological effects occur
    pub const fn maximum_level(self) -> f64 {
        match self {
            Self::Cortisol => 2.0,
            Self::Oxytocin => 1.5,
            Self::Dopamine => 3.0,
            Self::Melatonin => 0.5,
            Self::Testosterone => 10.0,
            Self::Estrogen => 2.0,
            Self::Insulin => 5.0,
            Self::Thyroid => 0.5,
        }
    }
}

/// Individual hormone instance with current level and production modifiers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hormone {
    /// Current concentration in appropriate units
    pub level: f64,
    /// Production rate modifier (1.0 = normal, >1.0 = increased, <1.0 = decreased)
    pub production_modifier: f64,
    /// Decay rate modifier (1.0 = normal, >1.0 = faster decay, <1.0 = slower decay)
    pub decay_modifier: f64,
}

impl Hormone {
    /// Create a new hormone at baseline level
    pub fn new(hormone_type: HormoneType) -> Self {
        Self {
            level: hormone_type.baseline_production() / hormone_type.decay_rate(),
            production_modifier: 1.0,
            decay_modifier: 1.0,
        }
    }

    /// Update hormone level for one tick
    /// Returns the change in level for audit logging
    pub fn update(&mut self, hormone_type: HormoneType) -> f64 {
        let old_level = self.level;
        
        // Apply baseline production with modifiers
        let production = hormone_type.baseline_production() * self.production_modifier;
        
        // Apply decay with modifiers
        let decay = self.level * hormone_type.decay_rate() * self.decay_modifier;
        
        // Update level
        self.level += production - decay;
        
        // Clamp to physiological bounds
        self.level = self.level
            .clamp(hormone_type.minimum_level(), hormone_type.maximum_level());
        
        let change = self.level - old_level;
        trace!("Hormone {:?} changed by {:.6} (production: {:.6}, decay: {:.6})", 
               hormone_type, change, production, decay);
        
        change
    }

    /// Apply an external stimulus that affects hormone level
    /// Used for stress responses, social bonding, achievement rewards, etc.
    pub fn apply_stimulus(&mut self, stimulus: f64) {
        self.level = (self.level + stimulus)
            .clamp(0.0, 100.0); // Emergency upper bound
        debug!("Applied stimulus {:.6} to hormone, new level: {:.6}", stimulus, self.level);
    }
}

/// Complete endocrine system managing all hormones
/// Provides deterministic hormone interactions and behavioral effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndocrineSystem {
    /// All hormones indexed by type for deterministic iteration
    hormones: BTreeMap<HormoneType, Hormone>,
    /// Tick counter for tracking temporal dynamics
    tick: u64,
}

impl EndocrineSystem {
    /// Create a new endocrine system with baseline hormone levels
    pub fn new() -> Self {
        let mut hormones = BTreeMap::new();
        
        // Initialize all hormones at baseline
        for hormone_type in [
            HormoneType::Cortisol,
            HormoneType::Oxytocin,
            HormoneType::Dopamine,
            HormoneType::Melatonin,
            HormoneType::Testosterone,
            HormoneType::Estrogen,
            HormoneType::Insulin,
            HormoneType::Thyroid,
        ] {
            hormones.insert(hormone_type, Hormone::new(hormone_type));
        }
        
        Self {
            hormones,
            tick: 0,
        }
    }

    /// Update all hormones for one tick
    /// Returns a map of hormone changes for audit logging
    pub fn update(&mut self) -> BTreeMap<HormoneType, f64> {
        let mut changes = BTreeMap::new();
        
        for (hormone_type, hormone) in &mut self.hormones {
            let change = hormone.update(*hormone_type);
            changes.insert(*hormone_type, change);
        }
        
        self.tick += 1;
        
        debug!("Endocrine system tick {}: {:?}", self.tick, changes);
        changes
    }

    /// Get current level of a specific hormone
    pub fn get_level(&self, hormone_type: HormoneType) -> f64 {
        self.hormones
            .get(&hormone_type)
            .map(|h| h.level)
            .unwrap_or(0.0)
    }

    /// Apply stress response (increases cortisol, decreases oxytocin)
    /// Used for threats, conflicts, dangerous situations
    pub fn apply_stress_response(&mut self, intensity: f64) {
        if let Some(cortisol) = self.hormones.get_mut(&HormoneType::Cortisol) {
            cortisol.apply_stimulus(intensity * 0.1);
        }
        if let Some(oxytocin) = self.hormones.get_mut(&HormoneType::Oxytocin) {
            oxytocin.apply_stimulus(-intensity * 0.05);
        }
        debug!("Applied stress response with intensity {:.6}", intensity);
    }

    /// Apply reward response (increases dopamine)
    /// Used for achievements, successful actions, positive feedback
    pub fn apply_reward_response(&mut self, magnitude: f64) {
        if let Some(dopamine) = self.hormones.get_mut(&HormoneType::Dopamine) {
            dopamine.apply_stimulus(magnitude * 0.2);
        }
        debug!("Applied reward response with magnitude {:.6}", magnitude);
    }

    /// Apply bonding response (increases oxytocin)
    /// Used for social connection, trust-building, physical contact
    pub fn apply_bonding_response(&mut self, strength: f64) {
        if let Some(oxytocin) = self.hormones.get_mut(&HormoneType::Oxytocin) {
            oxytocin.apply_stimulus(strength * 0.15);
        }
        debug!("Applied bonding response with strength {:.6}", strength);
    }

    /// Apply circadian rhythm effects
    /// Should be called each tick with current time-of-day information
    pub fn update_circadian(&mut self, light_level: f64) {
        // Melatonin inversely proportional to light
        if let Some(melatonin) = self.hormones.get_mut(&HormoneType::Melatonin) {
            let target_melatonin = (1.0 - light_level) * 0.5;
            let current = melatonin.level;
            melatonin.apply_stimulus((target_melatonin - current) * 0.01);
        }
        
        // Cortisol follows opposite pattern (higher in morning)
        if let Some(cortisol) = self.hormones.get_mut(&HormoneType::Cortisol) {
            let target_cortisol = light_level * 0.3 + 0.1;
            let current = cortisol.level;
            cortisol.apply_stimulus((target_cortisol - current) * 0.01);
        }
    }

    /// Check if hormone levels support action execution
    /// Returns BioVeto reasons if hormones would prevent action
    pub fn check_action_viability(&self) -> Vec<EndocrineBioVetoReason> {
        let mut vetoes = Vec::new();
        
        // High cortisol prevents complex cognitive tasks
        if self.get_level(HormoneType::Cortisol) > 1.5 {
            vetoes.push(EndocrineBioVetoReason::StressOverload);
        }
        
        // Low dopamine prevents motivated action
        if self.get_level(HormoneType::Dopamine) < 0.3 {
            vetoes.push(EndocrineBioVetoReason::Anhedonia);
        }
        
        // Low melatonin prevents alert action
        if self.get_level(HormoneType::Melatonin) > 0.4 {
            vetoes.push(EndocrineBioVetoReason::ExcessiveFatigue);
        }
        
        vetoes
    }

    /// Get current hormone state for cognitive processing
    /// Hormones affect perception weighting, decision making, and emotional responses
    pub fn get_cognitive_modulators(&self) -> CognitiveModulators {
        CognitiveModulators {
            stress_level: self.get_level(HormoneType::Cortisol),
            reward_sensitivity: self.get_level(HormoneType::Dopamine),
            social_bonding: self.get_level(HormoneType::Oxytocin),
            alertness: 1.0 - self.get_level(HormoneType::Melatonin),
            confidence: self.get_level(HormoneType::Testosterone),
            nurturing: self.get_level(HormoneType::Estrogen),
        }
    }
}

/// Hormone effects on cognitive processing
/// Used by cognition system to weight decisions and perceptions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveModulators {
    /// Current stress level (0.0 = calm, 1.0 = extreme stress)
    pub stress_level: f64,
    /// Sensitivity to rewards and achievements (0.0 = anhedonia, 1.0 = high sensitivity)
    pub reward_sensitivity: f64,
    /// Drive for social connection and bonding (0.0 = isolated, 1.0 = highly social)
    pub social_bonding: f64,
    /// Alertness and wakefulness (0.0 = sleepy, 1.0 = fully alert)
    pub alertness: f64,
    /// Confidence and assertiveness (0.0 = timid, 1.0 = highly confident)
    pub confidence: f64,
    /// Nurturing and caregiving drive (0.0 = detached, 1.0 = highly nurturing)
    pub nurturing: f64,
}

/// Reasons why endocrine system might veto an action
/// These are deterministic and auditable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndocrineBioVetoReason {
    /// Cortisol too high for complex cognitive tasks
    StressOverload,
    /// Dopamine too low for motivated action
    Anhedonia,
    /// Melatonin too high for alert action
    ExcessiveFatigue,
    /// Testosterone too low for assertive action
    LowConfidence,
    /// General hormonal imbalance
    HormonalImbalance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hormone_baseline_levels() {
        let system = EndocrineSystem::new();
        
        // Verify all hormones are at reasonable baseline levels
        assert!(system.get_level(HormoneType::Cortisol) > 0.0);
        assert!(system.get_level(HormoneType::Dopamine) > 0.0);
        assert!(system.get_level(HormoneType::Oxytocin) > 0.0);
    }

    #[test]
    fn test_stress_response() {
        let mut system = EndocrineSystem::new();
        let initial_cortisol = system.get_level(HormoneType::Cortisol);
        let initial_oxytocin = system.get_level(HormoneType::Oxytocin);
        
        system.apply_stress_response(1.0);
        
        assert!(system.get_level(HormoneType::Cortisol) > initial_cortisol);
        assert!(system.get_level(HormoneType::Oxytocin) < initial_oxytocin);
    }

    #[test]
    fn test_deterministic_updates() {
        let mut system1 = EndocrineSystem::new();
        let mut system2 = EndocrineSystem::new();
        
        // Run identical updates
        for _ in 0..100 {
            system1.update();
            system2.update();
        }
        
        // Systems should be identical
        assert_eq!(system1, system2);
    }
}
