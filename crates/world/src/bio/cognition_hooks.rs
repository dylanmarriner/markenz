/*!
# Cognition Integration Hooks

**Purpose:** Provide deterministic cognition interfaces without LLM implementation.

**Why it exists:** Human equivalence law requires cognition integration points
for perception, volition, and decision-making, even if full LLM
cognition is not implemented in this phase.

**Determinism guarantees:**
- All perception encoding is deterministic
- All volition constraints are rule-based
- All decision hooks provide consistent interfaces
- No external AI dependencies in core loops

**How it affects replay:** Same perceptions and constraints will produce
identical decision patterns across replays.
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, trace};
use super::bio::complete_biology::BiologicalState;

/// Perception input from world
/// Encodes all sensory information available to agents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Perception {
    /// Visual perception
    pub visual: VisualPerception,
    /// Auditory perception
    pub auditory: AuditoryPerception,
    /// Somatic perception (body state)
    pub somatic: SomaticPerception,
    /// Environmental perception
    pub environmental: EnvironmentalPerception,
    /// Social perception (other agents)
    pub social: SocialPerception,
}

impl Perception {
    /// Create new perception from world state
    pub fn from_world_state(
        agent_id: &str,
        world_state: &crate::World,
        biological_state: &BiologicalState,
    ) -> Self {
        Self {
            visual: VisualPerception::from_world(agent_id, world_state),
            auditory: AuditoryPerception::new(),
            somatic: SomaticPerception::from_biological_state(biological_state),
            environmental: EnvironmentalPerception::from_world(world_state),
            social: SocialPerception::from_world(agent_id, world_state),
        }
    }
    
    /// Get all perceptible objects
    pub fn get_perceptible_objects(&self) -> Vec<PerceptibleObject> {
        let mut objects = Vec::new();
        objects.extend(self.visual.objects.clone());
        objects.extend(self.auditory.sounds.clone());
        objects.extend(self.environmental.objects.clone());
        objects
    }
    
    /// Check if perception indicates danger
    pub fn indicates_danger(&self) -> bool {
        self.visual.threat_level > to_fixed(70.0) ||
            self.auditory.threat_level > to_fixed(60.0) ||
            self.somatic.pain_level > to_fixed(30.0)
    }
}

/// Visual perception
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualPerception {
    /// Visible objects
    pub objects: Vec<PerceptibleObject>,
    /// Light level (0-10000)
    pub light_level: BioFixed,
    /// Threat level (0-10000)
    pub threat_level: BioFixed,
    /// Visibility distance
    pub visibility_distance: u64,
}

impl VisualPerception {
    /// Create visual perception from world state
    pub fn from_world(agent_id: &str, world_state: &crate::World) -> Self {
        let mut objects = Vec::new();
        let mut threat_level = to_fixed(0.0);
        
        // Get agent's position
        if let Some(agent) = world_state.agents.get(agent_id) {
            let agent_position = &agent.position;
            
            // Find visible objects within range
            for object in world_state.objects.values() {
                let distance = agent_position.distance_to(&object.position);
                
                if distance <= 50 { // 50 unit vision range
                    let perceptible = PerceptibleObject {
                        id: object.id.clone(),
                        object_type: object.object_type.clone(),
                        position: object.position,
                        distance,
                        properties: Self::extract_visual_properties(object),
                    };
                    objects.push(perceptible);
                    
                    // Update threat level based on object type
                    threat_level = (threat_level + Self::get_object_threat(&object.object_type)).min(to_fixed(100.0));
                }
            }
        }
        
        Self {
            objects,
            light_level: to_fixed(80.0), // Default daylight
            threat_level,
            visibility_distance: 50,
        }
    }
    
    /// Extract visual properties from object
    fn extract_visual_properties(object: &crate::WorldObject) -> BTreeMap<String, String> {
        let mut properties = BTreeMap::new();
        properties.insert("size".to_string(), format!("{:?}", object.properties.size));
        properties.insert("material".to_string(), object.properties.material.clone());
        properties.insert("durability".to_string(), format!("{}", object.durability));
        properties
    }
    
    /// Get threat level for object type
    fn get_object_threat(object_type: &str) -> BioFixed {
        match object_type {
            "weapon" => to_fixed(80.0),
            "predator" => to_fixed(90.0),
            "fire" => to_fixed(70.0),
            "unknown" => to_fixed(30.0),
            _ => to_fixed(10.0),
        }
    }
}

/// Auditory perception
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditoryPerception {
    /// Perceived sounds
    pub sounds: Vec<PerceptibleObject>,
    /// Threat level (0-10000)
    pub threat_level: BioFixed,
}

impl AuditoryPerception {
    /// Create new auditory perception
    pub fn new() -> Self {
        Self {
            sounds: Vec::new(),
            threat_level: to_fixed(0.0),
        }
    }
}

/// Somatic perception (body state)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SomaticPerception {
    /// Pain level (0-10000)
    pub pain_level: BioFixed,
    /// Hunger level (0-10000)
    pub hunger_level: BioFixed,
    /// Thirst level (0-10000)
    pub thirst_level: BioFixed,
    /// Fatigue level (0-10000)
    pub fatigue_level: BioFixed,
    /// Temperature sensation
    pub temperature_sensation: TemperatureSensation,
}

impl SomaticPerception {
    /// Create somatic perception from biological state
    pub fn from_biological_state(biological_state: &BiologicalState) -> Self {
        Self {
            pain_level: to_fixed(100.0) - biological_state.health, // Inverse of health
            hunger_level: biological_state.hunger,
            thirst_level: biological_state.thirst,
            fatigue_level: biological_state.fatigue,
            temperature_sensation: TemperatureSensation::from_temperature(biological_state.body_temperature),
        }
    }
}

/// Temperature sensation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemperatureSensation {
    /// Too cold
    TooCold,
    /// Cold
    Cold,
    /// Comfortable
    Comfortable,
    /// Warm
    Warm,
    /// Too hot
    TooHot,
}

impl TemperatureSensation {
    /// Create temperature sensation from body temperature
    pub fn from_temperature(temperature: BioFixed) -> Self {
        match from_fixed(temperature) {
            t if t < 35.0 => Self::TooCold,
            t if t < 36.0 => Self::Cold,
            t if t <= 37.5 => Self::Comfortable,
            t if t <= 38.5 => Self::Warm,
            _ => Self::TooHot,
        }
    }
}

/// Environmental perception
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalPerception {
    /// Environmental objects
    pub objects: Vec<PerceptibleObject>,
    /// Weather conditions
    pub weather: WeatherConditions,
    /// Time of day
    pub time_of_day: TimeOfDay,
}

impl EnvironmentalPerception {
    /// Create environmental perception from world
    pub fn from_world(world_state: &crate::World) -> Self {
        Self {
            objects: Vec::new(), // Would be populated from terrain
            weather: WeatherConditions::new(),
            time_of_day: TimeOfDay::from_tick(world_state.tick),
        }
    }
}

/// Weather conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherConditions {
    /// Temperature (fixed-point Celsius)
    pub temperature: BioFixed,
    /// Humidity (0-10000)
    pub humidity: BioFixed,
    /// Wind speed (fixed-point m/s)
    pub wind_speed: BioFixed,
    /// Precipitation level
    pub precipitation: PrecipitationLevel,
}

impl WeatherConditions {
    /// Create new weather conditions
    pub fn new() -> Self {
        Self {
            temperature: to_fixed(20.0), // 20°C
            humidity: to_fixed(50.0), // 50%
            wind_speed: to_fixed(5.0), // 5 m/s
            precipitation: PrecipitationLevel::None,
        }
    }
}

/// Precipitation levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrecipitationLevel {
    /// No precipitation
    None,
    /// Light rain
    Light,
    /// Moderate rain
    Moderate,
    /// Heavy rain
    Heavy,
    /// Storm
    Storm,
}

/// Time of day
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeOfDay {
    /// Dawn (5-7)
    Dawn,
    /// Morning (7-12)
    Morning,
    /// Afternoon (12-17)
    Afternoon,
    /// Evening (17-20)
    Evening,
    /// Night (20-5)
    Night,
}

impl TimeOfDay {
    /// Create time of day from tick
    pub fn from_tick(tick: u64) -> Self {
        let hour = (tick / 100) % 24; // 100 ticks per hour
        
        match hour {
            5..=7 => Self::Dawn,
            7..=12 => Self::Morning,
            12..=17 => Self::Afternoon,
            17..=20 => Self::Evening,
            _ => Self::Night,
        }
    }
}

/// Social perception
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialPerception {
    /// Other agents
    pub agents: Vec<PerceptibleAgent>,
    /// Social context
    pub context: SocialContext,
}

impl SocialPerception {
    /// Create social perception from world
    pub fn from_world(agent_id: &str, world_state: &crate::World) -> Self {
        let mut agents = Vec::new();
        
        // Get agent's position
        if let Some(current_agent) = world_state.agents.get(agent_id) {
            let agent_position = &current_agent.position;
            
            // Find other agents within social range
            for (other_id, other_agent) in &world_state.agents {
                if other_id != agent_id {
                    let distance = agent_position.distance_to(&other_agent.position);
                    
                    if distance <= 30 { // 30 unit social range
                        let perceptible = PerceptibleAgent {
                            id: other_id.clone(),
                            distance,
                            relationship: Relationship::Unknown,
                            emotional_state: EmotionalState::from_biological_state(&other_agent.biological_state),
                            visible: true,
                        };
                        agents.push(perceptible);
                    }
                }
            }
        }
        
        Self {
            agents,
            context: SocialContext::new(),
        }
    }
}

/// Perceptible object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptibleObject {
    /// Object ID
    pub id: String,
    /// Object type
    pub object_type: String,
    /// Position
    pub position: crate::WorldCoordinate,
    /// Distance from agent
    pub distance: i64,
    /// Properties
    pub properties: BTreeMap<String, String>,
}

/// Perceptible agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptibleAgent {
    /// Agent ID
    pub id: String,
    /// Distance from agent
    pub distance: i64,
    /// Relationship type
    pub relationship: Relationship,
    /// Emotional state
    pub emotional_state: EmotionalState,
    /// Whether visible
    pub visible: bool,
}

/// Relationship types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Relationship {
    /// Unknown relationship
    Unknown,
    /// Stranger
    Stranger,
    /// Acquaintance
    Acquaintance,
    /// Friend
    Friend,
    /// Family
    Family,
    /// Enemy
    Enemy,
}

/// Emotional state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Primary emotion
    pub primary: Emotion,
    /// Secondary emotion
    pub secondary: Option<Emotion>,
    /// Intensity (0-10000)
    pub intensity: BioFixed,
}

impl EmotionalState {
    /// Create emotional state from biological state
    pub fn from_biological_state(biological_state: &BiologicalState) -> Self {
        let primary = if biological_state.stress_level > to_fixed(70.0) {
            Emotion::Fear
        } else if biological_state.fatigue > to_fixed(80.0) {
            Emotion::Exhaustion
        } else if biological_state.hunger > to_fixed(80.0) {
            Emotion::Hunger
        } else if biological_state.endocrine.get_hormonal_effects().reproductive_drive > to_fixed(70.0) {
            Emotion::Arousal
        } else {
            Emotion::Neutral
        };
        
        Self {
            primary,
            secondary: None,
            intensity: to_fixed(50.0), // Moderate intensity
        }
    }
}

/// Emotions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Emotion {
    /// Neutral emotion
    Neutral,
    /// Fear response
    Fear,
    /// Anger response
    Anger,
    /// Joy response
    Joy,
    /// Sadness response
    Sadness,
    /// Disgust response
    Disgust,
    /// Surprise response
    Surprise,
    /// Hunger drive
    Hunger,
    /// Thirst drive
    Thirst,
    /// Pain sensation
    Pain,
    /// Fatigue state
    Exhaustion,
    /// Sexual arousal
    Arousal,
}

/// Social context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext {
    /// Current location type
    pub location_type: LocationType,
    /// Social density
    pub social_density: SocialDensity,
}

impl SocialContext {
    /// Create new social context
    pub fn new() -> Self {
        Self {
            location_type: LocationType::OpenSpace,
            social_density: SocialDensity::Sparse,
        }
    }
}

/// Location types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationType {
    /// Open space
    OpenSpace,
    /// Enclosed space
    EnclosedSpace,
    /// Private space
    PrivateSpace,
    /// Public space
    PublicSpace,
}

/// Social density
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocialDensity {
    /// No other agents
    Empty,
    /// Few agents
    Sparse,
    /// Moderate number
    Moderate,
    /// Many agents
    Dense,
    /// Crowded
    Crowded,
}

/// Volition constraints and decision-making
/// Manages agent intentions and biological constraints
pub struct VolitionSystem {
    /// Current intentions
    pub intentions: BTreeMap<String, Intention>,
    /// Active constraints
    pub constraints: Vec<VolitionConstraint>,
}

impl VolitionSystem {
    /// Create new volition system
    pub fn new() -> Self {
        Self {
            intentions: BTreeMap::new(),
            constraints: Vec::new(),
        }
    }
    
    /// Add new intention
    pub fn add_intention(&mut self, intention: Intention) {
        self.intentions.insert(intention.id.clone(), intention);
        debug!("Added intention: {} (priority: {})", intention.id, intention.priority);
    }
    
    /// Check if action is allowed by volition
    pub fn check_action_permission(
        &self,
        action: &crate::InputEvent,
        biological_state: &BiologicalState,
        perception: &Perception,
    ) -> Result<(), VolitionVetoReason> {
        // Check biological constraints first
        if let Err(bio_veto) = biological_state.check_action_viability(action) {
            return Err(VolitionVetoReason::Biological(bio_veto));
        }
        
        // Check perceptual constraints
        if perception.indicates_danger() && !self.is_defensive_action(action) {
            return Err(VolitionVetoReason::Perceptual("Perceived danger but action is not defensive".to_string()));
        }
        
        // Check intention consistency
        if !self.action_matches_intentions(action) {
            return Err(VolitionVetoReason::Intentional("Action does not match current intentions".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if action is defensive
    fn is_defensive_action(&self, action: &crate::InputEvent) -> bool {
        match action {
            crate::InputEvent::MoveAgent { .. } => true, // Can retreat
            crate::InputEvent::UseTool { tool_type, .. } => {
                tool_type.contains("weapon") || tool_type.contains("shield")
            },
            _ => false,
        }
    }
    
    /// Check if action matches current intentions
    fn action_matches_intentions(&self, action: &crate::InputEvent) -> bool {
        // For now, allow all actions if intentions exist
        // In full implementation, this would check against specific goals
        !self.intentions.is_empty()
    }
    
    /// Update intentions based on perception
    pub fn update_intentions_from_perception(&mut self, perception: &Perception) {
        // Add survival intentions if danger perceived
        if perception.indicates_danger() {
            self.add_intention(Intention {
                id: "survival".to_string(),
                goal: IntentionGoal::Survive,
                priority: 100,
                urgency: 100,
            });
        }
        
        // Add resource acquisition intentions if hungry/thirsty
        if perception.somatic.hunger_level > to_fixed(70.0) {
            self.add_intention(Intention {
                id: "find_food".to_string(),
                goal: IntentionGoal::AcquireResource("food".to_string()),
                priority: 80,
                urgency: 70,
            });
        }
        
        if perception.somatic.thirst_level > to_fixed(70.0) {
            self.add_intention(Intention {
                id: "find_water".to_string(),
                goal: IntentionGoal::AcquireResource("water".to_string()),
                priority: 85,
                urgency: 75,
            });
        }
    }
}

/// Agent intention
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intention {
    /// Unique intention ID
    pub id: String,
    /// Goal to achieve
    pub goal: IntentionGoal,
    /// Priority level (0-100)
    pub priority: u8,
    /// Urgency level (0-100)
    pub urgency: u8,
}

/// Intention goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntentionGoal {
    /// Survive (avoid danger)
    Survive,
    /// Acquire resource
    AcquireResource(String),
    /// Explore area
    Explore,
    /// Rest and recover
    Rest,
    /// Social interaction
    SocialInteract,
    /// Complete task
    CompleteTask(String),
}

/// Volition constraint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolitionConstraint {
    /// Biological constraint
    Biological(BioVetoReason),
    /// Perceptual constraint
    Perceptual(String),
    /// Intentional constraint
    Intentional(String),
    /// Physical constraint
    Physical(String),
}

/// Volition veto reasons
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolitionVetoReason {
    /// Biological veto
    Biological(BioVetoReason),
    /// Perceptual veto
    Perceptual(String),
    /// Intentional veto
    Intentional(String),
}

/// Event emission for cognition
/// Emits observable cognitive events
pub struct CognitionEventEmitter {
    /// Event history
    pub events: Vec<CognitionEvent>,
}

impl CognitionEventEmitter {
    /// Create new event emitter
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Emit perception event
    pub fn emit_perception(&mut self, perception: &Perception) {
        let event = CognitionEvent {
            event_type: CognitionEventType::Perception,
            timestamp: 0, // Will be set by world
            data: CognitionEventData::Perception(perception.clone()),
        };
        self.events.push(event);
        trace!("Emitted perception event");
    }
    
    /// Emit intention event
    pub fn emit_intention(&mut self, intention: &Intention) {
        let event = CognitionEvent {
            event_type: CognitionEventType::Intention,
            timestamp: 0,
            data: CognitionEventData::Intention(intention.clone()),
        };
        self.events.push(event);
        trace!("Emitted intention event: {}", intention.id);
    }
    
    /// Emit decision event
    pub fn emit_decision(&mut self, decision: &Decision) {
        let event = CognitionEvent {
            event_type: CognitionEventType::Decision,
            timestamp: 0,
            data: CognitionEventData::Decision(decision.clone()),
        };
        self.events.push(event);
        trace!("Emitted decision event: {}", decision.action_description);
    }
    
    /// Get events for observation
    pub fn get_events(&self) -> &[CognitionEvent] {
        &self.events
    }
    
    /// Clear old events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

/// Cognition events for observation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitionEvent {
    /// Type of cognition event
    pub event_type: CognitionEventType,
    /// Tick when event occurred
    pub timestamp: u64,
    /// Event data
    pub data: CognitionEventData,
}

/// Cognition event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CognitionEventType {
    /// Perception event
    Perception,
    /// Intention event
    Intention,
    /// Decision event
    Decision,
    /// Learning event
    Learning,
    /// Memory event
    Memory,
}

/// Cognition event data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CognitionEventData {
    /// Perception data
    Perception(Perception),
    /// Intention data
    Intention(Intention),
    /// Decision data
    Decision(Decision),
    /// Learning data
    Learning(LearningEvent),
    /// Memory data
    Memory(MemoryEvent),
}

/// Decision made by agent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decision {
    /// Action chosen
    pub chosen_action: crate::InputEvent,
    /// Reasoning for decision
    pub reasoning: String,
    /// Confidence level (0-10000)
    pub confidence: BioFixed,
    /// Action description
    pub action_description: String,
}

/// Learning event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningEvent {
    /// Type of learning
    pub learning_type: LearningType,
    /// Content learned
    pub content: String,
    /// Strength of learning (0-10000)
    pub strength: BioFixed,
}

/// Learning types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningType {
    /// Skill acquisition
    Skill,
    /// Knowledge acquisition
    Knowledge,
    /// Pattern recognition
    Pattern,
    /// Social learning
    Social,
}

/// Memory event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryEvent {
    /// Type of memory operation
    pub memory_type: MemoryType,
    /// Memory content
    pub content: String,
    /// Memory importance (0-10000)
    pub importance: BioFixed,
}

/// Memory types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryType {
    /// Store memory
    Store,
    /// Retrieve memory
    Retrieve,
    /// Forget memory
    Forget,
    /// Consolidate memory
    Consolidate,
}

// Import from complete_biology module
use super::bio::complete_biology::{to_fixed, BioFixed};
