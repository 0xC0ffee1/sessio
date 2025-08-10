use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use axum::{extract::State, Json, http::HeaderMap};
use log::{info, error, warn};
use uuid::Uuid;
use serde_json;
use chrono;
use anyhow::{Result, bail, Context};
use crate::db::{DatabaseRepository, models};
use crate::models::{InstallRequest, InstallResponse, ErrorResponse, DeviceMetadata};
use sessio_coordinator_common::common::{HeartbeatRequest, HeartbeatResponse};
use crate::models::{Server, Session, Client};
use crate::auth::jwt::{generate_device_jwt_token, extract_jwt_token_from_headers, validate_device_jwt_token};
use sessio_coordinator_common::common::{ServerPacket, Packet, Auth, AuthResponse};
use tokio::sync::mpsc;
use std::collections::HashMap;

fn validate_public_key(public_key: &str) -> Result<()> {
    if public_key.is_empty() {
        anyhow::bail!("Public key cannot be empty");
    }
    if public_key.len() > 1000 {
        anyhow::bail!("Public key too long (max 1000 characters)");
    }
    Ok(())
}

fn validate_install_key(install_key: &str) -> Result<()> {
    if install_key.is_empty() {
        anyhow::bail!("Install key cannot be empty");
    }
    if install_key.len() < 16 || install_key.len() > 64 {
        anyhow::bail!("Install key length invalid (16-64 characters)");
    }
    if !install_key.chars().all(|c| c.is_ascii_alphanumeric()) {
        anyhow::bail!("Install key contains invalid characters");
    }
    Ok(())
}

pub async fn auth_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(auth_data): Json<Auth>,
) -> Result<Json<AuthResponse>, String> {
    let mut server = state.lock().await;
    
    // Validate JWT token and extract device/account information
    let (device_id, account_id_str) = match validate_device_jwt_token(&auth_data.jwt_token) {
        Some((device_id, account_id)) => (device_id, account_id),
        None => {
            return Ok(Json(AuthResponse {
                success: false,
                token: None,
                status_msg: Some("Invalid or expired JWT token".to_string()),
            }));
        }
    };
    
    // Parse account_id as UUID
    let account_id = match Uuid::parse_str(&account_id_str) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(Json(AuthResponse {
                success: false,
                token: None,
                status_msg: Some("Invalid account ID format in JWT token".to_string()),
            }));
        }
    };
    
    // Verify device_id matches the one in the auth request
    if device_id != auth_data.id {
        return Ok(Json(AuthResponse {
            success: false,
            token: None,
            status_msg: Some("Device ID mismatch in JWT token".to_string()),
        }));
    }
    
    // Update last seen for the device
    if let Err(e) = server.db.update_device_last_seen(&device_id, account_id).await {
        error!("Failed to update device last seen: {}", e);
    }
    
    // Get device info from database
    let device = match server.db.get_device_by_id(&device_id, account_id).await {
        Ok(Some(device)) => device,
        Ok(None) => {
            return Ok(Json(AuthResponse {
                success: false,
                token: None,
                status_msg: Some("Device not found in database".to_string()),
            }));
        }
        Err(e) => {
            error!("Database error: {}", e);
            return Ok(Json(AuthResponse {
                success: false,
                token: None,
                status_msg: Some("Database error".to_string()),
            }));
        }
    };

    // Create the client and insert it into the map
    let client = Client {
        ws_sender: None, // Will be set when WebSocket connects
        id: device_id.clone(),
        session_ids: Vec::new(),
        ipv6: auth_data.ipv6,
        ipv4: auth_data.ipv4.unwrap_or_else(|| "0.0.0.0:0".parse().unwrap()),
        device_uuid: device.id,
        account_id,
    };
    
    server.clients.insert(device_id.clone(), client);


    Ok(Json(AuthResponse {
        success: true,
        token: None,
        status_msg: None,
    }))
}



