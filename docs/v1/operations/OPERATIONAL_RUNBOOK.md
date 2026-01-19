# Operational Runbook

**Status:** PRODUCTION  
**Context:** Local / On-Prem / Air-Gapped  
**Parent Doc:** [README.md](../../README.md)

---

## 1. System Boot Sequence

The Markenz cluster is deployed as a unitary `docker-compose` stack.

### 1.1 Prerequisites

* Docker Engine 24+
* Nvidia Container Toolkit (optional, for WebGPU server-side rendering if active)
* 32GB RAM Minimum (for full agent population simulation)

### 1.2 Start Command

```bash
# In repo root
./tools/ops/boot_sequence.sh
```

*This wrapper runs `docker compose up -d` but performs pre-flight checks first.*

### 1.3 The Boot Check List (Automatic)

1. **Volume Check:** Verifies Postgres data volume permissions.
2. **Hash Check:** Verifies the last `world_hash` in DB matches the `snapshots` on disk.
3. **Governance Check:** Scans `human_equivalence` config.
4. **Network Check:** Ensures NO outbound internet access for the `engine` container.

### 1.4 Post-Boot Validation

Running `docker compose logs -f engine`:

* *Expect:* `[INFO] Markenz Engine v2.0 Online`
* *Expect:* `[INFO] Governance Laws Loaded: 14 Active`
* *Expect:* `[INFO] Simulating Tick 10420... Hash: a1b2...`

---

## 2. Monitoring & Auditing

### 2.1 Health Checks

* **Engine Liveness:** Monitor `apps/engine` log output. If output stops for > 1 second, the engine has stalled.
* **Determinism:** Runs periodically via cron.

    ```bash
    docker exec markenz-auditor /bin/verify_last_1000_ticks
    ```

### 2.2 Replay Verification (Manual)

To audit a specific incident:

1. Identify the Tick range (e.g., T=5000 to T=5100).
2. Run the replay tool:

    ```bash
    ./tools/audits/run_replay.sh --start 5000 --end 5100 --compare-db
    ```

3. Review generated PDF report in `artifacts/audits/`.

---

## 3. Failure Recovery

### 3.1 "The Divergence" (Determinism Failure)

**Symptom:** Audit tool reports `HASH_MISMATCH` at Tick X.
**Procedure:**

1. **STOP THE WORLD.** `docker compose stop engine`.
2. **Snapshot:** Backup the current DB state.
3. **Analyze:** Use `run_replay.sh --verbose` to identify the specific Entity that diverged.
4. **Fix:** Patch the nondeterministic logic in Rust.
5. **Reset:**
    * This is a hard decision. We cannot "rewrite" history.
    * Option A: Rollback to Tick X-1, apply patch, and re-simulate forward (The "Time Travel" Fix). *Preferred.*
    * Option B: Accept new reality and fork chain. *Discouraged.*

### 3.2 "The Crash" (Panic)

**Symptom:** Engine container restarts repeatedly.
**Procedure:**

1. Read crash log (Panic message).
2. Events leading to panic are immutable in Postgres.
3. Fix the bug in Rust (handle the unexpected state gracefully).
4. Redeploy Engine.
5. Engine will auto-replay the crashing events (now safely handled) and output a corrected state.

---

## 4. Safe Shutdown

To ensure ledger integrity:

```bash
./tools/ops/graceful_shutdown.sh
```

1. Sends `SIGTERM` to Engine.
2. Engine finishes current Tick.
3. Engine commits final Hash and Snapshot.
4. Engine exits.
5. Database shuts down.

**NEVER** `kill -9` the engine unless in catastrophe.

---

## 5. Security Protocols

* **Air Gap:** Ensure the deployment server has no default route to the internet.
* **Secrets:** All `POSTGRES_PASSWORD` and `JWT_SECRET` keys must be rotated daily if in a shared environment (though ostensibly this is a single-tenant simulation).
