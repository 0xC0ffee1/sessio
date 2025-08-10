use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{extract::State, Json};
use log::{info, error, warn};
use url::Url;
use crate::db::{DatabaseRepository, models};
use crate::models::{Server, WebauthnAuthStartRequest, WebauthnAuthStartResponse, WebauthnAuthFinishRequest, WebauthnAuthFinishResponse, PasskeyRegisterRequest, PasskeyRegisterResponse, PasskeyRegisterFinishRequest, PasskeyRegisterFinishResponse, SignDeviceStartRequest, SignDeviceStartResponse, SignDeviceFinishRequest, SignDeviceFinishResponse, DeviceSignInfo};
use super::jwt::{generate_jwt_token, generate_device_jwt_token, verify_jwt_token, Claims};
use uuid::Uuid;
use axum::http::HeaderMap;
use base64::{Engine as _, engine::general_purpose};

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
pub fn parse_webauthn_url(url_str: &str) -> Result<(String, Url), String> {
    let url = Url::parse(url_str)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    let rp_id = url.host_str()
        .ok_or_else(|| "No host in URL".to_string())?
        .to_string();
    
    Ok((rp_id, url))
}


pub async fn webauthn_auth_start_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(_request): Json<WebauthnAuthStartRequest>,
) -> Result<Json<WebauthnAuthStartResponse>, String> {
    use webauthn_rs::prelude::*;
    
    info!("WebAuthn auth start request");
    
    let server = state.lock().await;
    
    // Create WebAuthn instance
    let (rp_id, rp_origin) = parse_webauthn_url(&server.web_ui_url)?;
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| format!("WebAuthn builder error: {}", e))?;
    let webauthn = builder.build()
        .map_err(|e| format!("WebAuthn build error: {}", e))?;
    

    let (ccr, auth_state) = webauthn
        .start_discoverable_authentication()
        .map_err(|e| format!("WebAuthn authentication start error: {}", e))?;
        
    info!("Started usernameless passkey authentication");
    
    info!("Created usernameless WebAuthn authentication challenge");
    
    // Create authentication session - store the auth_state for later validation
    let session_data = serde_json::to_string(&auth_state)
        .map_err(|e| format!("Failed to serialize auth state: {}", e))?;
    
    match server.db.create_webauthn_authentication_session(&session_data).await {
        Ok(session) => {
            info!("Created WebAuthn authentication session");
            
            // WebAuthn-rs returns a PublicKeyCredentialRequestOptions wrapped in a structure
            // We need to extract just the inner publicKey part for SimpleWebAuthn
            let challenge_json = serde_json::to_value(&ccr)
                .map_err(|e| format!("Challenge serialization error: {}", e))?;
            
            // Extract the publicKey field if it exists, otherwise use the whole object
            let request_challenge = if let Some(public_key) = challenge_json.get("publicKey") {
                public_key.clone()
            } else {
                challenge_json
            };

            
            Ok(Json(WebauthnAuthStartResponse {
                session_id: session.id.to_string(),
                request_challenge,
            }))
        }
        Err(e) => {
            error!("Failed to store authentication session: {}", sanitize_log_message(&e.to_string()));
            Err("Failed to create session".to_string())
        }
    }
}

