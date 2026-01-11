---
status: EXECUTABLE
scope: Governance (Markenz Universe)
authority: MARKENZ_GOVERNANCE_MASTER_ROADMAP
phase: 7
failure_mode: FAIL-CLOSED
depends_on: MARKENZ_GOVERNANCE_PHASE_6_GENETICS_AND_REPRODUCTION
---

# MARKENZ — GOVERNANCE PHASE 7: GOVERNANCE AND ECONOMY (LAWS, ELECTIONS, MARKETS)

## 1. Phase Objective

Implement deterministic rules and economy constraints enforced by PolicyValidate stage. This phase activates law proposal, voting, market economics, and law enforcement via veto.

## 2. Governance Domains In Scope

- **Law creation & amendment** (proposal, voting, activation)
- **Punishment & consequence** (courts, penalties, execution)
- **Property & ownership** (resource markets, disputes)
- **Violence & harm** (combat rules under law)

*Sourced from Section 4, PHASE 7, "Governance Domains Introduced/Expanded."*

## 3. Systems & Modules Touched

- `crates/world` (governance state: laws, elections, property registry)
- `apps/engine` — PolicyValidate pipeline (enhanced; evaluates all laws)
- `crates/economy` (markets, resource trading, taxation)
- `crates/justice` (court system if implemented; guilt/innocence determination)

*Sourced from Section 4, PHASE 7, "Engine Modules Touched."*

## 4. Event Types

All events introduced in Phase 7 MUST be defined and logged:

- `LawProposed` (proposer_id, law_text, vote_start_tick)
- `VoteRecorded` (agent_id, law_proposal_id, vote_value)
- `LawActivated` (law_id, activation_tick, version_hash)
- `PropertyTransferred` (item_id, from_agent, to_agent, method)
- `CriminalChargeRecorded` (agent_id, charge, tick)
- `TrialHeld` (agent_id, charge, verdict, sentence)
- `PenaltyExecuted` (agent_id, penalty_type, details)
- `TradeOccurred` (agent_a, agent_b, items_a, items_b, price)

*Sourced from Section 4, PHASE 7, "Event Types Added."*

## 5. Determinism Guarantees

After Phase 7 completion, the following properties MUST hold:

- **Law Evaluation Determinism:** Same law + action → identical veto decision and reason code.
- **Election Determinism:** Same vote sequence → identical winner (deterministic tiebreaker).
- **Market Determinism:** Same trade sequence → identical prices/inventory (price discovery deterministic).
- **Court Determinism:** Same evidence → identical verdict (deterministic judge decision if court implemented).
- **Penalty Execution:** Same sentence type + agent → identical outcome (execution, imprisonment, fine).

*Sourced from Section 4, PHASE 7, "Determinism Guarantees," and Section 1.1 "Determinism Law."*

## 6. Enforcement Rules

### PolicyValidate (Fully Active)

**Location in Event Pipeline:** After BioVeto, before PhysicsValidate, before Commit.

**Rules:**
- **Law Lookup:** Retrieve applicable laws for tick T, agent, action context.
- **Constraint Evaluation:** Test action against law constraints (deterministically).
- **Veto Decision:** If constraint violated → emit veto with reason; action rejected.
- **Audit:** Veto reason logged; appears in telemetry and event log.
- **No Admin Override:** Admins cannot override PolicyValidate; they can only propose new laws via InputEvents.

*Sourced from Section 2.2 "Policy Evaluation Flow," Section 7.2 "Runtime Veto Behavior."*

### Law Proposal & Voting

- **LawProposal InputEvent:** Agent submits law with text, category, vote duration, threshold.
- **Voting Period:** All agents can vote (yes/no) during voting period.
- **Vote Tally:** Majority vote wins (50% + 1); deterministic tiebreaker if needed (coin flip seeded by law ID).
- **Activation:** Approved law becomes active at next tick; appears in PolicyValidate stage.

*Sourced from Section 5.1 "How New Governance Laws Are Proposed," Section 5.4 "How Laws Become Active."*

