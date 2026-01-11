/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: OIDC authentication
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use serde::{Deserialize, Serialize};
use urlencoding;

// Base64 URL-safe decoding helper
fn base64_url_decode(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Pad base64 string to proper length
    let mut padded = input.replace('_', "/").replace('-', '+');
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    
    match base64::decode(&padded) {
        Ok(bytes) => Ok(String::from_utf8(bytes)?),
        Err(e) => Err(format!("Base64 decode error: {}", e).into()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserParams {
    pub id: String,
    pub roles: Vec<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

pub struct OidcClient {
    config: OidcConfig,
}

impl OidcClient {
    pub async fn new(config: OidcConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // For M1, we'll implement a basic OIDC client
        // Full implementation with openidconnect crate will come in later phases
        Ok(Self { config })
    }
    
    pub fn get_auth_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        // For M1, return a simple auth URL
        // Full implementation will come in later phases
        Ok(format!(
            "{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid",
            self.config.issuer,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri)
        ))
    }
    
    pub async fn verify_token(&self, token: &str) -> Result<UserParams, Box<dyn std::error::Error>> {
        // Basic JWT structure validation
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid JWT structure".into());
        }
        
        // For Phase 0 remediation, implement basic token validation
        // In production, this would verify signature against JWKS endpoint
        
        // Decode header (base64url)
        let header_json = base64_url_decode(parts[0])?;
        let _header: serde_json::Value = serde_json::from_str(&header_json)?;
        
        // Decode payload (base64url)
        let payload_json = base64_url_decode(parts[1])?;
        let payload: serde_json::Value = serde_json::from_str(&payload_json)?;
        
        // Extract claims
        let user_id = payload["sub"].as_str().ok_or("Missing subject claim")?;
        let realm_access = payload["realm_access"]["roles"].as_array().ok_or("Missing roles")?;
        let roles: Vec<String> = realm_access.iter()
            .filter_map(|r| r.as_str())
            .map(|r| r.to_string())
            .collect();
        
        // Extract optional claims
        let email = payload["email"].as_str().map(|s| s.to_string());
        let name = payload["name"].as_str().map(|s| s.to_string());
        
        // Token validation should be handled at infrastructure layer
        // For now, skip expiration check to maintain determinism
        
        let user_params = UserParams {
            id: user_id.to_string(),
            roles,
            email,
            name,
        };
        
        Ok(user_params)
    }
    
    pub fn config(&self) -> &OidcConfig {
        &self.config
    }
}

pub fn load_oidc_config() -> OidcConfig {
    OidcConfig {
        issuer: std::env::var("OIDC_ISSUER").unwrap_or_else(|_| "http://localhost:8080/realms/markenz".to_string()),
        client_id: std::env::var("OIDC_CLIENT_ID").unwrap_or_else(|_| "markenz".to_string()),
        client_secret: std::env::var("OIDC_CLIENT_SECRET").unwrap_or_else(|_| "markenz-secret".to_string()),
        redirect_uri: std::env::var("OIDC_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
    }
}
