use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{extract::State, Json, http::HeaderMap};
use log::{info, error, warn};
use uuid::Uuid;
use crate::db::DatabaseRepository;
use crate::models::{DeviceRequest, DeviceResponse, DeleteDeviceRequest, DeleteDeviceResponse, AuthorizedKeysRequest, AuthorizedKeysResponse, AuthorizedKey, UpdateDeviceRequest, UpdateDeviceResponse, DevicesWithCategoriesResponse, ErrorResponse};
use crate::models::Server;
use crate::auth::jwt::{extract_account_from_jwt, extract_account_id_from_device_jwt};
use anyhow::Result;

fn sanitize_log_message(msg: &str) -> String {
    // Remove potential sensitive data patterns from log messages
    let sensitive_patterns = [
        (r"password[=:\s]+[^\s]+", "password=***"),
        (r"secret[=:\s]+[^\s]+", "secret=***"),
        (r"token[=:\s]+[^\s]+", "token=***"),
        (r"key[=:\s]+[^\s]+", "key=***"),
        (r"\b\d{4}-\d{4}-\d{4}-\d{4}\b", "****-****-****-****"), // account numbers
    ];
    
    let mut sanitized = msg.to_string();
    for (pattern, replacement) in sensitive_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            sanitized = re.replace_all(&sanitized, replacement).to_string();
        }
    }
    sanitized
}

fn validate_device_id(device_id: &str) -> Result<()> {
    if device_id.is_empty() {
        anyhow::bail!("Device ID cannot be empty");
    }
    if device_id.len() > 100 {
        anyhow::bail!("Device ID too long (max 100 characters)");
    }
    if !device_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        anyhow::bail!("Device ID contains invalid characters");
    }
    Ok(())
}

pub async fn device_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(request): Json<DeviceRequest>,
) -> Result<Json<DeviceResponse>, Json<ErrorResponse>> {
    let server = state.lock().await;
    
    // Extract account_id from JWT token (frontend must be authenticated via passkey)
    let account_id_str = extract_account_from_jwt(&headers)
        .ok_or_else(|| Json(ErrorResponse {
            success: false,
            error: "Authentication required. Please login with your passkey.".to_string(),
        }))?;
    
    let account_id = uuid::Uuid::parse_str(&account_id_str)
        .map_err(|_| Json(ErrorResponse {
            success: false,
            error: "Invalid account ID format".to_string(),
        }))?;
    
    // Validate input
    if let Err(e) = validate_device_id(&request.device_id) {
        warn!("Invalid device ID: {}", e);
        return Err(Json(ErrorResponse {
            success: false,
            error: "Invalid device ID format".to_string(),
        }));
    }
    
    // Look up account by account_id from JWT
    let account = match server.db.get_account_by_id(account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => return Err(Json(ErrorResponse {
            success: false,
            error: "Account not found. Please re-authenticate.".to_string(),
        })),
        Err(e) => {
            error!("Failed to get account: {}", sanitize_log_message(&e.to_string()));
            return Err(Json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
            }));
        }
    };
    
    // Handle categories if provided
    let categories = request.categories.as_deref();
    
    match server.db.create_device_install_key(account.id, &request.device_id, categories).await {
        Ok(install_key) => {
            info!("Created install key for device: {} on account with categories: {:?}", request.device_id, categories);
            Ok(Json(DeviceResponse {
                install_key: install_key.install_key,
            }))
        }
        Err(e) => {
            error!("Failed to create install key: {}", sanitize_log_message(&e.to_string()));
            Err(Json(ErrorResponse {
                success: false,
                error: "Failed to create install key".to_string(),
            }))
        }
    }
}

pub async fn delete_device_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(request): Json<DeleteDeviceRequest>,
) -> Result<Json<DeleteDeviceResponse>, Json<ErrorResponse>> {
    let server = state.lock().await;
    
    // Extract account_id from JWT token (frontend must be authenticated via passkey)
    let account_id_str = extract_account_from_jwt(&headers)
        .ok_or_else(|| Json(ErrorResponse {
            success: false,
            error: "Authentication required. Please login with your passkey.".to_string(),
        }))?;
    
    let account_id = uuid::Uuid::parse_str(&account_id_str)
        .map_err(|_| Json(ErrorResponse {
            success: false,
            error: "Invalid account ID format".to_string(),
        }))?;
    
    // Look up account by account_id from JWT
    let account = match server.db.get_account_by_id(account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => return Ok(Json(DeleteDeviceResponse {
            success: false,
            message: "Account not found. Please re-authenticate.".to_string(),
        })),
        Err(e) => {
            error!("Failed to get account: {}", e);
            return Ok(Json(DeleteDeviceResponse {
                success: false,
                message: format!("Failed to get account: {}", e),
            }));
        }
    };
    
    match server.db.delete_device(&request.device_id, account.id).await {
        Ok(true) => {
            info!("Deleted device: {} from account_id: {}", request.device_id, account.id);
            Ok(Json(DeleteDeviceResponse {
                success: true,
                message: format!("Device '{}' has been successfully deleted", request.device_id),
            }))
        }
        Ok(false) => {
            Ok(Json(DeleteDeviceResponse {
                success: false,
                message: format!("Device '{}' not found in this account", request.device_id),
            }))
        }
        Err(e) => {
            error!("Failed to delete device: {}", e);
            Ok(Json(DeleteDeviceResponse {
                success: false,
                message: format!("Failed to delete device: {}", e),
            }))
        }
    }
}

