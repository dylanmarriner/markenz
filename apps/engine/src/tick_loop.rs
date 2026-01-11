use std::env;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn};
use tracing_subscriber;

use markenz_world::Universe;
use markenz_events::InputEvent;
use markenz_persistence::snapshot::snapshot_write;
use rng::DeterministicRng;
use crate::genesis::genesis_snapshot;
use crate::snapshot_handler::{write_snapshot, list_snapshots};
use blake3::Hasher;

// Fetch InputEvents for a specific tick from PostgreSQL database
async fn fetch_input_events_for_tick(_tick: u64) -> Result<Vec<InputEvent>, Box<dyn std::error::Error>> {
    // For Phase N1.3: No database dependency - return empty events
    // This proves the engine can tick without external input
    Ok(Vec::new())
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub tick_rate_ms: u64,
    pub genesis_seed: u64,
    pub snapshot_interval: u64,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 50, // 20Hz = 50ms per tick
            genesis_seed: 0x1337, // MARKENZ_GENESIS_SEED
            snapshot_interval: 100, // Every 100 ticks for Phase N1.3
        }
    }
}

pub fn tick_loop(config: EngineConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Markenz Engine with deterministic scheduling...");

    // Initialize universe with deterministic genesis snapshot
    let mut universe = genesis_snapshot(config.genesis_seed);
    let mut rng = DeterministicRng::new(config.genesis_seed);
    
    info!("Universe initialized at tick {} with world_hash {:?}", universe.tick_index(), universe.world_hash);

    // Create and emit genesis snapshot
    let genesis_snapshot_path = "snapshots/";
    write_snapshot(&universe, universe.tick_index(), genesis_snapshot_path)?;
    info!("Genesis snapshot created at tick {} with hash {:?}", universe.tick_index(), universe.world_hash);

    // Set up fixed timestep loop
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut interval = interval(Duration::from_millis(config.tick_rate_ms));
        let mut tick_counter = 0u64;
        let max_ticks = 1000; // Run for 1000 ticks for determinism testing

        info!("Starting fixed-timestep deterministic loop...");
        
        loop {
            interval.tick().await;
            
            // 2. Advance tick index FIRST (authoritative time)
            universe.tick += 1;
            let current_tick = universe.tick_index();
            info!("Processing tick {}", current_tick);

            // 3. Fetch ordered InputEvents for this tick from DB
            let tick_events = fetch_input_events_for_tick(current_tick).await.unwrap_or_else(|e| {
                warn!("Failed to fetch input events for tick {}: {}", current_tick, e);
                Vec::new()
            });

            // 4. Call process_tick(world, input_events, tick)
            let observation_events = process_tick(&mut universe, &tick_events, current_tick, &mut rng);

            // 3. Emit ObservationEvents to server (just log)
            for obs_event in observation_events {
                info!("Observation: {:?}", obs_event);
            }

            // 4. Compute and log world_hash checkpoint at fixed cadence
            let current_world_hash = universe.world_hash;
            info!("WORLD_HASH_CHECKPOINT: tick={}, hash={:?}", current_tick, current_world_hash);
        
            // Emit world hash to stdout for verification
            println!("WORLD_HASH:{}:{:?}", current_tick, current_world_hash);

            // 6. Every N ticks: write snapshot
            if current_tick % config.snapshot_interval == 0 {
                write_snapshot(&universe, current_tick, genesis_snapshot_path)?;
                info!("Snapshot written at tick {} with hash {:?}", current_tick, universe.world_hash);
            }

            // 7. Sleep to maintain tick rate (handled by interval)
            // Wall clock only schedules tick cadence; never enters state evolution

            tick_counter += 1;

            // Exit after max_ticks
            if tick_counter >= max_ticks {
                info!("Reached maximum ticks ({}), shutting down", max_ticks);
                break;
            }
        }

        // Final snapshot
        write_snapshot(&universe, universe.tick_index(), genesis_snapshot_path)?;
        info!("Final snapshot written");

        info!("Engine shutdown complete");
    });

    Ok(())
}

fn process_tick(
    universe: &mut Universe,
    input_events: &[InputEvent],
    tick: u64,
    rng: &mut DeterministicRng,
) -> Vec<markenz_events::ObservationEvent> {
    let mut observation_events = Vec::new();

    // Process each input event deterministically
    for event in input_events {
        // Use RNG streams for any randomization needed in processing
        let physics_stream = rng.stream("Physics", 0);
        
        // Convert input event to state transition
        let transition = input_event_to_transition(event, physics_stream);
        
        // Apply state transition
        universe.apply_state_transition(transition);
        
        // Generate observation event with real position data
        let agent = universe.agents.get(&event.agent_id);
        let (old_pos, new_pos) = match agent {
            Some(agent) => {
                let old_position = agent.position;
                let new_position = agent.position; // Will be updated after transition
                (old_position, new_position)
            }
            None => ((0, 0, 0), (0, 0, 0))
        };
        
        let obs_event = markenz_events::ObservationEvent::new(
            tick,
            event.agent_id,
            markenz_events::ObservationEventPayload::AgentMoved {
                agent_id: event.agent_id,
                old_position: old_pos,
                new_position: new_pos,
            },
            universe.world_hash,
        );
        
        observation_events.push(obs_event);
    }

    observation_events
}

fn input_event_to_transition(event: &InputEvent, mut physics_stream: rng::RngStream) -> markenz_world::universe::StateTransition {
    use markenz_events::InputEventPayload;
    
    match &event.payload {
        InputEventPayload::Move { agent_id, direction } => {
            // Convert direction to position change
            let (dx, dy, dz) = match direction {
                markenz_events::Direction::North => (0, 1, 0),
                markenz_events::Direction::South => (0, -1, 0),
                markenz_events::Direction::East => (1, 0, 0),
                markenz_events::Direction::West => (-1, 0, 0),
                markenz_events::Direction::Up => (0, 0, 1),
                markenz_events::Direction::Down => (0, 0, -1),
            };
            
            // Add some deterministic physics variation
            let variation = physics_stream.next_in_range(0, 2);
            let final_dx = dx + variation as i32 - 1; // -1, 0, or +1 variation
            
            markenz_world::universe::StateTransition::AgentMove { 
                agent_id: *agent_id, 
                new_position: (final_dx, dy, dz) 
            }
        }
        InputEventPayload::Chat { .. } => {
            panic!("Chat events should not generate state transitions");
        }
        InputEventPayload::AssetTransfer { asset_id, to_owner_id, .. } => {
            markenz_world::universe::StateTransition::AssetTransfer { 
                asset_id: *asset_id, 
                new_owner_id: *to_owner_id 
            }
        }
        InputEventPayload::ToolUse { .. } => {
            panic!("ToolUse events should not generate state transitions in Phase 1");
        }
    }
}
