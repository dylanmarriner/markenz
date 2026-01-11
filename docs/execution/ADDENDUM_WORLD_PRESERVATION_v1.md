# ADDENDUM: WORLD PRESERVATION & IMPORT MANDATE

# AUTHORITY: KAIZA-MCP · AMP

# PARENT PLAN: MARKENZ_UNIFIED_INTEGRATION_MASTER_PLAN_v2

# MODE: NON-DESTRUCTIVE ADDENDUM

# STATUS: BINDING

---

## 1. PRESERVATION MANDATE

The physical world state of the Gemini Universe is **CANONICAL** and **PRECIOUS**.
Windsurf is **STRICTLY FORBIDDEN** from regenerating, approximating, or replacing these assets with "defaults".

### 1.1 Canonical Assets

The following entities must be **losslessly imported** from the Gemini 1.0 export:

1. **THE HOUSE (Homestead):**
    * Exact structure (walls, windows, doors).
    * Exact location relative to world origin.
    * Interior layout and furniture state.

2. **THE SHED (Tool Storage):**
    * Exact structure.
    * Workbench and tool rack configuration.
    * Contents (see Tools).

3. **TOOLS & EQUIPMENT:**
    * All tools (Hoe, Axe, Watering Can, etc.).
    * Durability states.
    * Modifications/Upgrades (if any).
    * Locations (e.g., "On Rack", "In Inventory").

4. **VEHICLES:**
    * The Truck (Color, condition, fuel state).
    * The Tractor (Attachments, condition).
    * Location and orientation.

5. **WORLD LAYOUT:**
    * The specific arrangement of House, Shed, and Fields.
    * Terrain topology immediately surrounding the homestead.

---

## 2. PRESERVATION INVARIANTS

The following invariants must hold true at Tick 0 and for all eternity:

* **INV-WORLD-001 (No Regeneration):** The homestead region must NEVER be procedurally regenerated. It is static, imported history.
* **INV-WORLD-002 (No Replacement):** No "New House" or "Default Shed". The specific instances from Gemini must be instantiated.
* **INV-WORLD-003 (No Abstraction Loss):** If the Gemini export contains detail (e.g., tool durability = 87%), Markenz must validly represent it.
* **INV-WORLD-004 (Stable Hashes):** Re-importing the same Gemini export data must produce bit-identical Markenz asset state.

---

## 3. IMPLEMENTATION GATES

### 3.1 Deterministic Import

* Function: `fn import_world_assets(gemini_export: &JsonBlob) -> MarkenzAssets`
* Constraint: Pure function.
* Verification: `blake3(output)` must be stable across multiple runs.

### 3.2 Replay Consistency

* Replaying the import events must result in the exact same in-memory object graph.

### 3.3 Forbidden Actions (Hard Gates)

Windsurf MUST NOT:

1. Initialize a "new" House or Shed via `House::new()`.
2. Discard tool durability data.
3. Alter vehicle locations to "spawn points" if real coordinates exist.
4. Proceed if Asset Import Hash implies data corruption.