pub async fn update_device_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(request): Json<UpdateDeviceRequest>,
) -> Result<Json<UpdateDeviceResponse>, Json<ErrorResponse>> {
    let server = state.lock().await;
    
    // Extract account_id from JWT token (frontend must be authenticated via passkey)
    let account_id_str = extract_account_from_jwt(&headers)
        .ok_or_else(|| Json(ErrorResponse {
            success: false,
            error: "Authentication required. Please login with your passkey.".to_string(),
        }))?;
    
    let account_id = match uuid::Uuid::parse_str(&account_id_str) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Json(ErrorResponse {
            success: false,
            error: "Invalid account ID format".to_string(),
        }))
    };
    
    // Handle categories assignment if provided
    let category_ids = if let Some(category_names) = &request.categories {
        let mut ids = Vec::new();
        for category_name in category_names {
            if !category_name.is_empty() {
                // Create category if it doesn't exist, or get existing one
                match server.db.create_category(account_id, category_name).await {
                    Ok(category) => ids.push(category.id),
                    Err(e) => {
                        error!("Failed to create/get category: {}", e);
                        return Err(Json(ErrorResponse {
                            success: false,
                            error: "Failed to process category".to_string(),
                        }));
                    }
                }
            }
        }
        Some(ids)
    } else {
        None // Keep existing categories if not specified
    };
    
    // First get the device to find its database UUID
    let device = match server.db.get_device_by_id(&request.device_id, account_id).await {
        Ok(Some(device)) => device,
        Ok(None) => return Err(Json(ErrorResponse {
            success: false,
            error: "Device not found".to_string(),
        })),
        Err(e) => {
            error!("Failed to get device: {}", e);
            return Err(Json(ErrorResponse {
                success: false,
                error: "Failed to get device".to_string(),
            }));
        }
    };
    
    // Update the device basic info (os_name)
    let primary_category_id = category_ids.as_ref().and_then(|ids| ids.first().copied());
    match server.db.update_device(
        &request.device_id,
        account_id,
        request.os_name.as_deref(),
        primary_category_id,
        false, // Don't update category_id field in the device table
    ).await {
        Ok(_) => {
            // If categories were provided, update the many-to-many relationship
            if let Some(ref category_ids) = category_ids {
                if let Err(e) = server.db.set_device_categories(device.id, category_ids).await {
                    error!("Failed to update device categories: {}", e);
                    return Err(Json(ErrorResponse {
                        success: false,
                        error: "Failed to update device categories".to_string(),
                    }));
                }
                info!("Updated device: {} for account_id: {} with categories: {:?}", request.device_id, account_id, category_ids);
            } else {
                info!("Updated device: {} for account_id: {}", request.device_id, account_id);
            }
            
            Ok(Json(UpdateDeviceResponse {
                success: true,
                message: Some("Device updated successfully".to_string()),
            }))
        }
        Err(e) => {
            error!("Failed to update device: {}", e);
            Err(Json(ErrorResponse {
                success: false,
                error: format!("Failed to update device: {}", e),
            }))
        }
    }
}


//todo fix invalid account id format for device jwt
pub async fn devices_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
) -> Result<Json<DevicesWithCategoriesResponse>, Json<ErrorResponse>> {
    let server = state.lock().await;
    
    // Extract account_id from JWT token (accept both user and device tokens)
    let account_id_str = extract_account_from_jwt(&headers)
        .or_else(|| extract_account_id_from_device_jwt(&headers))
        .ok_or_else(|| Json(ErrorResponse {
            success: false,
            error: "Authentication required. Please provide a valid user or device JWT token.".to_string(),
        }))?;
    
    let account_id = match uuid::Uuid::parse_str(&account_id_str) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Json(ErrorResponse {
            success: false,
            error: "Invalid account ID format".to_string(),
        }))
    };
    
    let devices_result = server.db.get_devices_with_categories(account_id).await;
    let categories_result = server.db.get_categories_by_account_id(account_id).await;
    
    match (devices_result, categories_result) {
        (Ok(devices), Ok(categories)) => {
            info!("Retrieved {} devices and {} categories for account_id: {}", devices.len(), categories.len(), account_id);
            Ok(Json(DevicesWithCategoriesResponse {
                devices,
                categories,
            }))
        }
        (Err(e), _) | (_, Err(e)) => {
            error!("Failed to get devices or categories: {}", e);
            Err(Json(ErrorResponse {
                success: false,
                error: format!("Failed to get devices or categories: {}", e),
            }))
        }
    }
}

pub async fn authorized_keys_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(_request): Json<AuthorizedKeysRequest>,
) -> Result<Json<AuthorizedKeysResponse>, String> {
    let server = state.lock().await;
    
    // Extract account_id from device JWT token in Authorization header
    let account_id = match extract_account_id_from_device_jwt(&headers) {
        Some(account_id_str) => {
            match uuid::Uuid::parse_str(&account_id_str) {
                Ok(uuid) => uuid,
                Err(_) => return Err("Invalid account ID format".to_string()),
            }
        }
        None => return Err("Missing or invalid device JWT token".to_string()),
    };
    
    match server.db.get_authorized_keys_by_account_id(account_id).await {
        Ok(key_pairs) => {
            let keys: Vec<AuthorizedKey> = key_pairs
                .into_iter()
                .map(|(device_id, public_key, os_name, signature, signed_at, signer_credential_id)| AuthorizedKey {
                    device_id,
                    public_key,
                    os_name,
                    signature,
                    signed_at,
                    signer_credential_id,
                })
                .collect();
            
            info!("Retrieved {} signed authorized keys for account_id: {}", keys.len(), account_id);
            Ok(Json(AuthorizedKeysResponse {
                keys,
            }))
        }
        Err(e) => {
            error!("Failed to get authorized keys: {}", e);
            Err(format!("Failed to get authorized keys: {}", e))
        }
    }
}
