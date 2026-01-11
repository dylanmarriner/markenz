---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_4_COGNITION
phase: 4
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Agent Decision Logic · Planning · Memory · Observable Intent
requires: PLAN_PHASE_3_BIOLOGY (100% complete)
---

# PLAN PHASE 4: COGNITION
## (Deterministic Decision Logic · Planning · Memory · Inner Monologue)

**AUDIENCE:** Windsurf executor  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement deterministic agent cognition (NO LLM, NO external AI):
- Perception: agents perceive nearby entities, terrain, resources
- Intent formation: agents form goals based on bio-state and perception
- Planning: agents plan action sequences (movement, gathering, crafting)
- Memory: persistent memory of locations, agents, outcomes
- Observable inner monologue: thought process streamed as events
- Deterministic: identical seed + perception → identical decisions

---

## 2. COGNITION ARCHITECTURE

### 2.1 Perception (crates/world/src/cognition/perception.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perception {
    pub nearby_agents: Vec<(u64, f32)>,          // (agent_id, distance)
    pub nearby_assets: Vec<(u64, String, f32)>,  // (asset_id, type, distance)
    pub nearby_resources: Vec<(String, f32)>,    // (resource_type, abundance)
    pub terrain_biome: BiomeType,
    pub time_of_day: u32,                        // 0-23999 (ticks in day)
}

impl Perception {
    pub fn perceive(
        agent: &Agent,
        universe: &Universe,
    ) -> Result<Self, String> {
        let perception_range = 20.0;  // 20 block radius
        
        let mut nearby_agents = Vec::new();
        for (other_id, other) in &universe.agents {
            if other_id == &agent.id {
                continue;
            }
            let dist = agent.distance_to(&other.position);
            if dist < perception_range {
                nearby_agents.push((*other_id, dist));
            }
        }
        
        let mut nearby_assets = Vec::new();
        for (asset_id, asset) in &universe.assets {
            if let AssetLocation::World(x, y, z) = asset.location {
                let dist = agent.distance_to(&(x, y, z));
                if dist < perception_range {
                    nearby_assets.push((*asset_id, asset.name.clone(), dist));
                }
            }
        }
        
        let biome = universe.terrain.biome_at(agent.position.0, agent.position.1);
        let resources = BiomeMap::resources_in_biome(biome);
        let nearby_resources = resources.iter()
            .map(|r| (r.name().to_string(), 1.0))  // Abundance TBD in Phase 5
            .collect();
        
        Ok(Self {
            nearby_agents,
            nearby_assets,
            nearby_resources,
            terrain_biome: biome,
            time_of_day: (universe.tick % 24000) as u32,
        })
    }
}
```

### 2.2 Intent Formation (crates/world/src/cognition/intent.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Intent {
    Forage,        // Gather food/resources
    Explore,       // Move to new location
    Rest,          // Stay put, recover
    Build,         // Construct asset
    Craft,         // Create tool/item
    Socialize,     // Interact with other agent
    Hunt,          // Move toward prey (Phase 5+)
}

pub struct IntentPlanner;

impl IntentPlanner {
    pub fn form_intent(
        agent: &Agent,
        perception: &Perception,
        memory: &AgentMemory,
        rng: &mut RngStream,
    ) -> Intent {
        // Decision tree based on bio-state and perception
        
        // Immediate need: high hunger
        if agent.bio_state.hunger > 70.0 {
            if perception.nearby_resources.len() > 0 {
                return Intent::Forage;
            } else {
                return Intent::Explore;
            }
        }
        
        // High exhaustion: need rest
        if agent.bio_state.exhaustion > 60.0 {
            return Intent::Rest;
        }
        
        // See another agent: consider socializing
        if perception.nearby_agents.len() > 0 && rng.next_f64() > 0.5 {
            return Intent::Socialize;
        }
        
        // Explore if idle
        Intent::Explore
    }
}
```

### 2.3 Planning (crates/world/src/cognition/planning.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: Intent,
    pub steps: Vec<ActionStep>,
    pub created_tick: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionStep {
    MoveTo { x: f32, y: f32, z: f32 },
    Gather { resource: String },
    Mine,
    Build { blueprint_id: u64 },
    Craft { recipe_id: u64 },
    Chat { target_agent: u64, message: String },
}

pub struct Planner;

impl Planner {
    pub fn plan(
        agent: &Agent,
        intent: Intent,
        perception: &Perception,
        universe: &Universe,
    ) -> Plan {
        let mut steps = Vec::new();
        
        match intent {
            Intent::Forage => {
                // Step 1: Move to resource location
                if let Some((resource, _dist)) = perception.nearby_resources.first() {
                    let (target_x, target_y) = Self::find_resource_location(
                        resource,
                        perception.terrain_biome,
                    );
                    steps.push(ActionStep::MoveTo {
                        x: target_x,
                        y: target_y,
                        z: universe.terrain.height_at(target_x, target_y),
                    });
                    
                    // Step 2: Gather
                    steps.push(ActionStep::Gather {
                        resource: resource.clone(),
                    });
                }
            },
            Intent::Explore => {
                // Random movement
                let new_x = agent.position.0 + (rand() - 0.5) * 10.0;
                let new_y = agent.position.1 + (rand() - 0.5) * 10.0;
                steps.push(ActionStep::MoveTo {
                    x: new_x,
                    y: new_y,
                    z: universe.terrain.height_at(new_x, new_y),
                });
            },
            Intent::Rest => {
                // No action steps; agent pauses
            },
            _ => {},
        }
        
        Plan {
            goal: intent,
            steps,
            created_tick: universe.tick,
        }
    }
    
