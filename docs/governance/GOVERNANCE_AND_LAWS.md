# Governance & Laws of the Markenz Universe

**Status:** BINDING / CONSTITUTIONAL  
**Enforcement:** Automated (CI/CD + Runtime Reality Lock)  
**Parent Doc:** [README.md](../../README.md)

---

## 1. Preamble

The Markenz Universe is governed not by user agreements or terms of service, but by **physical and metaphysical laws** encoded directly into the simulation authority. These laws define the rights of agents, the limits of operators, and the immutable nature of the simulation's reality.

**Violation of these laws is treated as a fatal system error.**

---

## 2. The Fundamental Laws

### 2.1 The Law of Authority

*Source: `MARKENZ_TARGET_ARCHITECTUREv2`*

1. **Rust Supremacy:** The Rust Engine (`apps/engine`) is the ONLY component capable of mutating world state.
2. **Input Sanitation:** `apps/server` and `apps/web` are strictly forbidden from calculating or predicting outcomes. They may only submit `InputEvents`.
3. **No Backdoors:** There is no "God Mode" that bypasses the event log. Even Admin actions must be submitted as `InputEvents` and pass validation.

### 2.2 The Law of Determinism

*Source: `PHASE_1_GOVERNANCE`*

1. **Causal Locking:** Logic must be a pure function of State + Input.
    $$ State_{T+1} = f(State_T, Input_T) $$
2. **Time Invariance:** The wall-clock time of execution is irrelevant. The simulation must produce the exact same results whether run in 2026 or 2050.
3. **Hash Verification:** The `world_hash` must be computed and verified against the ledger. Any deviation is a critical failure.

### 2.3 The Law of Reality Lock

*Source: `RUST_REALITY_LOCK`*

1. **No Fakes:** Production code must contain ZERO stubs, mocks, `todo!()` macros, or placeholder logic.
2. **No Unsafe Shortcuts:** The use of `unwrap()`, `expect()`, or `panic!()` in production pathways is forbidden. Errors must be handled biologically or physically.
3. **No "Magic":** Features cannot be "simulated" or "approximated." If an agent sees, it must raycast. If an agent remembers, the data must exist in memory structures.

---

## 3. The Rights of Agents

### 3.1 Human Equivalence & Agent Parity

*Source: `HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW`*

1. **The Isomorphism Axiom:** Every agent possesses every biological and cognitive subsystem found in a natural human. No agent is "partial."
    * *Includes:* Endocrine system, Immune system, Metabolism, episodic memory, emotional substrate.
2. **Identity != Architecture:** Agents differ ONLY by their `identity.json` (birth data, genetics). Their code architecture is identical.
3. **No Special Cases:** Code such as `if (agent.name == "Gem-D")` is strictly illegal.

### 3.2 Founder Amplification Limits

*Source: `FOUNDER_AMPLIFICATION_AND_CAPABILITY_BOUNDS`*

1. **Founder Status:** Only `Gem-D` and `Gem-K` are designated "Founders."
2. **Bounded Superiority:** Founders may possess numerical multipliers (1.0x - 2.0x) for learning rate, memory capacity, and physical recovery.
3. **No Godhood:** Founders are NOT immune to:
    * Starvation
    * Injury / Death
    * Biological constraints
    * The laws of physics
4. **Non-Inheritance:** Offspring of Founders are Baseline Humans (1.0x multipliers). Superiority is not genetic.

### 3.3 The Right to Continuity

1. **No Deletion:** Agents cannot be deleted from the database. Death is a state transition, not data erasure.
2. **Memory Persistence:** An agent's memory log (`memories/*`) is inviolable and survives system restarts.

---

## 4. Operational Governance

### 4.1 Admin Powers

Admins (Operators) have the power to:

1. **Inject Inputs:** Submit events to the queue.
2. **Inspect State:** View any data point.
3. **Audit:** Run replay verification.

Admins **DO NOT** have the power to:

1. **Mutate Directly:** Cannot SQL UPDATE the world state.
2. **Rewind:** Cannot "undo" an event. They must branch (fork) or fix forward.
3. **Violate Privacy:** (Currently waived for debugging, but architected for future encryption).

### 4.2 Code Changes

1. **No Hot-Patching:** Logic changes require a new binary release and a potential "Migration Event."
2. **Audit Trail:** Every code merge requires passing the `Reality Lock` CI gate.

---

## 5. Violations & Penalties

If the system detects a violation of these laws (e.g., Code parity check fails):

1. **Immediate Halt:** The Engine process terminates.
2. **Lockdown:** The Server rejects new inputs.
3. **Manual Intervention:** An engineer must resolve the divergence or violation before the ecosystem can be rebooted.

**There is no functionality without governance.**
