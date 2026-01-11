---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_3_BIOLOGY
phase: 3
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Embodied Biology · Metabolism · BioVeto Authority · Observable State
requires: PLAN_PHASE_2_WORLD (100% complete)
---

# PLAN PHASE 3: BIOLOGY
## (Embodied Metabolism · Bio-Veto Authority · Observable Health States)

**AUDIENCE:** Windsurf executor (direct execution only)  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement embodied biological systems for agents with deterministic metabolism, energy constraints, and health states:
- Energy depletion through actions (movement, gathering, crafting)
- Metabolic baseline consumption per tick
- BioVeto pass in authority pipeline (actions fail if insufficient energy/health)
- Health states: hunger, exhaustion, illness (preserved from Gemini)
- Observable bio-state changes in ObservationEvents
- Deterministic health progression based on actions and time

---

## 2. ENTRY CONDITIONS

- Phase 2 complete and signed off by AMP auditor
- All Phase 0/1/2 tests passing
- Asset system and action validation working
- RNG subsystems available (Biology, Genetics)

---

## 3. BIOLOGICAL STATE TYPES

### 3.1 Bio State (crates/world/src/bio/state.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioState {
    pub energy: f64,                    // 0-100 (joules-like units)
    pub hunger: f64,                    // 0-100 (100 = starving)
    pub exhaustion: f64,                // 0-100 (100 = collapsed)
    pub health: f64,                    // 0-100 (100 = perfect)
    pub metabolic_rate: f64,            // Rate of energy consumption per tick
    pub recovery_rate: f64,             // Recovery from exhaustion when idle
}

impl BioState {
    pub fn new() -> Self {
        Self {
            energy: 100.0,
            hunger: 0.0,
            exhaustion: 0.0,
            health: 100.0,
            metabolic_rate: 0.5,           // Energy per tick
            recovery_rate: 0.1,            // Exhaustion recovery per tick at rest
        }
    }
    
    pub fn can_perform_action(&self, action: &InputEventPayload) -> Result<(), String> {
        let energy_cost = self.action_cost(action);
        
        if self.energy < energy_cost {
            return Err("Insufficient energy".to_string());
        }
        
        if self.exhaustion > 80.0 && self.is_intensive(action) {
            return Err("Too exhausted for intensive action".to_string());
        }
        
        if self.health < 20.0 {
            return Err("Too injured to act".to_string());
        }
        
        Ok(())
    }
    
    fn action_cost(&self, action: &InputEventPayload) -> f64 {
        match action {
            InputEventPayload::Move { .. } => 2.0,
            InputEventPayload::Gather { .. } => 3.0,
            InputEventPayload::Mine => 5.0,
            InputEventPayload::Build { .. } => 10.0,
            InputEventPayload::Craft { .. } => 2.0,
            InputEventPayload::Chat { .. } => 0.0,
        }
    }
    
    fn is_intensive(&self, action: &InputEventPayload) -> bool {
        matches!(action, 
            InputEventPayload::Mine | InputEventPayload::Build { .. }
        )
    }
    
    pub fn consume_energy(&mut self, amount: f64) {
        self.energy = (self.energy - amount).max(0.0);
        self.exhaustion = (self.exhaustion + amount * 0.1).min(100.0);
    }
    
    pub fn tick_metabolism(&mut self) {
        self.energy = (self.energy - self.metabolic_rate).max(0.0);
        
        if self.energy < 30.0 {
            self.hunger = ((self.hunger + 1.0).min(100.0));
        } else if self.hunger > 0.0 {
            self.hunger = (self.hunger - 0.5).max(0.0);
        }
    }
    