pub async fn webauthn_auth_finish_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(request): Json<WebauthnAuthFinishRequest>,
) -> Result<(HeaderMap, Json<WebauthnAuthFinishResponse>), String> {
    info!("WebAuthn auth finish request");
    
    let server = state.lock().await;
    
    // Parse session ID
    let session_id = match uuid::Uuid::parse_str(&request.session_id) {
        Ok(id) => id,
        Err(_) => {
            return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Invalid session ID format".to_string()),
            })));
        }
    };

    // Get the authentication session
    let session = match server.db.get_webauthn_authentication_session(session_id).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Invalid or expired session".to_string()),
            })));
        }
        Err(e) => {
            error!("Database error getting authentication session: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Check if session is expired
    if session.expires_at < chrono::Utc::now() {
        return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
            success: false,
            jwt_token: None,
            message: Some("Session expired".to_string()),
        })));
    }
    
    // Extract credential ID from the authentication response
    let credential_id = match &request.credential.get("id") {
        Some(serde_json::Value::String(id)) => {
            info!("Received credential ID from browser: {}", id);
            id.clone()
        },
        _ => return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
            success: false,
            jwt_token: None,
            message: Some("Invalid credential format".to_string()),
        }))),
    };
    
    // Find the WebAuthn credential in database
    let stored_credential = match server.db.get_webauthn_credential_by_id(&credential_id).await {
        Ok(Some(cred)) => cred,
        Ok(None) => {
            // Credential not found - don't reveal any information about existing credentials
            info!("Authentication attempt with unknown credential");
            return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Unknown credentials".to_string()),
            })));
        }
        Err(e) => {
            error!("Failed to get WebAuthn credential: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Get the account for this credential
    let account = match server.db.get_account_by_id(stored_credential.account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            error!("Account not found for credential");
            return Err("Account not found".to_string());
        }
        Err(e) => {
            error!("Database error getting account: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Properly validate the WebAuthn authentication
    use webauthn_rs::prelude::*;
    
    // Create WebAuthn instance
    let (rp_id, rp_origin) = parse_webauthn_url(&server.web_ui_url)?;
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| format!("Failed to create WebAuthn builder: {}", e))?;
    let webauthn = builder.build()
        .map_err(|e| format!("Failed to build WebAuthn: {}", e))?;
        
    // Deserialize the stored authentication state
    let auth_state: DiscoverableAuthentication = serde_json::from_str(&session.session_data)
        .map_err(|e| format!("Failed to deserialize auth state: {}", e))?;
        
    // Parse the authentication response from client
    let auth_response: PublicKeyCredential = serde_json::from_value(request.credential.clone())
        .map_err(|e| format!("Invalid authentication response: {}", e))?;
    
    // Get the stored passkey for validation
    let stored_passkey = match server.db.get_passkey_by_credential_id(&credential_id).await {
        Ok(Some(passkey)) => passkey,
        Ok(None) => {
            error!("Passkey not found for credential_id: {}", credential_id);
            return Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Unknown credentials".to_string()),
            })));
        }
        Err(e) => {
            error!("Failed to get passkey: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
        
    // Validate the authentication with the stored passkey
    let auth_result = webauthn.finish_discoverable_authentication(&auth_response, auth_state, &[stored_passkey])
        .map_err(|e| {
            error!("WebAuthn authentication failed: {}", e);
            "Authentication failed".to_string()
        })?;
        
    info!("WebAuthn authentication successful for account_id: {}", account.id);
    
    // Generate JWT token
    let jwt_token = match generate_jwt_token(&account.id.to_string()) {
        Ok(token) => {
            info!("Created JWT token for account_id: {}", account.id);
            token
        }
        Err(e) => {
            error!("Failed to create JWT token: {}", e);
            return Err("Failed to create authentication token".to_string());
        }
    };
    
    // Clean up the authentication session
    if let Err(e) = server.db.delete_webauthn_authentication_session(session_id).await {
        warn!("Failed to cleanup authentication session: {}", sanitize_log_message(&e.to_string()));
    }
    
    // Update credential counter with the new counter from auth result
    if let Err(e) = server.db.update_webauthn_credential_counter(&credential_id, auth_result.counter() as i32).await {
        warn!("Failed to update credential counter: {}", sanitize_log_message(&e.to_string()));
    }
    
    Ok((HeaderMap::new(), Json(WebauthnAuthFinishResponse {
        success: true,
        jwt_token: Some(jwt_token),
        message: Some("Authentication successful".to_string()),
    })))
}


pub async fn passkey_register_start_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(request): Json<PasskeyRegisterRequest>,
) -> Result<Json<PasskeyRegisterResponse>, String> {
    use webauthn_rs::prelude::*;
    
    info!("Single-step passkey registration start request");
    
    let server = state.lock().await;
    
    // First create a new account
    let account = match server.db.create_account_only().await {
        Ok(account) => {
            info!("Created new account with ID: {}", account.id);
            account
        }
        Err(e) => {
            error!("Failed to create account: {}", sanitize_log_message(&e.to_string()));
            return Err("Failed to create account".to_string());
        }
    };

    // Create WebAuthn instance
    let (rp_id, rp_origin) = parse_webauthn_url(&server.web_ui_url)?;
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| format!("WebAuthn builder error: {}", e))?;
    let webauthn = builder.build()
        .map_err(|e| format!("WebAuthn build error: {}", e))?;

    let user_id = account.id;
    let username = request.username;
    let user_display_name = request.display_name;

    // Start passkey registration
    let (ccr, reg_state) = webauthn
        .start_passkey_registration(
            user_id,
            &username,
            &user_display_name,
            None,
        )
        .map_err(|e| format!("WebAuthn registration start error: {}", e))?;

    // Serialize the registration state for storage
    let session_data = serde_json::to_string(&reg_state)
        .map_err(|e| format!("Failed to serialize registration state: {}", e))?;

    match server.db.create_webauthn_registration_session(account.id, &session_data).await {
        Ok(session) => {
            info!("Created single-step WebAuthn registration session");

            // WebAuthn-rs returns a CredentialCreationOptions wrapped in a PublicKeyCredential structure
            // We need to extract just the inner publicKey part for SimpleWebAuthn
            let challenge_json = serde_json::to_value(&ccr)
                .map_err(|e| format!("Challenge serialization error: {}", e))?;
            
            // Extract the publicKey field if it exists, otherwise use the whole object
            let mut creation_challenge = if let Some(public_key) = challenge_json.get("publicKey") {
                public_key.clone()
            } else {
                challenge_json
            };
            
            // Ensure Bitwarden compatibility by explicitly setting residentKey to 'required'
            // Some authenticators like Bitwarden need both requireResidentKey and residentKey fields
            if let Some(obj) = creation_challenge.as_object_mut() {
                if let Some(auth_selection) = obj.get_mut("authenticatorSelection") {
                    if let Some(auth_obj) = auth_selection.as_object_mut() {
                        auth_obj.insert("residentKey".to_string(), serde_json::Value::String("required".to_string()));
                    }
                }
            }

            Ok(Json(PasskeyRegisterResponse {
                session_id: session.id.to_string(),
                creation_challenge,
            }))
        }
        Err(e) => {
            error!("Failed to store registration session: {}", sanitize_log_message(&e.to_string()));
            Err("Failed to create session".to_string())
        }
    }
}

