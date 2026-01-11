/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Encrypted database persistence
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use rusqlite::{Connection, Result as SqlResult, params};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::sim::{Event, SimTime};

pub struct EncryptedDatabase {
    conn: Connection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KvStore {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventRecord {
    pub seq: u64,
    pub tick: u64,
    pub source: String,
    pub payload_blob: Vec<u8>,
    pub hash: String,
    pub prev_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotRecord {
    pub tick: u64,
    pub state_blob: Vec<u8>,
    pub hash: String,
}

impl EncryptedDatabase {
    pub fn new_with_key<P: AsRef<Path>>(path: P, key: &str) -> SqlResult<Self> {
        // Open database with encryption key
        let conn = Connection::open(path)?;
        
        // Set encryption key - this must be done FIRST
        conn.pragma_update(None, "key", key)?;
        
        // Enable foreign keys and WAL mode
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                seq INTEGER PRIMARY KEY,
                tick INTEGER NOT NULL,
                source TEXT NOT NULL,
                payload_blob BLOB NOT NULL,
                hash TEXT NOT NULL,
                prev_hash TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshots (
                tick INTEGER PRIMARY KEY,
                state_blob BLOB NOT NULL,
                hash TEXT NOT NULL
            )",
            [],
        )?;
        
        // Verify encryption is working by trying to read
        let test_result: SqlResult<i64> = conn.query_row("SELECT COUNT(*) FROM kv_store", [], |row| row.get(0));
        match test_result {
            Ok(_) => {}, // Database is properly encrypted and accessible
            Err(_) => {
                // If we can't read, either encryption failed or database is corrupted
                panic!("Database encryption verification failed. Check your master key.");
            }
        }
        
        Ok(Self { conn })
    }
    
    pub fn new<P: AsRef<Path>>(path: P) -> SqlResult<Self> {
        // Try to get key from environment or file
        let key = std::env::var("MASTER_KEY")
            .or_else(|_| {
                // Try to read from file
                std::fs::read_to_string("master.key")
                    .map_err(|_| std::env::VarError::NotPresent)
            })
            .unwrap_or_else(|_| {
                // For development, use a default key
                panic!("No master key found. Set MASTER_KEY env var or create master.key file");
            });
        
        Self::new_with_key(path, key.trim())
    }
    
    pub fn append_event(&mut self, event: &Event, prev_hash: &str) -> SqlResult<String> {
        let payload_blob = serde_json::to_vec(&event.kind)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        let event_data = format!("{}{}{}{}", event.seq, event.tick.as_u64(), event.source(), serde_json::to_string(&event.kind).unwrap());
        let hash = blake3::hash(event_data.as_bytes()).to_hex().to_string();
        
        self.conn.execute(
            "INSERT INTO events (seq, tick, source, payload_blob, hash, prev_hash) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                event.seq,
                event.tick.as_u64(),
                event.source(),
                payload_blob,
                hash,
                prev_hash
            ],
        )?;
        
        Ok(hash)
    }
    
    pub fn get_events_since(&self, since_tick: SimTime) -> SqlResult<Vec<EventRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT seq, tick, source, payload_blob, hash, prev_hash FROM events WHERE tick >= ?1 ORDER BY seq"
        )?;
        
