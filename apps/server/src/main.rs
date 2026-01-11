/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Initialize Axum server main entry point
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use axum::{Router, routing::get, response::Json, http::StatusCode, extract::Request, extract::State};
use tower_http::cors::CorsLayer;
use tracing_subscriber::fmt;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use markenz_server::world::World;
use markenz_server::api::websocket;

#[tokio::main]
async fn main() {
    fmt::init();
    
    println!("🚀 Starting Markenz server...");
    
    // Initialize world
    let world = Arc::new(RwLock::new(World::new(16))); // 16x16x16 chunks
    println!("🌍 World initialized");
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/config", get(auth_config))
        .route("/auth/me", get(auth_me))
        .route("/admin/command", get(admin_command))
        .route("/ws/main", get(websocket::websocket_handler))
        .route("/ws/chunks", get(websocket::chunk_websocket))
        .with_state(world)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    println!("🏥 Health check received...");
    Json(json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

async fn auth_config() -> Json<serde_json::Value> {
    Json(json!({
        "issuer": std::env::var("OIDC_ISSUER").unwrap_or_else(|_| "http://localhost:8080/realms/markenz".to_string()),
        "client_id": std::env::var("OIDC_CLIENT_ID").unwrap_or_else(|_| "markenz".to_string())
    }))
}

async fn auth_me(
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

async fn admin_command() -> Json<serde_json::Value> {
    Json(json!({
        "status": "admin_command_received"
    }))
}

async fn test_database() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "server_working"
    })))
}