pub async fn install_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(request): Json<InstallRequest>,
) -> Result<Json<InstallResponse>, Json<ErrorResponse>> {
    let server = state.lock().await;
    
    // Validate input
    if let Err(e) = validate_install_key(&request.install_key) {
        warn!("Invalid install key: {}", e);
        return Err(Json(ErrorResponse {
            success: false,
            error: "Invalid install key format".to_string(),
        }));
    }
    
    if let Some(ref public_key) = request.public_key {
        if let Err(e) = validate_public_key(public_key) {
            warn!("Invalid public key in install request: {}", e);
            return Err(Json(ErrorResponse {
                success: false,
                error: "Invalid public key format".to_string(),
            }));
        }
    }
    
    // Validate install key
    let install_key = match server.db.validate_install_key(&request.install_key).await {
        Ok(Some(key)) => key,
        Ok(None) => return Err(Json(ErrorResponse {
            success: false,
            error: "Invalid or expired install key".to_string(),
        })),
        Err(e) => {
            error!("Failed to validate install key: {}", e);
            return Err(Json(ErrorResponse {
                success: false,
                error: format!("Failed to validate install key: {}", e),
            }));
        }
    };
    

    // Get the device_id that was stored with the install key (provided by frontend)
    let device_id = install_key.device_id.clone()
        .ok_or_else(|| {
            error!("Install key {} has no device_id associated with it", request.install_key);
            Json(ErrorResponse {
                success: false,
                error: "Install key has no device ID - this should not happen".to_string(),
            })
        })?;
    
    info!("Using device_id '{}' from install key {}", device_id, request.install_key);


    // Get account using the install key's account_id - NOT by device_id!
    // Device IDs are only unique within an account, not globally!
    let account = match server.db.get_account_by_id(install_key.account_id).await {
        Ok(Some(acc)) => acc,
        Ok(None) => {
            error!("Install key has invalid account_id: {}", install_key.account_id);
            return Err(Json(ErrorResponse {
                success: false,
                error: "Invalid install key - account not found".to_string(),
            }));
        }
        Err(e) => {
            error!("Failed to get account: {}", e);
            return Err(Json(ErrorResponse {
                success: false,
                error: format!("Failed to get account: {}", e),
            }));
        }
    };
    
    // Check if device already exists for this account
    let device_exists = match server.db.get_device_by_id(&device_id, install_key.account_id).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            error!("Failed to check device existence: {}", e);
            return Err(Json(ErrorResponse {
                success: false,
                error: format!("Failed to check device: {}", e),
            }));
        }
    };
    
    // Create or update device if needed
    if !device_exists {
        let metadata = serde_json::to_value(&request.metadata)
            .unwrap_or_else(|_| serde_json::json!({}));
            
        // Extract categories from install key and create/get all categories
        let mut category_ids = Vec::new();
        if let Some(categories_array) = install_key.categories.as_array() {
            for category_value in categories_array {
                if let Some(category_name) = category_value.as_str() {
                    if !category_name.is_empty() {
                        // Create category if it doesn't exist, or get existing one
                        match server.db.create_category(install_key.account_id, category_name).await {
                            Ok(category) => {
                                info!("Using category '{}' for device {}", category_name, device_id);
                                category_ids.push(category.id);
                            },
                            Err(e) => {
                                warn!("Failed to create/get category '{}': {}", category_name, e);
                            }
                        }
                    }
                }
            }
        }
        
        match server.db.create_or_update_device_with_categories(
            install_key.account_id,
            &device_id,
            Some(&request.metadata.os_name),
            request.public_key.as_deref(),
            metadata,
            &category_ids,
        ).await {
            Ok(_) => {
                info!("Created device {} for account_id {} with category_ids: {:?}", device_id, account.id, category_ids);
            }
            Err(e) => {
                error!("Failed to create device: {}", e);
                return Err(Json(ErrorResponse {
                    success: false,
                    error: format!("Failed to create device: {}", e),
                }));
            }
        }
    }
    
    // Mark install key as used
    if let Err(e) = server.db.mark_install_key_used(install_key.id).await {
        error!("Failed to mark install key as used: {}", e);
    }
    
    info!("Device {} registered to account_id {}", device_id, account.id);
    
    // Generate 6-month JWT token for WebSocket authentication
    let (jwt_token, jwt_expires_at) = match generate_device_jwt_token(&device_id, &account.id.to_string()) {
        Ok(token) => {
            // Update the jwt_token_issued_at field in database
            if let Err(e) = server.db.update_device_jwt_token_issued_at(&device_id, account.id).await {
                error!("Failed to update jwt_token_issued_at for device {}: {}", device_id, e);
            }
            
            // Calculate expiration time (6 months from now)
            let expires_at = chrono::Utc::now() + chrono::Duration::days(6 * 30);
            (Some(token), Some(expires_at))
        },
        Err(e) => {
            error!("Failed to generate JWT token for device {}: {}", device_id, e);
            (None, None)
        }
    };
    
    // Include full passkey JSON for signature verification (instead of just COSE key)
    // We need to get the JSON-serialized Passkey from webauthn_credentials table
    let (passkey_json, passkey_credential_id) = if let Some(cred_id) = &account.passkey_credential_id {
        // Get the stored WebAuthn credential which contains the full JSON Passkey
        match server.db.get_webauthn_credential_by_id(cred_id).await {
            Ok(Some(credential)) => {
                // The credential.public_key contains the JSON-serialized Passkey
                let passkey_json = String::from_utf8(credential.public_key.clone())
                    .unwrap_or_else(|_| {
                        error!("Failed to convert passkey bytes to UTF-8 string");
                        "{}".to_string()
                    });
                (Some(passkey_json), Some(cred_id.clone()))
            }
            Ok(None) => {
                warn!("Passkey credential not found for credential_id: {}", cred_id);
                (None, None)
            }
            Err(e) => {
                error!("Failed to get passkey credential: {}", e);
                (None, None)
            }
        }
    } else {
        (None, None)
    };
    
    Ok(Json(InstallResponse {
        account_id: account.id.to_string(),
        device_id: device_id,
        jwt_token,
        jwt_token_expires_at: jwt_expires_at,
        passkey_public_key: passkey_json, // Now contains full JSON Passkey instead of base64 COSE key
        passkey_credential_id,
    }))
}

pub async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
