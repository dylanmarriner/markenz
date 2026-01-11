use crate::types::Agent;

pub struct NutritionSystem;

impl NutritionSystem {
    pub fn eat(
        agent: &mut Agent,
        food_name: &str,
        quantity: u32,
    ) -> Result<(), String> {
        // Remove food from inventory
        let mut removed = 0u32;
        agent.inventory.retain(|_, asset| {
            if asset.name == food_name && removed < quantity {
                removed += 1;
                false  // Remove this item
            } else {
                true
            }
        });
        
        if removed < quantity {
            return Err("Not enough food".to_string());
        }
        
        // Restore energy
        let energy_restored = quantity as f64 * 20.0;  // 20 energy per food item
        agent.bio_state.energy = (agent.bio_state.energy + energy_restored).min(100.0);
        agent.bio_state.hunger = (agent.bio_state.hunger - quantity as f64 * 10.0).max(0.0);
        
        Ok(())
    }
}
