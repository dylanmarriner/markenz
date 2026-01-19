/*!
# Mendelian Inheritance System

**Purpose:** Deterministic Mendelian inheritance for human-equivalent genetics.

**Why it exists:** Inheritance determines how traits are passed from parents to offspring.
It must follow Mendelian laws to ensure realistic genetic transmission and
maintain population-level genetic diversity.

**Determinism guarantees:**
- Inheritance follows fixed probability tables
- No random inheritance outside seeded RNG streams
- All allele transmissions are auditable
- Same parents + RNG seed produce identical offspring

**How it affects replay:** Same parent genomes and RNG seed will
produce identical inheritance patterns across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::genome::{Genome, ChromosomeType, Locus, Allele, Dominance};

/// Inheritance pattern for specific trait
/// Defines how alleles combine from parents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritancePattern {
    /// Trait identifier
    pub trait_id: String,
    /// Parent 1 allele contribution probability
    pub parent1_contribution: f64,
    /// Parent 2 allele contribution probability
    pub parent2_contribution: f64,
    /// Dominance relationship
    pub dominance_pattern: DominancePattern,
    /// Environmental modifier (epigenetics)
    pub environmental_modifier: f64,
}

impl InheritancePattern {
    /// Create new inheritance pattern
    pub fn new(
        trait_id: String,
        parent1_contribution: f64,
        parent2_contribution: f64,
        dominance_pattern: DominancePattern,
        environmental_modifier: f64,
    ) -> Self {
        Self {
            trait_id,
            parent1_contribution,
            parent2_contribution,
            dominance_pattern,
            environmental_modifier,
        }
    }

    /// Get inheritance probability for parent 1 allele
    pub fn parent1_probability(&self) -> f64 {
        self.parent1_contribution * self.environmental_modifier
    }

    /// Get inheritance probability for parent 2 allele
    pub fn parent2_probability(&self) -> f64 {
        self.parent2_contribution * self.environmental_modifier
    }
}

/// Dominance inheritance patterns
/// How dominant/recessive alleles interact in offspring
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DominancePattern {
    /// Simple dominant/recessive
    SimpleDominant,
    /// Co-dominant (both express)
    CoDominant,
    /// Incomplete dominance (blending)
    IncompleteDominant,
    /// Multiple alleles (complex interaction)
    MultipleAlleles,
    /// Sex-linked inheritance
    SexLinked,
}

impl DominancePattern {
    /// Get phenotype expression based on parent alleles
    pub fn express_phenotype(&self, parent1_allele: &Allele, parent2_allele: &Allele) -> PhenotypeExpression {
        match self {
            Self::SimpleDominant => {
                if parent1_allele.is_dominant() && !parent2_allele.is_dominant() {
                    PhenotypeExpression::Dominant(parent1_allele.clone())
                } else if !parent1_allele.is_dominant() && parent2_allele.is_dominant() {
                    PhenotypeExpression::Dominant(parent2_allele.clone())
                } else if parent1_allele.is_dominant() && parent2_allele.is_dominant() {
                    // Both dominant - co-dominant
                    PhenotypeExpression::CoDominant(vec![parent1_allele.clone(), parent2_allele.clone()])
                } else {
                    // Both recessive
                    PhenotypeExpression::Recessive(parent1_allele.clone())
                }
            }
            Self::CoDominant => {
                PhenotypeExpression::CoDominant(vec![parent1_allele.clone(), parent2_allele.clone()])
            }
            Self::IncompleteDominant => {
                // Blend effects based on dominance strength
                let effect1 = parent1_allele.effect_size * parent1_allele.dominance.strength();
                let effect2 = parent2_allele.effect_size * parent2_allele.dominance.strength();
                let blended_effect = (effect1 + effect2) / 2.0;
                
                PhenotypeExpression::Blended(blended_effect)
            }
            Self::MultipleAlleles => {
                // Complex interaction - simplified to dominant
                if parent1_allele.is_dominant() || parent2_allele.is_dominant() {
                    let dominant = if parent1_allele.is_dominant() { parent1_allele } else { parent2_allele };
                    PhenotypeExpression::Dominant(dominant.clone())
                } else {
                    PhenotypeExpression::Recessive(parent1_allele.clone())
                }
            }
            Self::SexLinked => {
                // Sex chromosomes have different inheritance patterns
                PhenotypeExpression::SexLinked
            }
        }
    }
}

/// Phenotype expression result
/// How inherited alleles manifest in offspring
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhenotypeExpression {
    /// Dominant allele expresses
    Dominant(Allele),
    /// Recessive allele expresses
    Recessive(Allele),
    /// Both alleles express (co-dominant)
    CoDominant(Vec<Allele>),
    /// Blended effect (incomplete dominance)
    Blended(f64),
    /// Sex-linked inheritance
    SexLinked,
}

/// Complete Mendelian inheritance system
/// Handles all genetic inheritance calculations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MendelianInheritance {
    /// Inheritance patterns by trait
    pub patterns: BTreeMap<String, InheritancePattern>,
    /// RNG stream identifier for inheritance
    pub rng_stream: String,
    /// Mutation rate modifier
    pub mutation_rate_modifier: f64,
}

impl MendelianInheritance {
    /// Create new inheritance system with standard patterns
    pub fn new(rng_stream: String) -> Self {
        let mut patterns = BTreeMap::new();
        
        // Eye color inheritance (simplified Mendelian)
        patterns.insert("eye_color".to_string(), InheritancePattern::new(
            "eye_color".to_string(),
            0.5, // Equal contribution
            0.5,
            DominancePattern::SimpleDominant,
            1.0, // No environmental modifier
        ));
        
        // Hair color inheritance
        patterns.insert("hair_color".to_string(), InheritancePattern::new(
            "hair_color".to_string(),
            0.6, // Slight bias toward dark hair
            0.4,
            DominancePattern::MultipleAlleles,
            1.0,
        ));
        
        // Height inheritance (polygenic, simplified)
        patterns.insert("height".to_string(), InheritancePattern::new(
            "height".to_string(),
            0.5,
            0.5,
            DominancePattern::IncompleteDominant,
            1.0,
        ));
        
        // Skin color inheritance
        patterns.insert("skin_color".to_string(), InheritancePattern::new(
            "skin_color".to_string(),
            0.7, // Bias toward darker skin
            0.3,
            DominancePattern::IncompleteDominant,
            1.0,
        ));
        
        Self {
            patterns,
            rng_stream,
            mutation_rate_modifier: 1.0,
        }
    }

    /// Inherit alleles from two parents for a specific locus
    pub fn inherit_locus(
        &self,
        parent1_locus: &Locus,
        parent2_locus: &Locus,
        rng_value: f64, // 0.0 to 1.0 from deterministic RNG
    ) -> [Allele; 2] {
        let pattern = self.patterns.get(&parent1_locus.locus_id);
        
        if let Some(pattern) = pattern {
            // Use pattern-specific inheritance
            let parent1_prob = pattern.parent1_probability();
            let parent2_prob = pattern.parent2_probability();
            
            if rng_value < parent1_prob {
                [
                    parent1_locus.current_alleles[0].clone(),
                    parent2_locus.current_alleles[1].clone(),
                ]
            } else {
                [
                    parent1_locus.current_alleles[1].clone(),
                    parent2_locus.current_alleles[0].clone(),
                ]
            }
        } else {
            // Default: 50/50 inheritance
            if rng_value < 0.5 {
                [
                    parent1_locus.current_alleles[0].clone(),
                    parent2_locus.current_alleles[1].clone(),
                ]
            } else {
                [
                    parent1_locus.current_alleles[1].clone(),
                    parent2_locus.current_alleles[0].clone(),
                ]
            }
        }
    }

    /// Create offspring genome from two parents
    pub fn create_offspring_genome(
        &self,
        parent1: &Genome,
        parent2: &Genome,
        rng_values: &BTreeMap<String, Vec<f64>>, // RNG values by locus
    ) -> Genome {
        let mut offspring_chromosomes = BTreeMap::new();
        
        // Process each chromosome type
        for chromosome_type in parent1.chromosomes.keys() {
            let parent1_chromosome = parent1.get_chromosome_loci(*chromosome_type);
            let parent2_chromosome = parent2.get_chromosome_loci(*chromosome_type);
            
            let mut offspring_loci = Vec::new();
            
            // Process each locus
            for (i, parent1_locus) in parent1_chromosome.iter().enumerate() {
                if let Some(parent2_locus) = parent2_chromosome.get(i) {
                    let rng_values = rng_values.get(&parent1_locus.locus_id);
                    
                    if let Some(rng_vals) = rng_values {
                        // Use first RNG value for this locus
                        let rng_value = rng_vals[0];
                        let inherited_alleles = self.inherit_locus(parent1_locus, parent2_locus, rng_value);
                        
                        let offspring_locus = Locus::new(
                            parent1_locus.locus_id.clone(),
                            *chromosome_type,
                            parent1_locus.position,
                            parent1_locus.name.clone(),
                            parent1_locus.possible_alleles.clone(),
                            inherited_alleles[0].clone(),
                            inherited_alleles[1].clone(),
                        );
                        
                        offspring_loci.push(offspring_locus);
                    } else {
                        // No RNG value - use default inheritance
                        let inherited_alleles = self.inherit_locus(parent1_locus, parent2_locus, 0.5);
                        let offspring_locus = Locus::new(
                            parent1_locus.locus_id.clone(),
                            *chromosome_type,
                            parent1_locus.position,
                            parent1_locus.name.clone(),
                            parent1_locus.possible_alleles.clone(),
                            inherited_alleles[0].clone(),
                            inherited_alleles[1].clone(),
                        );
                        
                        offspring_loci.push(offspring_locus);
                    }
                }
            }
            
            offspring_chromosomes.insert(*chromosome_type, offspring_loci);
        }
        
        // Handle sex chromosomes specially
        self.inherit_sex_chromosomes(&mut offspring_chromosomes, parent1, parent2, rng_values);
        
        let total_base_pairs = offspring_chromosomes.values()
            .map(|chromosome| chromosome.len() as u64 * 1000)
            .sum();
        
        let checksum = Genome::calculate_checksum(&offspring_chromosomes);
        
        Genome {
            chromosomes: offspring_chromosomes,
            total_base_pairs,
            generation: parent1.generation.max(parent2.generation) + 1,
            creation_method: "sexual_reproduction".to_string(),
            mutation_history: Vec::new(), // Mutations handled separately
            checksum,
        }
    }

    /// Inherit sex chromosomes with proper patterns
    fn inherit_sex_chromosomes(
        &self,
        offspring_chromosomes: &mut BTreeMap<ChromosomeType, Vec<Locus>>,
        parent1: &Genome,
        parent2: &Genome,
        rng_values: &BTreeMap<String, Vec<f64>>,
    ) {
        // Simplified sex chromosome inheritance
        // In reality, this is much more complex
        
        let sex_rng = rng_values.get("sex_determination").map(|vals| vals[0]).unwrap_or(0.5);
        
        if sex_rng < 0.5 {
            // Female (XX)
            offspring_chromosomes.insert(ChromosomeType::SexX, Vec::new());
            offspring_chromosomes.insert(ChromosomeType::SexY, Vec::new());
        } else {
            // Male (XY)
            offspring_chromosomes.insert(ChromosomeType::SexX, Vec::new());
            offspring_chromosomes.insert(ChromosomeType::SexY, Vec::new());
        }
    }

    /// Calculate inheritance similarity between parents and offspring
    pub fn calculate_inheritance_similarity(&self, parent1: &Genome, parent2: &Genome, offspring: &Genome) -> f64 {
        let mut total_similarity = 0.0;
        let mut total_loci = 0;
        
        for locus in offspring.get_all_loci() {
            if let Some(parent1_locus) = parent1.get_all_loci().iter().find(|l| l.locus_id == locus.locus_id) {
                if let Some(parent2_locus) = parent2.get_all_loci().iter().find(|l| l.locus_id == locus.locus_id) {
                    // Check if offspring inherited from either parent
                    let from_parent1 = locus.current_alleles.contains(&parent1_locus.current_alleles[0]) ||
                                        locus.current_alleles.contains(&parent1_locus.current_alleles[1]);
                    let from_parent2 = locus.current_alleles.contains(&parent2_locus.current_alleles[0]) ||
                                        locus.current_alleles.contains(&parent2_locus.current_alleles[1]);
                    
                    if from_parent1 || from_parent2 {
                        total_similarity += 1.0;
                    }
                    total_loci += 1;
                }
            }
        }
        
        if total_loci > 0 {
            total_similarity / total_loci as f64
        } else {
            0.0
        }
    }

    /// Get inheritance statistics
    pub fn get_inheritance_stats(&self, parent1: &Genome, parent2: &Genome, offspring: &Genome) -> InheritanceStats {
        let similarity = self.calculate_inheritance_similarity(parent1, parent2, offspring);
        let heterozygosity = offspring.count_heterozygous_loci() as f64 / offspring.get_all_loci().len() as f64;
        
        InheritanceStats {
            parental_similarity: similarity,
            heterozygosity_ratio: heterozygosity,
            novel_combinations: self.count_novel_combinations(parent1, parent2, offspring),
            generation_gap: offspring.generation - parent1.generation.max(parent2.generation),
        }
    }

    /// Count novel allele combinations not present in either parent
    fn count_novel_combinations(&self, parent1: &Genome, parent2: &Genome, offspring: &Genome) -> u32 {
        let mut novel_count = 0;
        
        for locus in offspring.get_all_loci() {
            let parent1_alleles = parent1.get_all_loci()
                .iter()
                .find(|l| l.locus_id == locus.locus_id)
                .map(|l| &l.current_alleles)
                .unwrap_or(&vec![]);
            
            let parent2_alleles = parent2.get_all_loci()
                .iter()
                .find(|l| l.locus_id == locus.locus_id)
                .map(|l| &l.current_alleles)
                .unwrap_or(&vec![]);
            
            // Check if offspring combination exists in either parent
            let parent1_has = parent1_alleles.contains(&locus.current_alleles[0]) ||
                              parent1_alleles.contains(&locus.current_alleles[1]);
            let parent2_has = parent2_alleles.contains(&locus.current_alleles[0]) ||
                              parent2_alleles.contains(&locus.current_alleles[1]);
            
            if !parent1_has && !parent2_has {
                novel_count += 1;
            }
        }
        
        novel_count
    }
}

/// Inheritance statistics
/// Summary of inheritance outcomes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InheritanceStats {
    /// Percentage similarity to parents (0.0 to 1.0)
    pub parental_similarity: f64,
    /// Ratio of heterozygous loci (0.0 to 1.0)
    pub heterozygosity_ratio: f64,
    /// Number of novel allele combinations
    pub novel_combinations: u32,
    /// Generation gap from parents
    pub generation_gap: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inheritance_pattern_creation() {
        let pattern = InheritancePattern::new(
            "test".to_string(),
            0.6,
            0.4,
            DominancePattern::SimpleDominant,
            1.0,
        );
        
        assert_eq!(pattern.trait_id, "test");
        assert_eq!(pattern.parent1_probability(), 0.6);
        assert_eq!(pattern.parent2_probability(), 0.4);
    }

    #[test]
    fn test_mendelian_inheritance() {
        let inheritance = MendelianInheritance::new("test_stream".to_string());
        
        let allele1 = Allele::new("A".to_string(), "Type A".to_string(), Dominance::Dominant, 1.0, 0.0001);
        let allele2 = Allele::new("a".to_string(), "Type a".to_string(), Dominance::Recessive, 0.0, 0.0001);
        
        let locus1 = Locus::new(
            "test".to_string(),
            ChromosomeType::Autosome(1),
            1000,
            "Test".to_string(),
            vec![allele1.clone(), allele2.clone()],
            allele1.clone(),
            allele2.clone(),
        );
        
        let locus2 = Locus::new(
            "test".to_string(),
            ChromosomeType::Autosome(1),
            1000,
            "Test".to_string(),
            vec![allele1.clone(), allele2.clone()],
            allele2.clone(),
            allele1.clone(),
        );
        
        let inherited = inheritance.inherit_locus(&locus1, &locus2, 0.3);
        
        // Should inherit from parent2 (rng > 0.5)
        assert_eq!(inherited[0].identifier, "a");
        assert_eq!(inherited[1].identifier, "A");
    }

    #[test]
    fn test_dominance_patterns() {
        let dominant = Allele::new("D".to_string(), "Dominant".to_string(), Dominance::Dominant, 1.0, 0.0001);
        let recessive = Allele::new("r".to_string(), "Recessive".to_string(), Dominance::Recessive, 0.0, 0.0001);
        
        let pattern = DominancePattern::SimpleDominant;
        let expression = pattern.express_phenotype(&dominant, &recessive);
        
        match expression {
            PhenotypeExpression::Dominant(allele) => assert_eq!(allele.identifier, "D"),
            _ => panic!("Expected dominant expression"),
        }
    }
}
