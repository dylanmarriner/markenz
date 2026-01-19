# Repository Structure & Ownership Model

**Status:** FINAL  
**Parent Doc:** [README.md](../../README.md)

---

## 1. Directory Tree

The Markenz monorepo uses a strict workspace structure.

```text
markenz/
├── apps/                  # Deployable Applications
│   ├── engine/            # [RUST] Authoritative Simulation Binary (The Kernel)
│   ├── server/            # [TS] Control Plane & Event Gateway
│   └── web/               # [REACT] Visualization & Inspector
├── crates/                # [RUST] Shared Libraries & Logic Modules
│   ├── biology/           # Metabolic, Endocrine, Immune systems
│   ├── cognition/         # Planning, Memory, Drives, Language
│   ├── genetics/          # Genome, Inheritance, Mutation
│   ├── physics/           # Fixed-point Physics, Collision, Spatial Grid
│   ├── world/             # World State, Entities, Inventory
│   ├── shared-types/      # Types shared between Engine and Tools
│   └── determinism/       # RNG, Hashing, Replay Harness
├── docs/                  # [MD] Canonical Documentation (You are here)
├── infra/                 # Infrastructure as Code
│   ├── postgres/          # DB Schemas & Init
│   └── keycloak/          # Identity Config
├── tools/                 # Operational Utilities
│   ├── audits/            # Replay Verification & PDF Generation
│   └── ci/                # CI Scripts & Guardrails
├── Dockerfile             # Multi-stage build definition
└── Cargo.toml             # Workspace Root
```

---

## 2. Ownership & Boundaries

### 2.1 The Iron Boundary (`apps/engine`)

* **Owner:** Core Simulation Team.
* **Rules:**
  * Imports ONLY from `crates/*`.
  * NEVER imports network libraries (no `reqwest`, `tokio-net`).
  * NEVER imports system time (`std::time`).
  * **Strictly CPU-bound.**

### 2.2 The Biology Domain (`crates/biology`, `crates/genetics`)

* **Owner:** Biological Modeling Team.
* **Responsibility:** Enforce Human Equivalence.
* **Rules:**
  * Must model real physiological pathways.
  * Logic here creates the "BioVeto".

### 2.3 The Cognition Domain (`crates/cognition`)

* **Owner:** AI/Cognition Team.
* **Responsibility:** Model the Mind.
* **Rules:**
  * Deterministic planning (GOAP/HTN).
  * No calls to external LLMs.
  * Inner monologue generation.

### 2.4 The Web Domain (`apps/web`, `apps/server`)

* **Owner:** Frontend/Platform Team.
* **Responsibility:** PX (Player Experience) & Ops.
* **Rules:**
  * Read-only view of the simulation.
  * Cannot dictate simulation logic.

---

## 3. Module propagation

### 3.1 Crate Dependency Graph

To preserve compile times and logical separation:

```text
engine -> (world, physics, biology, cognition, genetics)
biology -> (genetics, shared-types)
cognition -> (world, biology, shared-types)
physics -> (shared-types)
world -> (physics, shared-types)
```

**Circular Dependencies are forbidden.**

### 3.2 Shared Types Strategy

* `crates/shared-types` contains the canonical `InputEvent`, `ObservationEvent`, and `AgentID` definitions.
* This crate is compiled to WASM for use in `apps/server` (if using Rust-based validation) or strictly synced via JSON Schema generation.

---

## 4. Forbidden Actions

1. **No Logic in Apps:** Do not write simulation logic in `apps/engine/main.rs`. Put it in a `crate`. The app is just a runner.
2. **No "Utils" Crates:** `crates/utils` is a smell. Use specific domain crates.
3. **No Leaky Abstractions:** `crates/biology` should not need to know about `crates/physics` collision algorithms. Integration happens in `apps/engine` or via traits in `crates/world`.

---

## 5. File Naming Standards

* Rust: `snake_case.rs`
* Typescript: `PascalCase.tsx` (Components), `camelCase.ts` (Logic).
* Docs: `SCREAMING_SNAKE_CASE.md` (for major governance), `kebab-case.md` for informational.
