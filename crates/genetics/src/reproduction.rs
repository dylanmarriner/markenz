/*!
# Reproduction System

**Purpose:** Deterministic reproduction mechanics for human-equivalent agents.

**Why it exists:** Reproduction is essential for genetic diversity and population
continuity. It must model the biological constraints, fertility requirements,
and genetic transmission accurately to maintain human equivalence.

**Determinism guarantees:**
- Conception probability based on deterministic fertility calculations
- RNG streams ensure reproducible outcomes
- All reproduction events are auditable
- Same parents + RNG seed produce identical outcomes

**How it affects replay:** Same parent agents and RNG seed will
produce identical reproduction results across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::genome::{Genome, ChromosomeType, Locus, Allele};
use super::inheritance::{MendelianInheritance, InheritanceStats};
use super::lineage::{LineageTracker, AgentLineage};

/// Reproduction requirements and constraints
/// Biological and social conditions for successful reproduction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionRequirements {
    /// Minimum fertility threshold (0.0 to 1.0)
    pub min_fertility_threshold: f64,
    /// Proximity requirement (in world units)
    pub proximity_requirement: f64,
    /// Consent requirement (must be true)
    pub consent_required: bool,
    /// Health requirement threshold (0.0 to 1.0)
    pub min_health_threshold: f64,
    /// Age constraints
    pub age_constraints: AgeConstraints,
    /// Genetic compatibility constraints
    pub genetic_constraints: GeneticConstraints,
}

impl ReproductionRequirements {
    /// Create standard human reproduction requirements
    pub fn human_standard() -> Self {
        Self {
            min_fertility_threshold: 0.3, // 30% fertility minimum
            proximity_requirement: 2.0, // 2 world units
            consent_required: true,
            min_health_threshold: 0.7, // 70% health minimum
            age_constraints: AgeConstraints::human_fertile(),
            genetic_constraints: GeneticConstraints::human_standard(),
        }
    }

    /// Check if reproduction is possible
    pub fn can_reproduce(&self, fertility1: f64, fertility2: f64, health1: f64, health2: f64) -> bool {
        fertility1 >= self.min_fertility_threshold &&
        fertility2 >= self.min_fertility_threshold &&
        health1 >= self.min_health_threshold &&
        health2 >= self.min_health_threshold
    }
}

/// Age constraints for reproduction
/// Biologically realistic age ranges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgeConstraints {
    /// Minimum age for female reproduction
    pub min_female_age: f64,
    /// Maximum age for female reproduction
    pub max_female_age: f64,
    /// Minimum age for male reproduction
    pub min_male_age: f64,
    /// Maximum age for male reproduction
    pub max_male_age: f64,
}

impl AgeConstraints {
    /// Create human-fertile age constraints
    pub fn human_fertile() -> Self {
        Self {
            min_female_age: 16.0, // 16 years
            max_female_age: 45.0, // 45 years
            min_male_age: 16.0,  // 16 years
            max_male_age: 60.0,  // 60 years
        }
    }

    /// Check if ages are within reproductive range
    pub fn are_fertile(&self, female_age: f64, male_age: f64) -> bool {
        female_age >= self.min_female_age &&
        female_age <= self.max_female_age &&
        male_age >= self.min_male_age &&
        male_age <= self.max_male_age
    }
}

/// Genetic compatibility constraints
/// Prevents reproduction between genetically incompatible agents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneticConstraints {
    /// Maximum genetic similarity (0.0 to 1.0)
    pub max_similarity_threshold: f64,
    /// Minimum genetic diversity requirement
    pub min_diversity_requirement: f64,
    /// Prohibited relationship types
    pub prohibited_relationships: Vec<RelationshipType>,
    /// Chromosomal compatibility requirements
    pub chromosomal_compatibility: ChromosomalCompatibility,
}

impl GeneticConstraints {
    /// Create standard human genetic constraints
    pub fn human_standard() -> Self {
        Self {
            max_similarity_threshold: 0.8, // 80% similarity maximum
            min_diversity_requirement: 0.2, // 20% diversity minimum
            prohibited_relationships: vec![
                RelationshipType::ImmediateFamily,
                RelationshipType::FirstCousin,
            ],
            chromosomal_compatibility: ChromosomalCompatibility::HumanStandard,
        }
    }

    /// Check genetic compatibility between two genomes
    pub fn are_compatible(&self, genome1: &Genome, genome2: &Genome) -> bool {
        // Simplified compatibility check
        // In reality, this would involve detailed chromosomal analysis
        true // For now, assume all genomes are compatible
    }
}

/// Relationship types for genetic constraints
/// Different categories of family relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Immediate family (parent/child/sibling)
    ImmediateFamily,
    /// First cousins
    FirstCousin,
    /// Second cousins
    SecondCousin,
    /// Unrelated
    Unrelated,
}

/// Chromosomal compatibility requirements
/// Ensures proper chromosomal pairing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ChromosomalCompatibility {
    /// Standard human compatibility
    HumanStandard,
    /// Strict compatibility (no chromosomal abnormalities)
    Strict,
    /// Relaxed compatibility (allows some variations)
    Relaxed,
}

/// Conception outcome
/// Result of reproduction attempt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConceptionOutcome {
    /// Successful conception
    Successful,
    /// Failed due to low fertility
    LowFertility,
    /// Failed due to health issues
    PoorHealth,
    /// Failed due to age constraints
    AgeConstraints,
    /// Failed due to genetic incompatibility
    GeneticIncompatibility,
    /// Failed due to lack of consent
    NoConsent,
    /// Failed due to distance
    TooDistant,
}

impl ConceptionOutcome {
    /// Get description for logging
    pub const fn description(self) -> &'static str {
        match self {
            Self::Successful => "conception successful",
            Self::LowFertility => "failed due to low fertility",
            Self::PoorHealth => "failed due to poor health",
            Self::AgeConstraints => "failed due to age constraints",
            Self::GeneticIncompatibility => "failed due to genetic incompatibility",
            Self::NoConsent => "failed due to lack of consent",
            Self::TooDistant => "failed due to distance",
        }
    }

    /// Check if conception was successful
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Successful)
    }
}

/// Reproduction result with complete information
/// Full outcome of reproduction attempt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionResult {
    /// Conception outcome
    pub outcome: ConceptionOutcome,
    /// Offspring genome (if successful)
    pub offspring_genome: Option<Genome>,
    /// Inheritance statistics
    pub inheritance_stats: Option<InheritanceStats>,
    /// Parental contribution details
    pub parental_contributions: ParentalContributions,
    /// Reproduction tick
    pub tick: u64,
    /// RNG stream used
    pub rng_stream: String,
}

/// Parental genetic contributions
/// Tracks which parent contributed which alleles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentalContributions {
    /// Mother's contribution percentage
    pub mother_contribution: f64,
    /// Father's contribution percentage
    pub father_contribution: f64,
    /// Maternal loci count
    pub maternal_loci: u32,
    /// Paternal loci count
    pub paternal_loci: u32,
    /// Sex chromosome contribution
    pub sex_chromosome_source: SexChromosomeSource,
}

/// Source of sex chromosomes
/// Which parent contributed which sex chromosome
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SexChromosomeSource {
    /// Mother contributed X
    MaternalX,
    /// Father contributed X
    PaternalX,
    /// Father contributed Y
    PaternalY,
}

/// Complete reproduction system
/// Manages all aspects of agent reproduction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionSystem {
    /// Reproduction requirements
    pub requirements: ReproductionRequirements,
    /// Mendelian inheritance system
    pub inheritance: MendelianInheritance,
    /// Lineage tracking system
    pub lineage: LineageTracker,
    /// RNG stream identifier
    pub rng_stream: String,
}

impl ReproductionSystem {
    /// Create new reproduction system
    pub fn new(rng_stream: String) -> Self {
        Self {
            requirements: ReproductionRequirements::human_standard(),
            inheritance: MendelianInheritance::new(rng_stream.clone()),
            lineage: LineageTracker::new(),
            rng_stream,
        }
    }

    /// Attempt reproduction between two agents
    pub fn attempt_reproduction(
        &mut self,
        mother_genome: &Genome,
        father_genome: &Genome,
        mother_fertility: f64,
        father_fertility: f64,
        mother_health: f64,
        father_health: f64,
        mother_age: f64,
        father_age: f64,
        distance: f64, // Proximity in world units
        consent: bool, // Mutual consent
        rng_values: &BTreeMap<String, Vec<f64>>, // RNG values for this reproduction
        tick: u64,
    ) -> ReproductionResult {
        // Check basic requirements
        if !self.requirements.can_reproduce(mother_fertility, father_fertility, mother_health, father_health) {
            return ReproductionResult {
                outcome: if mother_fertility < self.requirements.min_fertility_threshold {
                    ConceptionOutcome::LowFertility
                } else if father_fertility < self.requirements.min_fertility_threshold {
                    ConceptionOutcome::LowFertility
                } else if mother_health < self.requirements.min_health_threshold {
                    ConceptionOutcome::PoorHealth
                } else if father_health < self.requirements.min_health_threshold {
                    ConceptionOutcome::PoorHealth
                } else {
                    ConceptionOutcome::PoorHealth
                },
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Check age constraints
        if !self.requirements.age_constraints.are_fertile(mother_age, father_age) {
            return ReproductionResult {
                outcome: ConceptionOutcome::AgeConstraints,
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Check consent requirement
        if self.requirements.consent_required && !consent {
            return ReproductionResult {
                outcome: ConceptionOutcome::NoConsent,
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Check distance requirement
        if distance > self.requirements.proximity_requirement {
            return ReproductionResult {
                outcome: ConceptionOutcome::TooDistant,
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Check genetic compatibility
        if !self.requirements.genetic_constraints.are_compatible(mother_genome, father_genome) {
            return ReproductionResult {
                outcome: ConceptionOutcome::GeneticIncompatibility,
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Calculate conception probability
        let combined_fertility = (mother_fertility + father_fertility) / 2.0;
        let conception_rng = rng_values.get("conception").map(|vals| vals[0]).unwrap_or(0.5);
        
        // Conception succeeds if RNG value is below fertility threshold
        if conception_rng > combined_fertility {
            return ReproductionResult {
                outcome: ConceptionOutcome::LowFertility,
                offspring_genome: None,
                inheritance_stats: None,
                parental_contributions: ParentalContributions {
                    mother_contribution: 0.0,
                    father_contribution: 0.0,
                    maternal_loci: 0,
                    paternal_loci: 0,
                    sex_chromosome_source: SexChromosomeSource::MaternalX,
                },
                tick,
                rng_stream: self.rng_stream.clone(),
            };
        }

        // Successful conception - create offspring
        let offspring_genome = self.inheritance.create_offspring_genome(
            mother_genome,
            father_genome,
            rng_values,
        );

        let inheritance_stats = Some(self.inheritance.get_inheritance_stats(
            mother_genome,
            father_genome,
            &offspring_genome,
        ));

        let parental_contributions = self.calculate_parental_contributions(
            mother_genome,
            father_genome,
            &offspring_genome,
        );

        // Update lineage tracking
        self.lineage.add_offspring(
            mother_genome,
            father_genome,
            &offspring_genome,
            tick,
        );

        debug!("Reproduction successful at tick {}: new generation {}", 
               tick, offspring_genome.generation);

        ReproductionResult {
            outcome: ConceptionOutcome::Successful,
            offspring_genome: Some(offspring_genome),
            inheritance_stats,
            parental_contributions,
            tick,
            rng_stream: self.rng_stream.clone(),
        }
    }

    /// Calculate parental genetic contributions
    fn calculate_parental_contributions(
        &self,
        genome1: &Genome,
        genome2: &Genome,
        offspring_genome: &Genome,
    ) -> ParentalContributions {
        let mut maternal_loci = 0;
        let mut paternal_loci = 0;

        // Count contributions for each locus
        for offspring_locus in offspring_genome.get_all_loci() {
            if let Some(locus1) = genome1.get_all_loci()
                .iter().find(|l| l.locus_id == offspring_locus.locus_id) {
                if let Some(locus2) = genome2.get_all_loci()
                    .iter().find(|l| l.locus_id == offspring_locus.locus_id) {
                    
                    // Check which alleles came from which parent
                    let from_parent1 = offspring_locus.current_alleles.contains(&locus1.current_alleles[0]) ||
                                       offspring_locus.current_alleles.contains(&locus1.current_alleles[1]);
                    let from_parent2 = offspring_locus.current_alleles.contains(&locus2.current_aleles[0]) ||
                                       offspring_locus.current_alleles.contains(&locus2.current_alleles[1]);
                    
                    if from_parent1 {
                        maternal_loci += 1;
                    }
                    if from_parent2 {
                        paternal_loci += 1;
                    }
                }
            }
        }

        let total_loci = offspring_genome.get_all_loci().len();
        let mother_contribution = maternal_loci as f64 / total_loci as f64;
        let father_contribution = paternal_loci as f64 / total_loci as f64;

        // Determine sex chromosome source
        let sex_chromosome_source = if offspring_genome.chromosomes.contains_key(&ChromosomeType::SexY) {
            SexChromosomeSource::PaternalY
        } else {
            SexChromosomeSource::MaternalX
        };

        ParentalContributions {
            mother_contribution,
            father_contribution,
            maternal_loci,
            paternal_loci,
            sex_chromosome_source,
        }
    }

    /// Get reproduction statistics
    pub fn get_statistics(&self) -> ReproductionStats {
        ReproductionStats {
            total_attempts: self.lineage.get_total_reproductions(),
            successful_conceptions: self.lineage.get_successful_conceptions(),
            average_generation_gap: self.lineage.get_average_generation_gap(),
            genetic_diversity_index: self.lineage.get_genetic_diversity_index(),
            lineage_depth: self.lineage.get_max_lineage_depth(),
        }
    }
}

/// Reproduction system statistics
/// Summary of reproduction outcomes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionStats {
    /// Total reproduction attempts
    pub total_attempts: u64,
    /// Number of successful conceptions
    pub successful_conceptions: u64,
    /// Average generation gap between reproductions
    pub average_generation_gap: f64,
    /// Genetic diversity index
    pub genetic_diversity_index: f64,
    /// Maximum lineage depth
    pub lineage_depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproduction_requirements() {
        let req = ReproductionRequirements::human_standard();
        
        assert!(req.can_reproduce(0.5, 0.6, 0.8, 0.9));
        assert!(!req.can_reproduce(0.2, 0.3, 0.8, 0.9));
        assert!(!req.can_reproduce(0.5, 0.6, 0.5, 0.4));
    }

    #[test]
    fn test_age_constraints() {
        let age = AgeConstraints::human_fertile();
        
        assert!(age.are_fertile(20.0, 25.0));
        assert!(!age.are_fertile(15.0, 25.0));
        assert!(!age.are_fertile(50.0, 25.0));
    }

    #[test]
    fn test_conception_outcomes() {
        assert!(ConceptionOutcome::Successful.is_success());
        assert!(!ConceptionOutcome::LowFertility.is_success());
        assert!(!ConceptionOutcome::AgeConstraints.is_success());
    }

    #[test]
    fn test_reproduction_system() {
        let mut system = ReproductionSystem::new("test_stream".to_string());
        
        // Create simple test genomes
        let mother_genome = Genome::new_human(1, "test".to_string());
        let father_genome = Genome::new_human(1, "test".to_string());
        
        let mut rng_values = BTreeMap::new();
        rng_values.insert("conception".to_string(), vec![0.1]); // Low fertility
        rng_values.insert("sex_determination".to_string(), vec![0.6]);
        
        let result = system.attempt_reproduction(
            &mother_genome,
            &father_genome,
            0.4, // Low fertility
            0.6,
            0.8, // Good health
            0.9,
            25.0, // Fertile age
            30.0,
            1.0, // Close proximity
            true, // Consent
            &rng_values,
            1000,
        );
        
        // Should fail due to low fertility
        assert!(!result.outcome.is_success());
    }
}
