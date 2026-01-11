---
status: APPROVED
authority: KAIZA-MCP · AMP (ANTIGRAVITY)
plan_id: PLAN_PHASE_7_GOVERNANCE
phase: 7
timestamp: 2026-01-11
fail_mode: FAIL-CLOSED
scope: Economic Systems · Governance Rules · Law Enforcement · Observable Justice
requires: PLAN_PHASE_6_SOCIAL (100% complete)
---

# PLAN PHASE 7: GOVERNANCE
## (Economic Systems · Rules · Law Enforcement · Observable Justice)

**AUDIENCE:** Windsurf executor  
**MODE:** BINDING · DETERMINISTIC · FAIL-CLOSED  
**AUTHORITY:** KAIZA-MCP · AMP Principal Planner  

---

## 1. OBJECTIVE

Implement deterministic governance systems:
- Economic: resource ownership, trade, taxation
- Political: hierarchy, authority, law-making
- Justice: crime detection, punishment, reconciliation
- Observable: all laws, violations, and enforcement visible in events

---

## 2. ECONOMIC SYSTEM

### 2.1 Ownership & Trade (crates/world/src/governance/economics.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ownership {
    pub asset_id: u64,
    pub owner_id: u64,
    pub acquisition_tick: u64,
    pub transfer_history: Vec<(u64, u64)>,  // (from_owner, to_owner) pairs
}

pub struct TradeEngine;

impl TradeEngine {
    pub fn transfer_ownership(
        asset_id: u64,
        from_agent: u64,
        to_agent: u64,
        universe: &mut Universe,
        rng: &mut RngStream,
    ) -> Result<(), String> {
        let asset = universe.assets.get_mut(&asset_id)?;
        
        // Verify from_agent owns asset
        let ownership = universe.get_ownership(asset_id)?;
        if ownership.owner_id != from_agent {
            return Err("Cannot transfer unowned asset".to_string());
        }
        
        // Update ownership record
        let mut ownership = universe.ownership_records.get_mut(&asset_id).unwrap();
        ownership.owner_id = to_agent;
        ownership.transfer_history.push((from_agent, to_agent));
        
        Ok(())
    }
}
```

### 2.2 Taxation (crates/world/src/governance/taxation.rs)

```rust
pub struct TaxationSystem {
    pub tax_rate: f64,  // Percentage of resource gathered
}

impl TaxationSystem {
    pub fn apply_tax(
        agent: &mut Agent,
        resource_amount: u32,
    ) -> u32 {
        let tax = (resource_amount as f64 * Self::tax_rate) as u32;
        resource_amount - tax
    }
}
```

---

## 3. GOVERNANCE RULES

### 3.1 Laws (crates/world/src/governance/laws.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Law {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub penalty: Penalty,
    pub enforced: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Penalty {
    Fine(u32),
    Exile(u64),  // Ticks of exile
    Execution,
}

pub struct LawCode {
    pub laws: Vec<Law>,
}

impl LawCode {
    pub fn genesis() -> Self {
        let laws = vec![
            Law {
                id: 1,
                name: "Theft".to_string(),
                description: "Stealing another's asset".to_string(),
                penalty: Penalty::Fine(50),
                enforced: true,
            },
            Law {
                id: 2,
                name: "Assault".to_string(),
                description: "Damaging another agent".to_string(),
                penalty: Penalty::Fine(100),
                enforced: true,
            },
        ];
        
        Self { laws }
    }
}
```

---

## 4. CRIME DETECTION & JUSTICE