### Law Versioning & Conflict Resolution

- **Version Format:** `law_id::version` (e.g., `LAW_THEFT_001::v1`, `LAW_THEFT_001::v2`).
- **Conflict Resolution:** If new law contradicts existing law, voting decides precedence (newer law takes precedence unless explicitly repealed).
- **Constraint:** Laws may NEVER contradict Section 1 (Constitutional Principles); violating laws are rejected.

*Sourced from Section 5.2 "How Conflicts Are Resolved," Section 5.3 "How Laws Are Versioned."*

### Court System (If Implemented)

- **Criminal Charges:** `CriminalChargeRecorded` event emitted when agent breaks law.
- **Trial Process:** Evidence presented; trial verdict determined.
- **Sentence:** If guilty, `TrialHeld` event specifies sentence (fine, imprisonment, banishment, execution).
- **Execution:** `PenaltyExecuted` event shows penalty carried out.

*Sourced from Section 3.8 "Punishment & Consequence."*

### Market Economics

- **Resource Pricing:** Markets determine prices; buyers/sellers submit offers.
- **Trade Execution:** `TradeOccurred` event shows agents, items exchanged, price agreed.
- **Price Discovery:** Market prices emerge from supply/demand (deterministic given RNG seed).

### Property Transfer

- **Ownership Change:** Only via explicit transfer events:
  - `PropertyTransferred` event (explicit transfer, gift, trade, theft-detected, inheritance).
  - No silent ownership changes.
- **Dispute Resolution:** If property dispute, court system determines rightful owner (Phase 7+).

*Sourced from Section 3.2 "Property & Ownership."*

## 7. Audit & Replay Requirements

### Law Change Log

- Full text of every law proposal.
- Vote results (vote count, tally, winner).
- Activation tick.
- Version hash for audit verification.
- Audit report: law history timeline, supersessions, repealals.

### Election Audit

- Votes recorded: agent_id, law_proposal_id, vote_value (yes/no).
- Vote tally: yes_count, no_count, quorum met.
- Winner determination: tie-breaker rule applied if needed.
- Audit report: election integrity (no duplicate votes, all valid agents).

### Property Transfer Chain-of-Custody

- Every property transfer logged with: item_id, from_agent, to_agent, method, tick.
- Full chain reconstructed: genesis item location → current owner.
- Audit report: item history, ownership path, current owner validity.

### Court Records (If Implemented)

- Charges: agent_id, charge text, tick charged.
- Evidence: list of evidence items/witnesses.
- Verdict: guilty/not guilty, judge determination rule.
- Sentence: penalty type, duration/severity.
- Audit report: case history, sentence execution.

### Economy Metrics

- Market prices: resource type, price per unit, timestamp.
- Trade volume: number of trades, total value.
- Resource distribution: agent inventory, aggregate resources per region.
- Audit report: market stability, inflation/deflation trends.

*Sourced from Section 4, PHASE 7, "Audit & Replay Requirements."*

## 8. Tests (MANDATORY)

All tests MUST be executable and MUST pass before proceeding to Phase 8.

### 8.1 Law Enforcement Test

**Requirement:** Action vetoed correctly; veto reason logged; action fails.

**Acceptance Criteria:**
- Law: "Theft is prohibited; penalty: 20 days imprisonment."
- Agent A attempts to take item owned by Agent B (without permission).
- PolicyValidate stage evaluates law; veto emitted with reason "THEFT_PROHIBITED".
- ActionAttempted event shows success=false, veto_code="LAW_THEFT_001::v1".
- Replay same action → identical veto decision.
- Test automated; CI gated.

### 8.2 Election Determinism Test

**Requirement:** Same vote sequence → same winner.

**Acceptance Criteria:**
- Law proposal: "All resources are communal."
- Voting agents: [A, B, C, D, E].
- Vote sequence: [yes, no, yes, yes, no] → 3 yes, 2 no → proposal passes.
- Replay same vote sequence → identical outcome (proposal passes).
- Test tie-breaker: [yes, no, yes, no, yes] → 3 yes, 2 no → passes (deterministic tie-breaker if needed).
- Test automated; CI gated.

