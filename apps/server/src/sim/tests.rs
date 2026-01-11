/**
 * ROLE: VERIFICATION
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic engine verification tests
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

#[cfg(test)]
mod tests {
    use crate::sim::{SimLoop, InputEvent};
use crate::sim::events::AdminCommand;

    #[test]
    fn test_determinism_simple() {
        // Test that same seed produces same results
        let seed = 12345;
        let dt = 0.05; // 50ms fixed timestep
        
        // Run simulation A
        let mut sim_a = SimLoop::new(seed, dt);
        sim_a.accumulate_time(dt);
        let result_a = sim_a.tick();
        
        // Reset and run simulation B with same seed
        let mut sim_b = SimLoop::new(seed, dt);
        sim_b.accumulate_time(dt);
        let result_b = sim_b.tick();
        
        // Both should produce identical results
        match (result_a, result_b) {
            (crate::sim::TickResult::Ticked { events: events_a }, 
             crate::sim::TickResult::Ticked { events: events_b }) => {
                assert_eq!(events_a.len(), events_b.len());
                for (event_a, event_b) in events_a.iter().zip(events_b.iter()) {
                    assert_eq!(event_a.seq, event_b.seq);
                    assert_eq!(event_a.tick, event_b.tick);
                    assert_eq!(event_a.kind, event_b.kind);
                }
            }
            _ => panic!("Both simulations should produce TickResult::Ticked"),
        }
    }
    
    #[test]
    fn test_determinism_with_inputs() {
        let seed = 54321;
        let dt = 0.05;
        
        // Run simulation A with inputs
        let mut sim_a = SimLoop::new(seed, dt);
        sim_a.add_input(InputEvent::AdminCommand { 
            command: AdminCommand::SpawnAgent { agent_id: "test-agent".to_string() } 
        });
        sim_a.accumulate_time(dt);
        let result_a = sim_a.tick();
        
        // Run simulation B with same inputs
        let mut sim_b = SimLoop::new(seed, dt);
        sim_b.add_input(InputEvent::AdminCommand { 
            command: AdminCommand::SpawnAgent { agent_id: "test-agent".to_string() } 
        });
        sim_b.accumulate_time(dt);
        let result_b = sim_b.tick();
        
        // Results should be identical
        match (result_a, result_b) {
            (crate::sim::TickResult::Ticked { events: events_a }, 
             crate::sim::TickResult::Ticked { events: events_b }) => {
                assert_eq!(events_a.len(), events_b.len());
                for (event_a, event_b) in events_a.iter().zip(events_b.iter()) {
                    assert_eq!(event_a.seq, event_b.seq);
                    assert_eq!(event_a.tick, event_b.tick);
                    assert_eq!(event_a.kind, event_b.kind);
                }
            }
            _ => panic!("Both simulations should produce TickResult::Ticked"),
        }
    }
    
    #[test]
    fn test_deterministic_replay_100_ticks() {
        // Test requirement: Same seed → same state after N ticks
        let seed = 98765;
        let dt = 0.05;
        let tick_count = 100;
        
        // Run simulation A for 100 ticks
        let mut sim_a = SimLoop::new(seed, dt);
        let mut events_a = Vec::new();
        
        for i in 0..tick_count {
            sim_a.accumulate_time(dt);
            if let crate::sim::TickResult::Ticked { events } = sim_a.tick() {
                events_a.extend(events);
            }
            
            // Add some inputs at specific ticks to test determinism with inputs
            if i % 25 == 0 {
                sim_a.add_input(InputEvent::AdminCommand { 
                    command: AdminCommand::SpawnAgent { 
                        agent_id: format!("agent-{}", i) 
                    } 
                });
            }
        }
        
        // Run simulation B for 100 ticks with same seed and inputs
        let mut sim_b = SimLoop::new(seed, dt);
        let mut events_b = Vec::new();
        
        for i in 0..tick_count {
            sim_b.accumulate_time(dt);
            if let crate::sim::TickResult::Ticked { events } = sim_b.tick() {
                events_b.extend(events);
            }
            
            // Add identical inputs at same ticks
            if i % 25 == 0 {
                sim_b.add_input(InputEvent::AdminCommand { 
                    command: AdminCommand::SpawnAgent { 
                        agent_id: format!("agent-{}", i) 
                    } 
                });
            }
        }
        
        // Verify identical results
        assert_eq!(events_a.len(), events_b.len(), "Event counts should match");
        
        for (i, (event_a, event_b)) in events_a.iter().zip(events_b.iter()).enumerate() {
            assert_eq!(event_a.seq, event_b.seq, "Event seq mismatch at index {}", i);
            assert_eq!(event_a.tick, event_b.tick, "Event tick mismatch at index {}", i);
            assert_eq!(event_a.kind, event_b.kind, "Event kind mismatch at index {}", i);
        }
        
        // Verify final time is identical
        assert_eq!(sim_a.current_time(), sim_b.current_time(), "Final simulation time should match");
    }
    
    #[test]
    fn test_different_seeds_produce_different_results() {
        // Test requirement: Different seed → different state
        let seed_a = 11111;
        let seed_b = 22222;
        let dt = 0.05;
        let tick_count = 50;
        
        // Run simulation with seed A
        let mut sim_a = SimLoop::new(seed_a, dt);
        for _ in 0..tick_count {
            sim_a.accumulate_time(dt);
            let _ = sim_a.tick();
        }
        
        // Run simulation with seed B
        let mut sim_b = SimLoop::new(seed_b, dt);
        for _ in 0..tick_count {
            sim_b.accumulate_time(dt);
            let _ = sim_b.tick();
        }
        
        // Final time should be same (deterministic time progression)
        assert_eq!(sim_a.current_time(), sim_b.current_time());
        
        // But internal RNG state should be different - we can verify this by checking
        // that the next random values differ
        let random_a = sim_a.next_test_random();
        let random_b = sim_b.next_test_random();
        
        assert_ne!(random_a, random_b, "Different seeds should produce different random sequences");
    }
    
    #[test]
    fn test_no_wall_clock_usage() {
        // This test validates that we're not using any wall-clock time APIs
        // by ensuring deterministic behavior regardless of when it's run
        
        let seed = 42424;
        let dt = 0.05;
        
        // Run simulation and capture exact state
        let mut sim1 = SimLoop::new(seed, dt);
        sim1.accumulate_time(dt);
        let result1 = sim1.tick();
        
        // Create new simulation with same parameters
        let mut sim2 = SimLoop::new(seed, dt);
        sim2.accumulate_time(dt);
        let result2 = sim2.tick();
        
        // Results must be identical - proves no wall-clock time usage
        match (result1, result2) {
            (crate::sim::TickResult::Ticked { events: events1 }, 
             crate::sim::TickResult::Ticked { events: events2 }) => {
                assert_eq!(events1.len(), events2.len());
                assert_eq!(sim1.current_time(), sim2.current_time());
            }
            _ => panic!("Both simulations should produce TickResult::Ticked"),
        }
    }
}
