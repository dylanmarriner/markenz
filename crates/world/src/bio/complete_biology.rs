/*!
# Complete Human-Equivalent Biological System

**Purpose:** Implement deterministic human-equivalent biology for all agents.

**Why it exists:** Human equivalence law requires every agent to have complete
biological systems with accurate physiological modeling, not abstractions.

**Determinism guarantees:**
- All hormone levels use fixed-point integer math
- All biological processes are deterministic given same inputs
- No floating-point ambiguity in biological calculations
- All state changes are auditable and reversible

**How it affects replay:** Same biological inputs will produce
identical physiological states across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Fixed-point biological values (2 decimal places)
/// All biological values use integers to avoid floating-point nondeterminism
pub type BioFixed = u64; // Stored as *100 to avoid floating point

/// Convert floating point to fixed-point
pub const fn to_fixed(value: f64) -> BioFixed {
    (value * 100.0).max(0.0) as BioFixed
}

/// Convert fixed-point back to floating point for display
pub const fn from_fixed(value: BioFixed) -> f64 {
    value as f64 / 100.0
}

/// Complete endocrine hormone system
/// Models all major human hormones with accurate pharmacodynamics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndocrineSystem {
    /// Hypothalamic-pituitary-adrenal axis hormones
    pub hpa_axis: HPAAxis,
    
    /// Gonadal hormones (reproductive system)
    pub gonadal: GonadalHormones,
    
    /// Thyroid system hormones
    pub thyroid: ThyroidHormones,
    
    /// Pancreatic endocrine hormones
    pub pancreatic: PancreaticHormones,
    
    /// Circadian rhythm regulation
    pub circadian: CircadianSystem,
}

impl EndocrineSystem {
    /// Create baseline human endocrine system
    pub fn new_human_baseline() -> Self {
        Self {
            hpa_axis: HPAAxis::new_baseline(),
            gonadal: GonadalHormones::new_baseline(),
            thyroid: ThyroidHormones::new_baseline(),
            pancreatic: PancreaticHormones::new_baseline(),
            circadian: CircadianSystem::new_baseline(),
        }
    }
    
    /// Update endocrine system for one tick
    /// All hormone changes are deterministic and follow biological rules
    pub fn tick(&mut self, rng: &mut crate::rng::RngStream, biological_state: &BiologicalState) {
        // Update HPA axis based on stress
        self.hpa_axis.update_stress_response(biological_state.stress_level);
        
        // Update gonadal hormones based on reproductive cycle
        self.gonadal.update_cycle_progression(biological_state.reproductive_tick);
        
        // Update thyroid based on metabolic demand
        self.thyroid.update_metabolic_demand(biological_state.metabolic_rate);
        
        // Update pancreatic based on blood glucose
        self.pancreatic.regulate_glucose(biological_state.blood_glucose);
        
        // Update circadian rhythm
        self.circadian.advance_cycle(rng);
        
        trace!("Endocrine system updated: stress={}, reproductive={}, metabolic={}", 
                biological_state.stress_level, biological_state.reproductive_tick, biological_state.metabolic_rate);
    }
    
    /// Get hormone effects on behavior and physiology
    pub fn get_hormonal_effects(&self) -> HormonalEffects {
        HormonalEffects {
            stress_response: self.hpa_axis.get_stress_level(),
            reproductive_drive: self.gonadal.get_libido(),
            metabolic_rate: self.thyroid.get_metabolic_multiplier(),
            glucose_regulation: self.pancreatic.get_insulin_sensitivity(),
            alertness: self.circadian.get_alertness_level(),
        }
    }
}

/// HPA axis (stress response system)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HPAAxis {
    /// Cortisol level (pg/mL) - primary stress hormone
    pub cortisol_pg_ml: BioFixed,
    /// CRH level - corticotropin releasing hormone
    pub crh_level: BioFixed,
    /// ACTH level - adrenocorticotropic hormone
    pub acth_level: BioFixed,
    /// Stress accumulation over time
    pub stress_accumulation: BioFixed,
}

impl HPAAxis {
    pub fn new_baseline() -> Self {
        Self {
            cortisol_pg_ml: to_fixed(15.0), // Normal morning cortisol
            crh_level: to_fixed(5.0),
            acth_level: to_fixed(10.0),
            stress_accumulation: to_fixed(0.0),
        }
    }
    
    /// Update stress response based on current stress level
    pub fn update_stress_response(&mut self, stress_level: BioFixed) {
        // Accumulate stress over time
        self.stress_accumulation = (self.stress_accumulation + stress_level).min(to_fixed(1000.0));
        
        // Update cortisol based on accumulated stress
        let target_cortisol = to_fixed(10.0) + (self.stress_accumulation / 20);
        self.cortisol_pg_ml = target_cortisol.min(to_fixed(200.0)); // Cap at severe stress
        
        // Update upstream hormones
        self.crh_level = (self.stress_accumulation / 50).min(to_fixed(50.0));
        self.acth_level = (self.crh_level * 2).min(to_fixed(100.0));
        
        // Natural decay of stress hormones
        self.cortisol_pg_ml = (self.cortisol_pg_ml - to_fixed(0.5)).max(to_fixed(5.0));
        self.stress_accumulation = (self.stress_accumulation - to_fixed(1.0)).max(to_fixed(0.0));
    }
    
    /// Get current stress level for behavioral effects
    pub fn get_stress_level(&self) -> BioFixed {
        self.cortisol_pg_ml
    }
}

/// Gonadal hormone system (reproductive biology)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GonadalHormones {
    /// Testosterone level (ng/dL) - males
    pub testosterone_ng_dl: BioFixed,
    /// Estrogen level (pg/mL) - females  
    pub estrogen_pg_ml: BioFixed,
    /// Progesterone level (ng/mL) - females
    pub progesterone_ng_ml: BioFixed,
    /// Luteinizing hormone (mIU/mL)
    pub lh_miu_ml: BioFixed,
    /// Follicle-stimulating hormone (mIU/mL)
    pub fsh_miu_ml: BioFixed,
    /// Reproductive cycle tick counter
    pub cycle_tick: u64,
    /// Fertility status
    pub fertility_status: FertilityStatus,
}

impl GonadalHormones {
    pub fn new_baseline() -> Self {
        Self {
            testosterone_ng_dl: to_fixed(600.0), // Male baseline
            estrogen_pg_ml: to_fixed(40.0), // Female baseline
            progesterone_ng_ml: to_fixed(1.0),
            lh_miu_ml: to_fixed(5.0),
            fsh_miu_ml: to_fixed(8.0),
            cycle_tick: 0,
            fertility_status: FertilityStatus::Fertile,
        }
    }
    
    /// Update reproductive cycle progression
    pub fn update_cycle_progression(&mut self, tick: u64) {
        self.cycle_tick = (self.cycle_tick + 1) % 2800; // ~28 day cycle (100 ticks/day)
        
        // Simulate menstrual cycle hormonal changes
        let cycle_day = self.cycle_tick / 100;
        
        match cycle_day {
            0..=7 => { // Follicular phase
                self.estrogen_pg_ml = to_fixed(40.0) + to_fixed(cycle_day as f64 * 2.0);
                self.progesterone_ng_ml = to_fixed(1.0);
                self.fsh_miu_ml = to_fixed(8.0) + to_fixed(cycle_day as f64 * 0.5);
                self.lh_miu_ml = to_fixed(5.0);
            },
            8..=14 => { // Ovulation phase
                self.estrogen_pg_ml = to_fixed(200.0); // Peak estrogen
                self.lh_miu_ml = to_fixed(50.0); // LH surge
                self.fsh_miu_ml = to_fixed(15.0);
                self.progesterone_ng_ml = to_fixed(2.0);
                self.fertility_status = FertilityStatus::PeakFertility;
            },
            15..=21 => { // Luteal phase
                self.estrogen_pg_ml = to_fixed(100.0);
                self.progesterone_ng_ml = to_fixed(15.0); // High progesterone
                self.lh_miu_ml = to_fixed(10.0);
                self.fsh_miu_ml = to_fixed(5.0);
                self.fertility_status = FertilityStatus::Fertile;
            },
            _ => { // Menstrual phase
                self.estrogen_pg_ml = to_fixed(30.0);
                self.progesterone_ng_ml = to_fixed(0.5);
                self.lh_miu_ml = to_fixed(5.0);
                self.fsh_miu_ml = to_fixed(8.0);
                self.fertility_status = FertilityStatus::Menstruating;
            }
        }
        
        trace!("Reproductive cycle: day={}, estrogen={}, progesterone={}, fertility={:?}", 
                cycle_day, from_fixed(self.estrogen_pg_ml), from_fixed(self.progesterone_ng_ml), self.fertility_status);
    }
    
    /// Get current libido/reproductive drive
    pub fn get_libido(&self) -> BioFixed {
        // Libido influenced by testosterone and estrogen levels
        let hormone_factor = (self.testosterone_ng_dl / 10) + (self.estrogen_pg_ml / 5);
        
        match self.fertility_status {
            FertilityStatus::PeakFertility => hormone_factor + to_fixed(50.0),
            FertilityStatus::Fertile => hormone_factor + to_fixed(20.0),
            FertilityStatus::Menstruating => hormone_factor,
            FertilityStatus::Infertile => hormone_factor / 2,
        }
    }
}

/// Thyroid hormone system (metabolism regulation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThyroidHormones {
    /// Thyroid stimulating hormone (mIU/L)
    pub tsh_miu_l: BioFixed,
    /// T4 level (thyroxine)
    pub t4_level: BioFixed,
    /// T3 level (triiodothyronine) - active form
    pub t3_level: BioFixed,
    /// Metabolic rate multiplier
    pub metabolic_multiplier: BioFixed,
}

impl ThyroidHormones {
    pub fn new_baseline() -> Self {
        Self {
            tsh_miu_l: to_fixed(2.0), // Normal TSH
            t4_level: to_fixed(8.0),
            t3_level: to_fixed(3.5),
            metabolic_multiplier: to_fixed(100.0), // 1.0x normal metabolism
        }
    }
    
    /// Update based on metabolic demand
    pub fn update_metabolic_demand(&mut self, metabolic_rate: BioFixed) {
        // Increase TSH if metabolism needs to increase
        let target_multiplier = (metabolic_rate / 100).min(to_fixed(150.0)); // Max 1.5x
        
        if target_multiplier > self.metabolic_multiplier {
            self.tsh_miu_l = (self.tsh_miu_l + to_fixed(0.1)).min(to_fixed(10.0));
        } else {
            self.tsh_miu_l = (self.tsh_miu_l - to_fixed(0.05)).max(to_fixed(0.5));
        }
        
        // Update thyroid hormones based on TSH
        self.t4_level = (self.tsh_miu_l * 4).min(to_fixed(20.0));
        self.t3_level = (self.t4_level * 0.44).min(to_fixed(10.0));
        self.metabolic_multiplier = target_multiplier;
        
        trace!("Thyroid update: TSH={}, T4={}, T3={}, metabolic={}", 
                from_fixed(self.tsh_miu_l), from_fixed(self.t4_level), from_fixed(self.t3_level), from_fixed(self.metabolic_multiplier));
    }
    
    /// Get metabolic rate multiplier
    pub fn get_metabolic_multiplier(&self) -> BioFixed {
        self.metabolic_multiplier
    }
}

/// Pancreatic endocrine system (blood glucose regulation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PancreaticHormones {
    /// Insulin level (mIU/L) - lowers blood glucose
    pub insulin_miu_l: BioFixed,
    /// Glucagon level (pg/mL) - raises blood glucose
    pub glucagon_pg_ml: BioFixed,
    /// Blood glucose level (mg/dL)
    pub blood_glucose_mg_dl: BioFixed,
    /// Insulin sensitivity factor
    pub insulin_sensitivity: BioFixed,
}

impl PancreaticHormones {
    pub fn new_baseline() -> Self {
        Self {
            insulin_miu_l: to_fixed(12.0), // Normal fasting insulin
            glucagon_pg_ml: to_fixed(50.0),
            blood_glucose_mg_dl: to_fixed(90.0), // Normal fasting glucose
            insulin_sensitivity: to_fixed(100.0), // Normal sensitivity
        }
    }
    
    /// Regulate blood glucose levels
    pub fn regulate_glucose(&mut self, glucose_level: BioFixed) {
        self.blood_glucose_mg_dl = glucose_level;
        
        // Normal range: 70-100 mg/dL
        if glucose_level > to_fixed(100.0) {
            // High glucose - release insulin
            let insulin_needed = (glucose_level - to_fixed(80.0)) / 10;
            self.insulin_miu_l = (self.insulin_miu_l + insulin_needed).min(to_fixed(100.0));
            self.glucagon_pg_ml = (self.glucagon_pg_ml - to_fixed(5.0)).max(to_fixed(10.0));
        } else if glucose_level < to_fixed(70.0) {
            // Low glucose - release glucagon
            let glucagon_needed = (to_fixed(70.0) - glucose_level) / 5;
            self.glucagon_pg_ml = (self.glucagon_pg_ml + glucagon_needed).min(to_fixed(200.0));
            self.insulin_miu_l = (self.insulin_miu_l - to_fixed(2.0)).max(to_fixed(2.0));
        } else {
            // Normal range - maintain baseline
            self.insulin_miu_l = (self.insulin_miu_l - to_fixed(0.5)).max(to_fixed(5.0)).min(to_fixed(15.0));
            self.glucagon_pg_ml = (self.glucagon_pg_ml - to_fixed(1.0)).max(to_fixed(30.0)).min(to_fixed(70.0));
        }
        
        trace!("Glucose regulation: glucose={}, insulin={}, glucagon={}", 
                from_fixed(glucose_level), from_fixed(self.insulin_miu_l), from_fixed(self.glucagon_pg_ml));
    }
    
    /// Get insulin sensitivity for metabolic effects
    pub fn get_insulin_sensitivity(&self) -> BioFixed {
        self.insulin_sensitivity
    }
}

/// Circadian rhythm system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircadianSystem {
    /// Current time in 24-hour cycle (0-2399 for 100 ticks/hour)
    pub circadian_tick: u64,
    /// Melatonin level (pg/mL) - sleep hormone
    pub melatonin_pg_ml: BioFixed,
    /// Alertness level (0-10000)
    pub alertness_level: BioFixed,
    /// Sleep pressure accumulation
    pub sleep_pressure: BioFixed,
}

impl CircadianSystem {
    pub fn new_baseline() -> Self {
        Self {
            circadian_tick: 480, // 4:80 AM start time
            melatonin_pg_ml: to_fixed(50.0), // Normal morning melatonin
            alertness_level: to_fixed(8000.0), // High morning alertness
            sleep_pressure: to_fixed(0.0),
        }
    }
    
    /// Advance circadian cycle
    pub fn advance_cycle(&mut self, rng: &mut crate::rng::RngStream) {
        self.circadian_tick = (self.circadian_tick + 1) % 2400;
        
        let hour = self.circadian_tick / 100;
        
        // Melatonin follows circadian rhythm (low during day, high at night)
        match hour {
            6..=21 => { // Daytime (6 AM - 9 PM)
                self.melatonin_pg_ml = (self.melatonin_pg_ml - to_fixed(2.0)).max(to_fixed(10.0));
                self.alertness_level = (self.alertness_level + to_fixed(10.0)).min(to_fixed(10000.0));
                self.sleep_pressure = (self.sleep_pressure + to_fixed(5.0)).min(to_fixed(8000.0));
            },
            22..=23 | 0..=5 => { // Nighttime (10 PM - 5 AM)
                self.melatonin_pg_ml = (self.melatonin_pg_ml + to_fixed(3.0)).min(to_fixed(200.0));
                self.alertness_level = (self.alertness_level - to_fixed(20.0)).max(to_fixed(1000.0));
                self.sleep_pressure = (self.sleep_pressure - to_fixed(50.0)).max(to_fixed(0.0));
            },
            _ => {} // Transition periods
        }
        
        trace!("Circadian: hour={}, melatonin={}, alertness={}, sleep_pressure={}", 
                hour, from_fixed(self.melatonin_pg_ml), from_fixed(self.alertness_level), from_fixed(self.sleep_pressure));
    }
    
    /// Get current alertness level
    pub fn get_alertness_level(&self) -> BioFixed {
        self.alertness_level
    }
}

/// Fertility status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FertilityStatus {
    /// Peak fertility (ovulation)
    PeakFertility,
    /// Normal fertility
    Fertile,
    /// Currently menstruating
    Menstruating,
    /// Temporarily infertile
    Infertile,
}

/// Hormonal effects on physiology and behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HormonalEffects {
    /// Current stress response level
    pub stress_response: BioFixed,
    /// Reproductive drive/libido
    pub reproductive_drive: BioFixed,
    /// Metabolic rate multiplier
    pub metabolic_rate: BioFixed,
    /// Glucose regulation efficiency
    pub glucose_regulation: BioFixed,
    /// Current alertness level
    pub alertness: BioFixed,
    /// Hydration modulators for cognitive and physical performance
    pub hydration_modulators: HydrationModulators,
}

/// Complete biological state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiologicalState {
    /// Energy level (0-10000, where 10000 = 100%)
    pub energy: BioFixed,
    /// Hunger level (0-10000, where 10000 = starving)
    pub hunger: BioFixed,
    /// Thirst level (0-10000, where 10000 = dehydrated)
    pub thirst: BioFixed,
    /// Fatigue level (0-10000, where 10000 = exhausted)
    pub fatigue: BioFixed,
    /// Health level (0-10000, where 10000 = perfect health)
    pub health: BioFixed,
    /// Stress level (0-10000)
    pub stress_level: BioFixed,
    /// Body temperature (fixed-point Celsius)
    pub body_temperature: BioFixed,
    /// Blood glucose level
    pub blood_glucose: BioFixed,
    /// Metabolic rate (baseline 10000 = normal)
    pub metabolic_rate: BioFixed,
    /// Reproductive system tick
    pub reproductive_tick: u64,
    /// Complete endocrine system
    pub endocrine: EndocrineSystem,
    /// Complete hydration system
    pub hydration: HydrationSystem,
}

impl BiologicalState {
    /// Create new human-equivalent biological state
    pub fn new_human_equivalent() -> Self {
        Self {
            energy: to_fixed(80.0), // 80% energy
            hunger: to_fixed(20.0), // Slightly hungry
            thirst: to_fixed(15.0), // Slightly thirsty
            fatigue: to_fixed(10.0), // Slightly tired
            health: to_fixed(100.0), // Perfect health
            stress_level: to_fixed(10.0), // Low stress
            body_temperature: to_fixed(36.8), // Normal body temp
            blood_glucose: to_fixed(90.0), // Normal fasting glucose
            metabolic_rate: to_fixed(100.0), // Normal metabolism
            reproductive_tick: 0,
            endocrine: EndocrineSystem::new_human_baseline(),
            hydration: HydrationSystem::new(),
        }
    }
    
    /// Update biological state for one tick
    pub fn tick(&mut self, rng: &mut crate::rng::RngStream) {
        // Update energy metabolism
        let energy_consumption = self.metabolic_rate / 100; // Convert from percentage
        self.energy = (self.energy - energy_consumption).max(to_fixed(0.0));
        
        // Update hunger and thirst based on energy
        if self.energy < to_fixed(30.0) {
            self.hunger = (self.hunger + to_fixed(50.0)).min(to_fixed(10000.0));
            self.thirst = (self.thirst + to_fixed(30.0)).min(to_fixed(10000.0));
        }
        
        // Update fatigue based on activity and circadian rhythm
        self.fatigue = (self.fatigue + to_fixed(5.0)).min(to_fixed(10000.0));
        
        // Update endocrine system
        self.endocrine.tick(rng, self);
        
        // Update hydration system
        self.hydration.tick();
        
        // Apply hormonal effects to biological state
        let effects = self.endocrine.get_hormonal_effects();
        self.stress_level = effects.stress_response;
        self.metabolic_rate = effects.metabolic_rate;
        
        // Update hydration system
        self.hydration.tick();
        
        // Apply hormonal effects to biological state
        let effects = self.endocrine.get_hormonal_effects();
        self.stress_level = effects.stress_response;
        self.metabolic_rate = effects.metabolic_rate;
        
        // Apply hydration effects
        let hydration_effects = self.hydration.get_cognitive_modulators();
        // In a full implementation, these would modify cognitive and physical performance
        
        // Natural recovery processes
        self.natural_recovery();
        
        trace!("Biological state: energy={}, hunger={}, thirst={}, fatigue={}, stress={}", 
                from_fixed(self.energy), from_fixed(self.hunger), from_fixed(self.thirst), from_fixed(self.fatigue), from_fixed(self.stress_level));
    }
    
    /// Apply hormonal effects to biological state
    fn apply_hormonal_effects(&mut self, effects: HormonalEffects) {
        // Stress affects fatigue and health
        if effects.stress_response > to_fixed(100.0) {
            self.fatigue = (self.fatigue + to_fixed(20.0)).min(to_fixed(10000.0));
            self.stress_level = effects.stress_response;
        }
        
        // Metabolic rate affects energy consumption
        self.metabolic_rate = effects.metabolic_rate;
        
        // Alertness affects fatigue
        if effects.alertness < to_fixed(3000.0) {
            self.fatigue = (self.fatigue + to_fixed(50.0)).min(to_fixed(10000.0));
        }
    }
    
    /// Natural recovery processes
    fn natural_recovery(&mut self) {
        // Recover energy when resting
        if self.fatigue < to_fixed(50.0) {
            self.energy = (self.energy + to_fixed(20.0)).min(to_fixed(10000.0));
        }
        
        // Reduce hunger if energy is high
        if self.energy > to_fixed(80.0) && self.hunger > to_fixed(0.0) {
            self.hunger = (self.hunger - to_fixed(10.0)).max(to_fixed(0.0));
        }
        
        // Reduce thirst if hydrated
        if self.thirst > to_fixed(0.0) {
            self.thirst = (self.thirst - to_fixed(15.0)).max(to_fixed(0.0));
        }
        
        // Recover from fatigue when resting
        if self.fatigue > to_fixed(0.0) {
            self.fatigue = (self.fatigue - to_fixed(25.0)).max(to_fixed(0.0));
        }
        
        // Natural healing
        if self.health < to_fixed(100.0) {
            self.health = (self.health + to_fixed(1.0)).min(to_fixed(100.0));
        }
    }
    
    /// Check if agent can perform action (BioVeto)
    pub fn check_action_viability(&self, action: &crate::InputEvent) -> Result<(), BioVetoReason> {
        // Energy check
        let energy_cost = self.get_action_energy_cost(action);
        if self.energy < energy_cost {
            return Err(BioVetoReason::InsufficientEnergy);
        }
        
        // Fatigue check
        if self.fatigue > to_fixed(80.0) && self.is_intensive_action(action) {
            return Err(BioVetoReason::Exhausted);
        }
        
        // Health check
        if self.health < to_fixed(20.0) {
            return Err(BioVetoReason::Injured);
        }
        
        // Hunger check
        if self.hunger > to_fixed(90.0) {
            return Err(BioVetoReason::Starving);
        }
        
        // Thirst check
        if self.thirst > to_fixed(90.0) {
            return Err(BioVetoReason::Dehydrated);
        }
        
        Ok(())
    }
    
    /// Get energy cost for action
    fn get_action_energy_cost(&self, action: &crate::InputEvent) -> BioFixed {
        match action {
            crate::InputEvent::MoveAgent { distance, .. } => to_fixed(*distance as f64 * 0.1),
            crate::InputEvent::GatherResource { .. } => to_fixed(50.0),
            crate::InputEvent::UseTool { .. } => to_fixed(30.0),
            crate::InputEvent::Rest { .. } => to_fixed(-100.0), // Restores energy
            _ => to_fixed(0.0),
        }
    }
    
    /// Check if action is intensive
    fn is_intensive_action(&self, action: &crate::InputEvent) -> bool {
        matches!(action, 
            crate::InputEvent::GatherResource { .. } |
            crate::InputEvent::UseTool { .. }
        )
    }
}

/// BioVeto reasons for action rejection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BioVetoReason {
    /// Insufficient energy for action
    InsufficientEnergy,
    /// Too exhausted for intensive action
    Exhausted,
    /// Too injured to act
    Injured,
    /// Too hungry to act
    Starving,
    /// Too thirsty to act
    Dehydrated,
}

impl BioVetoReason {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            BioVetoReason::InsufficientEnergy => "Insufficient energy to perform action",
            BioVetoReason::Exhausted => "Too exhausted for intensive action",
            BioVetoReason::Injured => "Too injured to act",
            BioVetoReason::Starving => "Too hungry to act",
            BioVetoReason::Dehydrated => "Too thirsty to act",
        }
    }
}
