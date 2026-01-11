# Determinism & Replay Specification

**Status:** FINAL / PRODUCTION  
**Mechanism:** Cryptographic Hash Chain  
**Parent Doc:** [README.md](../../README.md)

---

## 1. Definition of Determinism

In the Markenz context, **determinism** is defined as bit-exact reproducibility.

**The Golden Equation:**
$$ \forall R \in Runs, \quad State(T) = \text{Apply}(\text{GenesisState}, \text{Inputs}[0..T]) $$

If this equation fails for even a single bit of state, the system is considered **broken**.

---

## 2. Components of Determinism

### 2.1 Fixed Timestep

* **Rate:** 20 Ticks per Second (configurable, fixed at compile/boot).
* **Delta Time (`dt`):** Constant. The Engine never uses "elapsed waltime."
* **Rule:** If the server lags, the simulation slows down. It does not skip ticks.

### 2.2 Deterministic RNG

* **Forbidden:** `rand::thread_rng()`, system time seeding.
* **Required:** `DeterministicRng` (PCG-based or ChaCha20).
* **Seeding Strategy:**
  * **Master Seed:** Defined at Genesis.
  * **Tick Seed:** Derived from `Hash(MasterSeed + TickIndex)`.
  * **Subsystem Streaming:** Each subsystem (Physics, Bio, AI) gets a derived child RNG to prevent "butterfly effects" across domains. (e.g., A physics collision calculation should not shift the RNG stream for an AI decision).

### 2.3 Data Structures

* **Forbidden:** `HashMap`, `HashSet` (iteration order is undefined/randomized by modern HashDoS protection).
* **Required:** `BTreeMap`, `IndexMap`, or sorted vectors for any collection that is iterated over during state calculation.

### 2.4 Floating Point Math

* **Issue:** IEEE 754 floats can vary across CPU architectures/compilers.
* **Solution:** Strict usage of `fixed_point` math or `ordered_float` with software-defined rounding modes for authoritative physics.

---

## 3. The Replay Verification Process

Replay is not just a feature; it is the **primary unit test** of the universe.

### 3.1 The Hashing Strategy

Every simulation tick produces a cryptographic hash of the mutable world state (The `WorldHash`).

**Input to Hash:**

1. Agent States (Health, Position, Hormones).
2. World Inventory.
3. Terrain Modifications.
4. Event Queue State.

**The Chain:**
`Hash(T) = SHA256( Hash(T-1) + DeterministicState(T) )`

### 3.2 Audit Tooling

The `tools/audits` binary performs the verification.

1. **Extract:** Downloads the `input_events` log and the committed `chain_hashes` from Postgres.
2. **Re-Run:** Initializes a headless Engine instance with the same Genesis seed.
3. **Execute:** Feeds inputs into the Engine tick-by-tick.
4. **Compare:** For every tick $T$, compares `CalculatedHash` vs `CommittedHash`.
5. **Verdict:**
    * **PASS:** All hashes match.
    * **FAIL:** Divergence detected at Tick $X$.

---

## 4. Failure Protocol

**What happens if Determinism breaks?**

1. **Alert:** CI/Production monitors scream.
2. **Halt:** The divergent branch is rejected.
3. **Diagnosis:**
    * Use binary-search replay to find the exact Tick of divergence.
    * Enable verbose "Diff Logging" to compare the state JSON of the Authority vs the Replay.
    * Identify the nondeterministic code (e.g., iteration over a generic Hash Map).
4. **Remediation:** Fix code, rebuild Engine.

---

## 5. What Constitutes a Failure?

The following are explicit failures of this specification:

* Replay produces a different Agent ID UUID.
* An Agent chooses to "Eat" in Run A but "Sleep" in Run B given identical inputs.
* Physics settling position varies by > 0.0001 units.
* RNG stream misalignment.

---

## 6. Audit Process for Contributors

Before submitting ANY PR that touches `apps/engine` or `crates/*`:

1. Run `./tools/ci/verify_determinism.sh`.
2. This script runs a short scenario (1000 ticks) twice.
3. It asserts $Hash_{Run1} == Hash_{Run2}$.
4. If this fails, **do not open the PR.**

Determinism is the bedrock of our reality.
