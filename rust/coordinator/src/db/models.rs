use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

// Database persistence models

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub passkey_public_key: Option<Vec<u8>>,
    pub passkey_credential_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InstallKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub install_key: String,
    pub device_id: Option<String>,
    pub category_name: Option<String>, // Keep for backward compatibility
    pub categories: JsonValue, // New field for multiple categories
    pub created_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: Uuid,
    pub account_id: Uuid,
    pub device_id: String,
    pub os_name: Option<String>,
    pub public_key: Option<String>,
    pub metadata: JsonValue,
    pub version: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub signature: Option<String>,
    pub signed_at: Option<DateTime<Utc>>,
    pub signer_credential_id: Option<String>,
    pub jwt_token_issued_at: Option<DateTime<Utc>>,
}

// Note: Session model removed - using memory-only sessions for hole-punching coordination

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthChallenge {
    pub id: Uuid,
    pub challenge_data: String,
    pub device_public_key: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}


// WebAuthn models for frontend passkey authentication
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebauthnCredential {
    pub id: Uuid,
    pub account_id: Uuid,
    pub credential_id: String,
    pub public_key: Vec<u8>,
    pub counter: i32,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub user_handle: Option<Vec<u8>>,
    pub backup_eligible: Option<bool>,
    pub backup_state: Option<bool>,
    pub attestation_type: Option<String>,
    pub user_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebauthnRegistrationSession {
    pub id: Uuid,
    pub account_id: Uuid,
    pub session_data: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebauthnAuthenticationSession {
    pub id: Uuid,
    pub session_data: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}


// Category models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Junction table model for device-category many-to-many relationship
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceCategory {
    pub id: Uuid,
    pub device_id: Uuid,
    pub category_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// Extended device response with categories info (supports multiple categories)
#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct DeviceWithCategories {
    #[serde(flatten)]
    pub device: Device,
    pub categories: Vec<Category>,
}

// Keep the old single category version for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct DeviceWithCategory {
    #[serde(flatten)]
    pub device: Device,
    pub category_name: Option<String>,
}

