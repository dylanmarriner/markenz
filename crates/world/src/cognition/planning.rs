use serde::{Deserialize, Serialize};
use crate::types::*;
use super::perception::Perception;
use super::intent::Intent;
use crate::types::Universe;
use rng::RngSubsystem;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: Intent,
    pub steps: Vec<ActionStep>,
    pub created_tick: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionStep {
    MoveTo { x: f32, y: f32, z: f32 },
    Gather { resource: String },
    Mine,
    Build { blueprint_id: u64 },
    Craft { recipe_id: u64 },
    Chat { target_agent: u64, message: String },
}

pub struct Planner;

impl Planner {
    pub fn plan(
        agent: &Agent,
        intent: Intent,
        perception: &Perception,
        universe: &mut Universe,
    ) -> Plan {
        let mut steps = Vec::new();
        
        match intent {
            Intent::Forage => {
                if let Some((resource, _dist)) = perception.nearby_resources.first() {
                    let (target_x, target_y) = Self::find_resource_location(
                        resource,
                        &perception.terrain_biome,
                    );
                    steps.push(ActionStep::MoveTo {
                        x: target_x,
                        y: target_y,
                        z: universe.terrain.height_at_world(target_x as i32, target_y as i32) as f32,
                    });
                    
                    steps.push(ActionStep::Gather {
                        resource: resource.clone(),
                    });
                }
            },
            Intent::Explore => {
                let rng = universe.global_seed.stream(RngSubsystem::Cognition, 0);
                let random_val = rng.next_f64() as f32;
                let new_x = agent.position.0 + (random_val - 0.5) * 10.0;
                let new_y = agent.position.1 + (random_val - 0.5) * 10.0;
                steps.push(ActionStep::MoveTo {
                    x: new_x,
                    y: new_y,
                    z: universe.terrain.height_at_world(new_x as i32, new_y as i32) as f32,
                });
            },
            Intent::Rest => {
            },
            _ => {},
        }
        
        Plan {
            goal: intent,
            steps,
            created_tick: universe.tick,
        }
    }
    
    fn find_resource_location(_resource: &str, _biome: &crate::terrain::Biome) -> (f32, f32) {
        (0.0, 0.0)
    }
}
