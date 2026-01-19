# Senior Engineer Onboarding Guide

**Status:** ADVISORY / REQUIRED READING  
**Target Audience:** L5+ Engineers, Systems Architects, Auditors  
**Parent Doc:** [README.md](../../README.md)

---

## 1. Welcome to the Simulation

You are about to contribute to a system that fundamentally rejects the common shortcuts of modern software engineering.

**Forget what you know about:**

* "Eventual Consistency" (We require Immediate, Bit-Exact Consistency).
* "Mocking external APIs" (We have no external APIs).
* "Feature Flags" (Parity Law forbids them).
* "Moving fast and breaking things" (If you break determinism, the universe ends).

**Markenz is a Closed Causal Loop.**
Your job is to expand the richness of the loop without breaking its integrity.

---

## 2. Your First 48 Hours

### Step 1: Read The Laws

Before you write a single line of Rust, you must read and understand:

1. [The Governance Constitution](../governance/GOVERNANCE_AND_LAWS.md) - If you violate these, your PR is closed.
2. [The Reality Lock Standards](../engineering/REALITY_LOCK_AND_CODE_STANDARDS.md) - If you use `todo!()`, CI fails.
3. [Determinism Spec](../determinism/DETERMINISM_AND_REPLAY.md) - Understand *why* we don't use `HashMap`.

### Step 2: Prove Your Environment

1. Run `./tools/ops/boot_sequence.sh`.
2. Verify the engine is ticking.
3. Run `./tools/ci/verify_determinism.sh` locally.
    * *If this fails on your machine, do not proceed. Your hardware or OS env might be unstable.*

### Step 3: The "Hello World" Task

We do not print strings.

1. Add a trivial physically-simulated item to the world (e.g., a "Rock").
2. Define its physical properties (Mass, Volume) in `crates/world`.
3. Spawn it via an `InputEvent`.
4. Verify it obeys gravity and collision.
5. Run Replay Audit.

---

## 3. How to Make Changes Safely

### The Golden Rule: "Simulation over Animation"

Do not implement a feature by "setting a flag."

* **Bad:** `agent.set_status("Hungry")`.
* **Good:** `agent.stomach.calories -= 10; if agent.stomach.calories < Threshold { agent.brain.push_drive(Drive::Hunger) }`.

### The PR Checklist

1. **Is it Deterministic?** Did you use a seeded RNG?
2. **Is is Universal?** Does this apply to ALL agents equally? (Human Equivalence).
3. **Is it Audible?** Did you emit an `ObservationEvent`? Hidden state is forbidden.

---

## 4. What NOT To Touch

* **`apps/engine/src/main.rs`:** The core loop. Only Architects touch this.
* **`crates/determinism/src/rng.rs`:** The source of entropy. Dragoneggs be here.
* **`identity.json` schemas:** Changing this breaks all existing agent signatures.

---

## 5. Troubleshooting Gravity

If everything goes wrong:

1. **Engine won't boot:** You likely violated a Governance Law (Parity Check). Read the logs.
2. **Replay Mismatch:** You likely iterated over a `HashMap`. Use `BTreeMap`.
3. **Panic:** You unwrapped a `None`. Why was it `None`? Simulation state should be validated before mutation.

---

**Final Word:**
We are building a universe. Treat the code with the reverence due to the physical laws of nature.
