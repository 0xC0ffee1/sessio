use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::{mpsc};
use uuid::Uuid;

use crate::db::DatabaseRepository;
    

pub struct Server {
    pub sessions: HashMap<String, Session>,
    pub clients: HashMap<String, Client>,
    pub db: Arc<DatabaseRepository>,
    pub coordinator_url: String,
    pub web_ui_url: String,
}

pub struct Session {
    //Server initiates a new session
    pub server_id: String,
    pub client_id: String,
    pub using_ipv6: bool,
}

pub struct Client {
    pub ws_sender: Option<mpsc::Sender<String>>, // WebSocket message sender (None until WS connects)
    pub id: String,
    pub session_ids: Vec<String>,
    ///Current ipv6
    pub ipv6: Option<SocketAddr>,
    ///Current ipv4
    pub ipv4: SocketAddr,
    /// Device UUID from database
    pub device_uuid: Uuid,
    /// Account UUID from database
    pub account_id: Uuid,
}


// Import response models that were in db/models.rs
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::db::models::{Device, Category};
use chrono::{DateTime, Utc};

// API Request/Response models
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRequest {
    pub device_id: String,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub install_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeviceRequest {
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeviceResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallRequest {
    pub install_key: String,
    pub public_key: Option<String>,
    pub metadata: DeviceMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceMetadata {
    pub os_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResponse {
    pub account_id: String,
    pub device_id: String,
    pub jwt_token: Option<String>,
    pub jwt_token_expires_at: Option<DateTime<Utc>>,
    pub passkey_public_key: Option<String>,
    pub passkey_credential_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]  
pub struct DevicesRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicesResponse {
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedKeysRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedKeysResponse {
    pub keys: Vec<AuthorizedKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedKey {
    pub device_id: String,
    pub public_key: String,
    pub os_name: String,
    pub signature: Option<String>,
    pub signed_at: Option<String>,
    pub signer_credential_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    pub device_id: String,
    pub os_name: Option<String>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevicesWithCategoriesResponse {
    pub devices: Vec<crate::db::models::DeviceWithCategories>,
    pub categories: Vec<Category>,
}

// Token expiration information for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenExpirationInfo {
    pub is_expired: bool,
    pub expires_at: DateTime<Utc>,
    pub time_until_expiry_seconds: Option<i64>,
    pub needs_refresh: bool, // true if expires within 5 minutes
}

// Enhanced API response wrapper that includes token status
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseWithTokenStatus<T> {
    #[serde(flatten)]
    pub data: T,
    pub token_status: Option<TokenExpirationInfo>,
}

// Challenge-response authentication requests/responses
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthChallengeRequest {
    pub device_public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthChallengeResponse {
    pub challenge_data: String,
    pub challenge_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthVerifyRequest {
    pub challenge_id: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthVerifyResponse {
    pub success: bool,
    pub session_token: Option<String>,
    pub account_id: Option<String>,
    pub message: Option<String>,
}

// WebAuthn API request/response models
#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnRegisterStartRequest {
    pub username: String,
    pub display_name: String,
}

// New single-step passkey registration that creates account + passkey in one go
#[derive(Debug, Serialize, Deserialize)]
pub struct PasskeyRegisterRequest {
    pub username: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasskeyRegisterResponse {
    pub session_id: String,
    pub creation_challenge: serde_json::Value, // PublicKeyCredentialCreationOptions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasskeyRegisterFinishRequest {
    pub session_id: String,
    pub credential: serde_json::Value, // PublicKeyCredential response
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasskeyRegisterFinishResponse {
    pub success: bool,
    pub jwt_token: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnRegisterStartResponse {
    pub session_id: String,
    pub creation_challenge: serde_json::Value, // PublicKeyCredentialCreationOptions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnRegisterFinishRequest {
    pub session_id: String,
    pub credential: serde_json::Value, // PublicKeyCredential response
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnRegisterFinishResponse {
    pub success: bool,
    pub jwt_token: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnAuthStartRequest {
    pub account_id: Option<String>, // Optional for usernameless flow
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnAuthStartResponse {
    pub session_id: String,
    pub request_challenge: serde_json::Value, // PublicKeyCredentialRequestOptions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnAuthFinishRequest {
    pub session_id: String,
    pub credential: serde_json::Value, // PublicKeyCredential response
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebauthnAuthFinishResponse {
    pub success: bool,
    pub jwt_token: Option<String>,
    pub message: Option<String>,
}

// Device signing models
#[derive(Debug, Serialize, Deserialize)]
pub struct SignDeviceStartRequest {
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignDeviceStartResponse {
    pub session_id: String,
    pub request_challenge: serde_json::Value,
    pub device_info: DeviceSignInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSignInfo {
    pub device_id: String,
    pub public_key: String,
    pub os_name: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignDeviceFinishRequest {
    pub session_id: String,
    pub credential: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignDeviceFinishResponse {
    pub success: bool,
    pub message: Option<String>,
}

// Token status check endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStatusRequest {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStatusResponse {
    pub frontend_token: Option<TokenExpirationInfo>,
    pub device_token: Option<DeviceTokenExpirationInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTokenExpirationInfo {
    pub is_expired: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub issued_at: Option<DateTime<Utc>>,
    pub time_until_expiry_seconds: Option<i64>,
    pub needs_refresh: bool, // true if expires within 30 days
}

