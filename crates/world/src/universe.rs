use std::collections::BTreeMap;
use blake3;
use crate::types::*;
use crate::bio::BioState;
use crate::cognition::memory::AgentMemory;
use rng::{GlobalSeed, RngSubsystem};
use crate::terrain::Terrain;
use markenz_events::InputEventPayload;

impl Universe {
    pub fn new(seed: u64) -> Self {
        let global_seed = GlobalSeed::from_genesis(seed);
        
        let mut universe = Universe {
            seed,
            tick: 0,
            agents: BTreeMap::new(),
            assets: BTreeMap::new(),
            terrain: Terrain::new(),
            state_hash: [0u8; 32],
            prev_state_hash: [0u8; 32],
            global_seed,
        };

        // Initialize with genesis configuration
        let config = Self::genesis_config();
        
        // Add genesis agents (Gem-D and Gem-K)
        for agent in config.genesis_agents {
            let _ = universe.agents.insert(agent.id, agent);
        }

        // Add genesis assets (House, Shed, Tools, Vehicles)
        for asset in config.genesis_assets {
            let _ = universe.assets.insert(asset.id, asset);
        }

        // Compute initial world hash
        universe.state_hash = universe.compute_hash();
        
        universe
    }
    
    pub fn rng_stream(
        &mut self,
        subsystem: RngSubsystem,
    ) -> &mut rng::RngStream {
        self.global_seed.stream(subsystem, 0)
    }

    pub fn genesis_config() -> UniverseConfig {
        UniverseConfig {
            seed: 1337, // MARKENZ_GENESIS_SEED
            genesis_agents: vec![
                Agent {
                    id: 1,
                    name: "Gem-D".to_string(),
                    position: (0.0, 0.0, 0.0),
                    state_hash: [0u8; 32],
                    inventory: BTreeMap::new(),
                    bio_state: BioState::new(),
                    memory: AgentMemory::new(),
                },
                Agent {
                    id: 2,
                    name: "Gem-K".to_string(),
                    position: (1.0, 0.0, 0.0),
                    state_hash: [0u8; 32],
                    inventory: BTreeMap::new(),
                    bio_state: BioState::new(),
                    memory: AgentMemory::new(),
                },
            ],
            genesis_assets: vec![
                Asset {
                    id: 100,
                    name: "House".to_string(),
                    location: AssetLocation::AtPosition((0.0, 0.0, 0.0)),
                    state: AssetState {
                        durability: 100.0,
                        ownership: Some(1),
                        properties: BTreeMap::new(),
                    },
                },
                Asset {
                    id: 101,
                    name: "Shed".to_string(),
                    location: AssetLocation::AtPosition((1.0, 0.0, 0.0)),
                    state: AssetState {
                        durability: 100.0,
                        ownership: Some(2),
                        properties: BTreeMap::new(),
                    },
                },
                Asset {
                    id: 102,
                    name: "Tool".to_string(),
                    location: AssetLocation::OnAgent(1),
                    state: AssetState {
                        durability: 50.0,
                        ownership: Some(1),
                        properties: BTreeMap::new(),
                    },
                },
                Asset {
                    id: 103,
                    name: "Vehicle".to_string(),
                    location: AssetLocation::OnAgent(2),
                    state: AssetState {
                        durability: 75.0,
                        ownership: Some(2),
                        properties: BTreeMap::new(),
                    },
                },
            ],
        }
    }

    pub fn apply_transition(&mut self, transition: &StateTransition) -> Result<(), String> {
        // Store previous hash
        self.prev_state_hash = self.state_hash;
        
        // Apply state changes based on input event
        match &transition.event.payload {
            InputEventPayload::Move { x, y, z } => {
                if let Some(agent) = self.agents.get_mut(&transition.event.source_agent_id) {
                    agent.position = (*x, *y, *z);
                }
            }
            InputEventPayload::Chat { text: _ } => {
                // Chat doesn't change world state in Phase 0
            }
            InputEventPayload::Gather { resource_type: _ } => {
                // Gathering not implemented in Phase 0
            }
            InputEventPayload::Craft { recipe_id: _ } => {
                // Crafting not implemented in Phase 0
            }
            InputEventPayload::Mine => {
                // Mining not implemented in Phase 0
            }
            InputEventPayload::Build { building_type: _ } => {
                // Building not implemented in Phase 0
            }
            // Phase 0 required events - no state changes
            InputEventPayload::BootEvent
            | InputEventPayload::TickAdvance
            | InputEventPayload::InputEventSubmitted
            | InputEventPayload::ObservationEvent
            | InputEventPayload::SnapshotTaken => {
                // System events - no world state changes
            }
        }
        
        // Compute new hash
        self.state_hash = self.compute_hash();
        
        Ok(())
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        
        // Include previous hash for hash-chain
        let _ = hasher.update(&self.prev_state_hash);
        
        // Include serialized state
        let serialized = bincode::serialize(self).expect("Failed to serialize universe");
        let _ = hasher.update(&serialized);
        
        hasher.finalize().into()
    }

    pub fn validate_all(&self) -> Result<(), String> {
        // Validate agents
        for (id, agent) in &self.agents {
            if agent.id != *id {
                return Err(format!("Agent ID mismatch: stored={}, actual={}", id, agent.id));
            }
            if agent.name.is_empty() {
                return Err(format!("Agent {} has empty name", id));
            }
        }
        
        // Validate assets
        for (id, asset) in &self.assets {
            if asset.id != *id {
                return Err(format!("Asset ID mismatch: stored={}, actual={}", id, asset.id));
            }
            if asset.name.is_empty() {
                return Err(format!("Asset {} has empty name", id));
            }
        }
        
        // Validate hash
        let computed_hash = self.compute_hash();
        if computed_hash != self.state_hash {
            return Err("World hash mismatch".to_string());
        }
        
        Ok(())
    }
}