### 8.3 Market Determinism Test

**Requirement:** Same trade sequence → consistent prices and inventory.

**Acceptance Criteria:**
- Market: agents trade food for tools.
- Trade sequence: [A sells 10 food to B for 5 tools] → price = 0.5 tools/food.
- Next trade: [C sells 5 food to D for 2.5 tools] → price = 0.5 tools/food (consistent).
- Replay same trade sequence → identical prices (supply/demand curve reproducible).
- Test automated; CI gated.

### 8.4 Court Determinism Test (If Implemented)

**Requirement:** Same evidence → same verdict.

**Acceptance Criteria:**
- Charge: Agent X "murdered" Agent Y (killed without law allowing it).
- Evidence: [X was near Y at death, X had weapon, 3 witnesses saw X].
- Judge verdict (deterministic rule): guilty (evidence overwhelming).
- Sentence: 30 days imprisonment.
- Replay same evidence → identical verdict (guilty), identical sentence.
- Test automated; CI gated.

### 8.5 Property Ownership Consistency Test

**Requirement:** Property registry accurate; no orphaned items; no double-ownership.

**Acceptance Criteria:**
- After 1000 ticks of trades/transfers, verify property registry.
- Every item has exactly one owner (or is "unowned").
- No item with missing owner_id.
- No item simultaneously owned by two agents.
- Chain-of-custody valid: owner in registry matches latest PropertyTransferred event.
- Test automated; audit report generated.

*Sourced from Section 4, PHASE 7, "Tests."*

## 9. CI / Compilation Gates

The following gates MUST pass before Phase 7 is considered complete:

1. **Law Proposal + Voting System Working:**
   - Agents can submit law proposals via InputEvent.
   - Voting system functional (agents vote; tally counted).
   - Laws activate after voting period.

2. **Economy/Markets Functional:**
   - Agents can propose trades; prices determined.
   - Trade execution produces `TradeOccurred` events.
   - Market prices stable (deterministic price discovery).

3. **Court System (If Implemented) Deterministic:**
   - Trial events produced.
   - Verdicts deterministic given same evidence.
   - Sentences executed correctly.

4. **Governance Enforcement Proven Deterministic:**
   - Same law + action → identical veto decision.
   - Election determinism test passes.
   - Replay produces identical governance state.

5. **Law Enforcement Visible:**
   - PolicyValidate vetoes appear in event log with reason.
   - UI shows law prohibitions (if governance UI implemented).
   - Veto reasons searchable in audit tool.

6. **Property Registry Consistent:**
   - All items have valid owners.
   - No double-ownership; no orphaned items.
   - Chain-of-custody traceable.

7. **Build Succeeds:**
   - `cargo build --release` succeeds.
   - `npm run build` succeeds.

*Sourced from Section 4, PHASE 7, "CI Gates," and Section 7.3 "CI Enforcement."*

## 10. Explicit Prohibitions

The following actions, patterns, and implementations are FORBIDDEN in Phase 7:

- **No laws contradicting Section 1** (Section 3.7 "Law Creation & Amendment").
  - Any law violating Constitutional Principles must be rejected by PolicyValidate.
  - Cannot amend Authority Law, Determinism Law, Transparency Law, etc.

- **No retroactive law enforcement** (Section 3.7 "Law Creation & Amendment").
  - Laws only apply prospectively from activation tick onward.
  - Cannot retroactively criminalize past actions.

- **No hidden laws** (Section 3.7 "Law Creation & Amendment").
  - All laws observable and searchable via audit tool.
  - No secret laws that apply but are not visible.

- **No unlogged property transfers** (Section 3.2 "Property & Ownership").
  - Every property transfer MUST emit `PropertyTransferred` event.
  - No silent ownership changes.

- **No rigged elections** (Section 4, PHASE 7, "Explicit Prohibition List").
  - Voting must be deterministic (seeded RNG).
  - No voting fraud or ballot stuffing.
  - Tally must be transparent and auditable.

