# MARKENZ SELF-EVOLUTION & SELF-GROWTH GOVERNING LAW v2

STATUS: GOVERNING · BINDING · MERGE-BLOCKING  
SCOPE: Applies to **all phases**, **all subsystems**, **all future work**  
PRECEDENCE: Equal authority with the other four governing documents  
ENFORCEMENT: AMP + CI + determinism replay

---

## 1. Purpose (Why This Exists)

Markenz is not a static simulation.

It is designed to:
- **grow** in scale, population, knowledge, and complexity
- **evolve** in behavior, culture, strategy, and society
- do so **without ever breaking determinism, auditability, or authority boundaries**

This document defines **how Markenz is allowed to evolve** and **what is permanently forbidden**.

---

## 2. Core Definitions (Locked)

### Self-Growing
Quantitative expansion of the universe over time.

Includes:
- Population increase (births, deaths, migration)
- Spatial expansion (new terrain chunks)
- Resource accumulation and depletion
- Infrastructure and economic scale
- Knowledge accumulation (skills, memories, cultural metrics)

Self-growth **adds state**.  
It does **not** change the rules.

---

### Self-Evolving
Qualitative change in behavior and structure **within fixed laws**.

Includes:
- Behavioral strategy shifts
- Habit formation and decay
- Cultural drift and norm emergence
- Skill specialization and abstraction
- Governance preferences changing over time

Self-evolution **changes parameters and distributions**, never the engine code.

---

## 3. Absolute Prohibitions (Non-Negotiable)

The system is **never allowed** to:

- ❌ Modify its own source code
- ❌ Modify determinism rules
- ❌ Modify authority boundaries
- ❌ Create new rules at runtime
- ❌ Bypass governance or law enforcement
- ❌ Perform unlogged or unhashable adaptation

Any violation is a **hard failure**.

---

## 4. Where Evolution Is Allowed to Live

All self-growth and self-evolution logic MUST live inside the **Rust authority**:

- `apps/engine`
- `crates/world`
- `crates/biology`
- `crates/genetics`
- `crates/cognition`
- `crates/events`

No evolution logic is permitted in:
- `apps/server`
- `apps/web`
- external services
- LLMs or plugins

---

## 5. Self-Growing Systems (Authoritative)

### 5.1 Population Growth
- Births via reproduction pipeline
- Death via biology, environment, and governance
- Migration via deterministic world expansion

All population changes:
- emit events
- affect census and lineage
- alter `world_hash`
- replay identically

---

### 5.2 Spatial Growth (World Expansion)
- World is **unbounded**
- Terrain chunks generated lazily
- Generation function uses:
  - Markenz root seed
  - chunk coordinates
  - deterministic biome + climate rules

Result:
- Infinite world
- Zero nondeterminism
- Identical expansion on replay

---

### 5.3 Knowledge & Capability Growth
- Skill trees expand procedurally
- Memories accumulate and decay
- Cultural knowledge propagates socially
- Technology emerges via:
  - discovery events
  - diffusion events
  - adoption thresholds

No unlocks by fiat.

---

## 6. Self-Evolution Mechanism (Critical)

### 6.1 Evolution Is State, Not Code
Evolution is implemented as **bounded parameter drift**.

Examples:
- Aggression thresholds
- Risk tolerance
- Planning horizon depth
- Trust decay rates
- Norm enforcement strength

All parameters:
- numeric
- bounded
- logged
- reversible via replay

---

### 6.2 Selection Pressure Model
Agents experience deterministic pressure from:
- biology (survival, reproduction)
- society (status, trust, punishment)
- environment (scarcity, danger)
- governance (laws, penalties)

Successful strategies:
- strengthen
- propagate genetically or socially

Unsuccessful strategies:
- weaken
- disappear naturally

No stochastic shortcut is allowed.

---

### 6.3 Cultural Evolution
Culture is modeled as:
- weighted norms
- shared expectations
- enforcement likelihoods

Culture evolves via:
- imitation
- punishment
- success bias
- generational transmission

Culture **cannot override law**.  
Law **can be influenced by culture** via governance.

---

## 7. Cognition Evolution (Without LLM)

### Allowed
- Habit strength adjustment
- Planning heuristic tuning
- Memory weighting changes
- Language phrasing preferences
- Concept clustering and abstraction growth

### Forbidden
- Runtime grammar rule creation
- Self-modifying planners
- Unbounded rule synthesis
- LLM-driven authority decisions

Cognition evolves **inside fixed templates**.

---

## 8. Governance of Evolution

### Meta-Rule
> Evolution itself is governed.

- Societies may suppress behaviors
- Laws may constrain strategies
- Penalties may alter evolutionary paths
- Cultural pressure may override individual preference

Admin powers:
- Observe evolution
- Inject lawful InputEvents
- Change policies via governance

Admins **cannot**:
- Directly modify evolutionary state
- Edit traits, habits, or culture values

---

## 9. Determinism Guarantees

Self-evolution does not weaken determinism because:

- All adaptations are pure state transitions
- All transitions are tick-indexed
- All randomness uses deterministic RNG
- All changes emit events
- All changes affect `world_hash`

Replay reproduces evolution **exactly**.

---

## 10. Transparency & Observability

Every evolutionary process must expose:

- Trait drift timelines
- Strategy fitness metrics
- Cultural norm weights
- Selection pressure sources
- Generational changes

Nothing evolves invisibly.

---

## 11. AMP Gate Additions (Binding)

This document adds the following **mandatory gates**:

- [ ] No self-modifying code
- [ ] All evolution logged and hashed
- [ ] Replay reproduces identical evolution
- [ ] No unbounded parameter growth
- [ ] Admin cannot directly edit evolution state
- [ ] Evolution metrics visible in UI

Failure of any gate = **STOP**.

---

## 12. Final Lock

Markenz is:
- alive
- adaptive
- growing
- evolving

But it remains:
- deterministic
- inspectable
- lawful
- replayable
- governed

This document ensures Markenz **evolves like a world**,  
not like an unbounded AI.

---

END OF GOVERNING LAW
