/*!
# MARKENZ Genetics Crate

**Purpose:** Deterministic genetics and inheritance system for human-equivalent agents.

**Governance Authority:** MARKENZ_GOVERNANCE_PHASE_6_GENETICS_AND_REPRODUCTION

**Determinism Guarantees:**
- All genetic operations use deterministic RNG streams
- Mendelian inheritance follows fixed probability tables
- No random mutations outside seeded RNG streams
- All genetic changes are auditable

**Core Systems:**
- Double-helix genome modeling
- Allele-based trait inheritance
- Mendelian reproduction mechanics
- Mutation with bounded rates
- Lineage tracking and generation indexing

**Founder Amplification Enforcement:**
- Founder amplification bounds are NOT inherited
- Offspring always start with baseline human capabilities
- Genetic validation prevents amplification leakage

**Audit Requirements:**
- All reproduction events logged with parent genetics
- Mutation events logged with RNG stream and seed
- Lineage tracking enables complete ancestry tracing
- Genetic hash contributions included in world hash

This crate enforces the genetic parity requirement from
HUMAN_EQUIVALENCE_AND_AGENT_PARITY_GOVERNING_LAW.md
*/

pub mod genome;
pub mod inheritance;
pub mod reproduction;
pub mod lineage;
pub mod mutation;

// Re-export primary interfaces
pub use genome::{Genome, Chromosome, Gene, Locus, Allele};
pub use inheritance::{MendelianInheritance, InheritancePattern, Dominance};
pub use reproduction::{ReproductionSystem, ReproductionResult, ConceptionOutcome};
pub use lineage::{LineageTracker, GenerationIndex, AgentLineage};
pub use mutation::{MutationSystem, MutationEvent, MutationType};
