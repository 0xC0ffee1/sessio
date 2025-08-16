use anyhow::{Context, Result};
use common::utils::config_types::{ServerSettings, ServerAccountData};
use common::utils::file_manager::FileManager;
use dirs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

/// Server configuration manager with improved file handling
pub struct ServerConfigManager {
    file_manager: FileManager,
    settings_cache: Option<ServerSettings>,
    account_data_cache: Option<ServerAccountData>,
    last_settings_check: Option<SystemTime>,
    last_account_data_check: Option<SystemTime>,
}

impl ServerConfigManager {
    /// Create a new server configuration manager
    pub fn new() -> Result<Self> {
        let sessio_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sessio");

        Ok(Self {
            file_manager: FileManager::new(sessio_dir),
            settings_cache: None,
            account_data_cache: None,
            last_settings_check: None,
            last_account_data_check: None,
        })
    }

    /// Load server settings with caching
    pub async fn load_settings(&mut self) -> Result<ServerSettings> {
        // Check if we need to reload from disk
        if let Some(last_check) = self.last_settings_check {
            if let Some(file_time) = self.file_manager.get_file_modified_time("server_settings.json").await? {
                if file_time <= last_check {
                    // Use cached version if available
                    if let Some(cached) = &self.settings_cache {
                        return Ok(cached.clone());
                    }
                }
            }
        }

        // Load from disk
        let settings: ServerSettings = self.file_manager.load_json("server_settings.json").await?;
        
        // Validate settings (skip private key validation for now as it might not exist yet)
        if let Err(e) = self.validate_settings_without_key(&settings) {
            log::error!("Invalid server settings: {}", e);
            return Err(anyhow::anyhow!("Invalid server settings: {}", e));
        }

        // Update cache
        self.settings_cache = Some(settings.clone());
        self.last_settings_check = Some(SystemTime::now());

        Ok(settings)
    }

    /// Save server settings
    pub async fn save_settings(&mut self, settings: &ServerSettings) -> Result<()> {
        // Validate before saving (skip private key validation)
        self.validate_settings_without_key(settings)
            .map_err(|e| anyhow::anyhow!("Settings validation failed: {}", e))?;

        self.file_manager.save_json("server_settings.json", settings).await?;
        
        // Update cache
        self.settings_cache = Some(settings.clone());
        self.last_settings_check = Some(SystemTime::now());

        Ok(())
    }

    /// Load server account data with caching
    pub async fn load_account_data(&mut self) -> Result<ServerAccountData> {
        // Check if we need to reload from disk
        if let Some(last_check) = self.last_account_data_check {
            if let Some(file_time) = self.file_manager.get_file_modified_time("server_account.json").await? {
                if file_time <= last_check {
                    // Use cached version if available
                    if let Some(cached) = &self.account_data_cache {
                        return Ok(cached.clone());
                    }
                }
            }
        }

        // Load from disk
        let account_data: ServerAccountData = self.file_manager.load_json("server_account.json").await?;
        
        // Update cache
        self.account_data_cache = Some(account_data.clone());
        self.last_account_data_check = Some(SystemTime::now());

        Ok(account_data)
    }

    /// Save server account data
    pub async fn save_account_data(&mut self, account_data: &ServerAccountData) -> Result<()> {
        self.file_manager.save_json("server_account.json", account_data).await?;
        
        // Update cache
        self.account_data_cache = Some(account_data.clone());
        self.last_account_data_check = Some(SystemTime::now());

        Ok(())
    }


    /// Update a specific setting
    pub async fn update_setting<T: serde::Serialize>(&mut self, field: &str, value: T) -> Result<()> {
        let mut settings = self.load_settings().await?;
        
        // Convert to JSON for field update
        let mut json_value = serde_json::to_value(&settings)?;
        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(field.to_string(), serde_json::to_value(&value)?);
        }
        
