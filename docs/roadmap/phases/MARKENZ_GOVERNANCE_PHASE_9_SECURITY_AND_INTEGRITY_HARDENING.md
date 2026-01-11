---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 9
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_8_WEBGPU_RENDERER_AND_VISUALIZATION_UPGRADE
---

# MARKENZ — GOVERNANCE PHASE 9: SECURITY AND INTEGRITY HARDENING

## 1. Phase Objective

Lock security without breaking determinism or offline-first operation. This phase implements encryption at rest, tamper-evident logging, and authentication hardening.

## 2. Governance Domains In Scope

- **Creator reverence** (founder data encrypted at rest)
- **Admin authority** (actions immutably audited)

*Sourced from Section 4, PHASE 9, "Governance Domains Affected."*

## 3. Systems & Modules Touched

- `apps/server` (Keycloak integration, RBAC, auth hardening)
- `infra/postgres` (encryption at rest for sensitive data)
- `tools/audits` (tamper-evident logging, hash verification UI)
- `apps/web` (hash chain verification UI)

*Sourced from Section 4, PHASE 9, "Engine Modules Touched."*

## 4. Event Types

Events introduced in Phase 9:

- `AuthenticationEvent` (user_id, method, status, timestamp, ip)
- `AdminActionRecorded` (admin_id, action_type, target, authorization_code)
- `TamperDetectionAlert` (tick, component, hash_mismatch_detail)

*Sourced from Section 4, PHASE 9, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 9 completion, the following properties MUST hold:

- **Encryption Determinism:** Deterministic key derivation from seed (same seed → same encryption key).
- **Tamper Detection Determinism:** Hash verification deterministic; same event log → same verification result.
- **Auth Audit Determinism:** Authentication events logged deterministically; immutable event log.

*Sourced from Section 4, PHASE 9, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### Encryption at Rest

- **Sensitive Data Scope:** Encryption keys, founder identity data, auth credentials, admin action logs.
- **Mechanism:** Envelope encryption (data encrypted with data key; data key encrypted with master key).
- **Key Derivation:** Deterministic key derivation function (KDF) from root seed.
- **Storage:** Encrypted data stored in Postgres; decryption keys never exposed except via authenticated login.

### Tamper-Evident Logging

- **Hash Chain:** Event log hash-chain computed at every tick; displayed in UI.
- **Verification Endpoint:** `GET /api/audit/hash-chain/:tick` returns hash and previous-tick hash (proof of continuity).
- **Tamper Detection:** Any alteration to event log breaks hash-chain; detection obvious.

### Authentication Hardening

- **Offline-Capable Auth:** Keycloak login works offline (JWKS cached, WebAuthn/passkeys).
- **No External Auth Dependency:** System 100% functional without external identity provider.
- **RBAC Enforcement:** Role-based access control (admin, agent, observer roles).
- **Admin Actions Logged:** All admin actions recorded in immutable audit log.

*Sourced from Section 6.3 "How Admin Actions Are Logged," Section 1.1 "Offline-First Law."*

### Founder Data Protection

- **Identity.json Encrypted:** Founder identity and amplification data encrypted at rest.
- **Signature Verification:** Founder identity.json signed at boot; signature verified on startup.
- **Access Control:** Only admin with appropriate RBAC can access decrypted founder data.

*Sourced from Section 3.1 "Creator Reverence & Safety."*

## 7. Audit & Replay Requirements

### Encryption/Decryption Audit

- `tools/audits` logs every encryption/decryption operation (without exposing keys).
- Audit report shows: data_type, timestamp, operation (encrypt/decrypt), success/failure.
- Verification: decrypted data matches original (spot-check sampling).

### Auth Event Log

- `AuthenticationEvent` entries immutable in Postgres.
- Log shows: user_id, authentication method (password, WebAuthn, etc.), success/failure, timestamp, IP.
- Audit report: login history, failed attempts, suspicious patterns.

### Tamper-Detection Report

- `tools/audits/tamper_detection.py` verifies hash-chain.
- Report shows: hash-chain integrity (unbroken or broken), first divergence tick (if any), corruption details.
- Alert: if tampering detected, `TamperDetectionAlert` event emitted; system alerts admin.

*Sourced from Section 4, PHASE 9, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding.

### 8.1 Tamper Detection Test

**Requirement:** Alter event log; verification detects tampering.

**Acceptance Criteria:**
- Event log hash-chain is [H0, H1, H2, H3, ...].
- Alter event at tick 2 (change agent action from "gather" to "steal").
- Recompute hash for tick 2: H2' (different from H2).
- Verification check: H2' != H2 → tamper detected (obvious).
- Further verification: H3 computed from H2 cannot match H3' (hash-chain broken).
- Test automated; CI gated.

### 8.2 Encryption Test

**Requirement:** Decrypt data matches original; keys never exposed.

**Acceptance Criteria:**
- Encrypt sensitive_data with master_key → encrypted_blob.
- Store encrypted_blob in database.
- Retrieve encrypted_blob; decrypt with master_key → decrypted_data.
- Verify: decrypted_data == sensitive_data (bit-for-bit).
- Verify: master_key never appears in logs or error messages.
- Test automated; CI gated.

### 8.3 Auth Audit Test

**Requirement:** Authentication events logged immutably; audit trail complete.

