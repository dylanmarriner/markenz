/*!
# Lineage Tracking System

**Purpose:** Deterministic lineage tracking for genetic ancestry and inheritance.

**Why it exists:** Lineage tracking provides complete ancestry records,
enables genetic diversity calculations, and prevents inbreeding. It must
maintain accurate family trees across generations.

**Determinism guarantees:**
- Lineage relationships are deterministic and auditable
- All ancestry calculations are reproducible
- No random lineage assignments outside defined rules
- Complete family tree traceability

**How it affects replay:** Same reproduction events will
produce identical lineage records across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::genome::Genome;

/// Lineage relationship types
/// Different types of family relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Parent to child
    Parent,
    /// Child to parent
    Child,
    /// Sibling relationship
    Sibling,
    /// Grandparent to grandchild
    Grandparent,
    /// Grandchild to grandparent
    Grandchild,
    /// Aunt/uncle to niece/nephew
    AuntUncle,
    /// Niece/nephew to aunt/uncle
    NieceNephew,
    /// First cousin
    FirstCousin,
    /// Second cousin
    SecondCousin,
    /// Distant relation
    Distant,
}

impl RelationshipType {
    /// Get relationship description
    pub const fn description(self) -> &'static str {
        match self {
            Self::Parent => "parent",
            Self::Child => "child",
            Self::Sibling => "sibling",
            Self::Grandparent => "grandparent",
            Self::Grandchild => "grandchild",
            Self::AuntUncle => "aunt/uncle",
            Self::NieceNephew => "niece/nephew",
            Self::FirstCousin => "first cousin",
            Self::SecondCousin => "second cousin",
            Self::Distant => "distant relation",
        }
    }

    /// Get generation distance
    pub const fn generation_distance(self) -> u32 {
        match self {
            Self::Parent => 1,
            Self::Child => 1,
            Self::Sibling => 0,
            Self::Grandparent => 2,
            Self::Grandchild => 2,
            Self::AuntUncle => 2,
            Self::NieceNephew => 2,
            Self::FirstCousin => 2,
            Self::SecondCousin => 4,
            Self::Distant => 6,
        }
    }
}

/// Individual lineage entry
/// Records a specific family relationship
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineageEntry {
    /// Unique identifier for this agent
    pub agent_id: String,
    /// Related agent's identifier
    pub related_agent_id: String,
    /// Type of relationship
    pub relationship: RelationshipType,
    /// Generation of related agent
    pub related_generation: u64,
    /// Tick when relationship was established
    pub tick: u64,
    /// Additional metadata
    pub metadata: BTreeMap<String, String>,
}

impl LineageEntry {
    /// Create new lineage entry
    pub fn new(
        agent_id: String,
        related_agent_id: String,
        relationship: RelationshipType,
        related_generation: u64,
        tick: u64,
    ) -> Self {
        Self {
            agent_id,
            related_agent_id,
            relationship,
            related_generation,
            tick,
            metadata: BTreeMap::new(),
        }
    }

    /// Create entry with metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Agent lineage information
/// Complete family tree for an agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentLineage {
    /// Agent's unique identifier
    pub agent_id: String,
    /// Generation number (0 = founders)
    pub generation: u64,
    /// Parent identifiers
    pub parents: [Option<String>; 2],
    /// Children identifiers
    pub children: Vec<String>,
    /// All lineage relationships
    pub relationships: Vec<LineageEntry>,
    /// Creation tick
    pub creation_tick: u64,
    /// Total descendants count
    pub descendant_count: u64,
    /// Ancestral diversity index
    pub ancestral_diversity: f64,
}

impl AgentLineage {
    /// Create new lineage for founder agent
    pub fn new_founder(agent_id: String, tick: u64) -> Self {
        Self {
            agent_id,
            generation: 0,
            parents: [None, None],
            children: Vec::new(),
            relationships: Vec::new(),
            creation_tick: tick,
            descendant_count: 0,
            ancestral_diversity: 1.0, // Founders have maximum diversity
        }
    }

    /// Create new lineage for offspring agent
    pub fn new_offspring(
        agent_id: String,
        mother_id: String,
        father_id: String,
        generation: u64,
        tick: u64,
    ) -> Self {
        let mut relationships = Vec::new();
        
        // Add parent relationships
        relationships.push(LineageEntry::new(
            agent_id.clone(),
            mother_id.clone(),
            RelationshipType::Parent,
            generation - 1,
            tick,
        ).with_metadata("parent_type".to_string(), "mother".to_string()));
        
        relationships.push(LineageEntry::new(
            agent_id.clone(),
            father_id.clone(),
            RelationshipType::Parent,
            generation - 1,
            tick,
        ).with_metadata("parent_type".to_string(), "father".to_string()));
        
        Self {
            agent_id,
            generation,
            parents: [Some(mother_id), Some(father_id)],
            children: Vec::new(),
            relationships,
            creation_tick: tick,
            descendant_count: 0,
            ancestral_diversity: 0.0, // Will be calculated
        }
    }

    /// Add child to this lineage
    pub fn add_child(&mut self, child_id: String, tick: u64) {
        self.children.push(child_id.clone());
        self.descendant_count += 1;
        
        // Add child relationship
        self.relationships.push(LineageEntry::new(
            self.agent_id.clone(),
            child_id.clone(),
            RelationshipType::Child,
            self.generation + 1,
            tick,
        ));
    }

    /// Add sibling relationship
    pub fn add_sibling(&mut self, sibling_id: String, sibling_generation: u64, tick: u64) {
        self.relationships.push(LineageEntry::new(
            self.agent_id.clone(),
            sibling_id.clone(),
            RelationshipType::Sibling,
            sibling_generation,
            tick,
        ));
    }

    /// Get all ancestors by generation
    pub fn get_ancestors_by_generation(&self) -> BTreeMap<u64, Vec<String>> {
        let mut ancestors = BTreeMap::new();
        
        for entry in &self.relationships {
            if matches!(entry.relationship, RelationshipType::Parent | RelationshipType::Grandparent) {
                ancestors.entry(entry.related_generation).or_insert_with(Vec::new).push(entry.related_agent_id.clone());
            }
        }
        
        ancestors
    }

    /// Get all descendants
    pub fn get_all_descendants(&self) -> Vec<String> {
        let mut descendants = Vec::new();
        
        for entry in &self.relationships {
            if matches!(entry.relationship, RelationshipType::Child | RelationshipType::Grandchild) {
                descendants.push(entry.related_agent_id.clone());
            }
        }
        
        descendants
    }

    /// Calculate inbreeding coefficient
    pub fn calculate_inbreeding_coefficient(&self, partner_lineage: &AgentLineage) -> f64 {
        // Simplified inbreeding coefficient calculation
        // In reality, this would involve complex pedigree analysis
        
        // Check for common ancestors in recent generations
        let my_ancestors = self.get_ancestors_by_generation();
        let partner_ancestors = partner_lineage.get_ancestors_by_generation();
        
        let mut common_ancestors = 0;
        let mut total_ancestors = 0;
        
        // Check up to 4 generations back
        for gen in 1..=4 {
            if let (Some(my_gen), Some(partner_gen)) = (my_ancestors.get(&gen), partner_ancestors.get(&gen)) {
                let my_set: std::collections::HashSet<_> = my_gen.iter().cloned().collect();
                let partner_set: std::collections::HashSet<_> = partner_gen.iter().cloned().collect();
                
                let common = my_set.intersection(&partner_set).count();
                common_ancestors += common;
                total_ancestors += my_set.len() + partner_set.len() - common;
            }
        }
        
        if total_ancestors > 0 {
            common_ancestors as f64 / total_ancestors as f64
        } else {
            0.0
        }
    }

    /// Update ancestral diversity index
    pub fn update_ancestral_diversity(&mut self, lineage_db: &LineageTracker) {
        // Calculate diversity based on genetic variation in ancestry
        // This is a simplified calculation
        
        let mut diversity_factors = Vec::new();
        
        // Factor 1: Number of unique ancestors in recent generations
        let recent_ancestors = self.get_ancestors_by_generation();
        let unique_ancestors: std::collections::HashSet<_> = recent_ancestors.values()
            .flatten()
            .collect();
        diversity_factors.push(unique_ancestors.len() as f64 / 100.0);
        
        // Factor 2: Generation depth
        diversity_factors.push((self.generation as f64 / 20.0).min(1.0));
        
        // Factor 3: Number of distinct family lines
        let mut family_lines = std::collections::HashSet::new();
        for ancestor in &unique_ancestors {
            if let Some(ancestor_lineage) = lineage_db.get_lineage(&ancestor) {
                family_lines.insert(ancestor_lineage.get_family_id());
            }
        }
        diversity_factors.push((family_lines.len() as f64 / 10.0).min(1.0));
        
        // Calculate weighted average
        let total_weight: f64 = diversity_factors.iter().sum();
        self.ancestral_diversity = if total_weight > 0.0 {
            diversity_factors.iter().map(|f| f / total_weight).sum()
        } else {
            0.0
        }
    }
}

/// Complete lineage tracking system
/// Manages all agent family trees and relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineageTracker {
    /// All agent lineages indexed by agent ID
    pub lineages: BTreeMap<String, AgentLineage>,
    /// Global reproduction statistics
    pub reproduction_stats: ReproductionStatistics,
    /// Maximum generation observed
    pub max_generation: u64,
    /// Total agents tracked
    pub total_agents: u64,
}

impl LineageTracker {
    /// Create new lineage tracker
    pub fn new() -> Self {
        Self {
            lineages: BTreeMap::new(),
            reproduction_stats: ReproductionStatistics::new(),
            max_generation: 0,
            total_agents: 0,
        }
    }

    /// Register new agent lineage
    pub fn register_agent(&mut self, agent_id: String, generation: u64, tick: u64) {
        let lineage = if generation == 0 {
            AgentLineage::new_founder(agent_id.clone(), tick)
        } else {
            // Offspring lineage will be added when reproduction occurs
            AgentLineage::new_founder(agent_id.clone(), tick)
        };
        
        self.lineages.insert(agent_id.clone(), lineage);
        self.total_agents += 1;
        self.max_generation = self.max_generation.max(generation);
        
        debug!("Registered agent {} at generation {} (tick {})", 
               agent_id, generation, tick);
    }

    /// Add offspring to lineage system
    pub fn add_offspring(
        &mut self,
        mother_id: String,
        father_id: String,
        offspring_id: String,
        tick: u64,
    ) {
        // Get parents' lineages
        let mother_generation = self.lineages.get(&mother_id).map(|l| l.generation).unwrap_or(0);
        let father_generation = self.lineages.get(&father_id).map(|l| l.generation).unwrap_or(0);
        let offspring_generation = mother_generation.max(father_generation) + 1;
        
        // Create offspring lineage
        let mut offspring_lineage = AgentLineage::new_offspring(
            offspring_id.clone(),
            mother_id,
            father_id,
            offspring_generation,
            tick,
        );
        
        // Update parents' lineages
        if let Some(mother_lineage) = self.lineages.get_mut(&mother_id) {
            mother_lineage.add_child(offspring_id.clone(), tick);
        }
        
        if let Some(father_lineage) = self.lineages.get_mut(&father_id) {
            father_lineage.add_child(offspring_id.clone(), tick);
        }
        
        // Add sibling relationships if there are existing children
        for child_id in &self.lineages.get(&mother_id).map(|l| &l.children).unwrap_or(&vec![]).iter()
            .chain(self.lineages.get(&father_id).map(|l| &l.children).unwrap_or(&vec![]).iter()) {
            if let Some(sibling_lineage) = self.lineages.get_mut(child_id) {
                sibling_lineage.add_sibling(offspring_id.clone(), offspring_generation, tick);
                offspring_lineage.add_sibling(child_id.clone(), sibling_lineage.generation, tick);
            }
        }
        
        // Update offspring diversity
        offspring_lineage.update_ancestral_diversity(self);
        
        // Register offspring
        self.lineages.insert(offspring_id, offspring_lineage);
        
        // Update statistics
        self.reproduction_stats.add_reproduction(tick);
        self.max_generation = self.max_generation.max(offspring_generation);
        
        debug!("Added offspring {} to lineage (generation {})", 
               offspring_id, offspring_generation);
    }

    /// Get lineage for specific agent
    pub fn get_lineage(&self, agent_id: &str) -> Option<&AgentLineage> {
        self.lineages.get(agent_id)
    }

    /// Check if two agents are related
    pub fn are_related(&self, agent1_id: &str, agent2_id: &str) -> bool {
        if let Some(lineage1) = self.lineages.get(agent1_id) {
            for entry in &lineage1.relationships {
                if entry.related_agent_id == agent2_id {
                    return true;
                }
            }
        }
        false
    }

    /// Calculate genetic diversity across population
    pub fn calculate_population_diversity(&self) -> PopulationDiversity {
        let mut diversity_metrics = PopulationDiversity::new();
        
        for lineage in self.lineages.values() {
            diversity_metrics.add_lineage_data(lineage);
        }
        
        diversity_metrics.finalize();
        diversity_metrics
    }

    /// Get maximum lineage depth
    pub fn get_max_lineage_depth(&self) -> u32 {
        self.lineages
            .values()
            .map(|l| self.calculate_lineage_depth(l))
            .max()
            .unwrap_or(0)
    }

    /// Calculate lineage depth for specific agent
    fn calculate_lineage_depth(&self, lineage: &AgentLineage) -> u32 {
        let mut max_depth = 0;
        let mut visited = std::collections::HashSet::new();
        let mut to_visit = vec![lineage.agent_id.clone()];
        
        while let Some(current_id) = to_visit.pop() {
            if visited.contains(&current_id) {
                continue;
            }
            
            visited.insert(current_id.clone());
            max_depth = max_depth.max(visited.len() as u32);
            
            if let Some(current_lineage) = self.lineages.get(&current_id) {
                // Add parents to visit queue
                for parent_id in &current_lineage.parents {
                    if let Some(parent_id) = parent_id {
                        to_visit.push(parent_id.clone());
                    }
                }
            }
        }
        
        max_depth
    }

    /// Get total reproductions
    pub fn get_total_reproductions(&self) -> u64 {
        self.reproduction_stats.total_reproductions
    }

    /// Get successful conceptions
    pub fn get_successful_conceptions(&self) -> u64 {
        self.reproduction_stats.successful_conceptions
    }

    /// Get average generation gap
    pub fn get_average_generation_gap(&self) -> f64 {
        self.reproduction_stats.average_generation_gap
    }

    /// Get genetic diversity index
    pub fn get_genetic_diversity_index(&self) -> f64 {
        self.reproduction_stats.genetic_diversity_index
    }
}

/// Reproduction statistics
/// Global statistics about reproduction events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionStatistics {
    /// Total reproduction attempts
    pub total_reproductions: u64,
    /// Successful conceptions
    pub successful_conceptions: u64,
    /// Average generation gap
    pub average_generation_gap: f64,
    /// Genetic diversity index
    pub genetic_diversity_index: f64,
}

impl ReproductionStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            total_reproductions: 0,
            successful_conceptions: 0,
            average_generation_gap: 0.0,
            genetic_diversity_index: 1.0,
        }
    }

    /// Add reproduction event
    pub fn add_reproduction(&mut self, tick: u64) {
        self.total_reproductions += 1;
        // Other statistics updated at lineage level
    }

    /// Update average generation gap
    pub fn update_generation_gap(&mut self, gap: f64) {
        if self.total_reproductions > 1 {
            let current_total = self.total_reproductions as f64 - 1.0;
            self.average_generation_gap = (self.average_generation_gap * current_total + gap) / (current_total + 1.0);
        } else {
            self.average_generation_gap = gap;
        }
    }

    /// Update genetic diversity index
    pub fn update_diversity_index(&mut self, diversity: f64) {
        self.genetic_diversity_index = (self.genetic_diversity_index + diversity) / 2.0;
    }
}

/// Population diversity metrics
/// Measures genetic variation across all agents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopulationDiversity {
    /// Average ancestral diversity
    pub average_ancestral_diversity: f64,
    /// Generation diversity spread
    pub generation_diversity_spread: f64,
    /// Family line count
    pub family_line_count: u32,
    /// Overall diversity index
    pub overall_diversity_index: f64,
}

impl PopulationDiversity {
    /// Create new diversity metrics
    pub fn new() -> Self {
        Self {
            average_ancestral_diversity: 0.0,
            generation_diversity_spread: 0.0,
            family_line_count: 0,
            overall_diversity_index: 0.0,
        }
    }

    /// Add lineage data to metrics
    pub fn add_lineage_data(&mut self, lineage: &AgentLineage) {
        self.average_ancestral_diversity += lineage.ancestral_diversity;
        self.family_line_count += 1;
    }

    /// Finalize diversity calculations
    pub fn finalize(&mut self) {
        if self.family_line_count > 0 {
            self.average_ancestral_diversity /= self.family_line_count as f64;
        }
        
        // Calculate overall diversity index
        self.overall_diversity_index = (self.average_ancestral_diversity + self.generation_diversity_spread) / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lineage_entry_creation() {
        let entry = LineageEntry::new(
            "agent1".to_string(),
            "agent2".to_string(),
            RelationshipType::Parent,
            1,
            1000,
        );
        
        assert_eq!(entry.agent_id, "agent1");
        assert_eq!(entry.related_agent_id, "agent2");
        assert_eq!(entry.relationship, RelationshipType::Parent);
        assert_eq!(entry.related_generation, 1);
    }

    #[test]
    fn test_agent_lineage() {
        let mut lineage = AgentLineage::new_founder("founder".to_string(), 1000);
        
        lineage.add_child("child1".to_string(), 2000);
        lineage.add_child("child2".to_string(), 3000);
        
        assert_eq!(lineage.children.len(), 2);
        assert_eq!(lineage.descendant_count, 2);
    }

    #[test]
    fn test_lineage_tracker() {
        let mut tracker = LineageTracker::new();
        
        tracker.register_agent("founder1".to_string(), 0, 1000);
        tracker.register_agent("founder2".to_string(), 0, 1000);
        
        tracker.add_offspring(
            "founder1".to_string(),
            "founder2".to_string(),
            "child1".to_string(),
            2000,
        );
        
        assert_eq!(tracker.total_agents, 3);
        assert_eq!(tracker.max_generation, 1);
        assert!(tracker.are_related("child1", "founder1"));
        assert!(tracker.are_related("child1", "founder2"));
    }
}
