use serde::{Serialize, Deserialize};
use super::rng_stream::RngSubsystem;

/// Record of a single RNG draw for audit purposes
/// 
/// This struct captures all metadata needed to replay and verify
/// deterministic random number generation across different subsystems.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RngDrawRecord {
    /// The tick at which this RNG draw occurred
    pub tick: u64,
    /// The subsystem that made this RNG draw (Physics, Biology, etc.)
    pub subsystem: RngSubsystem,
    /// Unique identifier for the RNG stream within the subsystem
    pub stream_id: u64,
    /// Source location where the RNG draw was made (file:line)
    pub callsite: String,
    /// The actual random value that was drawn
    pub value: u64,
    /// Tick-relative timestamp for ordering within the same tick
    pub timestamp: u64,
}

/// Audit log for all RNG draws in the system
/// 
/// This maintains a complete record of every random number generated
/// for deterministic replay and verification purposes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RngAuditLog {
    /// Internal storage of all RNG draw records
    records: Vec<RngDrawRecord>,
}

impl Default for RngAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl RngAuditLog {
    /// Create a new empty RNG audit log
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    
    /// Record an RNG draw in the audit log
    /// 
    /// # Arguments
    /// * `tick` - The current simulation tick
    /// * `subsystem` - The subsystem making the draw
    /// * `stream_id` - The stream identifier within the subsystem
    /// * `callsite` - Source location as "file:line"
    /// * `value` - The random value that was drawn
    pub fn record_draw(
        &mut self,
        tick: u64,
        subsystem: RngSubsystem,
        stream_id: u64,
        callsite: &str,
        value: u64,
    ) -> Result<(), String> {
        let record = RngDrawRecord {
            tick,
            subsystem,
            stream_id,
            callsite: callsite.to_string(),
            value,
            timestamp: tick,
        };
        
        self.records.push(record);
        Ok(())
    }
    
    /// Get all records in the audit log
    pub fn records(&self) -> &[RngDrawRecord] {
        &self.records
    }
    
    /// Get all records for a specific tick
    pub fn records_by_tick(&self, tick: u64) -> Vec<&RngDrawRecord> {
        self.records
            .iter()
            .filter(|record| record.tick == tick)
            .collect()
    }
    
    /// Get all records for a specific subsystem
    pub fn records_by_subsystem(&self, subsystem: RngSubsystem) -> Vec<&RngDrawRecord> {
        self.records
            .iter()
            .filter(|record| record.subsystem == subsystem)
            .collect()
    }
    
    /// Get all records for a specific subsystem and stream
    pub fn records_by_stream(&self, subsystem: RngSubsystem, stream_id: u64) -> Vec<&RngDrawRecord> {
        self.records
            .iter()
            .filter(|record| record.subsystem == subsystem && record.stream_id == stream_id)
            .collect()
    }
    
    /// Clear all records from the audit log
    pub fn clear(&mut self) {
        self.records.clear();
    }
    
    /// Get the number of records in the audit log
    pub fn len(&self) -> usize {
        self.records.len()
    }
    
    /// Check if the audit log is empty
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_log_record_draw() {
        let mut audit = RngAuditLog::new();
        
        audit.record_draw(1, RngSubsystem::Physics, 0, "physics.rs:42", 12345).unwrap();
        audit.record_draw(1, RngSubsystem::Biology, 0, "biology.rs:123", 67890).unwrap();
        
        assert_eq!(audit.len(), 2);
        
        let physics_records = audit.records_by_subsystem(RngSubsystem::Physics);
        assert_eq!(physics_records.len(), 1);
        assert_eq!(physics_records[0].value, 12345);
        
        let tick1_records = audit.records_by_tick(1);
        assert_eq!(tick1_records.len(), 2);
    }
    
    #[test]
    fn test_audit_log_filtering() {
        let mut audit = RngAuditLog::new();
        
        // Add test records
        audit.record_draw(1, RngSubsystem::Physics, 0, "physics.rs:10", 100).unwrap();
        audit.record_draw(1, RngSubsystem::Physics, 1, "physics.rs:20", 200).unwrap();
        audit.record_draw(2, RngSubsystem::Physics, 0, "physics.rs:30", 300).unwrap();
        audit.record_draw(1, RngSubsystem::Biology, 0, "biology.rs:10", 400).unwrap();
        
        // Test stream filtering
        let physics_stream0 = audit.records_by_stream(RngSubsystem::Physics, 0);
        assert_eq!(physics_stream0.len(), 2);
        assert_eq!(physics_stream0[0].value, 100);
        assert_eq!(physics_stream0[1].value, 300);
        
        // Test tick filtering
        let tick1_records = audit.records_by_tick(1);
        assert_eq!(tick1_records.len(), 3);
        
        // Test subsystem filtering
        let physics_records = audit.records_by_subsystem(RngSubsystem::Physics);
        assert_eq!(physics_records.len(), 3);
    }
}
