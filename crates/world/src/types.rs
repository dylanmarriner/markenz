use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use rng::GlobalSeed;
use markenz_events::InputEvent;
use crate::terrain::Terrain;
use crate::bio::BioState;
use crate::cognition::memory::AgentMemory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    House,
    Shed,
    Tool,
    Vehicle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentVitals {
    pub health: u32,
    pub energy: u32,
    pub mood: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: u64,
    pub name: String,
    pub position: (f32, f32, f32),  // x, y, z coordinates
    pub state_hash: [u8; 32],       // blake3 hash of agent state
    pub inventory: BTreeMap<u64, Asset>,      // Items carried by ID
    pub bio_state: BioState,        // Preserved from genesis
    pub memory: AgentMemory,         // Agent memory and cognition
}

impl Agent {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            position: (0.0, 0.0, 0.0),
            state_hash: [0u8; 32],
            inventory: BTreeMap::new(),
            bio_state: BioState::new(),
            memory: AgentMemory::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Asset {
    pub id: u64,
    pub name: String,
    pub location: AssetLocation,  // (AgentId | Position)
    pub state: AssetState,         // Location, durability, ownership
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetLocation {
    OnAgent(u64),  // Agent ID
    AtPosition((f32, f32, f32)),  // World coordinates
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetState {
    pub durability: f32,
    pub ownership: Option<u64>,  // Owner agent ID, if any
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub terrain_data: Vec<u8>,
    pub entities: Vec<u64>,
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Universe {
    pub seed: u64,                              // Genesis seed
    pub tick: u64,                              // Current tick
    pub agents: BTreeMap<u64, Agent>,           // Agents by ID
    pub assets: BTreeMap<u64, Asset>,           // Assets by ID
    pub terrain: Terrain,                       // World terrain
    pub state_hash: [u8; 32],                   // Current world_hash
    pub prev_state_hash: [u8; 32],              // Previous hash
    pub global_seed: GlobalSeed,                  // NEW: centralized RNG ownership
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniverseConfig {
    pub seed: u64,
    pub genesis_agents: Vec<Agent>,
    pub genesis_assets: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateTransition {
    pub event: InputEvent,
    pub before_state: Vec<u8>,
    pub after_state: Vec<u8>,
}