- **No arbitrary sentencing** (Section 3.8 "Punishment & Consequence").
  - Penalties must be specified by law (fine amount, imprisonment duration).
  - Judge cannot impose penalties outside legal bounds.

- **No torture or indefinite imprisonment** (Section 3.8 "Punishment & Consequence").
  - Penalties must be finite and humane.
  - Torture explicitly forbidden.

*Sourced from Section 4, PHASE 7, "Explicit Prohibition List (Phase 7)," Section 3.2, 3.7, 3.8.*

## 11. Phase Completion Criteria (Checklist)

Phase 7 is NOT complete until ALL of the following are satisfied:

- [ ] **Law proposal + voting system working** — Agents can propose laws; voting functional; laws activate
- [ ] **Economy/markets functional** — Agents trade items; prices determined deterministically; TradeOccurred events logged
- [ ] **Court system (if implemented) deterministic** — Trials held; verdicts deterministic; sentences executed
- [ ] **Governance enforcement proven deterministic** — Same law + action → identical veto; election determinism proven
- [ ] **Law enforcement visible** — PolicyValidate vetoes logged; veto reasons in event log and observable
- [ ] **Property registry consistent** — All items have valid owner; no double-ownership; chain-of-custody traceable
- [ ] **All mandatory tests pass** — Law enforcement, election, market, court (if implemented), property consistency tests
- [ ] **CI gates pass** — Build, law voting, economy, court, governance enforcement, property registry gates
- [ ] **Determinism maintained from Phase 6** — Replay test from Phase 6 still passes; genetics/reproduction determinism maintained
- [ ] **No laws violate Section 1** — Constitutional Principles enforced; violating laws rejected

*Sourced from Section 4, PHASE 7, "Phase Completion Criteria."*

## 12. Authority Statement

This phase plan is derived directly from MARKENZ_GOVERNANCE_MASTER_ROADMAP.md Sections 1.1, 2.2, 3.2, 3.7, 3.8, 4.0 (PHASE 7), 5.0, and 7.3. Any implementation deviating from this plan is invalid and must fail closed. The governance determinism guarantee, law-is-law principle (no admin bypass), and market fairness specified herein may never be weakened.

## Traceability

| Phase Section | Master Roadmap Reference |
|---|---|
| 1. Phase Objective | Section 4, PHASE 7, "Objective" |
| 2. Governance Domains In Scope | Section 4, PHASE 7, "Governance Domains Introduced/Expanded" |
| 3. Systems & Modules Touched | Section 4, PHASE 7, "Engine Modules Touched" |
| 4. Event Types | Section 4, PHASE 7, "Event Types Added" |
| 5. Determinism Guarantees | Section 4, PHASE 7, "Determinism Guarantees"; Section 1.1 "Determinism Law" |
| 6. Enforcement Rules | Section 2.2 "Policy Evaluation Flow"; Section 2.3 "Enforcement Points"; Section 3.2, 3.7, 3.8 "Governance Domains" |
| 7. Audit & Replay Requirements | Section 4, PHASE 7, "Audit & Replay Requirements"; Section 2.4 "Audit & Replay Implications" |
| 8. Tests (MANDATORY) | Section 4, PHASE 7, "Tests"; Section 7.3 "CI Enforcement" |
| 9. CI / Compilation Gates | Section 4, PHASE 7, "CI Gates"; Section 7.3 "CI Enforcement" |
| 10. Explicit Prohibitions | Section 4, PHASE 7, "Explicit Prohibition List (Phase 7)"; Section 3.7, 3.8 "Governance Domains" |
| 11. Phase Completion Criteria | Section 4, PHASE 7, "Phase Completion Criteria" |
| 12. Authority Statement | Section 1.0 "Governance Constitutional Principles"; Section 9.0 "Final Authority Statement" |

---

**Phase Status:** READY FOR EXECUTION  
**Authority:** MARKENZ_GOVERNANCE_MASTER_ROADMAP  
**Effective Date:** 2026-01-11  
**Requires:** Phase 6 (completed)