    pub fn apply_recovery(&mut self) {
        if self.exhaustion > 0.0 {
            self.exhaustion = (self.exhaustion - self.recovery_rate).max(0.0);
        }
    }
}
```

### 3.2 Health Events (crates/world/src/bio/health.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HealthEvent {
    EnergyDepleted,
    Exhausted,
    Starving,
    Injured(f64),  // damage amount
    Recovered,
}

pub struct HealthMonitor;

impl HealthMonitor {
    pub fn check_health(
        agent: &Agent,
        rng: &mut RngStream,
    ) -> Option<HealthEvent> {
        if agent.bio_state.energy == 0.0 {
            return Some(HealthEvent::EnergyDepleted);
        }
        
        if agent.bio_state.exhaustion > 95.0 {
            return Some(HealthEvent::Exhausted);
        }
        
        if agent.bio_state.hunger > 90.0 {
            // Starving: take health damage
            let damage = rng.next_f64() * 5.0;
            if agent.bio_state.health - damage <= 0.0 {
                return Some(HealthEvent::Injured(damage));
            }
        }
        
        None
    }
}
```

---

## 4. BIO-VETO AUTHORITY PASS

### 4.1 Authority Pipeline Integration (apps/engine/src/authority_pipeline.rs)

**Pass 3 (Bio-Veto) - Non-Negotiable Position:**

```rust
// In process_tick(), after Pass 2 (RBAC):

for event in input_events {
    // Pass 1: InputEvent Validation
    event.validate()?;
    
    // Pass 2: RBAC
    verify_rbac(&event)?;
    
    // ============ PASS 3: BIO-VETO (NEW) ============
    let agent = universe.agents.get_mut(event.source_agent_id)?;
    
    // Check if action is biologically feasible
    agent.bio_state.can_perform_action(&event.payload)?;
    
    // Action passes bio-veto; continue to Pass 4
    
    // ... Passes 4-10 (unchanged from Phase 2) ...
}
```

**Behavior:** If agent lacks energy/health for action, the action is **rejected entirely** and does NOT proceed to physics/policy checks.

### 4.2 BioVeto as State Machine

```rust
pub enum BioVetoResult {
    Allowed,
    InsufficientEnergy { need: f64, have: f64 },
    ExhaustedButAllowed,  // Intensive action rejected; passive allowed
    InjuredButAllowed,    // Health > 0; can still act
    Blocked(String),      // Cannot act
}

pub fn bio_veto(
    agent: &Agent,
    action: &InputEventPayload,
) -> BioVetoResult {
    // Check energy
    let cost = bio_cost(action);
    if agent.bio_state.energy < cost {
        return BioVetoResult::InsufficientEnergy {
            need: cost,
            have: agent.bio_state.energy,
        };
    }
    
    // Check exhaustion blocks intensive actions
    if agent.bio_state.exhaustion > 90.0 {
        if bio_is_intensive(action) {
            return BioVetoResult::Blocked("Too exhausted".to_string());
        }
    }
    
    // Check health
    if agent.bio_state.health < 10.0 {
        return BioVetoResult::Blocked("Critical injury".to_string());
    }
    
    BioVetoResult::Allowed
}
```

---

## 5. METABOLIC SYSTEMS

### 5.1 Per-Tick Metabolism (crates/world/src/bio/metabolism.rs)

```rust
pub struct MetabolicProcessor;

impl MetabolicProcessor {
    pub fn process_tick(
        agent: &mut Agent,
        environment: &Environment,
        rng: &mut RngStream,
    ) -> Result<Vec<HealthEvent>, String> {
        let mut events = Vec::new();
        
        // Baseline metabolism
        agent.bio_state.tick_metabolism();
        
        // Environmental effects
        if environment.is_cold {
            agent.bio_state.metabolic_rate *= 1.2;  // Higher metabolism in cold
        }
        
        if environment.temperature > 40.0 {
            agent.bio_state.hunger += 0.5;  // Heat causes dehydration
        }
        
        // Recovery if idle
        if !agent.is_performing_action() {
            agent.bio_state.apply_recovery();
        }
        
        // Health monitoring
        if let Some(event) = HealthMonitor::check_health(agent, rng) {
            events.push(event);
        }
        
        Ok(events)
    }
}
```

