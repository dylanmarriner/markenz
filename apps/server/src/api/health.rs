/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Health check endpoint
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */


use axum::{response::Json, http::StatusCode};
use serde_json::json;

pub async fn health() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "version": "0.1.0"
    })))
}