        let updated_settings: ServerSettings = serde_json::from_value(json_value)?;
        self.save_settings(&updated_settings).await
    }

    /// Check if HTTP coordinator is allowed
    pub async fn is_http_coordinator_allowed(&mut self) -> Result<bool> {
        let settings = self.load_settings().await?;
        Ok(settings.dangerously_use_http_coordinator.unwrap_or(false))
    }

    /// Get coordinator URL
    pub async fn get_coordinator_url(&mut self) -> Result<Url> {
        let settings = self.load_settings().await?;
        Url::parse(&settings.coordinator_url)
            .context("Invalid coordinator URL in settings")
    }

    /// Get device ID
    pub async fn get_device_id(&mut self) -> Result<String> {
        let settings = self.load_settings().await?;
        Ok(settings.device_id)
    }

    /// Get private key path
    pub async fn get_private_key_path(&mut self) -> Result<PathBuf> {
        let settings = self.load_settings().await?;
        Ok(settings.private_key_path)
    }

    /// Update account registration data
    pub async fn update_account_registration(
        &mut self, 
        jwt_token: &str, 
        device_id: &str, 
        coordinator_url: &str,
        passkey_public_key: Option<String>,
        passkey_credential_id: Option<String>
    ) -> Result<()> {
        // Update settings with coordinator URL and device ID
        let mut settings = self.load_settings().await?;
        settings.coordinator_url = coordinator_url.to_string();
        settings.device_id = device_id.to_string();
        self.save_settings(&settings).await?;
        
        // Update account data with JWT token and passkey info (contains account info)
        let account_data = ServerAccountData {
            jwt_token: jwt_token.to_string(),
            device_id: device_id.to_string(),
            registered_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            is_registered: true,
            passkey_public_key,
            passkey_credential_id,
        };
        
        self.save_account_data(&account_data).await?;
        
        Ok(())
    }

    /// Check if server is registered
    pub async fn is_registered(&mut self) -> Result<bool> {
        match self.load_account_data().await {
            Ok(account_data) => Ok(account_data.is_registered && !account_data.jwt_token.is_empty()),
            Err(_) => Ok(false), // Account data doesn't exist yet
        }
    }

    /// Get account information - returns (jwt_token, device_id)
    pub async fn get_account_info(&mut self) -> Result<(String, String)> {
        let account_data = self.load_account_data().await?;
        if !account_data.is_registered || account_data.jwt_token.is_empty() {
            return Err(anyhow::anyhow!("Server is not registered"));
        }
        Ok((account_data.jwt_token, account_data.device_id))
    }

    /// Get SSH configuration
    pub async fn get_ssh_config(&mut self) -> Result<SshConfig> {
        let settings = self.load_settings().await?;
        Ok(SshConfig {
            inactivity_timeout: settings.ssh_inactivity_timeout.unwrap_or(3600),
            auth_rejection_time: settings.auth_rejection_time.unwrap_or(3),
            max_concurrent_connections: settings.max_concurrent_connections.unwrap_or(100),
            enable_sftp: settings.enable_sftp.unwrap_or(true),
            enable_port_forwarding: settings.enable_port_forwarding.unwrap_or(true),
        })
    }

    /// Get authorized keys sync interval
    pub async fn get_authorized_keys_sync_interval(&mut self) -> Result<u64> {
        let settings = self.load_settings().await?;
        Ok(settings.authorized_keys_sync_interval.unwrap_or(300))
    }



    /// Validate configuration files
    pub async fn validate_config_files(&self) -> Result<()> {
        let settings_valid = self.file_manager.validate_json_file("server_settings.json").await?;
        let account_data_valid = self.file_manager.validate_json_file("server_account.json").await?;
        
        if !settings_valid {
            return Err(anyhow::anyhow!("server_settings.json is invalid or corrupted"));
        }
        
        if !account_data_valid {
            log::warn!("server_account.json is invalid or missing - server may not be registered");
        }
        
        Ok(())
    }

    /// Cleanup old backup files
    pub async fn cleanup_old_backups(&self) -> Result<()> {
        self.file_manager.cleanup_old_backups(5).await?;
        Ok(())
    }

    /// Force cache reload
    pub fn invalidate_cache(&mut self) {
        self.settings_cache = None;
        self.account_data_cache = None;
        self.last_settings_check = None;
        self.last_account_data_check = None;
    }

    /// Validate settings without checking private key existence
    fn validate_settings_without_key(&self, settings: &ServerSettings) -> Result<()> {
        // Validate coordinator URL
        if let Err(e) = Url::parse(&settings.coordinator_url) {
            return Err(anyhow::anyhow!("Invalid coordinator URL: {}", e));
        }

        // Validate device ID
        if settings.device_id.is_empty() {
            return Err(anyhow::anyhow!("Device ID cannot be empty"));
        }

        // Validate timeouts
        if let Some(timeout) = settings.ssh_inactivity_timeout {
            if timeout == 0 {
                return Err(anyhow::anyhow!("SSH inactivity timeout must be greater than 0"));
            }
        }

        if let Some(timeout) = settings.auth_rejection_time {
            if timeout == 0 {
                return Err(anyhow::anyhow!("Auth rejection time must be greater than 0"));
            }
        }

        Ok(())
    }
}

/// SSH configuration extracted from server settings
#[derive(Debug, Clone)]
pub struct SshConfig {
    pub inactivity_timeout: u64,
    pub auth_rejection_time: u64,
    pub max_concurrent_connections: u32,
    pub enable_sftp: bool,
    pub enable_port_forwarding: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[tokio::test]
    async fn test_server_config_manager() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("HOME", temp_dir.path());
        
        let mut manager = ServerConfigManager::new().unwrap();
        
        // Test loading default settings
        let settings = manager.load_settings().await.unwrap();
        assert_eq!(settings.device_id, "server-device-unknown");
        assert_eq!(settings.dangerously_use_http_coordinator, Some(false));
        
        // Test updating settings
        let mut updated_settings = settings.clone();
        updated_settings.device_id = "test-server".to_string();
        manager.save_settings(&updated_settings).await.unwrap();
        
        // Test loading updated settings
        let loaded_settings = manager.load_settings().await.unwrap();
        assert_eq!(loaded_settings.device_id, "test-server");
        
        // Test account data (should fail initially)
        assert!(manager.load_account_data().await.is_err());
        
        // Test creating account data
        manager.update_account_registration("jwt-token-123", "server-1", "https://coordinator.example.com", None, None).await.unwrap();
        let account_data = manager.load_account_data().await.unwrap();
        assert_eq!(account_data.jwt_token, "jwt-token-123");
        assert_eq!(account_data.device_id, "server-1");
        assert!(account_data.is_registered);
    }
}