        let rows = stmt.query_map([since_tick.as_u64()], |row| {
            Ok(EventRecord {
                seq: row.get(0)?,
                tick: row.get(1)?,
                source: row.get(2)?,
                payload_blob: row.get(3)?,
                hash: row.get(4)?,
                prev_hash: row.get(5)?,
            })
        })?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        
        Ok(events)
    }
    
    pub fn get_latest_event_hash(&self) -> SqlResult<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT hash FROM events ORDER BY seq DESC LIMIT 1")?;
        let rows = stmt.query_map([], |row| {
            row.get::<_, String>(0)
        })?;
        
        for row in rows {
            return Ok(Some(row?));
        }
        
        Ok(None)
    }
    
    pub fn save_snapshot(&mut self, tick: SimTime, state_blob: Vec<u8>) -> SqlResult<String> {
        let hash = blake3::hash(&state_blob).to_hex().to_string();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO snapshots (tick, state_blob, hash) VALUES (?1, ?2, ?3)",
            params![tick.as_u64(), state_blob, hash],
        )?;
        
        Ok(hash)
    }
    
    pub fn get_latest_snapshot(&self) -> SqlResult<Option<SnapshotRecord>> {
        let mut stmt = self.conn.prepare("SELECT tick, state_blob, hash FROM snapshots ORDER BY tick DESC LIMIT 1")?;
        let rows = stmt.query_map([], |row| {
            Ok(SnapshotRecord {
                tick: row.get(0)?,
                state_blob: row.get(1)?,
                hash: row.get(2)?,
            })
        })?;
        
        for row in rows {
            return Ok(Some(row?));
        }
        
        Ok(None)
    }
    
    pub fn get_kv(&self, key: &str) -> SqlResult<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM kv_store WHERE key = ?1")?;
        let rows = stmt.query_map([key], |row| {
            row.get::<_, String>(0)
        })?;
        
        for row in rows {
            return Ok(Some(row?));
        }
        
        Ok(None)
    }
    
    pub fn set_kv(&mut self, key: &str, value: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
    
    pub fn get_current_tick(&self) -> SqlResult<SimTime> {
        match self.get_kv("current_tick")? {
            Some(tick_str) => {
                let tick = tick_str.parse::<u64>()
                    .map_err(|_| rusqlite::Error::InvalidColumnType(0, "current_tick".to_string(), rusqlite::types::Type::Text))?;
                Ok(SimTime::from_u64(tick))
            }
            None => Ok(SimTime::zero())
        }
    }
    
    pub fn set_current_tick(&mut self, tick: SimTime) -> SqlResult<()> {
        self.set_kv("current_tick", &tick.as_u64().to_string())
    }
    
    pub fn get_genesis_seed(&self) -> SqlResult<Option<u64>> {
        match self.get_kv("genesis_seed")? {
            Some(seed_str) => {
                let seed = seed_str.parse::<u64>()
                    .map_err(|_| rusqlite::Error::InvalidColumnType(0, "genesis_seed".to_string(), rusqlite::types::Type::Text))?;
                Ok(Some(seed))
            }
            None => Ok(None)
        }
    }
    
    pub fn verify_chain(&self) -> SqlResult<bool> {
        let mut stmt = self.conn.prepare("SELECT seq, hash, prev_hash FROM events ORDER BY seq")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, u64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?;
        
        let mut prev_hash: Option<String> = None;
        for row in rows {
            let (seq, hash, prev_hash_from_db) = row?;
            
            if let Some(expected_prev) = prev_hash {
                if expected_prev != prev_hash_from_db {
                    eprintln!("Chain break at seq {}: expected {}, got {}", seq, expected_prev, prev_hash_from_db);
                    return Ok(false);
                }
            }
            
            prev_hash = Some(hash);
        }
        
        Ok(true)
    }
}

impl Event {
    fn source(&self) -> String {
        match &self.kind {
            crate::sim::EventKind::Input(_) => "input".to_string(),
            crate::sim::EventKind::Tick => "tick".to_string(),
            crate::sim::EventKind::AgentSpawn { agent_id } => format!("agent:{}", agent_id),
            crate::sim::EventKind::AgentMove { agent_id, .. } => format!("agent:{}", agent_id),
            crate::sim::EventKind::Chat { agent_id, .. } => format!("agent:{}", agent_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::{EventKind, InputEvent};
    use tempfile::tempdir;
    
    #[test]
    fn test_encryption_enforced() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let key = "test_key_123456789012345678901234567890";
        
        // Create database with key
        let mut db = EncryptedDatabase::new_with_key(&db_path, key).unwrap();
        db.set_kv("test", "value").unwrap();
        drop(db);
        
        // Try to open without key - should fail
        let result = EncryptedDatabase::new_with_key(&db_path, "");
        assert!(result.is_err());
        
        // Try to open with wrong key - should fail
        let result = EncryptedDatabase::new_with_key(&db_path, "wrong_key");
        assert!(result.is_err());
        
        // Open with correct key - should succeed
        let db = EncryptedDatabase::new_with_key(&db_path, key).unwrap();
        let value = db.get_kv("test").unwrap().unwrap();
        assert_eq!(value, "value");
    }
    
    #[test]
    fn test_event_chain() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let key = "test_key_123456789012345678901234567890";
        
        let mut db = EncryptedDatabase::new_with_key(&db_path, key).unwrap();
        
        let time = SimTime::from_u64(1);
        let event = Event::new(1, time, EventKind::Tick);
        
        let hash1 = db.append_event(&event, "genesis").unwrap();
        
        let event2 = Event::new(2, time, EventKind::Input(InputEvent::Chat { 
            user_id: "test".to_string(), 
            message: "hello".to_string() 
        }));
        let hash2 = db.append_event(&event2, &hash1).unwrap();
        
        assert!(db.verify_chain().unwrap());
    }
}