    fn find_resource_location(resource: &str, biome: BiomeType) -> (f32, f32) {
        // Find nearest location of resource type
        // (Simplified; full implementation scans nearby terrain)
        (0.0, 0.0)
    }
}
```

### 2.4 Memory System (crates/world/src/cognition/memory.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentMemory {
    pub known_locations: BTreeMap<String, (f32, f32)>,  // Location name → coordinates
    pub known_agents: BTreeMap<u64, AgentKnowledge>,    // Other agent info
    pub experience_log: Vec<MemoryTrace>,               // Historical events
    pub skill_levels: BTreeMap<String, f64>,            // Skill proficiency
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentKnowledge {
    pub agent_id: u64,
    pub name: String,
    pub last_seen_tick: u64,
    pub trust_level: f64,  // -1.0 to 1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryTrace {
    pub tick: u64,
    pub event: String,
    pub location: (f32, f32),
    pub outcome: bool,  // success/failure
}

impl AgentMemory {
    pub fn new() -> Self {
        Self {
            known_locations: BTreeMap::new(),
            known_agents: BTreeMap::new(),
            experience_log: Vec::new(),
            skill_levels: BTreeMap::new(),
        }
    }
    
    pub fn record_event(
        &mut self,
        tick: u64,
        event: String,
        location: (f32, f32),
        outcome: bool,
    ) {
        self.experience_log.push(MemoryTrace {
            tick,
            event,
            location,
            outcome,
        });
    }
    
    pub fn recall_location(&self, location_name: &str) -> Option<(f32, f32)> {
        self.known_locations.get(location_name).copied()
    }
}
```

---

## 3. OBSERVABLE INNER MONOLOGUE

### 3.1 Thought Events (crates/world/src/cognition/thoughts.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thought {
    pub tick: u64,
    pub agent_id: u64,
    pub content: String,
    pub emotion: Option<String>,  // "happy", "fearful", etc.
}

pub fn emit_thought_event(
    tick: u64,
    agent: &Agent,
    perception: &Perception,
    intent: &Intent,
    memory: &AgentMemory,
) -> ObservationEvent {
    let thought_content = format!(
        "Agent {} perceives: {:?} agents, {:?} resources. Intent: {:?}. Recall: {:?} known locations.",
        agent.name,
        perception.nearby_agents.len(),
        perception.nearby_resources.len(),
        intent,
        memory.known_locations.len()
    );
    
    let payload = json!({
        "type": "inner_monologue",
        "agent_id": agent.id,
        "content": thought_content,
        "intent": format!("{:?}", intent),
        "bio_state": {
            "energy": agent.bio_state.energy,
            "hunger": agent.bio_state.hunger,
        },
    });
    
    ObservationEvent {
        tick,
        event_type: "inner_monologue".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    }
}
```

---

## 4. COGNITION IN AUTHORITY PIPELINE

### 4.1 Perception Pass (Pass 3, after Phase 4 introduction)

```rust
// In authority pipeline, after BioVeto (Pass 3):

// Pass 4: Perception Pass
let perception = Perception::perceive(&agent, &universe)?;

// Pass 5: Intent Pass
let intent = IntentPlanner::form_intent(&agent, &perception, &agent.memory, rng)?;

// Pass 6: Planning Pass
let plan = Planner::plan(&agent, intent, &perception, &universe);

// Emit inner monologue
let thought = emit_thought_event(universe.tick, &agent, &perception, &intent, &agent.memory);
observations.push(thought);

// Pass 7-12: Physical execution of plan (unchanged)
```

---

## 5. DETERMINISTIC COGNITION GUARANTEE

All cognition deterministic: identical seed + perception → identical intent and plan.

---

## 6. TEST SUITE

**TEST-PERCEPTION-001**: Agents perceive nearby entities correctly  
**TEST-INTENT-001**: Intent formation deterministic  
**TEST-PLANNING-001**: Plans generated consistently  
**TEST-MEMORY-001**: Memory persists across ticks  
**TEST-MONOLOGUE-001**: Inner monologue observable  

---

## 7. SUCCESS CRITERIA

- [ ] Perception system implemented
- [ ] Intent formation deterministic
- [ ] Planning produces valid action sequences
- [ ] Memory persists and recalls correctly
- [ ] Inner monologue observable in events
- [ ] All tests passing
- [ ] No regression from Phase 3

---

## 8. FORBIDDEN ACTIONS

- No external LLM calls
- No TODO/FIXME in cognition
- No non-deterministic randomness
- Cannot skip memory recording
- Inner monologue always enabled

---

## END OF PLAN

**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_4_COGNITION  
**Timestamp:** 2026-01-11
