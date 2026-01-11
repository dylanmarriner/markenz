use serde_json::json;
use crate::bio::BioState;

#[derive(Debug, Clone)]
pub struct ObservationEvent {
    pub tick: u64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub hash: [u8; 32],
}

pub fn bio_state_to_observation(
    tick: u64,
    agent_id: u64,
    bio_before: &BioState,
    bio_after: &BioState,
) -> Option<ObservationEvent> {
    // Only emit if state changed
    if (bio_before.energy - bio_after.energy).abs() < 0.01
        && (bio_before.hunger - bio_after.hunger).abs() < 0.01
        && (bio_before.exhaustion - bio_after.exhaustion).abs() < 0.01
        && (bio_before.health - bio_after.health).abs() < 0.01 {
        return None;
    }
    
    let payload = json!({
        "type": "bio_state_changed",
        "agent_id": agent_id,
        "energy": {
            "before": bio_before.energy,
            "after": bio_after.energy,
            "delta": bio_after.energy - bio_before.energy
        },
        "hunger": {
            "before": bio_before.hunger,
            "after": bio_after.hunger,
        },
        "exhaustion": {
            "before": bio_before.exhaustion,
            "after": bio_after.exhaustion,
        },
        "health": {
            "before": bio_before.health,
            "after": bio_after.health,
        }
    });
    
    let hash = blake3::hash(serde_json::to_string(&payload).unwrap().as_bytes())
            .as_bytes()[..32]
            .try_into()
            .unwrap();
    
    Some(ObservationEvent {
        tick,
        event_type: "bio_state_changed".to_string(),
        payload,
        hash,
    })
}