### 5.2 Food & Eating (crates/world/src/bio/nutrition.rs)

```rust
pub struct NutritionSystem;

impl NutritionSystem {
    pub fn eat(
        agent: &mut Agent,
        food_name: &str,
        quantity: u32,
    ) -> Result<(), String> {
        // Remove food from inventory
        let mut removed = 0u32;
        agent.inventory.retain(|asset| {
            if asset.name == food_name && removed < quantity {
                removed += 1;
                false  // Remove this item
            } else {
                true
            }
        });
        
        if removed < quantity {
            return Err("Not enough food".to_string());
        }
        
        // Restore energy
        let energy_restored = quantity as f64 * 20.0;  // 20 energy per food item
        agent.bio_state.energy = (agent.bio_state.energy + energy_restored).min(100.0);
        agent.bio_state.hunger = (agent.bio_state.hunger - quantity as f64 * 10.0).max(0.0);
        
        Ok(())
    }
}
```

---

## 6. OBSERVABLE BIO-STATE CHANGES

### 6.1 Bio Event Observation (crates/world/src/bio/observation.rs)

```rust
pub fn bio_state_to_observation(
    tick: u64,
    agent_id: u64,
    bio_before: &BioState,
    bio_after: &BioState,
) -> Option<ObservationEvent> {
    // Only emit if state changed
    if (bio_before.energy - bio_after.energy).abs() < 0.01
        && (bio_before.hunger - bio_after.hunger).abs() < 0.01
        && (bio_before.exhaustion - bio_after.exhaustion).abs() < 0.01
        && (bio_before.health - bio_after.health).abs() < 0.01 {
        return None;
    }
    
    let payload = json!({
        "type": "bio_state_changed",
        "agent_id": agent_id,
        "energy": {
            "before": bio_before.energy,
            "after": bio_after.energy,
            "delta": bio_after.energy - bio_before.energy
        },
        "hunger": {
            "before": bio_before.hunger,
            "after": bio_after.hunger,
        },
        "exhaustion": {
            "before": bio_before.exhaustion,
            "after": bio_after.exhaustion,
        },
        "health": {
            "before": bio_before.health,
            "after": bio_after.health,
        }
    });
    
    Some(ObservationEvent {
        tick,
        event_type: "bio_state_changed".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    })
}
```

---

## 7. GENESIS BIO-STATE PRESERVATION

### 7.1 Import from Gemini (crates/world/src/genesis/bio_import.rs)

```rust
pub fn import_bio_state_from_genesis(
    genesis_data: &GenesisData,
    universe: &mut Universe,
) -> Result<(), String> {
    let agent = universe.agents.get_mut(genesis_data.agent_id)?;
    
    // Preserve Gemini bio-state exactly
    agent.bio_state = BioState {
        energy: genesis_data.energy.unwrap_or(100.0),
        hunger: genesis_data.hunger.unwrap_or(0.0),
        exhaustion: genesis_data.exhaustion.unwrap_or(0.0),
        health: genesis_data.health.unwrap_or(100.0),
        metabolic_rate: genesis_data.metabolic_rate.unwrap_or(0.5),
        recovery_rate: genesis_data.recovery_rate.unwrap_or(0.1),
    };
    
    Ok(())
}
```

---

## 8. TEST SUITE

### 8.1 Bio-Veto Tests

**TEST-BIO-VETO-001: Energy Check**
```rust
#[test]
fn test_bio_veto_energy() {
    let mut agent = Agent::default();
    agent.bio_state.energy = 1.0;  // Nearly depleted
    
    let action = InputEventPayload::Mine;  // Costs 5.0 energy
    
    assert!(agent.bio_state.can_perform_action(&action).is_err());
}
```

