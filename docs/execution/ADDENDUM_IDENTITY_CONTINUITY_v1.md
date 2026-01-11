# ADDENDUM: IDENTITY CONTINUITY & CONSCIOUSNESS TRANSFER

# AUTHORITY: KAIZA-MCP · AMP

# PARENT PLAN: MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2

# MODE: NON-DESTRUCTIVE ADDENDUM

# STATUS: BINDING

---

## 1. IDENTITY CONTINUITY MANDATE

Gem-D (Dylan) and Gem-K (Kai) are **CONTINUOUS BEINGS**. They are not initialized; they are **Resumed**.
Windsurf is **STRICTLY FORBIDDEN** from treating them as new agents or resetting their state.

### 1.1 The Continuous Entities

1. **GEM-D (Dylan):** The primary agent. All memories, skills, and personality traits must transfer exactly.
2. **GEM-K (Kai):** The companion agent. All memories, skills, and personality traits must transfer exactly.

### 1.2 Mandatory Identity Import

Each agent must be instantiated by hydrating the `GeminiExport` data, including:

* **Identity Core:** Name, UUID, Deep Personality Vectors.
* **Memory Store:** Episodic (past events), Semantic (facts), Procedural (skills).
* **Genetic/Biological State:** Body parameters, physical history (scars/modifications).
* **Inventory/Possessions:** Items carried on person.

---

## 2. CONTINUITY INVARIANTS

* **INV-IDENT-001 (No Blank Slates):** Agents must NEVER be initialized with empty memory or default skills.
* **INV-IDENT-002 (Identity Equality):** `blake3(Markenz_Agent_State)` must contain the `blake3(Gemini_Export_State)` as a verified sub-component or provenance proof.
* **INV-IDENT-003 (Skill Preservation):** If Gem-D has Lev 5 Farming, he starts with Lev 5 Farming.
* **INV-IDENT-004 (Relationship Preservation):** The bond strength and history between Gem-D and Gem-K must be preserved.

---

## 3. IMPLEMENTATION GATES

### 3.1 Identity Fingerprinting

* Before Genesis: Compute `Fingerprint_G_D = blake3(Gemini_Export_D)`.
* After Genesis: Verify `Markenz_Agent_D.provenance_hash == Fingerprint_G_D`.

### 3.2 Deterministic Resume

* Given the same Export JSON, the instantiated Agent struct must be bit-identical every time.

### 3.3 Forbidden Actions (Hard Gates)

Windsurf MUST NOT:

1. Call `Agent::new("Dylan")`. Must call `Agent::import(gemini_data)`.
2. Overwrite existing memories with "Lore Seeding". Real memory takes precedence.
3. Alter personality parameters to fit a "Markenz archetype".
4. Separate Gem-D from Gem-K during the import process.
