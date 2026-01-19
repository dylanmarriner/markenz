/*!
# MARKENZ Biology Crate

**Purpose:** Deterministic human-equivalent biological systems for agents.

**Governance Authority:** MARKENZ_GOVERNANCE_PHASE_3_EMBODIED_BIOLOGY_V1

**Determinism Guarantees:**
- All biological processes are deterministic given same seed + inputs
- No wall-clock time influences biological state
- Fixed-point arithmetic for all biological calculations
- BTreeMap-based deterministic ordering for all collections

**Core Systems:**
- Endocrine system with explicit hormone modeling
- Metabolism with energy balance and nutrient processing
- Nutrition with macro/micronutrient tracking
- Hydration and waste management
- Fatigue and recovery cycles
- Injury and healing mechanics
- Reproductive biology with fertility cycles

**BioVeto Integration:**
All biological systems can emit veto reasons that prevent agent actions
when physiological constraints would make actions impossible or dangerous.

**Audit Requirements:**
- Every hormone change logged with source and magnitude
- All metabolic events tracked with energy balance
- Injury and healing progress auditable
- Reproductive state changes observable

This crate enforces the biological parity requirement from
HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW.md
*/

pub mod endocrine;
pub mod metabolism;
pub mod nutrition;
pub mod hydration;
pub mod fatigue;
pub mod injury;
pub mod reproductive;
pub mod state;
pub mod veto;

// Re-export primary interfaces
pub use state::{BiologicalState, BioVetoReason};
pub use endocrine::{EndocrineSystem, Hormone, HormoneType};
pub use metabolism::{MetabolicSystem, EnergyBalance};
pub use nutrition::{NutritionSystem, NutrientType, NutrientStore};
pub use hydration::{HydrationSystem, HydrationLevel};
pub use fatigue::{FatigueSystem, FatigueLevel};
pub use injury::{InjurySystem, Wound, HealingStage};
pub use reproductive::{ReproductiveSystem, FertilityStatus, ReproductiveCycle};
pub use veto::{BioVeto, VetoSeverity};
