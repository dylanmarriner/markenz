/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: tools
 * PURPOSE: Auth bootstrap for IdP setup
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use std::process::Command;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Markenz Auth Bootstrap Tool");
    
    // Check if Keycloak is running
    let keycloak_health = Command::new("curl")
        .args(&["-s", "http://localhost:8080/health/ready"])
        .output();
    
    match keycloak_health {
        Ok(output) if output.status.success() => {
            println!("✅ Keycloak is running");
            setup_keycloak()?;
        }
        _ => {
            println!("❌ Keycloak not found. Starting with docker compose...");
            start_keycloak()?;
        }
    }
    
    println!("🎉 Auth bootstrap complete!");
    Ok(())
}

fn setup_keycloak() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Setting up Keycloak realm...");
    
    // Get admin token
    let token_output = Command::new("curl")
        .args(&[
            "-s", "-X", "POST",
            "http://localhost:8080/realms/master/protocol/openid-connect/token",
            "-H", "Content-Type: application/x-www-form-urlencoded",
            "-d", "client_id=admin-cli",
            "-d", "grant_type=password",
            "-d", "username=admin",
            "-d", "password=admin"
        ])
        .output()?;
    
    if !token_output.status.success() {
        return Err("Failed to get admin token".into());
    }
    
    let token_response: serde_json::Value = serde_json::from_slice(&token_output.stdout)?;
    let token = token_response["access_token"]
        .as_str()
        .ok_or("No access token in response")?;
    
    println!("✅ Got admin token");
    
    // Create realm if it doesn't exist
    let realm_check = Command::new("curl")
        .args(&[
            "-s", "-H", &format!("Authorization: Bearer {}", token),
            "http://localhost:8080/admin/realms/markenz"
        ])
        .output()?;
    
    if realm_check.status.success() {
        println!("✅ Realm 'markenz' already exists");
    } else {
        println!("📝 Creating realm 'markenz'...");
        let realm_data = json!({
            "realm": "markenz",
            "enabled": true,
            "roles": {
                "realm": [
                    {"name": "creator_admin"},
                    {"name": "observer"}
                ]
            },
            "clients": [
                {
                    "clientId": "markenz",
                    "name": "Markenz Web Client",
                    "description": "Markenz web application",
                    "enabled": true,
                    "clientAuthenticatorType": "client-secret",
                    "secret": "markenz-secret",
                    "redirectUris": ["http://localhost:3000/*"],
                    "webOrigins": ["http://localhost:3000"],
                    "protocol": "openid-connect",
                    "publicClient": false,
                    "standardFlowEnabled": true,
                    "directAccessGrantsEnabled": true
                }
            ]
        });
        
        let create_realm = Command::new("curl")
            .args(&[
                "-s", "-X", "POST",
                "-H", &format!("Authorization: Bearer {}", token),
                "-H", "Content-Type: application/json",
                "-d", &realm_data.to_string(),
                "http://localhost:8080/admin/realms"
            ])
            .output()?;
        
        if create_realm.status.success() {
            println!("✅ Realm 'markenz' created successfully");
        } else {
            eprintln!("❌ Failed to create realm: {}", String::from_utf8_lossy(&create_realm.stderr));
        }
    }
    
    // Print configuration
    println!("\n📋 Configuration:");
    println!("OIDC_ISSUER=http://localhost:8080/realms/markenz");
    println!("OIDC_CLIENT_ID=markenz");
    println!("OIDC_CLIENT_SECRET=markenz-secret");
    
    Ok(())
}

fn start_keycloak() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐳 Starting Keycloak with docker compose...");
    
    let output = Command::new("docker")
        .args(&["compose", "-f", "infra/auth/compose.yml", "up", "-d", "--profile", "keycloak"])
        .current_dir("..")
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Failed to start Keycloak: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    println!("⏳ Waiting for Keycloak to be ready...");
    std::thread::sleep(std::time::Duration::from_secs(10));
    
    // Wait for health check
    for _ in 0..30 {
        let health = Command::new("curl")
            .args(&["-s", "http://localhost:8080/health/ready"])
            .output()?;
        
        if health.status.success() {
            println!("✅ Keycloak is ready!");
            setup_keycloak()?;
            return Ok(());
        }
        
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    
    Err("Keycloak failed to start within timeout".into())
}
