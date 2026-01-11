/**
 * File: apps/engine/src/deterministic_world_loop.rs
 * Purpose: Fixed-timestep deterministic world loop for Phase 1
 * Phase plan authority: docs/plans/PLAN_PHASE_1_DETERMINISM.md
 */

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use blake3::Hasher;

use markenz_world::Universe;
use markenz_events::{InputEvent, ObservationEvent};
use rng::rng_stream::RngSubsystem;

/// Configuration for deterministic world loop execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicWorldConfig {
    /// Seed for deterministic RNG (must be identical for replay)
    pub genesis_seed: u64,
    /// Maximum number of ticks to simulate (0 = infinite)
    pub max_ticks: u64,
    /// Interval in ticks between snapshots (for state verification)
    pub snapshot_interval: u64,
    /// Milliseconds per tick (controls simulation speed)
    pub tick_rate_ms: u64,
}

impl Default for DeterministicWorldConfig {
    fn default() -> Self {
        Self {
            genesis_seed: 0x1337,
            max_ticks: 1000,
            snapshot_interval: 100,
            tick_rate_ms: 50,
        }
    }
}

/// Deterministic snapshot for world state replay
/// 
/// Captures complete world state at a specific tick for
/// deterministic replay and verification purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicSnapshot {
    /// The tick at which snapshot was taken
    pub tick: u64,
    /// Complete universe state at snapshot time
    pub universe: Universe,
    /// RNG state for deterministic continuation
    pub rng_state: rng::GlobalSeed,
    /// World hash at snapshot time
    pub world_hash: [u8; 32],
    /// Complete hash chain up to snapshot tick
    pub hash_chain: Vec<[u8; 32]>,
}

impl DeterministicSnapshot {
    /// Create snapshot from current world loop state
    pub fn from_world_loop(world_loop: &DeterministicWorldLoop) -> Self {
        Self {
            tick: world_loop.current_tick(),
            universe: world_loop.universe.clone(),
            rng_state: world_loop.rng().clone(),
            world_hash: world_loop.universe.state_hash,
            hash_chain: world_loop.hash_chain().to_vec(),
        }
    }
    
    /// Verify snapshot integrity
    pub fn verify_integrity(&self) -> Result<(), String> {
        // Verify hash chain length matches tick
        if self.hash_chain.len() != self.tick as usize + 1 {
            return Err(format!(
                "Hash chain length mismatch: expected {}, got {}",
                self.tick + 1,
                self.hash_chain.len()
            ));
        }
        
        // Verify world hash matches last hash in chain
        if let Some(&last_hash) = self.hash_chain.last() {
            if last_hash != self.world_hash {
                return Err(format!(
                    "World hash mismatch: chain ends with {:x?}, snapshot has {:x?}",
                    last_hash, self.world_hash
                ));
            }
        } else {
            return Err("Hash chain is empty".to_string());
        }
        
        Ok(())
    }
}

/// Deterministic world loop with hash-chain verification
/// 
/// This is the core execution engine for Phase 0/1 deterministic simulation.
/// All state changes are audited and verified through hash chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicWorldLoop {
    /// Configuration parameters for execution
    pub config: DeterministicWorldConfig,
    /// Current universe state
    pub universe: Universe,
    /// Deterministic RNG with audit logging
    pub rng: rng::GlobalSeed,
    /// Current simulation tick
    pub current_tick: u64,
    /// Complete hash chain for verification
    pub hash_chain: Vec<[u8; 32]>,
    /// All observation events generated
    observations: Vec<ObservationEvent>,
}

impl DeterministicWorldLoop {
    /// Create a new deterministic world loop with the given configuration
    pub fn new(config: DeterministicWorldConfig) -> Self {
        info!("Initializing deterministic world loop");
        
        let universe = Universe::new(config.genesis_seed);
        let rng = rng::GlobalSeed::from_genesis(config.genesis_seed);
        let genesis_hash = universe.state_hash;
        let mut hash_chain = Vec::new();
        hash_chain.push(genesis_hash);
        
        Self {
            config,
            universe,
            rng,
            current_tick: 0,
            hash_chain,
            observations: Vec::new(),
        }
    }
    
