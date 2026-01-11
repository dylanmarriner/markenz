#[cfg(test)]
mod tests {
    use crate::types::Universe; // Fixed: import Universe directly from types
    use crate::types::*;
    use crate::cognition::{perception::Perception, intent::IntentPlanner, planning::Planner, memory::AgentMemory};
    use crate::cognition::{intent::Intent, thoughts}; // Fixed: import intent and thoughts modules
    use crate::types::Agent; // Use Agent from types module

    #[test]
    fn test_perception_001() {
        let universe = Universe::new(1337);
        let agent = Agent::new(1, "TestAgent".to_string());
        
        let perception = Perception::perceive(&agent, &universe).unwrap();
        
        assert_eq!(perception.nearby_agents.len(), 1); // Should see the other agent
        assert_eq!(perception.time_of_day, 0);
        assert!(matches!(perception.terrain_biome, crate::terrain::Biome::Grassland));
    }

    #[test]
    fn test_intent_001() {
        let mut universe = Universe::new(1337);
        let mut agent = Agent::new(1, "TestAgent".to_string());
        agent.bio_state.hunger = 80.0; // High hunger
        
        let perception = Perception::perceive(&agent, &universe).unwrap();
        let memory = AgentMemory::new();
        let mut rng = universe.rng_stream(rng::RngSubsystem::Cognition);
        
        let intent = IntentPlanner::form_intent(&agent, &perception, &memory, &mut rng);
        
        assert_eq!(intent, Intent::Forage); // Fixed: use imported Intent directly
    }

    #[test]
    fn test_planning_001() {
        let mut universe = Universe::new(1337);
        let agent = Agent::new(1, "TestAgent".to_string());
        
        let perception = Perception::perceive(&agent, &universe).unwrap();
        let intent = Intent::Explore; // Fixed: use imported Intent directly
        
        let plan = Planner::plan(&agent, intent.clone(), &perception, &mut universe);
        
        assert_eq!(plan.goal, intent);
        assert!(!plan.steps.is_empty());
        assert_eq!(plan.created_tick, universe.tick);
    }

    #[test]
    fn test_memory_001() {
        let mut memory = AgentMemory::new();
        
        memory.record_event(1, "test event".to_string(), (0.0, 0.0), true);
        
        assert_eq!(memory.experience_log.len(), 1);
        assert_eq!(memory.experience_log[0].tick, 1);
        assert_eq!(memory.experience_log[0].event, "test event");
        assert_eq!(memory.experience_log[0].outcome, true);
    }

    #[test]
    fn test_monologue_001() {
        let universe = Universe::new(1337);
        let agent = Agent::new(1, "TestAgent".to_string());
        
        let perception = Perception::perceive(&agent, &universe).unwrap();
        let intent = Intent::Explore; // Fixed: use imported Intent directly
        let memory = AgentMemory::new();
        
        let thought = thoughts::emit_thought_event( // Fixed: use imported thoughts module
            universe.tick, 
            &agent, 
            &perception, 
            &intent, 
            &memory
        );
        
        assert_eq!(thought.tick, universe.tick);
        assert_eq!(thought.event_type, "inner_monologue");
    }
}
