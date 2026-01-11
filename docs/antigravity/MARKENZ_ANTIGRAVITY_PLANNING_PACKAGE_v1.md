# MARKENZ — ANTIGRAVITY PLANNING PACKAGE v1 (EXECUTABLE SPEC)

> MCP-ENFORCED · ZERO-STUB · ZERO-MOCK · ZERO-TODO  
> Single-universe · Offline · Deterministic · Event-sourced  
> Auth: OIDC (Keycloak default / authentik backup) + Passkeys at IdP  
> Storage: SQLCipher (encrypted SQLite) + encrypted secrets (master-key policy)  
> UI: Web (React) + WebGPU voxel renderer + God Dashboard + Chat

---

## 1) Non-negotiable invariants (hard fail if violated)

### I1 — Single universe
- Exactly one authoritative `UniverseId = "markenz"` per deployment.
- One sim loop, one event stream, one state tree.

### I2 — Deterministic lockstep
- State evolution is a pure function of: `(seed, ordered_input_events)`.
- Fixed timestep; deterministic system order; deterministic collection order.
- Sim-critical math uses fixed-point (Q16.16). No floats in sim.

### I3 — Event sourcing + replay
- Every mutation is an event.
- Snapshot + replay reconstructs identical state hash checkpoints.
- Divergence tooling pinpoints first mismatch (seq + system).

### I4 — Offline-first
- No internet required for core operation.
- Docker images and JS/Rust deps must be vendorable/prefetchable for offline builds.

### I5 — Zero stubs
- Repo must fail CI if it contains: TODO/FIXME, unimplemented!, panic!("TODO"), @ts-nocheck, mock data paths.

---

## 2) System decomposition

### 2.1 Processes
- `markenz-server` (Rust): sim, persistence, WS/REST, authZ, replay verifier.
- `markenz-web` (React): WebGPU view, dashboards, inspectors, chat, admin tools.
- IdP (Docker): **Keycloak** (default) OR **authentik** (backup), swap via compose profile.

### 2.2 Key modules (Rust)
- `crates/protocol`: event types, snapshots, hashing, ws messages (stable serialization).
- `crates/deterministic`: fixed-point, RNG streams, ordered collections.
- `apps/server`:
  - `auth`: OIDC token verification + role mapping
  - `eventlog`: append gate (monotonic seq), hash chaining
  - `sim`: ECS-ish deterministic scheduler + fixed timestep
  - `world_vox`: chunked voxel store + deterministic edits
  - `agents`: agent state + perception + planner + action selection (M1 minimal)
  - `persistence`: SQLCipher DB + migrations
  - `replay`: snapshot load + event replay + hash verify
  - `ws`: multi-client broadcast, snapshot+catch-up

### 2.3 Key modules (Web)
- `renderer_webgpu`: chunk streaming client + greedy meshing + GPU buffers
- `auth`: OIDC auth code + PKCE + session cookie handoff
- `dashboard`: entity inspector, mind panels, event log, controls
- `chat`: event-backed chat with agent replies
- `replay`: request server-side verification + show report

---

## 3) Single-universe concurrency model (authoritative)

### 3.1 Deterministic event ordering
All external inputs become **InputEvents** and are appended through a single append gate:

1. Authenticate request (cookie → session → user roles)
2. Validate payload schema + authorization
3. Assign `seq = last_seq + 1`
4. Persist event with `prev_hash` and computed `hash`
5. Enqueue for application at next tick boundary
6. Broadcast to clients (WS)

**Conflict handling:** server order is the only order. Client timestamps are informational.

### 3.2 Multi-client sync
- WS stream sends:
  - `Snapshot(seq, state_hash, render_seed_payload)`
  - `EventBatch(seq_start..seq_end)`
  - `Checkpoint(seq, state_hash)`
- Late join: snapshot + event catch-up.

---

## 4) Dual-IdP offline deployment + OIDC contract

### 4.1 OIDC contract (app-side)
- Authorization Code Flow + PKCE
- Strict issuer, audience, nonce/state checks
- JWKS fetched from IdP locally; cached; offline-safe refresh
- Normalized role claim required: `markenz_roles: ["creator_admin","observer"]`

### 4.2 Provider swap interface
Config (no code changes):
- `AUTH_PROVIDER=keycloak|authentik`
- `OIDC_ISSUER=...`
- `OIDC_CLIENT_ID=markenz-web`
- `OIDC_REDIRECT_URI=http://localhost:PORT/auth/callback`

Compose profiles:
- `docker compose --profile keycloak up -d`
- `docker compose --profile authentik up -d`

Bootstrap tool (repo-local):
- Applies realm/provider config via **offline import files** plus optional API patching.
- Creates Dylan/Kirsty users and assigns roles.
- Verifies passkey support reachable.

---

## 5) Encryption at rest (design + ops)

### 5.1 Storage targets
- Universe DB + event log + snapshots: SQLCipher
- Config secrets: encrypted under master key
- IdP DB volumes: enforce “encrypted volume” posture (best effort); refuse start if not satisfied by configured checks.

### 5.2 Master key policy
- `master.key` (32 bytes) generated first run.
- Stored as:
  - OS keychain (preferred) OR
  - `master.key.enc` sealed with Argon2id-derived KEK (passphrase required at start)

### 5.3 Operations
- `key init` → create encrypted master key material
- `key unlock` → unlock for server start
- `key rotate` → SQLCipher rekey + secrets rewrap
- `key backup` / `key restore` → encrypted & signed exports

---

## 6) Milestones (each ends in a shippable working build)

### M0 — Repo + offline bootstrap
**Acceptance**
- `just auth:keycloak-up` works
- `just auth:authentik-up` works
- `just audit nostubs` fails when forbidden tokens exist

### M1 — First working build (MANDATORY)
Includes:
- Rust server + web UI by URL
- OIDC login (Keycloak/authentik) + role-gated endpoints
- Passkeys handled by IdP (enroll via IdP UI link)
- Encrypted storage gate
- WebGPU voxel rendering + chunk streaming (1–4 chunks)
- Two agents (gem-d, gem-k) + transparency panels
- God controls (pause/step + world edit)
- Chat (event backed) + agent replies (symbolic NLG v0)
- Save/load + deterministic replay verification

**Acceptance (automated)**
1. Unauth `/api/state` → 401
2. Auth user without role tries `/api/god/*` → 403
3. Creator role can pause/step and world-edit; event log shows actor + seq.
4. Replay verifier: record 200 ticks + checkpoints, replay, all hashes match.
5. No-stub scan passes.

### M2+ — Expand toward full Markenz spec
(M2 world systems, M3 biology, M4 language/social, M5 genetics/repro, M6 scaling+governance)
Each milestone must add real systems with tests and determinism checks.

---

## 7) Repo scaffold (locked)

```
markenz/
  apps/server
  apps/web
  crates/protocol
  crates/deterministic
  infra/auth
  tools/auth-bootstrap
  tools/keyctl
  docs/antigravity
  docs/reports
```

This file is the authority for Windsurf M0/M1 implementation.