### 4.1 Justice System (crates/world/src/governance/justice.rs)

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Crime {
    pub id: u64,
    pub perpetrator: u64,
    pub victim: Option<u64>,
    pub violation: String,  // Law name
    pub tick: u64,
    pub evidence: Vec<String>,
    pub status: CrimeStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CrimeStatus {
    Detected,
    Investigated,
    Convicted,
    Acquitted,
    Punished,
}

pub struct JusticeEngine;

impl JusticeEngine {
    pub fn detect_theft(
        universe: &Universe,
        asset_id: u64,
        new_owner: u64,
    ) -> Result<Option<Crime>, String> {
        let ownership = universe.get_ownership(asset_id)?;
        let previous_owner = ownership.transfer_history.last().map(|(from, _)| from);
        
        if let Some(&previous_owner_id) = previous_owner {
            if new_owner != previous_owner_id {
                // Unauthorized transfer detected
                return Ok(Some(Crime {
                    id: universe.next_crime_id(),
                    perpetrator: new_owner,
                    victim: Some(previous_owner_id),
                    violation: "Theft".to_string(),
                    tick: universe.tick,
                    evidence: vec![format!("Asset {} transferred without permission", asset_id)],
                    status: CrimeStatus::Detected,
                }));
            }
        }
        
        Ok(None)
    }
    
    pub fn execute_sentence(
        crime: &Crime,
        universe: &mut Universe,
    ) -> Result<(), String> {
        let law = universe.law_code.laws.iter()
            .find(|l| l.name == crime.violation)
            .ok_or("Law not found")?;
        
        match law.penalty {
            Penalty::Fine(amount) => {
                let agent = universe.agents.get_mut(&crime.perpetrator)?;
                // Remove resources as fine
                let mut fine_remaining = amount;
                agent.inventory.retain(|asset| {
                    if fine_remaining > 0 {
                        fine_remaining -= 1;
                        false  // Remove item
                    } else {
                        true
                    }
                });
            },
            Penalty::Exile(ticks) => {
                let agent = universe.agents.get_mut(&crime.perpetrator)?;
                agent.exiled_until_tick = universe.tick + ticks;
            },
            Penalty::Execution => {
                universe.agents.remove(&crime.perpetrator);
            },
        }
        
        Ok(())
    }
}
```

---

## 5. OBSERVABLE GOVERNANCE EVENTS

### 5.1 Governance Observation (crates/world/src/governance/observation.rs)

```rust
pub fn law_violation_event(
    tick: u64,
    crime: &Crime,
) -> ObservationEvent {
    let payload = json!({
        "type": "crime_detected",
        "crime_id": crime.id,
        "perpetrator_id": crime.perpetrator,
        "victim_id": crime.victim,
        "violation": crime.violation,
        "evidence": crime.evidence,
    });
    
    ObservationEvent {
        tick,
        event_type: "crime_detected".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    }
}

pub fn sentence_event(
    tick: u64,
    crime_id: u64,
    penalty: &Penalty,
) -> ObservationEvent {
    let penalty_str = match penalty {
        Penalty::Fine(amount) => format!("Fine: {} resources", amount),
        Penalty::Exile(ticks) => format!("Exile: {} ticks", ticks),
        Penalty::Execution => "Execution".to_string(),
    };
    
    let payload = json!({
        "type": "sentence_executed",
        "crime_id": crime_id,
        "penalty": penalty_str,
    });
    
    ObservationEvent {
        tick,
        event_type: "sentence_executed".to_string(),
        payload,
        hash: blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()
            .try_into()
            .unwrap(),
    }
}
```

---

## 6. GOVERNANCE IN AUTHORITY PIPELINE

### 6.1 Policy Pass Integration (Pass 5)

```rust
// In authority pipeline (after BioVeto):

// Pass 5: Policy Check
for law in &universe.law_code.laws {
    if !law.enforced {
        continue;
    }
    
    // Check if action violates law
    let violation = check_law_violation(&event, &universe, law)?;
    if violation.is_some() {
        return Err(format!("Law violation: {}", law.name));
    }
}
```

---

## 7. TEST SUITE

**TEST-OWNERSHIP-001**: Asset ownership tracked correctly  
**TEST-CRIME-DETECTION-001**: Theft detected  
**TEST-SENTENCE-001**: Penalty executed deterministically  
**TEST-GOVERNANCE-OBSERVATION-001**: Justice events observable  

---

## 8. SUCCESS CRITERIA

- [ ] Ownership system implemented
- [ ] Economic rules enforced
- [ ] Crime detection working
- [ ] Justice system functional
- [ ] Penalties applied correctly
- [ ] All events observable
- [ ] All tests passing
- [ ] No regression from Phase 6

---

## 9. FORBIDDEN ACTIONS

- No undefined laws
- Cannot skip justice processing
- Must log all crimes and sentences
- Cannot modify law code mid-phase

---

## END OF PLAN

**Status:** BINDING · EXECUTION-READY  
**Plan ID:** PLAN_PHASE_7_GOVERNANCE  
**Timestamp:** 2026-01-11