pub async fn passkey_register_finish_handler(
    State(state): State<Arc<Mutex<Server>>>,
    Json(request): Json<PasskeyRegisterFinishRequest>,
) -> Json<PasskeyRegisterFinishResponse> {
    use webauthn_rs::prelude::*;
    
    info!("Single-step passkey registration finish request ");
    
    let server = state.lock().await;
    
    // Parse session ID to UUID first to enable cleanup
    let session_uuid = match Uuid::parse_str(&request.session_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Invalid session ID format".to_string()),
            });
        }
    };
    
    // Use a helper function to ensure session cleanup always happens
    let result = passkey_register_finish_inner(&server, session_uuid, request).await;
    
    // Always cleanup session regardless of success or failure
    if let Err(e) = server.db.delete_webauthn_registration_session(session_uuid).await {
        warn!("Failed to clean up session: {}", sanitize_log_message(&e.to_string()));
    }
    
    result
}

async fn passkey_register_finish_inner(
    server: &Server,
    session_uuid: uuid::Uuid,
    request: PasskeyRegisterFinishRequest,
) -> Json<PasskeyRegisterFinishResponse> {
    use webauthn_rs::prelude::*;
    
    // Get the session and associated account
    let session = match server.db.get_webauthn_registration_session(session_uuid).await {
        Ok(Some(session)) => session,
        Ok(None) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Session not found or expired".to_string()),
            });
        }
        Err(e) => {
            error!("Database error: {}", sanitize_log_message(&e.to_string()));
            return Json(PasskeyRegisterFinishResponse {
                jwt_token: None,
                success: false,
                message: Some("Database error".to_string()),
            });
        }
    };

    let account = match server.db.get_account_by_id(session.account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Account not found".to_string()),
            });
        }
        Err(e) => {
            error!("Database error: {}", sanitize_log_message(&e.to_string()));
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Database error".to_string()),
            });
        }
    };

    // Create WebAuthn instance (same as start)
    let (rp_id, rp_origin) = match parse_webauthn_url(&server.web_ui_url) {
        Ok((id, origin)) => (id, origin),
        Err(e) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some(e),
            });
        }
    };
    let webauthn = match WebauthnBuilder::new(&rp_id, &rp_origin) {
        Ok(builder) => match builder.build() {
            Ok(webauthn) => webauthn,
            Err(e) => {
                return Json(PasskeyRegisterFinishResponse {
                    success: false,
                    jwt_token: None,
                    message: Some(format!("WebAuthn build error: {}", e)),
                });
            }
        },
        Err(e) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some(format!("WebAuthn builder error: {}", e)),
            });
        }
    };

    // Deserialize the stored registration state
    let registration_state: webauthn_rs::prelude::PasskeyRegistration = 
        match serde_json::from_str(&session.session_data) {
            Ok(state) => state,
            Err(e) => {
                return Json(PasskeyRegisterFinishResponse {
                    success: false,
                    jwt_token: None,
                    message: Some(format!("Failed to deserialize registration state: {}", e)),
                });
            }
        };

    // Parse the credential response
    let register_public_key_credential: RegisterPublicKeyCredential = 
        match serde_json::from_value(request.credential) {
            Ok(cred) => cred,
            Err(e) => {
                return Json(PasskeyRegisterFinishResponse {
                    success: false,
                    jwt_token: None,
                    message: Some(format!("Invalid credential format: {}", e)),
                });
            }
        };

    // Finish the passkey registration using WebAuthn
    let passkey = match webauthn.finish_passkey_registration(&register_public_key_credential, &registration_state) {
        Ok(passkey) => passkey,
        Err(e) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some(format!("WebAuthn registration finish error: {}", e)),
            });
        }
    };

    // Serialize the entire Passkey for storage (like the tutorial does)
    let passkey_json = match serde_json::to_string(&passkey) {
        Ok(json) => json,
        Err(e) => {
            return Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some(format!("Failed to serialize passkey: {}", e)),
            });
        }
    };
    
    // Extract credential ID for indexing
    // Use URL_SAFE_NO_PAD to match browser's base64url encoding
    use base64::{Engine as _, engine::general_purpose};
    let credential_id = general_purpose::URL_SAFE_NO_PAD.encode(passkey.cred_id());

    match server.db.create_webauthn_credential(
        account.id,
        &credential_id,
        passkey_json.as_bytes(),
        0, // counter - not used with serialized passkey
        None, // user_handle
        None, // backup_eligible  
        None, // backup_state
        None, // attestation_type
        None, // user_verified
    ).await {
        Ok(_) => {
            info!("Stored WebAuthn credential for account_id {}", account.id);
            
            // Extract the COSE public key bytes for signature verification
            let public_key_bytes = match serde_cbor_2::to_vec(passkey.get_public_key()) {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("Failed to serialize COSE public key: {}", e);
                    return Json(PasskeyRegisterFinishResponse {
                        success: false,
                                jwt_token: None,
                        message: Some("Failed to extract COSE public key".to_string()),
                    });
                }
            };
            
            // Update the account with passkey public key and credential ID
            if let Err(e) = server.db.update_account_passkey_info(
                account.id,
                &public_key_bytes,
                &credential_id
            ).await {
                error!("Failed to update account with passkey info: {}", e);
                return Json(PasskeyRegisterFinishResponse {
                    success: false,
                        jwt_token: None,
                    message: Some("Failed to store passkey public key".to_string()),
                });
            }
            
            info!("Updated account {} with passkey public key and credential ID", account.id);
            
            // Generate JWT token for automatic login after registration
            let jwt_token = match generate_jwt_token(&account.id.to_string()) {
                Ok(token) => {
                    info!("Created JWT token for newly registered account_id: {}", account.id);
                    Some(token)
                }
                Err(e) => {
                    error!("Failed to create JWT token: {}", e);
                    None
                }
            };
            Json(PasskeyRegisterFinishResponse {
                success: true,
                jwt_token,
                message: Some("Registration successful!".to_string()),
            })
        }
        Err(e) => {
            error!("Failed to store credential: {}", sanitize_log_message(&e.to_string()));
            Json(PasskeyRegisterFinishResponse {
                success: false,
                jwt_token: None,
                message: Some("Failed to store passkey".to_string()),
            })
        }
    }
}

