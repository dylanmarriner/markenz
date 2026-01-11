---
status: APPROVED
---

# PLAN_PHASE_9_NORMALIZED
## Security + Integrity Hardening

**STATUS:** NORMALIZED · EXECUTABLE · PHASE 9 (GLOBAL)  
**AUTHORITY:** KAIZA-MCP · MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2.md (§5.10)

---

## 1. ENTRY CONDITION
Phase 8 complete with all gates passing.

---

## 2. SCOPE (LOCKED)

Lock security without breaking determinism or offline mode.

**Deliverables:**
- Keycloak primary authentication (WebAuthn/passkeys support)
- Authentik backup identity provider (failover only, never authoritative)
- Encryption at rest (AES-256-GCM, nondeterministic nonce handling)
- Tamper-evident audit logs (immutable, hash-chain verified)
- Immutable auth/admin audit trail (all IdP and admin actions logged)
- Integrity explorer UI (audit log browser, tampering detection)

---

## 3. NON-SCOPE

- Additional phases (Markenz complete at Phase 9)

---

## 4. PRESERVATION

All prior guarantees preserved. Security hardening must not break determinism.

---

## 5. DETERMINISM

### 5.1 Encryption Nonce
- GCM nonce derived deterministically from tick + key (NOT wall clock)
- Encryption/decryption preserves bit patterns
- Deterministic: same plaintext + key → same ciphertext (given same nonce derivation)

### 5.2 Audit Logs
- Immutable append-only tables
- Hash-chain verified at write time
- Replay produces identical audit entries

### 5.3 Offline-First
- Keycloak primary but cached JWKS enables offline
- Authentik backup only if Keycloak unavailable
- No network required for deterministic simulation

---

## 6. IMPLEMENTATION OBLIGATIONS

### 6.1 Encryption at Rest
Causal: Snapshot plaintext + master key  
State: Encrypted snapshot in DB  
Proof: Deterministic nonce derivation; same plaintext → same ciphertext

### 6.2 Immutable Audit Log
Causal: Auth events (login, logout, MFA setup, password change) + admin actions  
State: Audit log table (append-only)  
Proof: Hash-chain verified; no UPDATE/DELETE

### 6.3 Integrity Verification
Causal: Audit log query  
State: Tampering detection result  
Proof: Hash-chain walk; divergence detected and reported

---

## 7. REQUIRED ARTIFACTS

**Report:** WINDSURF_PHASE_9_EXECUTION_REPORT.md  
**Path:** /media/linnyux/development3/developing/gemini_universe/markenz/docs/reports/WINDSURF_PHASE_9_EXECUTION_REPORT.md

Must include: Encryption verification, audit log samples, tampering detection test results.

---

## 8. EXIT CRITERIA

### Keycloak & Authentication
- [ ] Keycloak boots and realm configured
- [ ] WebAuthn/passkeys enabled
- [ ] JWKS cached for offline operation
- [ ] Local authentication works without Keycloak (via cache)

### Authentik Backup
- [ ] Authentik boots
- [ ] Identical realm config to Keycloak
- [ ] Fails gracefully if Keycloak unavailable
- [ ] Never authoritative (Keycloak primary always)

### Encryption
- [ ] AES-256-GCM encryption functional
- [ ] Nonce derived deterministically
- [ ] Snapshots encrypted in DB
- [ ] Decryption recovers original plaintext
- [ ] Key stored securely (envvar or HSM)

### Audit Logs
- [ ] All auth events logged (login, logout, MFA, password change)
- [ ] All admin actions logged (laws, elections, penalties)
- [ ] Logs append-only (no UPDATE/DELETE)
- [ ] Hash-chain enforced via foreign keys
- [ ] Integrity verification endpoint functional

### Integrity Explorer UI
- [ ] Audit log searchable and filterable
- [ ] Tampering detection works (hash mismatch detected)
- [ ] Red flag displayed if divergence found
- [ ] Audit trail exportable (PDF or JSON)
- [ ] Hash-chain status visible

### Offline-First
- [ ] System boots without internet
- [ ] Keycloak cached JWKS enables offline auth
- [ ] Determinism unaffected by encryption
- [ ] Replay produces identical hash sequence

### AMP Sign-Off
- [ ] AMP approval (final phase sign-off)

---

## 9. GATES

**Gate 1: Security Integrity (TEST-SEC-001)**  
**Gate 2: Offline Capability (TEST-OFFLINE-001)**  
**Gate 3: Determinism Unbroken (TEST-DET-FINAL-001)**

STOP if any fail.

---

**END OF PHASE 9 NORMALIZED PLAN**
