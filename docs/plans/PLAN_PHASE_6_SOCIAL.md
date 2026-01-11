---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_6_SOCIAL
phase: 6
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Reproduction · Social Bonds · Cultural Traits · Group Dynamics
requires: PLAN_PHASE_5_GENETICS (100% complete)
---

# PLAN PHASE 6: SOCIAL
## (Reproduction · Social Bonds · Cultural Inheritance · Observable Relationships)

**AUDIENCE:** Windsurf executor  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement social systems for agents:
- Reproduction: agents reproduce (asexually for Phase 6), inheriting genetic + cultural traits
- Social bonds: trust, affinity, collaboration
- Cultural traits: learned behaviors, language, art (deterministic parameter drift)
- Group dynamics: coordination, coalition formation
- Observable relationships: friendship, kinship, rivalry logged

---

## 2. REPRODUCTION SYSTEM

### 2.1 Reproduction (crates/world/src/social/reproduction.rs)

```rust
pub struct ReproductionEngine;

impl ReproductionEngine {
    pub fn can_reproduce(agent: &Agent) -> bool {
        // Sufficient energy, good health, not exhausted
        agent.bio_state.energy > 50.0
            && agent.bio_state.health > 80.0
            && agent.bio_state.exhaustion < 30.0
    }
    
    pub fn reproduce(
        parent: &Agent,
        universe: &mut Universe,
        rng: &mut RngStream,
    ) -> Result<u64, String> {
        // Create offspring agent
        let offspring_id = universe.next_agent_id();
        let mut offspring = Agent {
            id: offspring_id,
            name: format!("{}_child", parent.name),
            position: parent.position,  // Born at parent location
            state_hash: [0; 32],
            inventory: Vec::new(),
            bio_state: BioState {
                energy: 50.0,
                hunger: 0.0,
                exhaustion: 0.0,
                health: 100.0,
                metabolic_rate: parent.bio_state.metabolic_rate,
                recovery_rate: parent.bio_state.recovery_rate,
            },
            genome: {
                let mut child_genome = parent.genome.clone();
                MutationEngine::mutate(&mut child_genome, rng)?;
                child_genome
            },
            culture: parent.culture.clone(),  // Inherit culture
            memory: AgentMemory::new(),  // Fresh memory
        };
        
        offspring.bio_state.apply_traits(&offspring.genome.express_traits());
        
        universe.agents.insert(offspring_id, offspring);
        
        Ok(offspring_id)
    }
}
```

---

## 3. SOCIAL BONDS

### 3.1 Relationships (crates/world/src/social/relationships.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub other_agent_id: u64,
    pub bond_type: BondType,
    pub strength: f64,  // -1.0 (enemy) to 1.0 (ally)
    pub history: Vec<InteractionRecord>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum BondType {
    Kinship,        // Parent, sibling, offspring
    Friendship,     // Collaborative history
    Rivalry,        // Competitive history
    Neutral,        // No relationship
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub tick: u64,
    pub event: String,  // "shared_food", "collaborated", "competed"
    pub impact: f64,    // Bond strength change
}

pub struct RelationshipManager;

impl RelationshipManager {
    pub fn update_bond(
        agent: &mut Agent,
        other_id: u64,
        event: String,
        impact: f64,
    ) {
        let rel = agent.relationships
            .iter_mut()
            .find(|r| r.other_agent_id == other_id);
        
        if let Some(rel) = rel {
            rel.strength = (rel.strength + impact).clamp(-1.0, 1.0);
            rel.history.push(InteractionRecord {
                tick: 0,  // Set by caller
                event,
                impact,
            });
        }
    }
}
```

---

## 4. CULTURAL TRAITS

### 4.1 Culture (crates/world/src/social/culture.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Culture {
    pub values: BTreeMap<String, f64>,  // Cultural trait → expression (-1.0 to 1.0)
    pub language: String,               // Language identifier
    pub traditions: Vec<String>,        // Cultural practices
}

impl Culture {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            language: "markenz_common".to_string(),
            traditions: Vec::new(),
        }
    }
    
    pub fn inherit_from_parent(parent: &Culture, rng: &mut RngStream) -> Self {
        let mut culture = parent.clone();
        
        // Small cultural drift (analogous to genetic mutation)
        for (_value_name, expression) in culture.values.iter_mut() {
            let drift = (rng.next_f64() - 0.5) * 0.05;
            *expression = (*expression + drift).clamp(-1.0, 1.0);
        }
        
        culture
    }
}
```

---

## 5. OBSERVABLE SOCIAL EVENTS

### 5.1 Social Observation (crates/world/src/social/observation.rs)

```rust
pub fn reproduction_event(
    tick: u64,
    parent_id: u64,
    offspring_id: u64,
    offspring_name: String,
) -> ObservationEvent {
    let payload = json!({
        "type": "reproduction",
        "parent_id": parent_id,
        "offspring_id": offspring_id,
        "offspring_name": offspring_name,
        "inheritance": {
            "genetic": true,
            "cultural": true,
        }
    });
    
    ObservationEvent {
        tick,
        event_type: "reproduction".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    }
}

pub fn relationship_change_event(
    tick: u64,
    agent_id: u64,
    other_id: u64,
    bond_type: BondType,
    strength_delta: f64,
) -> ObservationEvent {
    let payload = json!({
        "type": "relationship_changed",
        "agent_id": agent_id,
        "other_agent_id": other_id,
        "bond_type": format!("{:?}", bond_type),
        "strength_delta": strength_delta,
    });
    
    ObservationEvent {
        tick,
        event_type: "relationship_changed".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    }
}
```

---

## 6. TEST SUITE

**TEST-REPRODUCTION-001**: Offspring inherits genetics + culture  
**TEST-BONDS-001**: Relationship strength updates correctly  
**TEST-CULTURAL-DRIFT-001**: Cultural traits drift deterministically  
**TEST-SOCIAL-OBSERVATION-001**: Social events observable  

---

## 7. SUCCESS CRITERIA

- [ ] Reproduction system implemented
- [ ] Offspring created with inherited traits
- [ ] Relationships tracked and observable
- [ ] Cultural inheritance working
- [ ] Social events logged
- [ ] All tests passing
- [ ] No regression from Phase 5

---

## 8. FORBIDDEN ACTIONS

- No asexual reproduction in Phase 6 (asexual only)
- Cannot skip cultural inheritance
- No non-deterministic relationship changes
- Must log all social events

---

## END OF PLAN

**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_6_SOCIAL  
**Timestamp:** 2026-01-11
