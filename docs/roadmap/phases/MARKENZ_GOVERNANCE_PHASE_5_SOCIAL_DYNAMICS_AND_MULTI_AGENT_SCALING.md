---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 5
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_4_OFFLINE_COGNITION_ENGINE
---

# MARKENZ — GOVERNANCE PHASE 5: SOCIAL DYNAMICS AND MULTI-AGENT SCALING

## 1. Phase Objective

Implement emergent social systems; scale to dozens of agents without determinism drift. This phase extends cognitive systems to multi-agent interaction, reputation, and gossip.

## 2. Governance Domains In Scope

- **Social contracts & trust** (relationship graph, gossip, reputation)
- **Law creation & amendment** (election/voting mechanics introduced; activation in Phase 7)

*Sourced from Section 4, PHASE 5, "Governance Domains Expanded."*

## 3. Systems & Modules Touched

- `crates/world` (relationship graph expansion)
- `crates/social` (gossip propagation, reputation shifts)
- `apps/engine` — Telemetry throttling (deterministic, non-outcome-affecting)
- `apps/engine` — Multi-agent scaling infrastructure

*Sourced from Section 4, PHASE 5, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 5 MUST be defined and logged:

- `RelationshipUpdated` (agent_a, agent_b, weight_delta, cause)
- `Gossip` (agent_id, speaker, target, claim, propagation_vector)
- `ReputationShift` (agent_id, reputation_delta, source)
- `CultureMetric` (population, metric_type, value)

*Sourced from Section 4, PHASE 5, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 5 completion, the following properties MUST hold:

- **Relationship Evolution:** Same interactions (cooperation, betrayal, gift-giving) → identical trust weights between agents.
- **Gossip Propagation:** Same speech acts (agent A tells B that "X did Y") → identical reputation spreading across population.
- **Culture Emergence:** Same behavior patterns across population → identical norm emergence (shared beliefs about acceptable behavior).
- **Multi-Agent Determinism:** Dozens of agents running simultaneously → identical `world_hash` sequence (no floating-point or timing drift).

*Sourced from Section 4, PHASE 5, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### Relationship Graph

- **Agents have weights:** Every agent has a relationship weight to every other agent (range: -100 to +100).
- **Interaction Effects:** Cooperation increases weight; betrayal decreases weight.
- **Relationship Asymmetry:** A→B weight may differ from B→A weight (A may trust B more than B trusts A).
- **Reputation Integration:** Gossip about third parties affects reputation (affects B's weight to C based on A's claim).

### Gossip Propagation (Deterministic)

- **Speech act triggers gossip:** When agent A tells agent B "X did Y", gossip event emitted.
- **Propagation vector:** Gossip spreads to agents emotionally connected to B (relationship weight > threshold).
- **Decay:** Gossip fidelity decreases with distance from source (game of telephone effect).
- **Update rule:** Recipient updates reputation of subject deterministically based on prior reputation + claim + claim source trust.

### Reputation System

