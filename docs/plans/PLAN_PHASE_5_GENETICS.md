---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_5_GENETICS
phase: 5
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Genetic Traits · Trait Expression · Inheritance · Evolution Mechanics
requires: PLAN_PHASE_4_COGNITION (100% complete)
---

# PLAN PHASE 5: GENETICS
## (Heritable Traits · Genetic Expression · Inheritance · Observable Trait Drift)

**AUDIENCE:** Windsurf executor  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement genetic systems for agent evolution:
- Heritable traits: speed, strength, intelligence, energy efficiency, metabolism
- Genetic loci: discrete genes encoding trait expression
- Trait expression: deterministic phenotype from genotype + environment
- Inheritance: trait replication with mutation during reproduction (Phase 6)
- Observable evolution: trait drift logged as events
- Preservation: genetic markers from Gemini imported at genesis

---

## 2. GENETIC TYPES

### 2.1 Genome (crates/world/src/genetics/genome.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genome {
    pub genes: BTreeMap<String, Allele>,  // Trait name → genetic code
    pub lineage_id: u64,                  // Ancestry identifier
    pub mutation_rate: f64,               // Mutation probability (0.0-1.0)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allele {
    pub locus: String,                    // Gene location (e.g., "SPEED_ALLELE")
    pub dominance: Dominance,
    pub expression_value: f64,            // -1.0 to 1.0 (how much trait is expressed)
    pub parent_origin: ParentOrigin,      // Maternal or paternal
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Dominance {
    Dominant,
    Recessive,
    Codominant,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ParentOrigin {
    Maternal,
    Paternal,
    Genesis,
}

impl Genome {
    pub fn new_genesis(lineage_id: u64) -> Self {
        let mut genes = BTreeMap::new();
        
        genes.insert("SPEED".to_string(), Allele {
            locus: "SPEED_ALLELE".to_string(),
            dominance: Dominance::Codominant,
            expression_value: 0.0,  // Neutral baseline
            parent_origin: ParentOrigin::Genesis,
        });
        
        genes.insert("STRENGTH".to_string(), Allele {
            locus: "STRENGTH_ALLELE".to_string(),
            dominance: Dominance::Dominant,
            expression_value: 0.0,
            parent_origin: ParentOrigin::Genesis,
        });
        
        genes.insert("ENERGY_EFFICIENCY".to_string(), Allele {
            locus: "ENERGY_ALLELE".to_string(),
            dominance: Dominance::Recessive,
            expression_value: 0.0,
            parent_origin: ParentOrigin::Genesis,
        });
        
        Self {
            genes,
            lineage_id,
            mutation_rate: 0.01,  // 1% per reproduction
        }
    }
    
    pub fn express_traits(&self) -> TraitExpression {
        let speed_expr = self.genes.get("SPEED")
            .map(|a| a.expression_value)
            .unwrap_or(0.0);
        
        let strength_expr = self.genes.get("STRENGTH")
            .map(|a| a.expression_value)
            .unwrap_or(0.0);
        
        let efficiency_expr = self.genes.get("ENERGY_EFFICIENCY")
            .map(|a| a.expression_value)
            .unwrap_or(0.0);
        
        TraitExpression {
            speed: 1.0 + speed_expr * 0.5,           // ±50% speed
            strength: 1.0 + strength_expr * 0.3,     // ±30% strength
            energy_efficiency: 1.0 - efficiency_expr * 0.4,  // ±40% lower metabolism
        }
    }
}
```

### 2.2 Trait Expression (crates/world/src/genetics/traits.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraitExpression {
    pub speed: f64,                // Movement speed multiplier
    pub strength: f64,             // Damage/gathering speed multiplier
    pub energy_efficiency: f64,    // Metabolic rate multiplier
}

impl TraitExpression {
    pub fn apply_to_agent(&self, agent: &mut Agent) {
        agent.movement_speed = 1.0 * self.speed;
        agent.bio_state.metabolic_rate = 0.5 * self.energy_efficiency;
        agent.gather_speed = 1.0 * self.strength;
    }
}
```

---

## 3. GENETIC VARIATION

### 3.1 Mutation (crates/world/src/genetics/mutation.rs)

```rust
pub struct MutationEngine;

impl MutationEngine {
    pub fn mutate(
        genome: &mut Genome,
        rng: &mut RngStream,
    ) -> Result<(), String> {
        for (_trait_name, allele) in genome.genes.iter_mut() {
            if rng.next_f64() < genome.mutation_rate {
                // Small random drift to expression value
                let drift = (rng.next_f64() - 0.5) * 0.1;  // ±5% drift
                allele.expression_value = (allele.expression_value + drift)
                    .clamp(-1.0, 1.0);
            }
        }
        Ok(())
    }
}
```

---

## 4. OBSERVABLE GENETIC CHANGES

### 4.1 Trait Evolution Events (crates/world/src/genetics/observation.rs)

```rust
pub fn trait_expression_event(
    tick: u64,
    agent_id: u64,
    before: &TraitExpression,
    after: &TraitExpression,
) -> Option<ObservationEvent> {
    if (before.speed - after.speed).abs() < 0.01
        && (before.strength - after.strength).abs() < 0.01
        && (before.energy_efficiency - after.energy_efficiency).abs() < 0.01 {
        return None;
    }
    
    let payload = json!({
        "type": "trait_expression_changed",
        "agent_id": agent_id,
        "traits": {
            "speed": after.speed,
            "strength": after.strength,
            "energy_efficiency": after.energy_efficiency,
        }
    });
    
    Some(ObservationEvent {
        tick,
        event_type: "trait_expression_changed".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    })
}
```

---

## 5. GENESIS GENETIC IMPORT

### 5.1 Import from Gemini (crates/world/src/genesis/genetics_import.rs)

```rust
pub fn import_genome_from_genesis(
    genesis_data: &GenesisData,
    universe: &mut Universe,
) -> Result<(), String> {
    let agent = universe.agents.get_mut(genesis_data.agent_id)?;
    
    // Import genetic markers exactly as preserved from Gemini
    agent.genome = Genome {
        genes: genesis_data.genetic_traits.clone(),
        lineage_id: genesis_data.lineage_id,
        mutation_rate: genesis_data.mutation_rate.unwrap_or(0.01),
    };
    
    Ok(())
}
```

---

## 6. TEST SUITE

**TEST-GENETICS-001**: Traits correctly expressed from genotype  
**TEST-MUTATION-001**: Mutations deterministic and within bounds  
**TEST-INHERITANCE-001**: Traits inherited in reproduction (Phase 6)  
**TEST-TRAIT-OBSERVATION-001**: Trait changes observable  

---

## 7. SUCCESS CRITERIA

- [ ] Genetic types defined and integrated
- [ ] Trait expression implemented
- [ ] Mutations deterministic
- [ ] Genesis genetics imported
- [ ] Trait changes observable
- [ ] All tests passing
- [ ] No regression from Phase 4

---

## 8. FORBIDDEN ACTIONS

- No real-world Mendelian genetics (deterministic only)
- No non-deterministic mutations
- Cannot skip mutation logging
- Must preserve genesis lineage_id

---

## END OF PLAN

**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_5_GENETICS  
**Timestamp:** 2026-01-11
