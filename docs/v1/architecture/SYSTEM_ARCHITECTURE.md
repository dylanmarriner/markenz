# System Architecture Specification

**Status:** FINAL / PRODUCTION  
**Version:** 2.0.0 (Full Implementation)  
**Parent Doc:** [README.md](../../README.md)

---

## 1. Architectural Principles

The Markenz architecture is built upon four non-negotiable pillars:

1. **Authority Isolation:** Only the Rust Engine (`apps/engine`) can mutate world state. All other components are read-only observers or input gateways.
2. **Deterministic Causal Chain:** The state of the universe at Tick $T$ is a pure function of Tick $0$ (Genesis) + strictly ordered `InputEvents` $[0..T]$.
3. **Fail-Closed Integrity:** If any component detects a determinism violation, security breach, or biological invariant failure, the system halts immediately to preserve ledger integrity.
4. **Offline Autonomy:** The entire cluster (Engine, Server, DB, Web) runs fully air-gapped without external dependencies.

---

## 2. Component Breakdown

### 2.1 The Engine (`apps/engine`)

**Role:** The Sovereign Authority.
**Language:** Rust (Release Profile).

The Engine is a monolithic, single-threaded, fixed-timestep simulation loop. It is not a "service" in the microservices sense; it is a "world runner."

* **Responsibilities:**
  * **Tick Loop:** Advances simulation time by fixed `dt`.
  * **Event Ingestion:** Reads grounded `InputEvents` from the Log.
  * **Domain Simulation:** Executes Physics, Biology, Cognition, and Genetics crates.
  * **Governance Enforcement:** Evaluates all actions against `PolicyValidate` logic.
  * **Hashing:** Computes the Merkle root of the world state every tick.
  * **Emission:** Generates `ObservationEvents` and `Snapshots`.

* **Key Boundaries:**
  * **Input:** NEVER accepts direct API calls. Only reads from the immutable Event Log.
  * **Output:** NEVER writes directly to the user. Writes only to the Event Log/Snapshot Store.

### 2.2 The Server (`apps/server`)

**Role:** The Control Plane & Gateway.
**Language:** TypeScript (Node.js).

The Server manages the "human" side of the system: input validation, user identity, and real-time distribution of state.

* **Responsibilities:**
  * **Authentication:** Verifies JWTs against OIDC provider (Keycloak).
  * **RBAC:** Enforces "Observer" vs "Admin" permissions.
  * **Input Validation:** Schema validation (Is this a valid JSON event?) vs Logic validation (Is this legal? - *Engine handles this*).
  * **Event Ordering:** Assigns monotonic sequence IDs to incoming events and commits them to Postgres.
  * **Fanout:** Pushes `ObservationEvents` to connected WebSockets.

* **Constraint:** The Server CANNOT calculate world state. It does not know who is "hungry" or where an agent is located. It only knows what the Engine reports.

### 2.3 The Web Client (`apps/web`)

**Role:** The Window into Reality.
**Language:** React / TypeScript / WebGPU.

The Web Client is a rigorous visualization tool. It renders the state exactly as reported by the Engine.

* **Responsibilities:**
  * **Visualization:** Renders 2D/3D representation of the world grid and entities.
  * **Inspection:** Detailed deep-dive views into Agent internals (Biography, Biology, Cognition logs).
  * **Control:** Forms and tools to construct valid `InputEvents` for submission to Server.
  * **Replay:** Seek-bar interface to traverse historical Ticks using Snapshots + Diff logs.

### 2.4 The Ledger (`infra/postgres`)

**Role:** The Immutable Memory.

Postgres is treated as an append-only ledger for the primary event stream.

* **Schema:**
  * `input_events`: Strict sequence. Immutable.
  * `observation_events`: Derived stream.
  * `world_snapshots`: Binary blobs of full state at intervals (e.g., every 1000 ticks).
  * `chain_hashes`: Cryptographic link verifying log integrity.

### 2.5 Identity Provider (`infra/keycloak`)

**Role:** The Gatekeeper.
**Standard:** OIDC.

Manages human operators. It does *not* manage Agent identities (which are internal simulation data).

---

## 3. Data Flow & Causality

The flow of information is strictly unidirectional to preserve causality.

### 3.1 The Input Loop (The "Write" Path)

1. **Operator** initiates action in `apps/web` (e.g., "Spawn Item").
2. `apps/web` constructs JSON payload and signs with JWT.
3. `apps/server` authenticates JWT, verifies RBAC, and validates JSON schema.
4. `apps/server` assigns **Global Sequence ID** and appends to `input_events` table.
5. `apps/engine` (on next tick) reads pending events from `input_events`.
6. `apps/engine` processes event:
    * **Policy Check:** Is this legal? (e.g., Physics check, Ban check).
    * **Mutation:** Updates World State.
    * **Ack:** Emits `ObservationEvent` (Success/Fail).

### 3.2 The Observation Loop (The "Read" Path)

1. `apps/engine` completes Tick $T$.
2. `apps/engine` generates `StateDiff` (what changed?) and `ObservationEvents`.
3. `apps/engine` flushes to `observation_events` table.
4. `apps/server` (tailing the log) picks up new observations.
5. `apps/server` broadcasts via WebSocket to subscribed `apps/web` clients.
6. `apps/web` updates local state cache and re-renders frame.

---

## 4. Failure Modes & Recovery

### 4.1 Determinism Divergence

**Scenario:** A Replay Audit reveals that Tick 500 produced Hash `A` originally, but Hash `B` during replay.
**System Response:** **IMMEDIATE HALT.**
**Recovery:**

1. Engine locks.
2. Admin investigates logs to find the source of nondeterminism (e.g., unseeded HashMap iteration).
3. Code patch deployed.
4. System re-verified from Genesis.

### 4.2 Biological Veto

**Scenario:** Agent attempts Action `Run` but energy is `0`.
**System Response:** Use standard handling.

1. Engine `BioVeto` system blocks mutation.
2. Engine emits `ObservationEvent{ type: "Veto", reason: "Starvation" }`.
3. World State remains unchanged regarding position.

### 4.3 Breach of Reality (Panic)

**Scenario:** Rust code panics (e.g., index out of bounds).
**System Response:** Engine process crashes. Docker restarts it.
**Recovery:**

1. Engine restarts.
2. Engine reads last valid Snapshot.
3. Engine replays `InputEvents` from Snapshot Tick to Head.
4. Engine resumes normal operation.
*Note: This "Crash-Only" architecture ensures no corrupted state persists in memory.*

---

## 5. Technology Stack Summary

| Layer | Technology | Justification |
|-------|------------|---------------|
| **Engine** | Rust (Stable) | Safety, performance, strict typing, zero GC pauses. |
| **Logic** | ECS (Entity Component System) | Data locality, composition over inheritance, strict ownership. |
| **Server** | Node.js / Fastify | High concurrency for IO-bound event routing. |
| **DB** | PostgreSQL 16 | ACID compliance, JSONB power, proven reliability. |
| **Web** | React / Three.js / WebGPU | High-performance rendering of dense state. |
| **Ops** | Docker Compose | Reproducible, offline deployment. |

---

## 6. Documentation Gaps

None known. Architecture is fully implemented as verified by Phase 9 audits.