- **Public trust score:** Each agent has reputation (aggregate opinion of population).
- **Calculation:** Reputation(A) = mean(all agents' weights to A).
- **Observable:** Accessible via UI; affects social negotiation success.
- **Bounded:** Reputation clamped to [-100, +100].

*Sourced from Section 3.6 "Social Contracts & Trust."*

## 7. Audit & Replay Requirements

### Relationship Graph Snapshots

- Periodic snapshots of full relationship matrix (NxN matrix for N agents).
- Snapshot includes: agent_a, agent_b, weight, last_update_cause, last_update_tick.
- Snapshots stored in audit log; used for analysis.

### Reputation/Culture Metric Timelines

- Reputation per agent tracked over time (timeline shows reputation rise/fall).
- Population-level metrics (culture_metric) tracked: average reputation, polarization (std dev), faction formation.
- Report generated: "culture drift over phases" showing norm emergence.

### Gossip Propagation Log

- Every `Gossip` event logged with: speaker, target, listener, claim, propagation chain.
- Audit tool traces gossip path: "claim X originated from agent A, spread to B, then C, then D, ..."
- Fidelity degradation tracked: "original claim: 'X is trustworthy'; by 5th retelling: 'X is kind of okay'".

*Sourced from Section 4, PHASE 5, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 6.

### 8.1 Multi-Agent Determinism Test

**Requirement:** Dozens of agents running concurrently must produce identical `world_hash` sequence as single-agent runs.

**Acceptance Criteria:**
- Scenario: 20 agents in world with food, tools, enemies.
- Run 1: Agents act concurrently; produce hashes H[0..N].
- Run 2: Same seed, same 20 agents, identical order of action processing; produce hashes H'[0..N].
- H[i] == H'[i] for all i (bit-for-bit equality).
- Scaling test: 50 agents, 100 agents also tested.
- Test automated; CI gated.

### 8.2 Gossip Propagation Test

**Requirement:** Gossip spread must be deterministic; same claim + network → identical reputation shifts.

**Acceptance Criteria:**
- Initial state: Agent A's reputation = 50.
- Agent B hears gossip from C (credible source): "A is dishonest" (credibility weight: high).
- B updates reputation of A → 40 (deterministic calculation based on prior + claim credibility).
- Gossip spreads to D, E, F; all update A's reputation deterministically.
- Replay same gossip → identical reputation shifts at each agent.
- Test automated; CI gated.

### 8.3 Relationship Graph Consistency Test

**Requirement:** Relationship graph must remain consistent across all agents; no orphaned weights or contradictions.

**Acceptance Criteria:**
- After 100 ticks of multi-agent interaction, generate relationship snapshot.
- Verify symmetry where applicable (e.g., if A and B cooperate, both weights increase).
- Verify weight ranges: all weights in [-100, +100].
- Verify no NaN, Infinity, or undefined weights.
- Test automated; audit report generated.

### 8.4 Scaling Performance Test

**Requirement:** Tick advancement must remain stable under multi-agent load; no performance degradation that causes nondeterminism.

**Acceptance Criteria:**
- Tick time with 10 agents: T1 ms.
- Tick time with 50 agents: T50 ms (should scale linearly, T50 ≈ 5×T1).
- Tick time with 100 agents: T100 ms (should scale linearly, T100 ≈ 10×T1).
- No variance in tick time for same agent count (within 5% tolerance).
- Test automated; performance report generated.

### 8.5 Culture Metric Test

**Requirement:** Population-level metrics must reflect emergent norms; population behavior must trend toward stability.

**Acceptance Criteria:**
- After agents interact for 1000 ticks, population-level reputation polarization (std dev) should stabilize.
- Metrics show: average reputation converges, faction formation patterns consistent across replays.
- Culture drift report shows: norm emergence over phases (e.g., "friendliness norm" emerges as agents cooperate).
- Test automated; metric report generated.

*Sourced from Section 4, PHASE 5, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 5 is considered complete:

1. **Dozens of Agents Run Without Drift:**
   - 20, 50, 100 agents scenario tested.
   - Hash divergence check: none detected across full run.
   - Merge not blocked if scaling works.

2. **Relationship Graph Observable and Consistent:**
   - Relationship weights visible in UI (if needed).
   - Audit tool generates relationship snapshot report.
   - No corrupted weights detected.

3. **Tick Rate Stable Under Multi-Agent Load:**
   - Tick time scales linearly with agent count.
   - No variance (within tolerance) for same agent count.
   - Merge not blocked if performance acceptable.

4. **Replay Produces Identical Social State:**
   - Same seed + agents + interactions → identical relationship graph.
   - Reputation evolution reproducible.
   - Gossip propagation reproducible.
   - Test automated; CI gated.

5. **Determinism Maintained from Phase 4:**
   - Phase 4 determinism replay test still passes.
   - Thought streams, planning determinism maintained.

6. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 5, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 5:

- **No nondeterministic agent interaction** (Section 4, PHASE 5, "Explicit Prohibition List").
  - Relationship updates must be deterministic (same interaction → same weight change).
  - No floating-point weight calculations; use fixed-point or integer math.
  - No random relationship shuffling.

- **No telemetry throttling that affects outcomes** (Section 4, PHASE 5, "Explicit Prohibition List").
  - Telemetry may be sampled (e.g., report every 10th agent's vitals), but never affects world state.
  - Outcome computation must be unthrottled and deterministic.
  - Exception: UI rendering may be throttled/sampled (client-side only).

- **No hidden social state** (Section 1.1, "Transparency Law").
  - All relationship weights, reputation scores, gossip claims must be logged and observable.
  - No secret relationships or hidden reputation calculations.

- **No arbitrary reputation reset** (Section 3.6, "Social Contracts & Trust").
  - Reputation can only change via deterministic gossip/interaction.
  - Admin cannot directly edit reputation (must propose law or use interaction event).

- **No unbounded agent count or scaling assumptions** (Section 4, PHASE 5, "Explicit Prohibition List").
  - Scaling must be linear or better; no O(N²) algorithms that degrade with agent count.
  - Agent limit (if any) must be documented and enforced by CI.

*Sourced from Section 4, PHASE 5, "Explicit Prohibition List (Phase 5)," and Section 1.1 "Determinism Law", "Transparency Law".*

## 11. Phase Completion Criteria (Checklist)

Phase 5 is NOT complete until ALL of the following are satisfied:

- [ ] **Multi-agent scaling proven deterministic** — Dozens of agents produce identical hashes; no drift
- [ ] **Social dynamics observable** — Relationship graph, gossip, reputation all logged and auditable
- [ ] **Tick rate stable with dozens of agents** — Linear scaling; no variance within tolerance
- [ ] **Gossip propagation deterministic** — Same claim + network → identical reputation evolution
- [ ] **Relationship graph consistent** — No orphaned weights; all values in valid range; no contradictions
- [ ] **Culture metrics tracked** — Population-level trends observable; norms emerge deterministically
- [ ] **All mandatory tests pass** — Multi-agent determinism, gossip, relationship, scaling, culture metric tests
- [ ] **CI gates pass** — Build, multi-agent scaling, relationship consistency, tick stability, replay determinism gates
- [ ] **Agents interact socially** — Cooperation, betrayal, gift-giving affect relationship weights
- [ ] **Determinism maintained from Phase 4** — Replay test from Phase 4 still passes; cognition determinism maintained

*Sourced from Section 4, PHASE 5, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 3.6, 4.0 (PHASE 5), and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The multi-agent determinism guarantee and social dynamics specification herein may never be weakened.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 5, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 5, "Governance Domains Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 5, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 5, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 5, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 3.6 "Social Contracts & Trust" |
| 7. Audit & Replay Requirements | Section 4, PHASE 5, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 5, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 5, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 5, "Explicit Prohibition List (Phase 5)"; Section 1.1 "Determinism Law", "Transparency Law" |
| 11. Phase Completion Criteria | Section 4, PHASE 5, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 4 (completed)
