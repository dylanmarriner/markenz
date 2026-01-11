---
status: APPROVED
plan_type: execution
milestone: M1
authority: antigravity
blocks_execution_without: true
---

# MARKENZ M1 FOUNDATION (CANONICAL)

MODE: PLANNER · MCP-ENFORCED · FAIL-CLOSED
ROLE: Antigravity (Implementation-Grade Planner)
EXECUTION QUALITY: ZERO-STUB, ZERO-TODO, AUDIT-GRADE

---

## 0) OBJECTIVE (NON-NEGOTIABLE)

Produce a **scope-locked, implementation-grade M1 build** of Markenz that results in a **fully working vertical slice**, including:

- **Rust Simulation Server**: Authoritative state, fixed timestep, deterministic.
- **Web UI**: Reachable by URL, React + TS.
- **Auth**: OIDC (Keycloak/authentik) + Passkeys (IdP side) + Role enforcement.
- **Persistence**: Encrypted-at-rest (SQLCipher) + Event Log.
- **Rendering**: WebGPU voxel renderer (client-side) via chunk streaming.
- **Agents**: Two active agents (`gem-d`, `gem-k`) with inspectable minds.
- **Transparency**: State tree, event log, god controls.
- **Verification**: Deterministic replay with hash validation.

**M1 implies SHIPPABLE.** No "it works on my machine". It must pass "Offline Absolute" and "Determinism Absolute" gates.

---

## 1) SCOPE LOCK (M1 ONLY)

**IN SCOPE (MUST IMPLEMENT)**:
- Deterministic Event Loop & Event Bus.
- OIDC Auth Code Flow integration (Server & Client).
- Dual-IdP configuration (Keycloak default, authentik backup).
- Encrypted SQLite storage with master key management.
- Voxel World (Flat/Debug gen) + Chunk Streaming (Binary WebSocket).
- WebGPU Client Renderer (Basic block rendering).
- Chat System (Messages as InputEvents).
- Two Agents (Minimal "Wander/Chat" logic, NO full brain yet).
- God Controls (Pause, Step, Speed, Spawn).
- Replay Verifier Tool.

**OUT OF SCOPE (DO NOT TOUCH)**:
- Full Biology (Metabolism, Hormones) -> Deferred to M2.
- Full Culture/Goverance -> Deferred to M3.
- Large scale (more than 2 agents, more than 1 user).
- Advanced Physics/Collision (Basic AABB only).
- Complex Terrain Generation (Use flat/noise debug world).

---

## 2) ARCHITECTURE DECISIONS (LOCKED)

### 2.1 Server Framework: Axum (Rust)
- **Runtime**: `tokio`.
- **Loop Model**: 
    - **Sim Loop**: Dedicated high-priority thread/task running fixed timestep (e.g., 50ms).
    - **IO/Web**: `tokio` worker threads handling Axum routes and WebSockets.
    - **Communication**: `flume` or `tokio::sync::mpsc` channels. `InputEvents` flow IO -> Sim. `StateUpdates` flow Sim -> IO.
- **Middleware**: `listenfd` for dev reload, `tower_http` for CORS/Trace. Custom `AuthMiddleware` verifying JWT/Opaque tokens against IdP JWKS.

### 2.2 Determinism Stack
- **Tick Model**: Fixed Accumulator. `while accumulator >= dt { tick(); accumulator -= dt; }`.
- **TimeSource**: `struct SimTime(u64)`. NO system time in Sim.
- **RNG**: `ChaCha20`. 
    - `GlobalSeed` (from Genesis) -> `SystemSeed` (blake3 derived) -> `RngStream`.
- **Collections**: `BTreeMap` for state iteration. `Vec` with stable sort for lists.

### 2.3 Persistence + Encryption
- **Database**: SQLite with `sqlcipher` extension (via `rusqlite` bundled).
- **Schema**:
    - `events`: (tick, seq, source, payload_blob, hash, prev_hash).
    - `snapshots`: (tick, state_blob, hash).
    - `kv_store`: System metadata (genesis seed, current tick).
- **Key Management**:
    - Dev: `master.key` file (gitignored).
    - Prod: Environment variable or OS Keychain integration (via `keyring` crate in `tools`).
- **Safety**: Server PANICS if DB is not encrypted or key is invalid.

### 2.4 OIDC Strategy
- **Library**: `openidconnect` crate (Rust) / `react-oidc-context` (Web).
- **Flow**: Authorization Code Flow with PKCE.
- **Roles**: Claims mapped to `MarkenzRole` enum (`Creator`, `Observer`).
- **Profile Switching**:
    - Environment variables `OIDC_ISSUER`, `OIDC_CLIENT_ID` drive config.
    - `docker-compose.yml` uses profiles (`ext-keycloak`, `ext-authentik`).

### 2.5 Web + WebGPU
- **Stack**: React, TypeScript, Vite.
- **Rendering**: Native WebGPU API (via `wgpu` types in TS if avail, otherwise raw `GPUDevice`).
- **Protocol**:
    - `WS /ws/events`: JSON (prototyped) or MessagePack events.
    - `WS /ws/chunks`: Binary stream. `[Header: x, y, z][Data: Run-Length Encoded Voxels]`.
- **State Sync**: Server sends `TickUpdate` (Agent pos, World diffs). Client interpolates.

---

