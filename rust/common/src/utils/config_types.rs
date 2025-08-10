use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// Client configuration settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientSettings {
    /// Coordinator URL for holepunching and device management
    pub coordinator_url: String,
    /// Unique identifier for this client device
    pub device_id: String,
    /// JWT authentication token for coordinator authentication
    pub jwt_token: Option<String>,
    /// Timestamp when the client was registered
    pub registered_at: Option<i64>,
    /// Whether the client is registered with the coordinator
    pub is_registered: Option<bool>,
    /// Allow HTTP coordinator connections (dangerous, for development only)
    pub dangerously_use_http_coordinator: Option<bool>,
    /// Connection timeout in seconds
    pub connection_timeout: Option<u64>,
    /// Retry attempts for failed connections
    pub retry_attempts: Option<u32>,
    /// Authorized keys sync interval in seconds
    pub authorized_keys_sync_interval: Option<u64>,
    /// Known hosts sync interval in seconds
    pub known_hosts_sync_interval: Option<u64>,
    /// Full JSON-serialized Passkey for signature verification (from webauthn_credentials table)
    pub passkey_public_key: Option<String>,
    /// Passkey credential ID that signed this account
    pub passkey_credential_id: Option<String>,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            coordinator_url: "https://127.0.0.1:2223".to_string(),
            device_id: "client-device-unknown".to_string(),
            jwt_token: None,
            registered_at: None,
            is_registered: Some(false),
            dangerously_use_http_coordinator: Some(false),
            connection_timeout: Some(30),
            retry_attempts: Some(3),
            authorized_keys_sync_interval: Some(300), // 5 minutes
            known_hosts_sync_interval: Some(300), // 5 minutes
            passkey_public_key: None,
            passkey_credential_id: None,
        }
    }
}

/// Client user data (sessions, preferences, etc.)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ClientUserData {
    /// List of device IDs the user has connected to
    pub used_device_ids: Vec<String>,
    /// Saved sessions for reconnection
    pub saved_sessions: HashMap<String, SessionData>,
    /// User preferences
    pub preferences: UserPreferences,
    /// Recent connection history
    pub connection_history: Vec<ConnectionHistoryEntry>,
}

/// Server configuration settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerSettings {
    /// Coordinator URL for registration and device management
    pub coordinator_url: String,
    /// Unique identifier for this server device
    pub device_id: String,
    /// Path to the server's private key
    pub private_key_path: PathBuf,
    /// Allow HTTP coordinator connections (dangerous, for development only)
    pub dangerously_use_http_coordinator: Option<bool>,
    /// SSH inactivity timeout in seconds
    pub ssh_inactivity_timeout: Option<u64>,
    /// Authentication rejection time in seconds
    pub auth_rejection_time: Option<u64>,
    /// Maximum concurrent connections
    pub max_concurrent_connections: Option<u32>,
    /// Authorized keys sync interval in seconds
    pub authorized_keys_sync_interval: Option<u64>,
    /// Enable/disable SFTP subsystem
    pub enable_sftp: Option<bool>,
    /// Enable/disable port forwarding
    pub enable_port_forwarding: Option<bool>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            coordinator_url: "https://127.0.0.1:2223".to_string(),
            device_id: "server-device-unknown".to_string(),
            private_key_path: PathBuf::from("keys/ssh_host_ed25519_key"),
            dangerously_use_http_coordinator: Some(false),
            ssh_inactivity_timeout: Some(3600), // 1 hour
            auth_rejection_time: Some(3),
            max_concurrent_connections: Some(100),
            authorized_keys_sync_interval: Some(300), // 5 minutes
            enable_sftp: Some(true),
            enable_port_forwarding: Some(true),
        }
    }
}

/// Session data for saved sessions
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SessionData {
    /// Type of session (PTY, SFTP, etc.)
    pub session_type: SessionType,
    /// Unique session identifier
    pub session_id: Option<String>,
    /// Username for the session
    pub username: String,
    /// Target device ID
    pub device_id: String,
    /// Whether the session is currently active
    pub active: bool,
    /// Session-specific data
    pub session_specific_data: Option<SessionSpecificData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SessionType {
    Pty,
    Sftp,
    LocalPortForward,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SessionSpecificData {
    LocalPortForward {
        local_host: String,
        local_port: u32,
        remote_host: String,
        remote_port: u32,
    },
}

/// User preferences
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserPreferences {
    /// Default terminal type
    pub default_terminal: Option<String>,
    /// Default shell
    pub default_shell: Option<String>,
    /// Auto-reconnect to saved sessions
    pub auto_reconnect: Option<bool>,
    /// Log level preference
    pub log_level: Option<String>,
    /// Theme preference
    pub theme: Option<String>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_terminal: Some("xterm".to_string()),
            default_shell: Some("/bin/bash".to_string()),
            auto_reconnect: Some(false),
            log_level: Some("info".to_string()),
            theme: Some("dark".to_string()),
        }
    }
}

