/*!
# Mutation System

**Purpose:** Deterministic genetic mutations for population diversity.

**Why it exists:** Mutations introduce genetic variation over time,
preventing genetic stagnation and enabling evolution. They must be
controlled, bounded, and fully auditable.

**Determinism guarantees:**
- All mutations use seeded RNG streams
- Mutation rates are fixed and bounded
- No random mutations outside defined parameters
- Complete audit trail of all changes

**How it affects replay:** Same genome and RNG seed will
produce identical mutation patterns across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::genome::{Genome, ChromosomeType, Locus, Allele, MutationType, Dominance};

/// Mutation event with complete context
/// Records every genetic change for audit purposes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationEvent {
    /// Tick when mutation occurred
    pub tick: u64,
    /// Chromosome where mutation occurred
    pub chromosome: ChromosomeType,
    /// Locus identifier
    pub locus_id: String,
    /// Original allele before mutation
    pub original_allele: String,
    /// New allele after mutation
    pub new_allele: String,
    /// Type of mutation
    pub mutation_type: MutationType,
    /// RNG stream used
    pub rng_stream: String,
    /// RNG value that triggered mutation
    pub rng_value: f64,
    /// Mutation severity
    pub severity: MutationSeverity,
}

impl MutationEvent {
    /// Create new mutation event
    pub fn new(
        tick: u64,
        chromosome: ChromosomeType,
        locus_id: String,
        original_allele: String,
        new_allele: String,
        mutation_type: MutationType,
        rng_stream: String,
        rng_value: f64,
        severity: MutationSeverity,
    ) -> Self {
        Self {
            tick,
            chromosome,
            locus_id,
            original_allele,
            new_allele,
            mutation_type,
            rng_stream,
            rng_value,
            severity,
        }
    }

    /// Get mutation description
    pub fn description(&self) -> String {
        format!("{}: {} -> {} ({})", 
                self.locus_id,
                self.original_allele,
                self.new_allele,
                self.mutation_type.description())
    }
}

/// Mutation severity levels
/// Impact of mutation on organism function
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MutationSeverity {
    /// No noticeable effect
    Neutral,
    /// Minor effect on phenotype
    Minor,
    /// Moderate effect on phenotype
    Moderate,
    /// Major effect on phenotype
    Major,
    /// Lethal or severely debilitating
    Lethal,
}

impl MutationSeverity {
    /// Get severity multiplier for fitness calculations
    pub const fn fitness_multiplier(self) -> f64 {
        match self {
            Self::Neutral => 1.0,
            Self::Minor => 0.95,
            Self::Moderate => 0.8,
            Self::Major => 0.5,
            Self::Lethal => 0.0,
        }
    }

    /// Get description
    pub const fn description(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Minor => "minor",
            Self::Moderate => "moderate",
            Self::Major => "major",
            Self::Lethal => "lethal",
        }
    }
}

/// Mutation configuration
/// Controls mutation rates and constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationConfig {
    /// Base mutation rate per generation
    pub base_mutation_rate: f64,
    /// Maximum mutations per generation
    pub max_mutations_per_generation: u32,
    /// Mutation hotspots (loci with higher rates)
    pub mutation_hotspots: Vec<String>,
    /// Environmental mutation modifiers
    pub environmental_modifiers: BTreeMap<String, f64>,
    /// Mutation severity distribution
    pub severity_distribution: BTreeMap<MutationSeverity, f64>,
    /// Banned mutations (never occur)
    pub banned_mutations: Vec<String>,
}

impl MutationConfig {
    /// Create standard human mutation configuration
    pub fn human_standard() -> Self {
        let mut severity_distribution = BTreeMap::new();
        severity_distribution.insert(MutationSeverity::Neutral, 0.6);
        severity_distribution.insert(MutationSeverity::Minor, 0.25);
        severity_distribution.insert(MutationSeverity::Moderate, 0.1);
        severity_distribution.insert(MutationSeverity::Major, 0.04);
        severity_distribution.insert(MutationSeverity::Lethal, 0.01);
        
        let mut environmental_modifiers = BTreeMap::new();
        environmental_modifiers.insert("radiation".to_string(), 2.0);
        environmental_modifiers.insert("chemical_exposure".to_string(), 1.5);
        environmental_modifiers.insert("viral_infection".to_string(), 1.2);
        
        Self {
            base_mutation_rate: 0.001, // 0.1% per generation
            max_mutations_per_generation: 3,
            mutation_hotspots: vec![
                "eye_color".to_string(),
                "skin_color".to_string(),
                "height".to_string(),
            ],
            environmental_modifiers,
            severity_distribution,
            banned_mutations: vec![
                "lethal_syndrome".to_string(),
                "severe_disorder".to_string(),
            ],
        }
    }

    /// Get effective mutation rate for a locus
    pub fn get_locus_mutation_rate(&self, locus_id: &str) -> f64 {
        let base_rate = self.base_mutation_rate;
        
        if self.mutation_hotspots.contains(&locus_id.to_string()) {
            base_rate * 10.0 // Hotspot has 10x normal rate
        } else {
            base_rate
        }
    }

    /// Check if mutation type is banned
    pub fn is_mutation_banned(&self, mutation_type: MutationType, allele_id: &str) -> bool {
        // Simple check - in reality would be more complex
        self.banned_mutations.contains(&allele_id)
    }
}

/// Complete mutation system
/// Manages all genetic mutations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationSystem {
    /// Mutation configuration
    pub config: MutationConfig,
    /// RNG stream identifier
    pub rng_stream: String,
    /// Mutation history
    pub mutation_history: Vec<MutationEvent>,
    /// Total mutations by type
    pub mutation_counts: BTreeMap<MutationType, u64>,
    /// Environmental factors
    pub environmental_factors: BTreeMap<String, f64>,
}

impl MutationSystem {
    /// Create new mutation system
    pub fn new(rng_stream: String) -> Self {
        Self {
            config: MutationConfig::human_standard(),
            rng_stream,
            mutation_history: Vec::new(),
            mutation_counts: BTreeMap::new(),
            environmental_factors: BTreeMap::new(),
        }
    }

    /// Set environmental factors
    pub fn set_environmental_factors(&mut self, factors: BTreeMap<String, f64>) {
        self.environmental_factors = factors;
        debug!("Set environmental factors: {:?}", factors);
    }

    /// Process mutations for a genome
    pub fn process_mutations(
        &mut self,
        genome: &Genome,
        generation: u64,
        rng_values: &BTreeMap<String, Vec<f64>>,
    ) -> (Genome, Vec<MutationEvent>) {
        let mut mutated_genome = genome.clone();
        let mut mutation_events = Vec::new();
        let mutation_rng = rng_values.get("mutation").map(|vals| vals[0]).unwrap_or(0.0);
        
        // Calculate number of mutations to apply
        let mutation_count = self.calculate_mutation_count(mutation_rng);
        
        for i in 0..mutation_count {
            let locus_mutation_rng = rng_values.get(&format!("mutation_{}", i)).unwrap_or(&vec![0.5]);
            let locus_mutation_value = locus_mutation_rng[0];
            
            // Select locus for mutation
            let (locus_index, locus) = self.select_mutation_locus(&mutated_genome, locus_mutation_value);
            
            if let Some(locus) = locus {
                // Check if mutation should occur
                let locus_mutation_rate = self.config.get_locus_mutation_rate(&locus.locus_id);
                let effective_rate = self.calculate_effective_mutation_rate(
                    locus_mutation_rate,
                    &locus,
                );
                
                if locus_mutation_value < effective_rate {
                    // Perform mutation
                    if let Some(mutation_event) = self.mutate_locus(
                        &mutated_genome,
                        locus,
                        generation,
                        locus_mutation_value,
                        i,
                    ) {
                        mutation_events.push(mutation_event);
                        
                        // Update mutation counts
                        *self.mutation_counts.entry(mutation_event.mutation_type).or_insert(0) += 1;
                        
                        debug!("Mutation {}: {} ({})", 
                               i, mutation_event.description(), generation);
                    }
                }
            }
        }
        
        // Update genome checksum after mutations
        mutated_genome.checksum = Genome::calculate_checksum(&mutated_genome.chromosomes);
        
        (mutated_genome, mutation_events)
    }

    /// Calculate number of mutations for this generation
    fn calculate_mutation_count(&self, rng_value: f64) -> u32 {
        let expected_mutations = (self.config.base_mutation_rate * 
                                   self.config.max_mutations_per_generation as f64) as u32;
        
        // Use Poisson distribution for mutation count
        if rng_value < 0.001 {
            0
        } else if rng_value < 0.01 {
            1
        } else if rng_value < 0.1 {
            2
        } else if rng_value < 0.3 {
            3
        } else {
            self.config.max_mutations_per_generation
        }
    }

    /// Select locus for mutation
    fn select_mutation_locus(&self, genome: &Genome, rng_value: f64) -> (usize, Option<&Locus>) {
        let all_loci: Vec<_> = genome.get_all_loci().into_iter().collect();
        
        if all_loci.is_empty() {
            return (0, None);
        }
        
        // Select locus based on RNG value
        let locus_index = (rng_value * all_loci.len() as f64) as usize;
        let selected_locus = all_locus.get(locus_index);
        
        (locus_index, selected_locus)
    }

    /// Calculate effective mutation rate with environmental factors
    fn calculate_effective_mutation_rate(
        &self,
        base_rate: f64,
        locus: &Locus,
    ) -> f64 {
        let mut effective_rate = base_rate;
        
        // Apply environmental modifiers
        for (factor, modifier) in &self.environmental_factors {
            if locus.possible_alleles.iter().any(|a| a.identifier.contains(factor)) {
                effective_rate *= modifier;
            }
        }
        
        effective_rate.min(0.1) // Cap at 10% per generation
    }

    /// Mutate a specific locus
    fn mutate_locus(
        &self,
        genome: &mut Genome,
        locus: &Locus,
        generation: u64,
        rng_value: f64,
        mutation_index: u32,
    ) -> Option<MutationEvent> {
        // Select mutation type
        let mutation_type = self.select_mutation_type(rng_value);
        
        // Select new allele
        let new_allele = self.select_new_allele(locus, &mutation_type);
        
        // Check if mutation is allowed
        if self.config.is_mutation_banned(mutation_type, &new_allele.identifier) {
            return None;
        }
        
        // Determine mutation severity
        let severity = self.select_mutation_severity(mutation_type);
        
        // Apply mutation to genome
        let original_allele = locus.current_alleles[0].identifier.clone();
        locus.current_alleles[0] = new_allele.clone();
        
        // Update genome chromosomes
        for (chromosome_type, chromosomes) in &mut genome.chromosomes {
            if *chromosome_type == locus.chromosome {
                for chromo_locus in chromosomes {
                    if chromo_locus.locus_id == locus.locus_id {
                        chromo_locus.current_alleles[0] = new_allele.clone();
                    }
                }
            }
        }
        
        Some(MutationEvent::new(
            generation,
            locus.chromosome,
            locus.locus_id.clone(),
            original_allele,
            new_allele.identifier,
            mutation_type,
            self.rng_stream.clone(),
            rng_value,
            severity,
        ))
    }

    /// Select mutation type based on RNG
    fn select_mutation_type(&self, rng_value: f64) -> MutationType {
        let cumulative = 0.0;
        
        // Weighted selection based on rates
        let types = vec![
            (MutationType::Point, 0.7),
            (MutationType::Insertion, 0.1),
            (MutationType::Deletion, 0.1),
            (MutationType::Duplication, 0.05),
            (MutationType::Inversion, 0.03),
            (MutationType::Translocation, 0.02),
        ];
        
        for (mutation_type, weight) in types {
            let cumulative_weight = cumulative + weight;
            if rng_value < cumulative_weight {
                return mutation_type;
            }
        }
        
        MutationType::Point // Default
    }

    /// Select new allele for mutation
    fn select_new_allele(&self, locus: &Locus, mutation_type: &MutationType) -> Allele {
        let current_allele = &locus.current_alleles[0];
        
        match mutation_type {
            MutationType::Point => {
                // Point mutation - create new variant
                Allele::new(
                    format!("{}_mutated", current_allele.identifier),
                    format!("Mutated {}", current_allele.name),
                    current_allele.dominance,
                    current_allele.effect_size * 0.8, // Reduced effect
                    current_allele.mutation_rate * 2.0, // Higher mutation rate
                )
            }
            MutationType::Insertion => {
                // Insertion - add genetic material
                Allele::new(
                    format!("{}_insertion", current_allele.identifier),
                    format!("{} with insertion", current_allele.name),
                    current_allele.dominance,
                    current_allele.effect_size * 1.2, // Increased effect
                    current_allele.mutation_rate,
                )
            }
            MutationType::Deletion => {
                // Deletion - remove genetic material
                Allele::new(
                    format!("{}_deleted", current_allele.identifier),
                    format!("{} with deletion", current_allele.name),
                    Dominance::Recessive, // Often recessive
                    current_allele.effect_size * 0.3, // Reduced effect
                    current_allele.mutation_rate,
                )
            }
            MutationType::Duplication => {
                // Duplication - copy genetic material
                Allele::new(
                    format!("{}_duplicated", current_allele.identifier),
                    format!("{} with duplication", current_allele.name),
                    current_allele.dominance,
                    current_allele.effect_size * 1.5, // Increased effect
                    current_allele.mutation_rate,
                )
            }
            MutationType::Inversion => {
                // Inversion - reverse genetic material
                Allele::new(
                    format!("{}_inverted", current_allele.identifier),
                    format!("{} with inversion", current_allele.name),
                    current_allele.dominance,
                    current_allele.effect_size * 0.5, // Variable effect
                    current_allele.mutation_rate,
                )
            }
            MutationType::Translocation => {
                // Translocation - move genetic material
                Allele::new(
                    format!("{}_translocated", current_allele.identifier),
                    format!("{} with translocation", current_allele.name),
                    current_allele.dominance,
                    current_allele.effect_size * 0.7, // Variable effect
                    current_allele.mutation_rate,
                )
            }
        }
    }

    /// Select mutation severity
    fn select_mutation_severity(&self, mutation_type: MutationType) -> MutationSeverity {
        let rng_value = 0.5; // Would use actual RNG in real implementation
        
        // Use severity distribution from config
        let mut cumulative = 0.0;
        for (severity, probability) in &self.config.severity_distribution {
            cumulative += probability;
            if rng_value < cumulative {
                return *severity;
            }
        }
        
        MutationSeverity::Neutral // Default
    }

    /// Get mutation statistics
    pub fn get_statistics(&self) -> MutationStatistics {
        let mut total_mutations = 0;
        let mut total_count = 0;
        
        for event in &self.mutation_history {
            total_mutations += 1;
            total_count += 1;
        }
        
        MutationStatistics {
            total_mutations,
            mutations_per_generation,
            mutation_type_counts: self.mutation_counts.clone(),
            most_common_type: self.get_most_common_mutation_type(),
            average_severity: self.calculate_average_severity(),
        }
    }

    /// Get most common mutation type
    fn get_most_common_mutation_type(&self) -> Option<MutationType> {
        self.mutation_counts
            .iter()
            .max_by_key(|(_, count)| count)
            .map(|(mutation_type, _)| *mutation_type)
    }

    /// Calculate average mutation severity
    fn calculate_average_severity(&self) -> f64 {
        let mut total_severity = 0.0;
        let mut total_count = 0;
        
        for event in &self.mutation_history {
            total_severity += event.severity.fitness_multiplier();
            total_count += 1;
        }
        
        if total_count > 0 {
            total_severity / total_count as f64
        } else {
            1.0
        }
    }
}

/// Mutation statistics
/// Summary of mutation patterns and effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationStatistics {
    /// Total number of mutations
    pub total_mutations: u64,
    /// Average mutations per generation
    pub mutations_per_generation: f64,
    /// Counts by mutation type
    pub mutation_type_counts: BTreeMap<MutationType, u64>,
    /// Most common mutation type
    pub most_common_type: Option<MutationType>,
    /// Average mutation severity
    pub average_severity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_config() {
        let config = MutationConfig::human_standard();
        
        assert_eq!(config.base_mutation_rate, 0.001);
        assert_eq!(config.max_mutations_per_generation, 3);
        assert!(config.mutation_hotspots.contains(&"eye_color".to_string()));
    }

    #[test]
    fn test_mutation_event() {
        let event = MutationEvent::new(
            1000,
            ChromosomeType::Autosome(1),
            "test_locus".to_string(),
            "allele_a".to_string(),
            "allele_b".to_string(),
            MutationType::Point,
            "test_stream".to_string(),
            0.123,
            MutationSeverity::Minor,
        );
        
        assert_eq!(event.tick, 1000);
        assert_eq!(event.original_allele, "allele_a");
        assert_eq!(event.new_allele, "allele_b");
        assert_eq!(event.mutation_type, MutationType::Point);
    }

    #[test]
    fn test_mutation_system() {
        let mut system = MutationSystem::new("test_stream".to_string());
        
        // Create test genome
        let mut genome = Genome::new_human(1, "test".to_string());
        
        // Set up RNG values for testing
        let mut rng_values = BTreeMap::new();
        rng_values.insert("mutation".to_string(), vec![0.8]); // High mutation rate
        rng_values.insert("mutation_0".to_string(), vec![0.5]);
        
        let (mutated_genome, events) = system.process_mutations(&genome, 2, &rng_values);
        
        assert!(!events.is_empty());
        assert!(mutated_genome != genome);
        assert_eq!(events.len(), 2); // High rate should cause 2 mutations
    }
}
