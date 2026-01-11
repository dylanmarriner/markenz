/**
 * ROLE: INFRASTRUCTURE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: WebSocket handlers for real-time communication
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::world::World;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRequest {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub data: Vec<u8>, // RLE compressed
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(world): State<Arc<RwLock<World>>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, world))
}

async fn handle_socket(socket: WebSocket, world: Arc<RwLock<World>>) {
    let (mut sender, mut receiver) = socket.split();
    
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(request) = serde_json::from_str::<ChunkRequest>(&text) {
                    // Handle chunk request
                    let world_read = world.read().await;
                    if let Some(compressed_data) = world_read.get_chunk_data_compressed(request.x, request.y, request.z) {
                        let response = ChunkResponse {
                            x: request.x,
                            y: request.y,
                            z: request.z,
                            data: compressed_data,
                        };
                        
                        if let Ok(response_json) = serde_json::to_string(&response) {
                            let _ = sender.send(Message::Text(response_json)).await;
                        }
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                // Handle binary requests if needed
                tracing::debug!("Received binary data: {} bytes", data.len());
            }
            Ok(Message::Close(_)) => {
                tracing::debug!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

pub async fn chunk_websocket(
    ws: WebSocketUpgrade,
    State(world): State<Arc<RwLock<World>>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_chunk_socket(socket, world))
}

async fn handle_chunk_socket(socket: WebSocket, world: Arc<RwLock<World>>) {
    let (mut sender, mut receiver) = socket.split();
    
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(request) = serde_json::from_str::<ChunkRequest>(&text) {
                    // Handle chunk request for binary streaming
                    let world_read = world.read().await;
                    if let Some(compressed_data) = world_read.get_chunk_data_compressed(request.x, request.y, request.z) {
                        // Send binary chunk data
                        let mut binary_data = Vec::new();
                        
                        // Chunk header: x, y, z (4 bytes each, little endian)
                        binary_data.extend_from_slice(&request.x.to_le_bytes());
                        binary_data.extend_from_slice(&request.y.to_le_bytes());
                        binary_data.extend_from_slice(&request.z.to_le_bytes());
                        
                        // Chunk data
                        binary_data.extend_from_slice(&compressed_data);
                        
                        let _ = sender.send(Message::Binary(binary_data)).await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::debug!("Chunk WebSocket connection closed");
                break;
            }
            Err(e) => {
                tracing::error!("Chunk WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