/// Connection history entry
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConnectionHistoryEntry {
    /// Target device ID
    pub device_id: String,
    /// Username used for connection
    pub username: String,
    /// Timestamp of connection
    pub connected_at: i64,
    /// Whether connection was successful
    pub successful: bool,
    /// Error message if connection failed
    pub error_message: Option<String>,
    /// Duration of connection in seconds
    pub duration: Option<u64>,
}

/// Server account data (registration and identity information)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerAccountData {
    /// JWT token for coordinator authentication (contains account info)
    pub jwt_token: String,
    /// Unique identifier for this server device
    pub device_id: String,
    /// Registration timestamp
    pub registered_at: i64,
    /// Whether the server is registered
    pub is_registered: bool,
    /// Full JSON-serialized Passkey for signature verification (from webauthn_credentials table)
    pub passkey_public_key: Option<String>,
    /// Passkey credential ID that signed this account
    pub passkey_credential_id: Option<String>,
}

impl Default for ServerAccountData {
    fn default() -> Self {
        Self {
            jwt_token: String::new(),
            device_id: "server-device-unknown".to_string(),
            registered_at: 0,
            is_registered: false,
            passkey_public_key: None,
            passkey_credential_id: None,
        }
    }
}

/// Configuration validation
impl ClientSettings {
    pub fn validate(&self) -> Result<(), String> {
        // Validate coordinator URL
        if let Err(e) = Url::parse(&self.coordinator_url) {
            return Err(format!("Invalid coordinator URL: {}", e));
        }

        // Validate device ID
        if self.device_id.is_empty() {
            return Err("Device ID cannot be empty".to_string());
        }

        // Validate timeouts
        if let Some(timeout) = self.connection_timeout {
            if timeout == 0 {
                return Err("Connection timeout must be greater than 0".to_string());
            }
        }

        Ok(())
    }
}

impl ServerSettings {
    pub fn validate(&self) -> Result<(), String> {
        // Validate coordinator URL
        if let Err(e) = Url::parse(&self.coordinator_url) {
            return Err(format!("Invalid coordinator URL: {}", e));
        }

        // Validate device ID
        if self.device_id.is_empty() {
            return Err("Device ID cannot be empty".to_string());
        }

        // Validate private key path
        if !self.private_key_path.exists() {
            return Err(format!("Private key file not found: {:?}", self.private_key_path));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_settings_default() {
        let settings = ClientSettings::default();
        assert_eq!(settings.coordinator_url, "https://127.0.0.1:2223");
        assert_eq!(settings.device_id, "client-device-unknown");
        assert_eq!(settings.dangerously_use_http_coordinator, Some(false));
        assert_eq!(settings.connection_timeout, Some(30));
    }

    #[test]
    fn test_client_settings_validation() {
        let mut settings = ClientSettings::default();
        assert!(settings.validate().is_ok());

        settings.coordinator_url = "invalid-url".to_string();
        assert!(settings.validate().is_err());

        settings.coordinator_url = "https://valid.com".to_string();
        settings.device_id = "".to_string();
        assert!(settings.validate().is_err());

        settings.device_id = "valid-device".to_string();
        settings.connection_timeout = Some(0);
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_server_settings_default() {
        let settings = ServerSettings::default();
        assert_eq!(settings.coordinator_url, "https://127.0.0.1:2223");
        assert_eq!(settings.device_id, "server-device-unknown");
        assert_eq!(settings.dangerously_use_http_coordinator, Some(false));
        assert_eq!(settings.ssh_inactivity_timeout, Some(3600));
    }

    #[test]
    fn test_serialization() {
        let settings = ClientSettings::default();
        let serialized = serde_json::to_string(&settings).unwrap();
        let deserialized: ClientSettings = serde_json::from_str(&serialized).unwrap();
        assert_eq!(settings, deserialized);
    }
}