pub mod state;
pub mod complete_biology;
pub mod genetics;
pub mod cognition_hooks;
pub mod health;
pub mod metabolism;
pub mod nutrition;
pub mod observation;

#[cfg(test)]
mod tests;

pub use state::BioState;
pub use complete_biology::{EndocrineSystem, BiologicalState, BioVetoReason, BioFixed};
pub use genetics::{Genome, ReproductionSystem, OffspringResult};
pub use cognition_hooks::{Perception, VolitionSystem, CognitionEventEmitter};
pub use health::{HealthEvent, HealthMonitor};
pub use metabolism::{MetabolicProcessor, Environment};
pub use nutrition::NutritionSystem;
pub use observation::bio_state_to_observation;
