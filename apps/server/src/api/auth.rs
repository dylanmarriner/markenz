/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Auth endpoints
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use axum::{response::Json, http::StatusCode, extract::Request};
use serde_json::json;

pub async fn auth_config() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "issuer": std::env::var("OIDC_ISSUER").unwrap_or_else(|_| "http://localhost:8080/realms/markenz".to_string()),
        "client_id": std::env::var("OIDC_CLIENT_ID").unwrap_or_else(|_| "markenz".to_string())
    })))
}

pub async fn auth_me(
    req: Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Extract Authorization header
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    match auth_header {
        Some(token) => {
            // Verify JWT token using OIDC client
            let oidc_config = markenz_server::auth::oidc::load_oidc_config();
            let oidc_client = markenz_server::auth::oidc::OidcClient::new(oidc_config).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let user_params = oidc_client.verify_token(token)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            
            Ok(Json(json!({
                "id": user_params.id,
                "roles": user_params.roles,
                "email": user_params.email,
                "name": user_params.name
            })))
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