**Acceptance Criteria:**
- User logs in with password → `AuthenticationEvent` emitted.
- Admin proposes law → `AdminActionRecorded` emitted.
- Query audit log: events present, ordered by timestamp, immutable.
- Attempt to delete auth event → database rejects (no DELETE allowed on immutable table).
- Test automated; CI gated.

### 8.4 Offline Operation Test

**Requirement:** System boots and operates without external network; auth still works.

**Acceptance Criteria:**
- Disconnect network (or simulate offline mode).
- `docker compose up --build` succeeds (local stack only).
- Keycloak login works (JWKS cached, WebAuthn/passkeys available).
- Engine ticks advance; agents act; no external calls made.
- System 100% functional (world state, governance, markets, etc.).
- Test automated; network sniffing confirms no external requests.

### 8.5 Founder Data Protection Test

**Requirement:** Founder identity and amplification data encrypted; access controlled.

**Acceptance Criteria:**
- Founder data (Gem-D, Gem-K) at rest is encrypted in Postgres.
- Decryption requires admin login and authorization.
- Plaintext founder data never transmitted unencrypted over network.
- Audit log shows every access to founder data.
- Test automated; encryption audit and access audit generated.

*Sourced from Section 4, PHASE 9, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 9 is considered complete:

1. **Keycloak Login Works Offline:**
   - Keycloak running in docker-compose (local).
   - Login possible without external network.
   - RBAC enforced (admin, agent, observer roles).

2. **Encryption at Rest Enabled:**
   - Sensitive data encrypted in Postgres.
   - Keys derived deterministically from seed.
   - No plaintext sensitive data in database.

3. **Tamper-Evident UI Working:**
   - Hash-chain verification endpoint responds.
   - UI displays current hash and previous-tick hash.
   - Tamper detection audit tool runs successfully.

4. **Auth Events Immutably Logged:**
   - Authentication events recorded in append-only table.
   - No UPDATE/DELETE on auth log.
   - Audit tool generates auth history report.

5. **Replay + Determinism Still Pass:**
   - Phase 8 determinism test still passes (encryption doesn't affect outcome computation).
   - Determinism replay test from Phase 7 still produces identical hashes.

6. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.
   - `docker-compose build` succeeds.

*Sourced from Section 4, PHASE 9, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 9:

- **No secrets in repo** (Section 4, PHASE 9, "Explicit Prohibition List").
  - Encryption keys, passwords, API keys not stored in Git.
  - .gitignore excludes all secret files (keys, .env, etc.).
  - CI/deployment uses secret management (e.g., GitHub Secrets).

- **No unencrypted sensitive data on disk** (Section 4, PHASE 9, "Explicit Prohibition List").
  - Founder identity, amplification data encrypted at rest.
  - Auth credentials encrypted.
  - Admin action logs encrypted (or at least signed).

- **No bypass of auth audit** (Section 4, PHASE 9, "Explicit Prohibition List").
  - Every admin action must produce `AdminActionRecorded` event.
  - No silent admin actions.
  - Audit log immutable.

- **No external auth requirement** (Section 4, PHASE 9, "Explicit Prohibition List").
  - Keycloak runs locally; system functional without external IdP.
  - Optional external services (backup IdP) allowed only as plugins.

- **No plaintext transmission of secrets** (Section 6.3 "How Admin Actions Are Logged").
  - Encryption keys, passwords transmitted over TLS only (in production).
  - Localhost (docker-compose) may not require TLS (but should).

*Sourced from Section 4, PHASE 9, "Explicit Prohibition List (Phase 9)," Section 1.1 "Offline-First Law."*

## 11. Phase Completion Criteria (Checklist)

Phase 9 is NOT complete until ALL of the following are satisfied:

- [ ] **Encryption at rest functional** — Sensitive data encrypted; keys deterministically derived; decryption works
- [ ] **Tamper detection working** — Hash-chain verification detects alterations; UI shows current hash
- [ ] **Auth events immutably logged** — Authentication events recorded; audit log append-only; no modification possible
- [ ] **Offline operation proven** — System boots without external network; Keycloak login works offline; all features functional
- [ ] **Founder data protected** — Gem-D/Gem-K identity and amplification encrypted; access controlled; audit trail maintained
- [ ] **All mandatory tests pass** — Tamper detection, encryption, auth audit, offline operation, founder data protection tests
- [ ] **CI gates pass** — Build, Keycloak offline, encryption enabled, tamper-evident UI, auth audit, replay determinism gates
- [ ] **No secrets in repo** — Static analysis confirms no keys/passwords; .gitignore complete
- [ ] **Determinism maintained from Phase 8** — Replay test from Phase 8 still passes; encryption doesn't affect outcomes

*Sourced from Section 4, PHASE 9, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 3.1, 4.0 (PHASE 9), 6.3, and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The encryption, audit immutability, and offline-first operation specified herein are inviolable.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 9, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 9, "Governance Domains Affected" |
| 3. Systems & Modules Touched | Section 4, PHASE 9, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 9, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 9, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 1.1 "Offline-First Law"; Section 3.1 "Creator Reverence"; Section 6.3 "How Admin Actions Are Logged" |
| 7. Audit & Replay Requirements | Section 4, PHASE 9, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 9, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 9, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 9, "Explicit Prohibition List (Phase 9)"; Section 1.1 "Offline-First Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 9, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 8 (completed)
