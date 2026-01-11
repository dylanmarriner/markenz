use markenz_world::{Universe, Agent, Asset, AssetType, AgentVitals};
use markenz_world::types::{Chunk, BTreeMap};
use rng::DeterministicRng;
use blake3;
use tracing::info;

pub fn genesis_snapshot(rng_seed: u64) -> Universe {
    // Create universe with deterministic seed
    let mut universe = Universe::new(rng_seed);
    
    // Create minimal genesis agents (Gem-D and Gem-K)
    let gem_d = Agent {
        id: 1,
        name: "Gem-D".to_string(),
        state_hash: [0u8; 32],
        position: (0, 0, 0),
        inventory: BTreeMap::new(),
        vitals: AgentVitals {
            health: 100,
            energy: 100,
            mood: 50,
        },
    };
    
    let gem_k = Agent {
        id: 2,
        name: "Gem-K".to_string(),
        state_hash: [0u8; 32],
        position: (1, 0, 0),
        inventory: BTreeMap::new(),
        vitals: AgentVitals {
            health: 100,
            energy: 100,
            mood: 50,
        },
    };
    
    // Apply identity fingerprinting
    let gem_d_with_fingerprint = apply_identity_fingerprint(gem_d);
    let gem_k_with_fingerprint = apply_identity_fingerprint(gem_k);
    
    universe.agents.insert(gem_d_with_fingerprint.id, gem_d_with_fingerprint);
    universe.agents.insert(gem_k_with_fingerprint.id, gem_k_with_fingerprint);
    
    // Create minimal genesis assets
    let house = Asset {
        id: 100,
        asset_type: AssetType::House,
        owner_id: 1,
        position: (0, 0, 0),
        properties: BTreeMap::new(),
        inventory: BTreeMap::new(),
    };
    
    let shed = Asset {
        id: 101,
        asset_type: AssetType::Shed,
        owner_id: 2,
        position: (1, 0, 0),
        properties: BTreeMap::new(),
        inventory: BTreeMap::new(),
    };
    
    universe.assets.insert(house.id, house);
    universe.assets.insert(shed.id, shed);
    
    // Generate minimal deterministic terrain (single chunk for N1.3)
    let mut rng = DeterministicRng::new(rng_seed);
    let mut env_stream = rng.stream("Environment", 0);
    
    let terrain_data = generate_terrain_chunk(&mut env_stream, 0, 0);
    let chunk = Chunk {
        x: 0,
        y: 0,
        terrain_data,
        entities: vec![1, 2, 100, 101], // agents and assets
        properties: BTreeMap::new(),
    };
    universe.chunks.insert("0,0".to_string(), chunk);
    
    // Compute deterministic world hash
    universe.world_hash = markenz_world::hashing::world_hash(&universe);
    
    info!("Genesis world created at tick {} with hash {:?}", universe.tick_index(), universe.world_hash);
    universe
}

pub fn load_gem_d_from_export() -> Agent {
    Agent {
        id: 1,
        name: "Gem-D".to_string(),
        state_hash: [0u8; 32],
        position: (0, 0, 0),
        inventory: BTreeMap::new(),
        vitals: AgentVitals {
            health: 100,
            energy: 100,
            mood: 50,
        },
    }
}

pub fn load_gem_k_from_export() -> Agent {
    Agent {
        id: 2,
        name: "Gem-K".to_string(),
        state_hash: [0u8; 32],
        position: (1, 0, 0),
        inventory: BTreeMap::new(),
        vitals: AgentVitals {
            health: 100,
            energy: 100,
            mood: 50,
        },
    }
}

pub fn load_house_asset() -> Asset {
    Asset {
        id: 100,
        asset_type: AssetType::House,
        owner_id: 1,
        position: (0, 0, 0),
        properties: BTreeMap::new(),
    }
}

pub fn load_shed_asset() -> Asset {
    Asset {
        id: 101,
        asset_type: AssetType::Shed,
        owner_id: 2,
        position: (0, 1, 0),
        properties: BTreeMap::new(),
    }
}

pub fn load_tools_inventory() -> Vec<Asset> {
    vec![
        Asset {
            id: 102,
            asset_type: AssetType::Tool,
            owner_id: 1,
            position: (0, 0, 0),
            properties: {
                let mut props = BTreeMap::new();
                props.insert("tool_type".to_string(), "hammer".to_string());
                props
            },
        },
        Asset {
            id: 103,
            asset_type: AssetType::Tool,
            owner_id: 1,
            position: (0, 0, 0),
            properties: {
                let mut props = BTreeMap::new();
                props.insert("tool_type".to_string(), "wrench".to_string());
                props
            },
        },
    ]
}

pub fn load_vehicles_inventory() -> Vec<Asset> {
    vec![
        Asset {
            id: 104,
            asset_type: AssetType::Vehicle,
            owner_id: 2,
            position: (1, 0, 0),
            properties: {
                let mut props = BTreeMap::new();
                props.insert("vehicle_type".to_string(), "bicycle".to_string());
                props
            },
        },
    ]
}

fn apply_identity_fingerprint(mut agent: Agent) -> Agent {
    let mut hasher = blake3::Hasher::new();
    hasher.update(agent.name.as_bytes());
    
    // Hash the current state (excluding the state_hash field itself)
    let temp_hash = agent.state_hash;
    agent.state_hash = [0u8; 32];
    let state_bytes = bincode::serialize(&agent).unwrap();
    hasher.update(&state_bytes);
    
    let fingerprint = hasher.finalize();
    agent.state_hash = fingerprint.into();
    
    agent
}

fn generate_terrain_chunk(rng_stream: &mut rng::RngStream, x: i32, y: i32) -> Vec<u8> {
    let mut terrain_data = Vec::new();
    
    // Generate 4x4 terrain chunk deterministically
    for _ in 0..16 {
        let height = rng_stream.next_in_range(0, 255);
        terrain_data.push(height as u8);
    }
    
    terrain_data
}

fn apply_identity_fingerprint(mut agent: Agent) -> Agent {
    let mut hasher = blake3::Hasher::new();
    hasher.update(agent.name.as_bytes());
    
    // Hash the current state (excluding state_hash field itself)
    let temp_hash = agent.state_hash;
    agent.state_hash = [0u8; 32];
    let state_bytes = bincode::serialize(&agent).unwrap();
    hasher.update(&state_bytes);
    
    let fingerprint = hasher.finalize();
    agent.state_hash = fingerprint.into();
    
    agent
}
