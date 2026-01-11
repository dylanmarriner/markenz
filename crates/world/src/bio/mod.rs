pub mod state;
pub mod health;
pub mod metabolism;
pub mod nutrition;
pub mod observation;

#[cfg(test)]
mod tests;

pub use state::BioState;
pub use health::{HealthEvent, HealthMonitor};
pub use metabolism::{MetabolicProcessor, Environment};
pub use nutrition::NutritionSystem;
pub use observation::bio_state_to_observation;