    /// Run the deterministic world loop with the given input events
    /// 
    /// # Arguments
    /// * `input_events` - Map of tick to events for that tick
    /// 
    /// # Returns
    /// * `Ok(())` if execution completed successfully
    /// * `Err(DeterminismError)` if any determinism violation occurred
    pub fn run(&mut self, input_events: BTreeMap<u64, Vec<InputEvent>>) -> Result<(), DeterminismError> {
        info!("Starting deterministic world loop");
        
        while self.current_tick < self.config.max_ticks {
            self.current_tick += 1;
            self.rng.set_tick(self.current_tick);
            
            info!("Processing tick {}", self.current_tick);
            
            let tick_events = input_events.get(&self.current_tick)
                .cloned()
                .unwrap_or_default();
            
            self.process_tick(tick_events)?;
            
            info!("WORLD_HASH_CHECKPOINT: tick={}, hash={}", 
                self.current_tick, 
                hex::encode(self.universe.state_hash));
        }
        
        Ok(())
    }
    
    fn process_tick(&mut self, input_events: Vec<InputEvent>) -> Result<(), DeterminismError> {
        debug!("Processing {} input events for tick {}", input_events.len(), self.current_tick);
        
        self.observations.clear();
        
        for (sequence, event) in input_events.clone().into_iter().enumerate() {
            debug!("Processing event {}/{}: agent_id={}, payload={:?}", 
                sequence + 1, input_events.len(), event.source_agent_id, event.payload);
            
            let observation = self.apply_input_event(&event)?;
            
            if let Some(obs) = observation {
                self.observations.push(obs);
            }
        }
        
        let new_hash = self.compute_world_hash();
        self.universe.state_hash = new_hash;
        self.hash_chain.push(new_hash);
        
        Ok(())
    }
    
    fn apply_input_event(&mut self, event: &InputEvent) -> Result<Option<ObservationEvent>, DeterminismError> {
        use markenz_events::InputEventPayload;
        
        match &event.payload {
            InputEventPayload::Move { x, y, z } => {
                let physics_stream = self.rng.stream(RngSubsystem::Physics, 0);
                
                let variation = physics_stream.next_u64() % 4;
                let final_x = *x + (variation as f32 - 1.5) * 0.1;
                
                if let Some(agent) = self.universe.agents.get_mut(&event.source_agent_id) {
                    let old_position = agent.position;
                    agent.position = (final_x, *y, *z);
                    
                    let observation = ObservationEvent {
                        tick: self.current_tick,
                        event_type: "agent_moved".to_string(),
                        payload: serde_json::json!({
                            "agent_id": event.source_agent_id,
                            "old_position": old_position,
                            "new_position": agent.position,
                        }),
                        hash: [0u8; 32], // Will be computed
                    };
                    
                    Ok(Some(observation))
                } else {
                    warn!("Agent {} not found for movement event", event.source_agent_id);
                    Ok(None)
                }
            }
            
            InputEventPayload::BootEvent => {
                debug!("Boot event processed (no state change)");
                Ok(None)
            }
            
            payload => {
                warn!("Unsupported event payload in Phase 1: {:?}", payload);
                Ok(None)
            }
        }
    }
    
    fn compute_world_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        
        let _ = hasher.update(&self.current_tick.to_le_bytes());
        
        let universe_bytes = bincode::serialize(&self.universe)
            .expect("Universe serialization must succeed for hashing");
        let _ = hasher.update(&universe_bytes);
        
        *hasher.finalize().as_bytes()
    }
    
    /// Get the current RNG state
    pub fn rng(&self) -> &rng::GlobalSeed {
        &self.rng
    }
    
    /// Get mutable RNG state
    pub fn rng_mut(&mut self) -> &mut rng::GlobalSeed {
        &mut self.rng
    }
    
    /// Get the current universe state
    pub fn universe(&self) -> &Universe {
        &self.universe
    }
    
    /// Get mutable universe state
    pub fn universe_mut(&mut self) -> &mut Universe {
        &mut self.universe
    }
    
    /// Get the current hash chain
    pub fn hash_chain(&self) -> &[[u8; 32]] {
        &self.hash_chain
    }
    
    /// Get mutable hash chain
    pub fn hash_chain_mut(&mut self) -> &mut Vec<[u8; 32]> {
        &mut self.hash_chain
    }
    
    /// Get the current tick
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }
    
    /// Get current world state hash
    pub fn current_hash(&self) -> [u8; 32] {
        self.universe.state_hash
    }
    
    /// Set the current tick
    pub fn set_current_tick(&mut self, tick: u64) {
        self.current_tick = tick;
    }
    
    /// Get the RNG audit log for all random draws
    pub fn rng_audit_log(&self) -> &rng::RngAuditLog {
        self.rng.audit_log()
    }
    
    /// Get all observation events generated during execution
    pub fn observations(&self) -> &[ObservationEvent] {
        &self.observations
    }
}