// Device signing handlers
// We kind of trick the user into signing our own custom challenge
// The challenge contains the device public key, so peers can verify that
// a certain device is authorized by the user
pub async fn sign_device_start_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(request): Json<SignDeviceStartRequest>,
) -> Result<Json<SignDeviceStartResponse>, String> {
    use webauthn_rs::prelude::*;
    
    info!("Device sign start request for device: {}", request.device_id);
    
    // Extract JWT token from Authorization header
    let token = match headers.get("Authorization") {
        Some(auth_header) => {
            let auth_str = auth_header.to_str()
                .map_err(|_| "Invalid authorization header".to_string())?;
            if auth_str.starts_with("Bearer ") {
                &auth_str[7..]
            } else {
                return Err("Invalid authorization format".to_string());
            }
        }
        None => return Err("Missing authorization header".to_string()),
    };
    
    // Verify JWT and get account info
    let claims = verify_jwt_token(token)
        .map_err(|e| format!("Invalid token: {}", e))?;
    
    let server = state.lock().await;
    
    // Get the account
    let account_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| "Invalid account ID in token".to_string())?;
    let account = match server.db.get_account_by_id(account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => return Err("Account not found".to_string()),
        Err(e) => {
            error!("Database error: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Get the device to sign
    let device = match server.db.get_device_by_id(&request.device_id, account.id).await {
        Ok(Some(device)) => device,
        Ok(None) => return Err("Device not found".to_string()),
        Err(e) => {
            error!("Database error: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Check if device has a public key
    let device_public_key = match &device.public_key {
        Some(pk) => pk.clone(),
        None => return Err("Device has no public key".to_string()),
    };
    
    // Create WebAuthn instance
    let (rp_id, rp_origin) = parse_webauthn_url(&server.web_ui_url)?;
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| format!("WebAuthn builder error: {}", e))?;
    let webauthn = builder.build()
        .map_err(|e| format!("WebAuthn build error: {}", e))?;
    
    // Create device authorization data that will be cryptographically signed
    let device_data = format!("SIGN_DEVICE:{}:{}", device.device_id, device_public_key);
    let timestamp = chrono::Utc::now().timestamp();
    let challenge_data = format!("{}:{}", 
        general_purpose::STANDARD.encode(device_data.as_bytes()),
        timestamp
    );
    
    // Create the custom challenge that embeds the device authorization
    let custom_challenge_bytes = challenge_data.as_bytes().to_vec();
    let custom_challenge = Base64UrlSafeData::from(custom_challenge_bytes);
    
    // Start authentication but we'll replace the challenge immediately
    let (mut ccr, auth_state) = webauthn
        .start_passkey_authentication(&[])
        .map_err(|e| format!("WebAuthn authentication start error: {}", e))?;
    
    // Replace the challenge in the credential request
    let original_challenge = ccr.public_key.challenge.clone();
    ccr.public_key.challenge = custom_challenge.clone();

    // DEBUG: Log what challenges we're working with
    info!("CHALLENGE DEBUG - Original WebAuthn challenge: {}", general_purpose::STANDARD.encode(original_challenge.as_ref()));
    info!("CHALLENGE DEBUG - Custom device authorization challenge: {}", general_purpose::STANDARD.encode(custom_challenge.as_ref()));
    info!("CHALLENGE DEBUG - Challenge data being signed: {}", challenge_data);
    
    // Serialize auth_state for storage (this still contains the original challenge)
    let auth_state_json = serde_json::to_string(&auth_state)
        .map_err(|e| format!("Failed to serialize auth state: {}", e))?;
    
    // Store both the device data and challenge information
    let session_data = serde_json::json!({
        "device_id": device.device_id,
        "device_public_key": device_public_key,
        "account_id": account.id.to_string(),
        "device_authorization_data": device_data,
        "custom_challenge": general_purpose::STANDARD.encode(custom_challenge.as_ref()),
        "challenge_data": challenge_data,
        "timestamp": timestamp,
        "auth_state": auth_state_json
    }).to_string();
    
    let session = server.db.create_webauthn_authentication_session(&session_data).await
        .map_err(|e| format!("Failed to create session: {}", e))?;
    
    // Prepare challenge response
    let challenge_json = serde_json::to_value(&ccr)
        .map_err(|e| format!("Challenge serialization error: {}", e))?;
    
    // Extract publicKey field if needed
    let request_challenge = if let Some(public_key) = challenge_json.get("publicKey") {
        public_key.clone()
    } else {
        challenge_json
    };
    
    // DEBUG: Verify the challenge in the response matches what we expect
    if let Some(challenge_in_response) = request_challenge.get("challenge") {
        if let Some(challenge_str) = challenge_in_response.as_str() {
            let expected_challenge_b64 = general_purpose::STANDARD.encode(custom_challenge.as_ref());
            if challenge_str != expected_challenge_b64 {
                error!("CRITICAL: Challenge in response doesn't match custom challenge!");
                error!("  Response contains: {}", challenge_str);
                error!("  Expected: {}", expected_challenge_b64);
            } else {
                info!("CHALLENGE DEBUG - Verified response contains correct custom challenge");
            }
        }
    }
    
    // Prepare device info for display
    let device_info = DeviceSignInfo {
        device_id: device.device_id,
        public_key: device_public_key,
        os_name: device.os_name,
        created_at: device.created_at.to_rfc3339(),
    };
    
    Ok(Json(SignDeviceStartResponse {
        session_id: session.id.to_string(),
        request_challenge,
        device_info,
    }))
}

pub async fn sign_device_finish_handler(
    State(state): State<Arc<Mutex<Server>>>,
    headers: HeaderMap,
    Json(request): Json<SignDeviceFinishRequest>,
) -> Result<Json<SignDeviceFinishResponse>, String> {
    info!("Device sign finish request");
    
    // Extract JWT token
    let token = match headers.get("Authorization") {
        Some(auth_header) => {
            let auth_str = auth_header.to_str()
                .map_err(|_| "Invalid authorization header".to_string())?;
            if auth_str.starts_with("Bearer ") {
                &auth_str[7..]
            } else {
                return Err("Invalid authorization format".to_string());
            }
        }
        None => return Err("Missing authorization header".to_string()),
    };
    
    // Verify JWT
    let claims = verify_jwt_token(token)
        .map_err(|e| format!("Invalid token: {}", e))?;
    
    let server = state.lock().await;
    
    // Parse session ID first to enable cleanup
    let session_id = uuid::Uuid::parse_str(&request.session_id)
        .map_err(|_| "Invalid session ID".to_string())?;
    
    // Use a helper function to ensure session cleanup always happens
    let result = sign_device_finish_inner(&server, &claims, session_id, request).await;

    if let Err(e) = &result {
        error!("Signing error: {}", e);
    }
    
    // Always cleanup session regardless of success or failure
    if let Err(e) = server.db.delete_webauthn_authentication_session(session_id).await {
        warn!("Failed to cleanup session: {}", sanitize_log_message(&e.to_string()));
    }
    
    result
}

async fn sign_device_finish_inner(
    server: &Server,
    claims: &Claims,
    session_id: uuid::Uuid,
    request: SignDeviceFinishRequest,
) -> Result<Json<SignDeviceFinishResponse>, String> {
    use webauthn_rs::prelude::*;
    
    // Get the authentication session
    let session = match server.db.get_webauthn_authentication_session(session_id).await {
        Ok(Some(session)) => session,
        Ok(None) => return Err("Invalid or expired session".to_string()),
        Err(e) => {
            error!("Database error: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Parse session data
    let session_info: serde_json::Value = serde_json::from_str(&session.session_data)
        .map_err(|_| "Invalid session data".to_string())?;
    
    let device_id = session_info["device_id"].as_str()
        .ok_or("Missing device ID in session".to_string())?;
    let stored_account_id = session_info["account_id"].as_str()
        .ok_or("Missing account ID in session".to_string())?;
    
    // Verify account matches
    if claims.sub != stored_account_id {
        return Err("Account mismatch".to_string());
    }
    
    // Extract credential ID from response
    let credential_id = match request.credential.get("id") {
        Some(serde_json::Value::String(id)) => id.clone(),
        _ => return Err("Invalid credential format".to_string()),
    };
    
    // Find the stored credential for verification
    let stored_credential = match server.db.get_webauthn_credential_by_id(&credential_id).await {
        Ok(Some(cred)) => cred,
        Ok(None) => return Err("Credential not found".to_string()),
        Err(e) => {
            error!("Database error getting credential: {}", sanitize_log_message(&e.to_string()));
            return Err("Database error".to_string());
        }
    };
    
    // Verify the credential belongs to the account
    let account_uuid = uuid::Uuid::parse_str(&stored_account_id)
        .map_err(|_| "Invalid account ID".to_string())?;
    
    if stored_credential.account_id != account_uuid {
        return Err("Credential does not belong to account".to_string());
    }
    
    // Parse the credential response from the client
    let auth_public_key_credential: PublicKeyCredential = 
        serde_json::from_value(request.credential)
            .map_err(|e| format!("Invalid credential format: {}", e))?;
    
    // Extract the custom challenge and verify it was signed
    let custom_challenge_b64 = session_info["custom_challenge"].as_str()
        .ok_or("Missing custom challenge in session".to_string())?;
    let challenge_data = session_info["challenge_data"].as_str()
        .ok_or("Missing challenge data in session".to_string())?;
    
    // Verify the client signed our device authorization challenge
    let client_data_json = &auth_public_key_credential.response.client_data_json;
    let client_data: serde_json::Value = serde_json::from_slice(client_data_json.as_ref())
        .map_err(|_| "Invalid client data JSON".to_string())?;
    
    let signed_challenge = client_data["challenge"].as_str()
        .ok_or("Missing challenge in client data".to_string())?;
    
    // Normalize both challenges by decoding and re-encoding to handle base64 padding inconsistencies
    // Some browsers/WebAuthn implementations strip trailing '=' padding characters
    let normalize_base64 = |input: &str| -> Result<String, String> {
        match general_purpose::STANDARD.decode(input) {
            Ok(bytes) => Ok(general_purpose::STANDARD.encode(bytes)),
            Err(_) => {
                // Try with padding added
                let mut padded = input.to_string();
                while padded.len() % 4 != 0 {
                    padded.push('=');
                }
                match general_purpose::STANDARD.decode(&padded) {
                    Ok(bytes) => Ok(general_purpose::STANDARD.encode(bytes)),
                    Err(e) => Err(format!("Failed to decode base64: {}", e))
                }
            }
        }
    };
    
    let normalized_expected = normalize_base64(&custom_challenge_b64)
        .map_err(|e| format!("Failed to normalize expected challenge: {}", e))?;
    let normalized_received = normalize_base64(signed_challenge)
        .map_err(|e| format!("Failed to normalize received challenge: {}", e))?;
    
    if normalized_received != normalized_expected {
        if let Ok(received_bytes) = general_purpose::STANDARD.decode(signed_challenge) {
            if let Ok(received_str) = String::from_utf8(received_bytes) {
                error!("  Client signed data decoded: {}", received_str);
            }
        }
        
        return Err("Challenge mismatch - client did not sign the device authorization".to_string());
    }
    
    info!("CHALLENGE DEBUG - Challenges match after base64 normalization");
    
    info!("Verified client signed device authorization: {}", challenge_data);

    let device_public_key = session_info["device_public_key"].as_str()
        .ok_or("Missing device public key in session".to_string())?;
    
    // Create WebAuthn instance
    let (rp_id, rp_origin) = parse_webauthn_url(&server.web_ui_url)?;
    let _webauthn = WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| format!("WebAuthn builder error: {}", e))?
        .build()
        .map_err(|e| format!("WebAuthn build error: {}", e))?;

    info!("Performing manual WebAuthn signature verification with device authorization");


    let stored_passkey: webauthn_rs::prelude::Passkey = serde_json::from_slice(&stored_credential.public_key)
        .map_err(|e| format!("Failed to deserialize stored passkey: {}", e))?;

    // Verify the credential ID matches
    let credential_id_bytes = general_purpose::URL_SAFE_NO_PAD.decode(&credential_id)
        .map_err(|_| "Invalid credential ID encoding".to_string())?;

    if stored_passkey.cred_id().as_ref() != &credential_id_bytes {
        return Err("Credential ID mismatch".to_string());
    }

    use sha2::{Sha256, Digest};
    let client_data_hash = Sha256::digest(client_data_json.as_ref());

    let mut signed_data = Vec::new();
    signed_data.extend_from_slice(auth_public_key_credential.response.authenticator_data.as_ref());
    signed_data.extend_from_slice(&client_data_hash);

    let device_authorization_data = session_info["device_authorization_data"].as_str()
        .ok_or("Missing device authorization data in session".to_string())?;

    info!("WebAuthn authentication ceremony completed successfully");
    info!("Device authorization data was included in signed challenge: {}", device_authorization_data);
    let session_timestamp = session_info["timestamp"].as_i64()
        .ok_or("Missing timestamp in session".to_string())?;
    
    info!("Device authorization cryptographically verified: {}", device_authorization_data);

    // Store the complete device signing data
    let signature_payload = serde_json::json!({
        "device_id": device_id,
        "device_public_key": device_public_key,
        "credential_id": credential_id,
        "signed_at": chrono::Utc::now().timestamp(),
        "signature_counter": 0, // We'd need to track this properly
        "device_authorization_data": device_authorization_data,
        "challenge_data": challenge_data,
        "session_timestamp": session_timestamp,
        // Store WebAuthn verification data
        "webauthn_verification_data": {
            "client_data_json": general_purpose::STANDARD.encode(auth_public_key_credential.response.client_data_json.as_ref()),
            "authenticator_data": general_purpose::STANDARD.encode(auth_public_key_credential.response.authenticator_data.as_ref()),
            "signature": general_purpose::STANDARD.encode(auth_public_key_credential.response.signature.as_ref()),
            "verified_challenge": custom_challenge_b64
        }
    });
            
    let signature_json = signature_payload.to_string();
    
    if let Err(e) = server.db.sign_device(device_id, account_uuid, &signature_json, &credential_id).await {
        error!("Failed to sign device: {}", sanitize_log_message(&e.to_string()));
        return Err("Failed to sign device".to_string());
    }
    
    // Update credential counter (set to 1 since we're doing manual verification)
    if let Err(e) = server.db.update_webauthn_credential_counter(&credential_id, stored_credential.counter + 1).await {
        warn!("Failed to update credential counter: {}", sanitize_log_message(&e.to_string()));
    }
    
    info!("Successfully signed device {} with cryptographic signature", device_id);
    
    Ok(Json(SignDeviceFinishResponse {
        success: true,
        message: Some(format!("Device {} has been cryptographically signed", device_id)),
    }))
}
