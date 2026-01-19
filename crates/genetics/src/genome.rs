/*!
# Genome System

**Purpose:** Deterministic double-helix genome modeling for human-equivalent genetics.

**Why it exists:** The genome is the complete genetic blueprint that determines
all biological traits. It must be modeled accurately to ensure proper
inheritance, variation, and human-equivalent trait expression.

**Determinism guarantees:**
- Genome structure is fixed and deterministic
- Allele combinations follow predictable patterns
- No random genome generation outside seeded RNG
- All genome operations are auditable

**How it affects replay:** Same parent genomes and RNG seed will
produce identical offspring genomes across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};

/// Chromosome types in human genome
/// Humans have 23 pairs: 22 autosomes + 1 sex chromosome
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ChromosomeType {
    /// Autosomes 1-22
    Autosome(u8),
    /// Sex chromosomes (X and Y)
    SexX,
    SexY,
}

impl ChromosomeType {
    /// Get human-readable name
    pub const fn name(self) -> &'static str {
        match self {
            Self::Autosome(n) => match n {
                1 => "Chromosome 1",
                2 => "Chromosome 2",
                3 => "Chromosome 3",
                4 => "Chromosome 4",
                5 => "Chromosome 5",
                6 => "Chromosome 6",
                7 => "Chromosome 7",
                8 => "Chromosome 8",
                9 => "Chromosome 9",
                10 => "Chromosome 10",
                11 => "Chromosome 11",
                12 => "Chromosome 12",
                13 => "Chromosome 13",
                14 => "Chromosome 14",
                15 => "Chromosome 15",
                16 => "Chromosome 16",
                17 => "Chromosome 17",
                18 => "Chromosome 18",
                19 => "Chromosome 19",
                20 => "Chromosome 20",
                21 => "Chromosome 21",
                22 => "Chromosome 22",
                _ => "Unknown Autosome",
            },
            Self::SexX => "Chromosome X",
            Self::SexY => "Chromosome Y",
        }
    }

    /// Get total number of base pairs (approximate)
    pub const fn base_pairs(self) -> u64 {
        match self {
            Self::Autosome(n) => match n {
                1 => 249_250_621,
                2 => 243_199_373,
                3 => 198_295_559,
                4 => 191_273_063,
                5 => 180_915_260,
                6 => 171_115_067,
                7 => 159_138_663,
                8 => 146_364_022,
                9 => 141_213_431,
                10 => 135_534_747,
                11 => 135_006_522,
                12 => 133_275_309,
                13 => 114_364_328,
                14 => 107_349_540,
                15 => 102_531_392,
                16 => 90_358_034,
                17 => 83_257_441,
                18 => 80_373_285,
                19 => 58_617_616,
                20 => 63_025_520,
                21 => 48_129_216,
                22 => 50_817_468,
                _ => 50_000_000, // Average estimate
            },
            Self::SexX => 156_040_895,
            Self::SexY => 57_227_415,
        }
    }
}

/// Individual gene locus with specific alleles
/// Represents a specific location on a chromosome with possible variants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locus {
    /// Unique identifier for this locus
    pub locus_id: String,
    /// Chromosome where this locus is located
    pub chromosome: ChromosomeType,
    /// Position on chromosome (in base pairs)
    pub position: u64,
    /// Human-readable name/description
    pub name: String,
    /// Possible alleles at this locus
    pub possible_alleles: Vec<Allele>,
    /// Current allele pair (one from each parent)
    pub current_alleles: [Allele; 2],
}

impl Locus {
    /// Create a new locus with specified alleles
    pub fn new(
        locus_id: String,
        chromosome: ChromosomeType,
        position: u64,
        name: String,
        possible_alleles: Vec<Allele>,
        allele1: Allele,
        allele2: Allele,
    ) -> Self {
        Self {
            locus_id,
            chromosome,
            position,
            name,
            possible_alleles,
            current_alleles: [allele1, allele2],
        }
    }

    /// Get genotype as string representation
    pub fn genotype_string(&self) -> String {
        format!("{}/{}", 
                self.current_alleles[0].identifier,
                self.current_alleles[1].identifier)
    }

    /// Get phenotype expression based on dominance
    pub fn get_phenotype(&self) -> &Allele {
        // Simple dominance: if either allele is dominant, it expresses
        // If both are recessive, recessive expresses
        let allele1_dominant = self.current_alleles[0].is_dominant();
        let allele2_dominant = self.current_alleles[1].is_dominant();
        
        if allele1_dominant && allele2_dominant {
            // Co-dominance - express both (simplified to first)
            &self.current_alleles[0]
        } else if allele1_dominant {
            &self.current_alleles[0]
        } else if allele2_dominant {
            &self.current_alleles[1]
        } else {
            // Both recessive - express recessive trait
            &self.current_alleles[0]
        }
    }

    /// Check if locus is heterozygous (different alleles)
    pub fn is_heterozygous(&self) -> bool {
        self.current_alleles[0] != self.current_alleles[1]
    }

    /// Check if locus is homozygous (same alleles)
    pub fn is_homozygous(&self) -> bool {
        self.current_alleles[0] == self.current_alleles[1]
    }
}

/// Individual allele variant at a gene locus
/// Represents one version of a gene that can exist at a locus
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Allele {
    /// Unique identifier for this allele
    pub identifier: String,
    /// Human-readable name
    pub name: String,
    /// Whether this allele is dominant or recessive
    pub dominance: Dominance,
    /// Effect on phenotype (0.0 = no effect, 1.0 = full effect)
    pub effect_size: f64,
    /// Mutation rate probability per generation
    pub mutation_rate: f64,
}

impl Allele {
    /// Create a new allele
    pub fn new(
        identifier: String,
        name: String,
        dominance: Dominance,
        effect_size: f64,
        mutation_rate: f64,
    ) -> Self {
        Self {
            identifier,
            name,
            dominance,
            effect_size,
            mutation_rate,
        }
    }

    /// Check if allele is dominant
    pub fn is_dominant(&self) -> bool {
        matches!(self.dominance, Dominance::Dominant | Dominance::CoDominant)
    }

    /// Create common alleles for basic traits
    pub fn create_common_alleles() -> BTreeMap<String, Vec<Allele>> {
        let mut alleles = BTreeMap::new();
        
        // Eye color alleles
        alleles.insert("eye_color".to_string(), vec![
            Allele::new("brown".to_string(), "Brown".to_string(), Dominance::Dominant, 0.8, 0.0001),
            Allele::new("blue".to_string(), "Blue".to_string(), Dominance::Recessive, 0.6, 0.0001),
            Allele::new("green".to_string(), "Green".to_string(), Dominance::Recessive, 0.3, 0.0001),
        ]);
        
        // Hair color alleles
        alleles.insert("hair_color".to_string(), vec![
            Allele::new("brown".to_string(), "Brown".to_string(), Dominance::Dominant, 0.7, 0.0001),
            Allele::new("blonde".to_string(), "Blonde".to_string(), Dominance::Recessive, 0.5, 0.0001),
            Allele::new("black".to_string(), "Black".to_string(), Dominant::Dominant, 0.9, 0.0001),
            Allele::new("red".to_string(), "Red".to_string(), Dominance::Recessive, 0.4, 0.0001),
        ]);
        
        // Height alleles (simplified polygenic)
        alleles.insert("height".to_string(), vec![
            Allele::new("tall".to_string(), "Tall".to_string(), Dominance::Partial, 0.3, 0.0001),
            Allele::new("average".to_string(), "Average".to_string(), Dominance::Partial, 0.0, 0.0001),
            Allele::new("short".to_string(), "Short".to_string(), Dominance::Partial, -0.3, 0.0001),
        ]);
        
        // Skin color alleles
        alleles.insert("skin_color".to_string(), vec![
            Allele::new("dark".to_string(), "Dark".to_string(), Dominance::Dominant, 0.8, 0.0001),
            Allele::new("medium".to_string(), "Medium".to_string(), Dominance::Partial, 0.5, 0.0001),
            Allele::new("light".to_string(), "Light".to_string(), Dominance::Recessive, 0.3, 0.0001),
        ]);
        
        alleles
    }
}

/// Dominance patterns for alleles
/// Determines how alleles interact in phenotype expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dominance {
    /// Dominant allele masks recessive
    Dominant,
    /// Recessive allele only expresses when homozygous
    Recessive,
    /// Co-dominant alleles both express
    CoDominant,
    /// Partial dominance (incomplete dominance)
    Partial,
}

impl Dominance {
    /// Get dominance strength (0.0 = recessive, 1.0 = fully dominant)
    pub const fn strength(self) -> f64 {
        match self {
            Self::Dominant => 1.0,
            Self::CoDominant => 0.8,
            Self::Partial => 0.5,
            Self::Recessive => 0.0,
        }
    }
}

/// Complete double-helix genome
/// Contains all genetic information for an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genome {
    /// All chromosomes with their loci
    pub chromosomes: BTreeMap<ChromosomeType, Vec<Locus>>,
    /// Total number of base pairs
    pub total_base_pairs: u64,
    /// Generation number (0 = founders)
    pub generation: u64,
    /// Creation method (divine, reproduction, etc.)
    pub creation_method: String,
    /// Mutation history
    pub mutation_history: Vec<MutationEvent>,
    /// Checksum for integrity verification
    pub checksum: String,
}

impl Genome {
    /// Create a new genome with basic human traits
    pub fn new_human(generation: u64, creation_method: String) -> Self {
        let mut chromosomes = BTreeMap::new();
        let common_alleles = Allele::create_common_alleles();
        
        // Initialize chromosomes with common traits
        for i in 1..=22 {
            let chromosome = ChromosomeType::Autosome(i);
            let mut loci = Vec::new();
            
            // Add basic trait loci to some chromosomes
            match i {
                1 => {
                    // Eye color locus
                    if let Some(eye_alleles) = common_alleles.get("eye_color") {
                        loci.push(Locus::new(
                            "eye_color".to_string(),
                            chromosome,
                            100_000_000, // Approximate position
                            "Eye Color".to_string(),
                            eye_alleles.clone(),
                            eye_alleles[0].clone(),
                            eye_alleles[1].clone(),
                        ));
                    }
                }
                3 => {
                    // Hair color locus
                    if let Some(hair_alleles) = common_alleles.get("hair_color") {
                        loci.push(Locus::new(
                            "hair_color".to_string(),
                            chromosome,
                            200_000_000,
                            "Hair Color".to_string(),
                            hair_alleles.clone(),
                            hair_alleles[0].clone(),
                            hair_alleles[1].clone(),
                        ));
                    }
                }
                7 => {
                    // Height locus
                    if let Some(height_alleles) = common_alleles.get("height") {
                        loci.push(Locus::new(
                            "height".to_string(),
                            chromosome,
                            300_000_000,
                            "Height".to_string(),
                            height_alleles.clone(),
                            height_alleles[1].clone(),
                            height_alleles[1].clone(),
                        ));
                    }
                }
                15 => {
                    // Skin color locus
                    if let Some(skin_alleles) = common_alleles.get("skin_color") {
                        loci.push(Locus::new(
                            "skin_color".to_string(),
                            chromosome,
                            500_000_000,
                            "Skin Color".to_string(),
                            skin_alleles.clone(),
                            skin_alleles[0].clone(),
                            skin_alleles[1].clone(),
                        ));
                    }
                }
                _ => {
                    // Add placeholder loci for other chromosomes
                    loci.push(Locus::new(
                        format!("placeholder_{}", i),
                        chromosome,
                        (i as u64) * 10_000_000,
                        format!("Placeholder Gene {}", i),
                        vec![Allele::new("placeholder".to_string(), "Placeholder".to_string(), Dominance::Partial, 0.0, 0.0)],
                        Allele::new("placeholder".to_string(), "Placeholder".to_string(), Dominance::Partial, 0.0, 0.0),
                    ));
                }
            }
            
            chromosomes.insert(chromosome, loci);
        }
        
        // Add sex chromosomes
        chromosomes.insert(ChromosomeType::SexX, Vec::new());
        chromosomes.insert(ChromosomeType::SexY, Vec::new());
        
        let total_base_pairs = chromosomes.values()
            .map(|chromosome| chromosome.len() as u64 * 1000) // Simplified
            .sum();
        
        let checksum = Self::calculate_checksum(&chromosomes);
        
        Self {
            chromosomes,
            total_base_pairs,
            generation,
            creation_method,
            mutation_history: Vec::new(),
            checksum,
        }
    }

    /// Calculate genome checksum for integrity verification
    fn calculate_checksum(chromosomes: &BTreeMap<ChromosomeType, Vec<Locus>>) -> String {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        for (chromosome_type, loci) in chromosomes {
            chromosome_type.hash(&mut hasher);
            for locus in loci {
                locus.locus_id.hash(&mut hasher);
                locus.current_alleles[0].identifier.hash(&mut hasher);
                locus.current_alleles[1].identifier.hash(&mut hasher);
            }
        }
        
        format!("{:x}", hasher.finalize())
    }

    /// Verify genome integrity
    pub fn verify_integrity(&self) -> bool {
        let expected_checksum = Self::calculate_checksum(&self.chromosomes);
        self.checksum == expected_checksum
    }

    /// Get all loci across all chromosomes
    pub fn get_all_loci(&self) -> Vec<&Locus> {
        self.chromosomes
            .values()
            .flat_map(|chromosome| chromosome.iter())
            .collect()
    }

    /// Get loci for specific chromosome
    pub fn get_chromosome_loci(&self, chromosome: ChromosomeType) -> &[Locus] {
        self.chromosomes
            .get(&chromosome)
            .map(|loci| loci.as_slice())
            .unwrap_or(&[])
    }

    /// Get phenotype summary
    pub fn get_phenotype_summary(&self) -> BTreeMap<String, String> {
        let mut phenotype = BTreeMap::new();
        
        for locus in self.get_all_loci() {
            let expressed_allele = locus.get_phenotype();
            phenotype.insert(locus.name.clone(), expressed_allele.name.clone());
        }
        
        phenotype
    }

    /// Count heterozygous loci
    pub fn count_heterozygous_loci(&self) -> usize {
        self.get_all_loci()
            .iter()
            .filter(|locus| locus.is_heterozygous())
            .count()
    }

    /// Count homozygous loci
    pub fn count_homozygous_loci(&self) -> usize {
        self.get_all_loci()
            .iter()
            .filter(|locus| locus.is_homozygous())
            .count()
    }
}

/// Mutation event tracking
/// Records all genetic changes for audit purposes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationEvent {
    /// Tick when mutation occurred
    pub tick: u64,
    /// Chromosome where mutation occurred
    pub chromosome: ChromosomeType,
    /// Locus that mutated
    pub locus_id: String,
    /// Original allele before mutation
    pub original_allele: String,
    /// New allele after mutation
    pub new_allele: String,
    /// Type of mutation
    pub mutation_type: MutationType,
    /// RNG stream used for mutation
    pub rng_stream: String,
}

/// Types of genetic mutations
/// Different categories of genetic changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MutationType {
    /// Point mutation (single nucleotide change)
    Point,
    /// Insertion (new DNA added)
    Insertion,
    /// Deletion (DNA removed)
    Deletion,
    /// Duplication (DNA copied)
    Duplication,
    /// Inversion (DNA segment reversed)
    Inversion,
    /// Translocation (DNA moved between chromosomes)
    Translocation,
}

impl MutationType {
    /// Get mutation rate multiplier
    pub const fn rate_multiplier(self) -> f64 {
        match self {
            Self::Point => 1.0,
            Self::Insertion => 0.1,
            Self::Deletion => 0.1,
            Self::Duplication => 0.05,
            Self::Inversion => 0.02,
            Self::Translocation => 0.01,
        }
    }

    /// Description for logging
    pub const fn description(self) -> &'static str {
        match self {
            Self::Point => "point mutation",
            Self::Insertion => "insertion",
            Self::Deletion => "deletion",
            Self::Duplication => "duplication",
            Self::Inversion => "inversion",
            Self::Translocation => "translocation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_creation() {
        let genome = Genome::new_human(0, "divine_creation".to_string());
        
        assert_eq!(genome.generation, 0);
        assert_eq!(genome.creation_method, "divine_creation");
        assert!(genome.total_base_pairs > 0);
        assert!(genome.verify_integrity());
    }

    #[test]
    fn test_locus_operations() {
        let allele1 = Allele::new("A".to_string(), "Type A".to_string(), Dominance::Dominant, 1.0, 0.0001);
        let allele2 = Allele::new("a".to_string(), "Type a".to_string(), Dominance::Recessive, 0.0, 0.0001);
        
        let locus = Locus::new(
            "test_locus".to_string(),
            ChromosomeType::Autosome(1),
            1000,
            "Test Locus".to_string(),
            vec![allele1.clone(), allele2.clone()],
            allele1,
            allele2,
        );
        
        assert!(locus.is_heterozygous());
        assert!(!locus.is_homozygous());
        assert_eq!(locus.genotype_string(), "A/a");
    }

    #[test]
    fn test_dominance_patterns() {
        assert_eq!(Dominance::Dominant.strength(), 1.0);
        assert_eq!(Dominance::Recessive.strength(), 0.0);
        assert_eq!(Dominance::Partial.strength(), 0.5);
    }
}