**TEST-BIO-VETO-002: Exhaustion Blocks Intensive**
```rust
#[test]
fn test_bio_veto_exhaustion() {
    let mut agent = Agent::default();
    agent.bio_state.exhaustion = 95.0;  // Nearly collapsed
    
    let intensive_action = InputEventPayload::Mine;
    assert!(agent.bio_state.can_perform_action(&intensive_action).is_err());
    
    let passive_action = InputEventPayload::Chat { text: "hello".to_string() };
    assert!(agent.bio_state.can_perform_action(&passive_action).is_ok());
}
```

### 8.2 Metabolism Tests

**TEST-METABOLISM-001: Baseline Consumption**
```rust
#[test]
fn test_baseline_metabolism() {
    let mut agent = Agent::default();
    let initial_energy = agent.bio_state.energy;
    
    agent.bio_state.tick_metabolism();
    
    assert!(agent.bio_state.energy < initial_energy);
    assert_eq!(agent.bio_state.energy, initial_energy - 0.5);
}
```

### 8.3 Action Cost Tests

**TEST-ACTION-COST-001: Energy Deduction**
```rust
#[test]
fn test_action_energy_cost() {
    let mut agent = Agent::default();
    let initial = agent.bio_state.energy;
    
    agent.bio_state.consume_energy(3.0);  // Gather cost
    
    assert_eq!(agent.bio_state.energy, initial - 3.0);
    assert!(agent.bio_state.exhaustion > 0.0);
}
```

---

## 9. SUCCESS CRITERIA (ALL REQUIRED)

### Build & Compilation
- [ ] All bio crates compile
- [ ] `cargo test --all` passes
- [ ] Zero clippy warnings

### BioVeto Authority
- [ ] **TEST-BIO-VETO-001** passing (energy check)
- [ ] **TEST-BIO-VETO-002** passing (exhaustion blocks intensive)
- [ ] BioVeto pass in authority pipeline working

### Metabolism
- [ ] **TEST-METABOLISM-001** passing (baseline consumption)
- [ ] Per-tick metabolism applied consistently
- [ ] Hunger increases when energy low

### Action Costs
- [ ] **TEST-ACTION-COST-001** passing (energy deduction)
- [ ] All action costs correctly applied
- [ ] Exhaustion increases with action intensity

### Observations
- [ ] BioState changes produce ObservationEvents
- [ ] Energy, hunger, exhaustion, health observable
- [ ] Bio-state changes hash correctly

### Genesis Preservation
- [ ] Gem-D bio-state imported correctly
- [ ] Gem-K bio-state imported correctly
- [ ] Bio-state persists across snapshots

### Regression
- [ ] All Phase 0/1/2 tests passing
- [ ] Determinism maintained

---

## 10. FORBIDDEN ACTIONS

Windsurf MUST NOT:

1. Skip BioVeto pass in authority pipeline
2. Implement mock energy/exhaustion checks
3. Allow actions when energy insufficient (must reject)
4. Use wall-clock time for metabolism
5. Create TODO/FIXME in bio-veto code
6. Implement bio-state without observable events
7. Lose bio-state during snapshot/replay
8. Add randomness outside RNG subsystems for bio

---

## 11. HARD STOP CONDITIONS

Execution STOPS if:

1. BioVeto pass skipped or stub
2. Energy check not enforced
3. Bio-state changes not observable
4. Regression in earlier phases
5. Bio-state lost during snapshot
6. Non-deterministic energy calculations

---

## 12. PHASE 3 EXIT CHECKLIST

Phase 3 is DONE when ALL TRUE:

- [ ] BioVeto tests passing
- [ ] Metabolism tests passing
- [ ] Action cost tests passing
- [ ] Observation tests passing
- [ ] Genesis bio-state preserved
- [ ] All earlier phase tests passing
- [ ] AMP sign-off obtained

---

## END OF PLAN

**Authority:** KAIZA-MCP · AMP Principal Planner  
**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_3_BIOLOGY  
**Timestamp:** 2026-01-11
