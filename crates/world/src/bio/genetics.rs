/*!
# Genetics and Reproduction System

**Purpose:** Implement deterministic Mendelian genetics and human-equivalent reproduction.

**Why it exists:** Human equivalence law requires complete genetic system with
Mendelian inheritance, mutation, and lineage tracking.

**Determinism guarantees:**
- All genetic operations use seeded RNG streams
- Allele inheritance follows deterministic Mendelian ratios
- Mutation rates are bounded and logged
- No genetic drift or random variations outside rules

**How it affects replay:** Same parents and seed will produce
identical offspring genetics across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::bio::complete_biology::BiologicalState;

/// Complete double-helix genome
/// Models human genome with chromosomes and alleles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genome {
    /// Total base pairs (human genome ~3 billion)
    pub total_base_pairs: u64,
    /// Chromosome count (46 for humans)
    pub chromosome_count: u8,
    /// All genetic loci with alleles
    pub loci: BTreeMap<String, Locus>,
    /// Mutation history
    pub mutation_history: Vec<MutationEvent>,
    /// Parental genetic sources
    pub parental_genetics: ParentalGenetics,
    /// Generation number (0 = founders)
    pub generation: u64,
}

impl Genome {
    /// Create new founder genome (divine creation)
    pub fn new_founder(genome_id: &str, rng: &mut crate::rng::RngStream) -> Self {
        let mut loci = BTreeMap::new();
        
        // Create founder alleles (homozygous for all traits)
        loci.insert("HAIR_COLOR".to_string(), Locus::new_founder("HAIR_COLOR", "brown", "brown"));
        loci.insert("EYE_COLOR".to_string(), Locus::new_founder("EYE_COLOR", "blue", "blue"));
        loci.insert("HEIGHT".to_string(), Locus::new_founder("HEIGHT", "tall", "tall"));
        loci.insert("SKIN_COLOR".to_string(), Locus::new_founder("SKIN_COLOR", "light", "light"));
        loci.insert("INTELLIGENCE".to_string(), Locus::new_founder("INTELLIGENCE", "high", "high"));
        loci.insert("STRENGTH".to_string(), Locus::new_founder("STRENGTH", "average", "average"));
        loci.insert("FERTILITY".to_string(), Locus::new_founder("FERTILITY", "high", "high"));
        loci.insert("LONGEVITY".to_string(), Locus::new_founder("LONGEVITY", "average", "average"));
        
        Self {
            total_base_pairs: 3_000_000_000,
            chromosome_count: 46,
            loci,
            mutation_history: Vec::new(),
            parental_genetics: ParentalGenetics::divine_creation(),
            generation: 0,
        }
    }
    
    /// Create offspring genome through sexual reproduction
    pub fn new_offspring(
        mother_genome: &Genome,
        father_genome: &Genome,
        rng: &mut crate::rng::RngStream,
        generation: u64,
    ) -> Self {
        let mut loci = BTreeMap::new();
        let mut mutation_history = Vec::new();
        
        // Mendelian inheritance for each locus
        for (locus_id, mother_locus) in &mother_genome.loci {
            if let Some(father_locus) = father_genome.loci.get(locus_id) {
                let offspring_locus = Locus::mendelian_inheritance(
                    mother_locus,
                    father_locus,
                    rng,
                    generation,
                );
                
                // Check for mutations
                let mutated_locus = if rng.next_range(0, 1000) < 2 { // 0.2% mutation rate
                    let mutation = MutationEvent::new(
                        locus_id.clone(),
                        offspring_locus.alleles[0].clone(),
                        rng,
                        generation,
                    );
                    mutation_history.push(mutation.clone());
                    Locus::apply_mutation(&offspring_locus, &mutation)
                } else {
                    offspring_locus
                };
                
                loci.insert(locus_id.clone(), mutated_locus);
            }
        }
        
        Self {
            total_base_pairs: 3_000_000_000,
            chromosome_count: 46,
            loci,
            mutation_history,
            parental_genetics: ParentalGenetics::sexual_reproduction(
                mother_genome.parental_genetics.get_agent_id(),
                father_genome.parental_genetics.get_agent_id(),
            ),
            generation,
        }
    }
    
    /// Get expressed phenotype for a trait
    pub fn get_phenotype(&self, trait_id: &str) -> Option<String> {
        self.loci.get(trait_id).map(|locus| locus.expression.clone())
    }
    
    /// Check if genome has specific allele
    pub fn has_allele(&self, trait_id: &str, allele: &str) -> bool {
        self.loci.get(trait_id)
            .map(|locus| locus.alleles.contains(&allele.to_string()))
            .unwrap_or(false)
    }
}

/// Genetic locus with alleles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locus {
    /// Locus identifier
    pub locus_id: String,
    /// Two alleles (diploid)
    pub alleles: [String; 2],
    /// Dominance relationship
    pub dominance: DominancePattern,
    /// Expressed phenotype
    pub expression: String,
}

impl Locus {
    /// Create founder locus (homozygous)
    pub fn new_founder(locus_id: &str, allele: &str, expression: &str) -> Self {
        Self {
            locus_id: locus_id.to_string(),
            alleles: [allele.to_string(), allele.to_string()],
            dominance: DominancePattern::CompleteDominance,
            expression: expression.to_string(),
        }
    }
    
    /// Mendelian inheritance from parents
    pub fn mendelian_inheritance(
        mother_locus: &Locus,
        father_locus: &Locus,
        rng: &mut crate::rng::RngStream,
        generation: u64,
    ) -> Self {
        // Each parent contributes one allele
        let mother_allele = if rng.next_range(0, 2) == 0 {
            &mother_locus.alleles[0]
        } else {
            &mother_locus.alleles[1]
        };
        
        let father_allele = if rng.next_range(0, 2) == 0 {
            &father_locus.alleles[0]
        } else {
            &father_locus.alleles[1]
        };
        
        let alleles = [mother_allele.clone(), father_allele.clone()];
        
        // Determine expression based on dominance
        let expression = Self::determine_expression(&alleles, &mother_locus.dominance);
        
        Self {
            locus_id: mother_locus.locus_id.clone(),
            alleles,
            dominance: mother_locus.dominance.clone(),
            expression,
        }
    }
    
    /// Determine phenotype expression from alleles
    fn determine_expression(alleles: &[String; 2], dominance: &DominancePattern) -> String {
        match dominance {
            DominancePattern::CompleteDominance => {
                // First allele is dominant
                alleles[0].clone()
            },
            DominancePattern::IncompleteDominance => {
                // Heterozygous shows intermediate
                if alleles[0] == alleles[1] {
                    alleles[0].clone()
                    "Recessive".to_string()
                } else {
                    alleles[0].clone()
                }
            },
        }
    }
    
    /// Apply mutation to locus
    pub fn apply_mutation(original: &Locus, mutation: &MutationEvent) -> Self {
        alleles[allele_index] = mutation.new_allele.clone();
        
        // Update expression
        let expression = Self::determine_expression(&alleles, &original.dominance);
        
        Self {
            locus_id: original.locus_id.clone(),
            alleles,
            dominance: original.dominance.clone(),
            expression,
        }
    }
    
    /// Get expressed phenotype
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

/// Dominance patterns for genetic expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DominancePattern {
    /// First allele completely dominant
    CompleteDominance,
    /// Heterozygous shows intermediate
    IncompleteDominance,
    /// Both alleles expressed equally
    Codominant,
    /// Recessive allele only expressed when homozygous
    Recessive,
}

/// Mutation event tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationEvent {
    /// Locus that mutated
    pub locus_id: String,
    /// Original allele before mutation
    pub original_allele: String,
    /// New allele after mutation
    pub new_allele: String,
    /// Type of mutation
    pub mutation_type: MutationType,
    /// Generation when mutation occurred
    pub generation: u64,
    /// Tick when mutation occurred
    pub tick: u64,
}

impl MutationEvent {
    /// Create new mutation event
    pub fn new(
        locus_id: &str,
        original_allele: &str,
        rng: &mut crate::rng::RngStream,
        generation: u64,
    ) -> Self {
        let mutation_type = match rng.next_range(0, 4) {
            0 => MutationType::PointMutation,
            1 => MutationType::Insertion,
            2 => MutationType::Deletion,
            _ => MutationType::Substitution,
        };
        
        Self {
            locus_id: locus_id.to_string(),
            original_allele: original_allele.to_string(),
            new_allele: Self::generate_mutated_allele(&mutation_type, original_allele, rng),
            mutation_type,
            generation,
            tick: 0, // Will be set by reproduction system
        }
    }
    
    /// Generate mutated allele
    fn generate_mutated_allele(
        mutation_type: &MutationType,
        original: &str,
        rng: &mut crate::rng::RngStream,
    ) -> String {
        match mutation_type {
            MutationType::PointMutation => {
                // Single nucleotide change
                format!("{}-mut", original)
            },
            MutationType::Insertion => {
                // Nucleotide insertion
                format!("{}-ins-{}", original, rng.next_range(1, 100))
            },
            MutationType::Deletion => {
                // Nucleotide deletion
                format!("{}-del", original)
            },
            MutationType::Substitution => {
                // Nucleotide substitution
                format!("{}-sub-{}", original, rng.next_range(1, 4))
            },
        }
    }
}

/// Mutation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MutationType {
    /// Single nucleotide change
    PointMutation,
    /// Nucleotide insertion
    Insertion,
    /// Nucleotide deletion
    Deletion,
    /// Nucleotide substitution
    Substitution,
}

/// Parental genetic tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentalGenetics {
    /// Creation method
    pub creation_method: CreationMethod,
    /// Mother agent ID (if applicable)
    pub mother_id: Option<String>,
    /// Father agent ID (if applicable)
    pub father_id: Option<String>,
}

impl ParentalGenetics {
    /// Divine creation (founders)
    pub fn divine_creation() -> Self {
        Self {
            creation_method: CreationMethod::DivineCreation,
            mother_id: None,
            father_id: None,
        }
    }
    
    /// Sexual reproduction
    pub fn sexual_reproduction(mother_id: &str, father_id: &str) -> Self {
        Self {
            creation_method: CreationMethod::NaturalReproduction,
            mother_id: Some(mother_id.to_string()),
            father_id: Some(father_id.to_string()),
            environmental_factors: BTreeMap::new(),
        }
    }
    
    /// Set environmental factors
    pub fn set_environmental_factors(&mut self, factors: BTreeMap<String, f32>) {
        self.environmental_factors = factors.clone();
        debug!("Set environmental factors: {:?}", factors);
    }
    
    /// Get agent ID for tracking
    pub fn get_agent_id(&self) -> String {
        match self.creation_method {
            CreationMethod::DivineCreation => "divine".to_string(),
            CreationMethod::NaturalReproduction => {
                format!("{}-{}", 
                    self.mother_id.as_ref().unwrap_or("unknown"),
                    self.father_id.as_ref().unwrap_or("unknown"))
            }
        }
    }
}

/// Creation method enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreationMethod {
    /// Divine creation (founders)
    DivineCreation,
    /// Natural sexual reproduction
    NaturalReproduction,
}

/// Reproduction system
/// Manages agent reproduction with biological constraints
pub struct ReproductionSystem {
    /// Minimum biological requirements for reproduction
    pub biological_requirements: BiologicalRequirements,
    /// Fertility calculations
    pub fertility_calculator: FertilityCalculator,
}

impl ReproductionSystem {
    /// Create new reproduction system
    pub fn new() -> Self {
        Self {
            biological_requirements: BiologicalRequirements::new(),
            fertility_calculator: FertilityCalculator::new(),
        }
    }
    
    /// Check if reproduction is possible between two agents
    pub fn check_reproduction_viability(
        &self,
        mother: &BiologicalState,
        father: &BiologicalState,
        rng: &mut crate::rng::RngStream,
    ) -> Result<ReproductionViability, ReproductionBlockReason> {
        // Check biological requirements
        if !self.biological_requirements.meets_requirements(mother, father) {
            return Err(ReproductionBlockReason::BiologicalRequirements);
        }
        
        // Calculate fertility
        let mother_fertility = self.fertility_calculator.calculate_fertility(mother);
        let father_fertility = self.fertility_calculator.calculate_fertility(father);
        
        // Check fertility threshold
        if mother_fertility < to_fixed(50.0) || father_fertility < to_fixed(50.0) {
            return Err(ReproductionBlockReason::LowFertility);
        }
        
        // Probabilistic conception check
        let conception_probability = (mother_fertility + father_fertility) / 200; // Combined fertility
        if rng.next_range(0, 10000) > (conception_probability * 100) as u64 {
            return Err(ReproductionBlockReason::ConceptionFailure);
        }
        
        Ok(ReproductionViability {
            probability_of_success: conception_probability,
            gestation_period: 2700, // 27 days in ticks (100 ticks/day)
            offspring_count: 1, // Single birth for humans
        })
    }
    
    /// Create offspring from parents
    pub fn create_offspring(
        &self,
        mother_id: &str,
        father_id: &str,
        mother_genome: &Genome,
        father_genome: &Genome,
        rng: &mut crate::rng::RngStream,
        generation: u64,
    ) -> OffspringResult {
        // Create offspring genome
        let offspring_genome = Genome::new_offspring(
            mother_genome,
            father_genome,
            rng,
            generation + 1,
        );
        
        // Generate offspring ID
        let offspring_id = format!("gen{}-{}", generation + 1, rng.next_range(1000, 9999));
        
        // Create derived biological state (baseline human)
        let mut biological_state = BiologicalState::new_human_equivalent();
        
        // Apply genetic influences on biology
        Self::apply_genetic_influences(&mut biological_state, &offspring_genome);
        
        OffspringResult {
            agent_id: offspring_id,
            genome: offspring_genome,
            biological_state,
            creation_type: CreationMethod::NaturalReproduction,
        }
    }
    
    /// Apply genetic influences to biological state
    fn apply_genetic_influences(biological_state: &mut BiologicalState, genome: &Genome) {
        // Height affects strength and health
        if let Some(height) = genome.get_phenotype("HEIGHT") {
            match height.as_str() {
                "tall" => {
                    biological_state.health = (biological_state.health + to_fixed(5.0)).min(to_fixed(100.0));
                },
                "short" => {
                    biological_state.health = (biological_state.health - to_fixed(5.0)).max(to_fixed(80.0));
                },
                _ => {}
            }
        }
        
        // Intelligence affects metabolic rate
        if let Some(intelligence) = genome.get_phenotype("INTELLIGENCE") {
            match intelligence.as_str() {
                "high" => {
                    biological_state.metabolic_rate = (biological_state.metabolic_rate - to_fixed(10.0)).max(to_fixed(80.0));
                },
                "low" => {
                    biological_state.metabolic_rate = (biological_state.metabolic_rate + to_fixed(10.0)).min(to_fixed(120.0));
                },
                _ => {}
            }
        }
        
        // Fertility affects reproductive system
        if let Some(fertility) = genome.get_phenotype("FERTILITY") {
            match fertility.as_str() {
                "high" => {
                    biological_state.endocrine.gonadal.fertility_status = super::bio::complete_biology::FertilityStatus::Fertile;
                },
                "low" => {
                    biological_state.endocrine.gonadal.fertility_status = super::bio::complete_biology::FertilityStatus::Infertile;
                },
                _ => {}
            }
        }
        
        debug!("Applied genetic influences to offspring biology");
    }
}

/// Biological requirements for reproduction
pub struct BiologicalRequirements {
    /// Minimum age for reproduction
    pub minimum_age_ticks: u64,
    /// Minimum health threshold
    pub minimum_health: BioFixed,
    /// Maximum stress threshold
    pub maximum_stress: BioFixed,
    /// Energy requirement
    pub minimum_energy: BioFixed,
}

impl BiologicalRequirements {
    pub fn new() -> Self {
        Self {
            minimum_age_ticks: 365000, // ~10 years in ticks
            minimum_health: to_fixed(70.0), // 70% health minimum
            maximum_stress: to_fixed(60.0), // Stress must be below 60%
            minimum_energy: to_fixed(40.0), // 40% energy minimum
        }
    }
    
    /// Check if agents meet biological requirements
    pub fn meets_requirements(&self, mother: &BiologicalState, father: &BiologicalState) -> bool {
        mother.health >= self.minimum_health &&
            father.health >= self.minimum_health &&
            mother.endocrine.get_hormonal_effects().stress_response <= self.maximum_stress &&
            father.endocrine.get_hormonal_effects().stress_response <= self.maximum_stress &&
            mother.energy >= self.minimum_energy &&
            father.energy >= self.minimum_energy
    }
}

/// Fertility calculator
pub struct FertilityCalculator {
    /// Base fertility rate
    pub base_fertility: BioFixed,
}

impl FertilityCalculator {
    pub fn new() -> Self {
        Self {
            base_fertility: to_fixed(80.0), // 80% base fertility
        }
    }
    
    /// Calculate fertility based on biological state
    pub fn calculate_fertility(&self, biological_state: &BiologicalState) -> BioFixed {
        let mut fertility = self.base_fertility;
        
        // Age affects fertility (simplified)
        let age_factor = if biological_state.reproductive_tick < 182500 { // < 5 years
            to_fixed(0.0) // Pre-pubescent
        } else if biological_state.reproductive_tick > 1095000 { // > 30 years
            to_fixed(50.0) // Reduced fertility
        } else {
            to_fixed(100.0) // Peak fertility
        };
        
        // Health affects fertility
        let health_factor = biological_state.health;
        
        // Hormonal status affects fertility
        let hormonal_factor = match biological_state.endocrine.gonadal.fertility_status {
            super::bio::complete_biology::FertilityStatus::PeakFertility => to_fixed(150.0),
            super::bio::complete_biology::FertilityStatus::Fertile => to_fixed(100.0),
            super::bio::complete_biology::FertilityStatus::Menstruating => to_fixed(60.0),
            super::bio::complete_biology::FertilityStatus::Infertile => to_fixed(0.0),
        };
        
        // Combined fertility calculation
        fertility = fertility * age_factor / 10000;
        fertility = fertility * health_factor / 10000;
        fertility = fertility * hormonal_factor / 10000;
        
        fertility.min(to_fixed(100.0))
    }
}

/// Reproduction viability result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionViability {
    /// Probability of successful conception
    pub probability_of_success: BioFixed,
    /// Gestation period in ticks
    pub gestation_period: u64,
    /// Number of offspring
    pub offspring_count: u64,
}

/// Reproduction block reasons
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReproductionBlockReason {
    /// Biological requirements not met
    BiologicalRequirements,
    /// Low fertility
    LowFertility,
    /// Conception failed (probabilistic)
    ConceptionFailure,
}

/// Offspring creation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffspringResult {
    /// New agent ID
    pub agent_id: String,
    /// Complete genome
    pub genome: Genome,
    /// Biological state
    pub biological_state: BiologicalState,
    /// Creation method
    pub creation_type: CreationMethod,
}

// Import from complete_biology module
use super::bio::complete_biology::{to_fixed, BioFixed};
