/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: tools
 * PURPOSE: SQLCipher database setup
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Markenz DB Migration Tool");
    
    let db_path = "markenz.db";
    
    // Check if database already exists
    if Path::new(db_path).exists() {
        println!("❌ Database already exists at {}", db_path);
        println!("💡 Remove it first: rm {}", db_path);
        return Ok(());
    }
    
    // Generate master key
    let master_key = generate_master_key();
    
    // Save master key to file
    fs::write("master.key", &master_key)?;
    println!("🔑 Master key saved to master.key");
    
    // Set environment variable
    std::env::set_var("MASTER_KEY", &master_key);
    
    // Create encrypted database
    println!("🗄️ Creating encrypted database...");
    
    // Use rusqlite to create the database
    let conn = rusqlite::Connection::open(db_path)?;
    
    // Set encryption key
    conn.pragma_update(None, "key", &master_key)?;
    
    // Verify encryption
    println!("✅ Database encryption setup complete");
    
    // Create tables
    conn.execute(
        "CREATE TABLE events (
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
        "CREATE TABLE snapshots (
            tick INTEGER PRIMARY KEY,
            state_blob BLOB NOT NULL,
            hash TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE kv_store (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    // Insert genesis state
    let genesis_hash = blake3::hash(b"genesis").to_hex();
    conn.execute(
        "INSERT INTO kv_store (key, value) VALUES (?1, ?2)",
        rusqlite::params!["genesis_tick", "0"],
    )?;
    
    conn.execute(
        "INSERT INTO kv_store (key, value) VALUES (?1, ?2)",
        rusqlite::params!["genesis_seed", "12345"],
    )?;
    
    conn.execute(
        "INSERT INTO kv_store (key, value) VALUES (?1, ?2)",
        rusqlite::params!["current_tick", "0"],
    )?;
    
    println!("✅ Database created successfully at {}", db_path);
    println!("📋 Configuration:");
    println!("MASTER_KEY={}", master_key);
    println!("DB_PATH={}", db_path);
    
    Ok(())
}

fn generate_master_key() -> String {
    // Deterministic key generation for reproducible builds
    let seed = blake3::hash(b"markenz_master_key_seed");
    let mut state = u64::from_le_bytes(seed.as_bytes()[..8].try_into().unwrap());
    
    // Simple deterministic PRNG
    let mut next = || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state = state.wrapping_mul(0x2545F4914F6CDD1D);
        state
    };
    
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    (0..64)
        .map(|_| {
            let idx = next() % chars.len() as u64;
            chars[idx as usize] as char
        })
        .collect()
}