/// Errors that can occur during deterministic world loop execution
#[derive(Debug, thiserror::Error)]
pub enum DeterminismError {
    /// Events were processed out of order
    #[error("Invalid event order at tick {tick}: event_tick={event_tick}, reason={reason}")]
    InvalidEventOrder {
        /// The tick at which the error occurred
        tick: u64,
        /// The tick of the event that caused the error
        event_tick: u64,
        /// Human-readable reason for the error
        reason: String,
    },
    
    /// Hash chain continuity was violated
    #[error("Hash continuity violation at tick {tick}: expected={expected:x?}, actual={actual:x?}")]
    HashContinuityViolation {
        /// The tick at which the violation occurred
        tick: u64,
        /// Expected hash value
        expected: [u8; 32],
        /// Actual hash value that was computed
        actual: [u8; 32],
    },
    
    /// Error during snapshot operation
    #[error("Snapshot error at tick {tick}: {reason}")]
    SnapshotError {
        /// The tick at which the snapshot error occurred
        tick: u64,
        /// Human-readable reason for the error
        reason: String,
    },
    
    /// Snapshot integrity verification failed
    #[error("Snapshot integrity violation at tick {tick}: expected={expected:x?}, actual={actual:x?}")]
    SnapshotIntegrityViolation {
        /// The tick at which the violation occurred
        tick: u64,
        /// Expected snapshot hash
        expected: [u8; 32],
        /// Actual snapshot hash that was computed
        actual: [u8; 32],
    },
    
    /// RNG audit log requirements were violated
    #[error("RNG audit violation at tick {tick}: {reason}")]
    RngAuditViolation {
        /// The tick at which the violation occurred
        tick: u64,
        /// Human-readable reason for the violation
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use markenz_events::InputEventPayload;
    
    #[test]
    fn test_deterministic_world_loop_creation() {
        let config = DeterministicWorldConfig::default();
        let world_loop = DeterministicWorldLoop::new(config);
        
        assert_eq!(world_loop.current_tick(), 0);
        assert_eq!(world_loop.hash_chain().len(), 1);
        assert_ne!(world_loop.current_hash(), [0u8; 32]);
    }
    
    #[test]
    fn test_deterministic_replay() {
        let config = DeterministicWorldConfig {
            genesis_seed: 1337,
            max_ticks: 10,
            snapshot_interval: 5,
            tick_rate_ms: 50,
        };
        
        let mut input_events = BTreeMap::new();
        let _ = input_events.insert(1, vec![
            InputEvent {
                tick: 1,
                source_agent_id: 1,
                sequence: 1,
                payload: InputEventPayload::Move { x: 10.0, y: 20.0, z: 0.0 },
                hash: [0u8; 32],
                prev_hash: [0u8; 32],
            }
        ]);
        
        let mut world_loop1 = DeterministicWorldLoop::new(config.clone());
        world_loop1.run(input_events.clone()).unwrap();
        let hash1 = world_loop1.hash_chain().to_vec();
        
        let mut world_loop2 = DeterministicWorldLoop::new(config);
        world_loop2.run(input_events).unwrap();
        let hash2 = world_loop2.hash_chain().to_vec();
        
        assert_eq!(hash1.len(), hash2.len());
        for (i, (h1, h2)) in hash1.iter().zip(hash2.iter()).enumerate() {
            assert_eq!(h1, h2, "Hashes differ at tick {}: {:x?} vs {:x?}", i, h1, h2);
        }
    }
}
