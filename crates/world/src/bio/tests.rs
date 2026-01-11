#[cfg(test)]
mod tests {
    use crate::types::Agent;
    use markenz_events::InputEventPayload;
    use crate::bio::BioState;

    #[test]
    fn test_bio_veto_energy() {
        let mut agent = Agent::new(1, "test".to_string());
        agent.bio_state.energy = 1.0;  // Nearly depleted
        
        let action = InputEventPayload::Mine;  // Costs 5.0 energy
        
        assert!(agent.bio_state.can_perform_action(&action).is_err());
    }

    #[test]
    fn test_bio_veto_exhaustion() {
        let mut agent = Agent::new(1, "test".to_string());
        agent.bio_state.exhaustion = 95.0;  // Nearly collapsed
        
        let intensive_action = InputEventPayload::Mine;
        assert!(agent.bio_state.can_perform_action(&intensive_action).is_err());
        
        let passive_action = InputEventPayload::Chat { text: "hello".to_string() };
        assert!(agent.bio_state.can_perform_action(&passive_action).is_ok());
    }

    #[test]
    fn test_baseline_metabolism() {
        let mut bio_state = BioState::new();
        let initial_energy = bio_state.energy;
        
        bio_state.tick_metabolism();
        
        assert!(bio_state.energy < initial_energy);
        assert_eq!(bio_state.energy, initial_energy - 0.5);
    }

    #[test]
    fn test_action_energy_cost() {
        let mut bio_state = BioState::new();
        let initial = bio_state.energy;
        
        bio_state.consume_energy(3.0);  // Gather cost
        
        assert_eq!(bio_state.energy, initial - 3.0);
        assert!(bio_state.exhaustion > 0.0);
    }

    #[test]
    fn test_action_costs() {
        let bio_state = BioState::new();
        
        assert_eq!(bio_state.action_cost(&InputEventPayload::Move { x: 1.0, y: 2.0, z: 3.0 }), 2.0);
        assert_eq!(bio_state.action_cost(&InputEventPayload::Gather { resource_type: "wood".to_string() }), 3.0);
        assert_eq!(bio_state.action_cost(&InputEventPayload::Mine), 5.0);
        assert_eq!(bio_state.action_cost(&InputEventPayload::Build { building_type: "house".to_string() }), 10.0);
        assert_eq!(bio_state.action_cost(&InputEventPayload::Craft { recipe_id: 1 }), 2.0);
        assert_eq!(bio_state.action_cost(&InputEventPayload::Chat { text: "hello".to_string() }), 0.0);
    }

    #[test]
    fn test_recovery() {
        let mut bio_state = BioState::new();
        bio_state.exhaustion = 50.0;
        
        bio_state.apply_recovery();
        
        assert!(bio_state.exhaustion < 50.0);
        assert_eq!(bio_state.exhaustion, 50.0 - 0.1);
    }

    #[test]
    fn test_hunger_mechanic() {
        let mut bio_state = BioState::new();
        bio_state.energy = 20.0;  // Below 30 threshold
        bio_state.hunger = 10.0;
        
        bio_state.tick_metabolism();
        
        assert!(bio_state.hunger > 10.0);  // Hunger should increase
    }
}
