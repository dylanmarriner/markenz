# PATCH INSTRUCTIONS: PHASE 0 & PHASE 1

# TARGETS: PLAN_PHASE_0_BOOTSTRAP.md, PLAN_PHASE_1_DETERMINISM.md

# AUTHORITY: KAIZA-MCP

# MODE: SURGICAL INSERTION ONLY

---

## 1. TARGET: PLAN_PHASE_0_BOOTSTRAP.md

### INSERTION 1

**Location:** Under `### apps/engine/src/genesis.rs` -> `- **Required behaviors:**`
**Anchor:** `Initialize crates/world::Universe with genesis config`
**Text Block:**

```markdown
  - REQUIRED: Call `import_world_assets()` and `import_identities()` per ADDENDUM_WORLD_PRESERVATION_v1 and ADDENDUM_IDENTITY_CONTINUITY_v1
  - VERIFY: Assert `House` and `Shed` matches imported `blake3` hash
  - VERIFY: Assert `Gem-D` and `Gem-K` provenance hashes match export
```

### INSERTION 2

**Location:** Under `## Exit Criteria` -> `**Genesis & Assets (ALL REQUIRED):**` (Note: This section is implied or part of Infrastructure/Observability in Phase 0, if not present, add to `**Infrastructure (ALL REQUIRED):**`)
**Anchor:** `[ ] Engine boots and logs first 10 ticks to stdout`
**Text Block:**

```markdown
- [ ] Asset Import Verified: House/Shed/Tools match Gemini export hashes
- [ ] Identity Import Verified: Gem-D/Gem-K initialized from export, NOT defaults
```

### INSERTION 3

**Location:** Under `## Forbidden Actions`, Item 3
**Anchor:** `Create placeholder/stub/mock code;`
**Text Block:**

```markdown
3. Create placeholder/stub/mock code; every file must be complete and functional
   - CRITICAL: NO procedural generation of House/Shed; must be imported.
   - CRITICAL: NO blank-slate initialization of Gem-D/Gem-K.
```

---

## 2. TARGET: PLAN_PHASE_1_DETERMINISM.md

### INSERTION 1

**Location:** Under `### apps/engine/src/genesis.rs` -> `- **Required behaviors:**`
**Anchor:** `Load Gem-D and Gem-K agents from JSON export`
**Text Block:**

```markdown
  - STRICT: Enforce INV-IDENT-001 (No Blank Slates) per ADDENDUM_IDENTITY_CONTINUITY_v1
  - STRICT: Enforce INV-WORLD-001 (No Regeneration) per ADDENDUM_WORLD_PRESERVATION_v1
  - HASH: Store `original_export_hash` in `Agent` struct for audit
```

### INSERTION 2

**Location:** Under `## Determinism & Replay Gates` -> `**Gate 1: Cross-Run Hash Equality**`
**Anchor:** `Failure action: Stop Phase 1;`
**Text Block:**

```markdown
  - CHECK: Verify that `World` state includes exact imported assets (House/Shed/Truck) on every run
  - CHECK: Verify Gem-D/Gem-K identity hashes remain stable and match Import Hash
```

### INSERTION 3

**Location:** Under `## Forbidden Actions`
**Anchor:** `Proceed to Phase 2 without Phase 1 gates passing`
**Text Block:**

```markdown
11. Alter or "optimize" imported asset geometry/data (Must preserve ADDENDUM_WORLD_PRESERVATION constraints)
12. "Re-roll" or modify Gem-D/Gem-K traits during import (Must preserve ADDENDUM_IDENTITY_CONTINUITY constraints)
```