## 3) REPO SCAFFOLD (M1)

Delete existing loose files. Enforce this structure:

```
markenz/
├── apps/
│   ├── server/             # Rust Axum Server
│   │   ├── src/
│   │   │   ├── api/        # REST/WS endpoints
│   │   │   ├── auth/       # OIDC logic
│   │   │   ├── sim/        # Deterministic Loop
│   │   │   ├── world/      # Voxel storage
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── web/                # React Vite App
│       ├── src/
│       │   ├── features/
│       │   │   ├── auth/
│       │   │   ├── chat/
│       │   │   ├── renderer/ # WebGPU engine
│       │   │   └── debug/    # God panels
│       │   └── main.tsx
│       └── package.json
├── crates/
│   ├── protocol/           # Shared types (Events, WS Messages)
│   └── deterministic/      # Math, RNG, Collections
├── infra/
│   ├── docker-compose.yml  # Keycloak, Authentik, Postgres
│   └── config/             # Realm imports
├── tools/
│   ├── auth-bootstrap/     # script to setup IdP
│   ├── db-migrate/         # SQLCipher setup
│   └── replay-verify/      # Rust tool to checking logs
└── docs/plans/
```

---

## 4) REQUIRED ENDPOINTS & PROTOCOLS

### 4.1 REST (Server)
- `GET /health`: Returns 200 OK + Version.
- `GET /auth/config`: Returns OIDC config (Issuer, ClientID) for frontend.
- `GET /auth/me`: Validates session cookie/header, returns `UserParams { roles, id }`.
- `POST /admin/command`: (God Role) Inject special AdminEvent (Pause, Speed).
- `GET /world/snapshot`: Download latest snapshot (for late join).

### 4.2 WebSocket
- `GET /ws/main`: Multiplexed stream.
    - **C->S**: `InputEvent` (Chat, Move), `Subscription` (Chunk).
    - **S->C**: `TickUpdate` (Events executed, Agent updates), `ChunkData` (Binary), `Presence` (User list).

### 4.3 Schemas (Protobuf or JSON-Strict)
- **Event**: `{ "seq": u64, "tick": u64, "kind": "Chat"|"Move", "payload": "...", "sig": "..." }`
- **Chunk**: RLE compressed: `[u16 voxel_id, u16 count, ...]`

---

## 5) ACCEPTANCE TESTS (CI GATED)

**Test Command**: `just test-m1`

1.  **test_determinism_replay**:
    - Run Sim for 100 ticks with Seed A -> Hash A.
    - Reset. Run Sim for 100 ticks with Seed A -> Hash B.
    - Assert Hash A == Hash B.
2.  **test_offline_scan**:
    - Scan `apps/server/src/sim` for `reqwest`, `std::net`, `std::time`. MUST BE EMPTY.
    - Network IO only allowed in `apps/server/src/api` or `apps/server/src/auth`.
3.  **test_encryption_enforced**:
    - Create DB with key. Try open with empty key -> Must Fail.
4.  **test_auth_flow**:
    - (Mock IdP or integ test) Trace login flow -> Receive Token -> Verify Role.

---

## 6) PHASED EXECUTION PLAN (WINDSURF)

**Do not proceed to next step until previous passes verification.**

### PHASE 1: SCAFFOLD & CORE PROTOS
- **Action**: Create directory structure. Initialize Rust workspace (`server`, `protocol`, `deterministic`). Initialize generic `web` app.
- **Gate**: checked in, `cargo check` passes, `npm install` passes.

### PHASE 2: DETERMINISTIC ENGINE (RUST)
- **Action**: Implement `TimeSource`, `ChaosStream`, `SimLoop` (Sim-only, no IO).
- **Gate**: `test_determinism_simple` passes (pure logic test).

### PHASE 3: AUTH & INFRA
- **Action**: Set up `docker-compose` with Keycloak. Write `auth-bootstrap` tool. Implement OIDC verification in `server`.
- **Gate**: `curl localhost:3000/auth/me` returns 401 (unauth) or 200 (mock token).

### PHASE 4: PERSISTENCE & EVENT LOG
- **Action**: Implement SQLCipher storage. Wire up `EventBus` to `EventLog`.
- **Gate**: Appending events updates database. `test_encryption_enforced` passes.

### PHASE 5: WEBGPU & WORLD STREAMING
- **Action**: Implement basic Voxel World (Sim). Implement `/ws/chunks`. Implement WebGPU renderer (Client).
- **Gate**: Browser shows a rendered chunk. Camera moves.

### PHASE 6: AGENTS & INTERACTION
- **Action**: Spawn `gem-d`, `gem-k` (simple walkers). Implement Chat UI.
- **Gate**: Chat message appears in Event Log. Agents move deterministically.

---

## 7) WINDSURF EXECUTION CONSTRAINTS

1.  **Scope Restricted**: Windsurf may ONLY implement items in this plan.
2.  **MCP Only**: All file writes via `write_to_file`. No shell hacks.
3.  **Stop on Fail**: If any test fails, STOP and Report. Do not "try harder" blindly.
4.  **No Stubs**: `todo!()` or `// TODO` is strictly forbidden in committed code. Use `unimplemented!()` ONLY if blocked by missing dependency, and Report it.
5.  **Audit**: Before marking M1 complete, run `tools/audit/scan_stubs.sh`.

---
