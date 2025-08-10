use anyhow::{Context, Result};
use common::utils::config_types::{ClientSettings, ClientUserData, ConnectionHistoryEntry};
use common::utils::file_manager::FileManager;
use dirs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

/// Client configuration manager with improved file handling
pub struct ClientConfigManager {
    file_manager: FileManager,
    settings_cache: Option<ClientSettings>,
    user_data_cache: Option<ClientUserData>,
    last_settings_check: Option<SystemTime>,
    last_user_data_check: Option<SystemTime>,
}

impl ClientConfigManager {
    /// Create a new client configuration manager
    pub fn new() -> Result<Self> {
        let sessio_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sessio");

        Ok(Self {
            file_manager: FileManager::new(sessio_dir),
            settings_cache: None,
            user_data_cache: None,
            last_settings_check: None,
            last_user_data_check: None,
        })
    }

    /// Load client settings with caching
    pub async fn load_settings(&mut self) -> Result<ClientSettings> {
        // Check if we need to reload from disk
        if let Some(last_check) = self.last_settings_check {
            if let Some(file_time) = self.file_manager.get_file_modified_time("client_settings.json").await? {
                if file_time <= last_check {
                    // Use cached version if available
                    if let Some(cached) = &self.settings_cache {
                        return Ok(cached.clone());
                    }
                }
            }
        }

        // Load from disk
        let settings: ClientSettings = self.file_manager.load_json("client_settings.json").await?;
        
        // Validate settings
        if let Err(e) = settings.validate() {
            log::error!("Invalid client settings: {}", e);
            return Err(anyhow::anyhow!("Invalid client settings: {}", e));
        }

        // Update cache
        self.settings_cache = Some(settings.clone());
        self.last_settings_check = Some(SystemTime::now());

        Ok(settings)
    }

    /// Save client settings
    pub async fn save_settings(&mut self, settings: &ClientSettings) -> Result<()> {
        // Validate before saving
        settings.validate()
            .map_err(|e| anyhow::anyhow!("Settings validation failed: {}", e))?;

        self.file_manager.save_json("client_settings.json", settings).await?;
        
        // Update cache
        self.settings_cache = Some(settings.clone());
        self.last_settings_check = Some(SystemTime::now());

        Ok(())
    }

    /// Load user data with caching
    pub async fn load_user_data(&mut self) -> Result<ClientUserData> {
        // Check if we need to reload from disk
        if let Some(last_check) = self.last_user_data_check {
            if let Some(file_time) = self.file_manager.get_file_modified_time("client_user_data.json").await? {
                if file_time <= last_check {
                    // Use cached version if available
                    if let Some(cached) = &self.user_data_cache {
                        return Ok(cached.clone());
                    }
                }
            }
        }

        // Load from disk
        let user_data: ClientUserData = self.file_manager.load_json("client_user_data.json").await?;
        
        // Update cache
        self.user_data_cache = Some(user_data.clone());
        self.last_user_data_check = Some(SystemTime::now());

        Ok(user_data)
    }

    /// Save user data
    pub async fn save_user_data(&mut self, user_data: &ClientUserData) -> Result<()> {
        self.file_manager.save_json("client_user_data.json", user_data).await?;
        
        // Update cache
        self.user_data_cache = Some(user_data.clone());
        self.last_user_data_check = Some(SystemTime::now());

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
        
        let updated_settings: ClientSettings = serde_json::from_value(json_value)?;
        self.save_settings(&updated_settings).await
    }

    /// Add a device to the used devices list
    pub async fn add_used_device(&mut self, device_id: &str) -> Result<()> {
        let mut user_data = self.load_user_data().await?;
        
        if !user_data.used_device_ids.contains(&device_id.to_string()) {
            user_data.used_device_ids.push(device_id.to_string());
            self.save_user_data(&user_data).await?;
        }
        
        Ok(())
    }

    /// Add a connection history entry
    pub async fn add_connection_history(&mut self, device_id: &str, username: &str, successful: bool, error_message: Option<String>) -> Result<()> {
        let mut user_data = self.load_user_data().await?;
        
        let entry = ConnectionHistoryEntry {
            device_id: device_id.to_string(),
            username: username.to_string(),
            connected_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            successful,
            error_message,
            duration: None, // Will be filled in when connection ends
        };
        
        user_data.connection_history.push(entry);
        
        // Keep only last 100 entries
        if user_data.connection_history.len() > 100 {
            user_data.connection_history.drain(0..user_data.connection_history.len() - 100);
        }
        
        self.save_user_data(&user_data).await?;
        Ok(())
    }

    /// Save a session for reconnection
    pub async fn save_session(&mut self, session_id: &str, session_data: &common::utils::config_types::SessionData) -> Result<()> {
        let mut user_data = self.load_user_data().await?;
        user_data.saved_sessions.insert(session_id.to_string(), session_data.clone());
        self.save_user_data(&user_data).await?;
        Ok(())
    }

    /// Remove a saved session
    pub async fn remove_saved_session(&mut self, session_id: &str) -> Result<()> {
        let mut user_data = self.load_user_data().await?;
        user_data.saved_sessions.remove(session_id);
        self.save_user_data(&user_data).await?;
        Ok(())
    }

    /// Get saved sessions
    pub async fn get_saved_sessions(&mut self) -> Result<std::collections::HashMap<String, common::utils::config_types::SessionData>> {
        let user_data = self.load_user_data().await?;
        Ok(user_data.saved_sessions)
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

    /// Update account registration data
    pub async fn update_account_data(&mut self, jwt_token: &str, device_id: &str) -> Result<()> {
        let mut settings = self.load_settings().await?;
        settings.jwt_token = Some(jwt_token.to_string());
        settings.device_id = device_id.to_string();
        settings.is_registered = Some(true);
        settings.registered_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        );
        
        self.save_settings(&settings).await?;
        Ok(())
    }

    /// Check if client is registered
    pub async fn is_registered(&mut self) -> Result<bool> {
        let settings = self.load_settings().await?;
        Ok(settings.is_registered.unwrap_or(false) && 
           settings.jwt_token.is_some())
    }

    /// Get device ID
    pub async fn get_device_id(&mut self) -> Result<String> {
        let settings = self.load_settings().await?;
        Ok(settings.device_id)
    }

    /// Get account information
    pub async fn get_account_info(&mut self) -> Result<(Option<String>, Option<String>)> {
        let settings = self.load_settings().await?;
        Ok((None, settings.jwt_token))
    }

    /// Validate configuration files
    pub async fn validate_config_files(&self) -> Result<()> {
        let settings_valid = self.file_manager.validate_json_file("client_settings.json").await?;
        let user_data_valid = self.file_manager.validate_json_file("client_user_data.json").await?;
        
        if !settings_valid {
            return Err(anyhow::anyhow!("client_settings.json is invalid or corrupted"));
        }
        
        if !user_data_valid {
            return Err(anyhow::anyhow!("client_user_data.json is invalid or corrupted"));
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
        self.user_data_cache = None;
        self.last_settings_check = None;
        self.last_user_data_check = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[tokio::test]
    async fn test_client_config_manager() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("HOME", temp_dir.path());

        let mut manager = ClientConfigManager::new().unwrap();

        // Test loading default settings
        let settings = manager.load_settings().await.unwrap();
        assert_eq!(settings.device_id, "client-device-unknown");
        assert_eq!(settings.dangerously_use_http_coordinator, Some(false));

        // Test updating settings
        let mut updated_settings = settings.clone();
        updated_settings.device_id = "test-device".to_string();
        manager.save_settings(&updated_settings).await.unwrap();

        // Test loading updated settings
        let loaded_settings = manager.load_settings().await.unwrap();
        assert_eq!(loaded_settings.device_id, "test-device");

        // Test user data
        let user_data = manager.load_user_data().await.unwrap();
        assert!(user_data.used_device_ids.is_empty());

        // Test adding device
        manager.add_used_device("device-1").await.unwrap();
        let updated_user_data = manager.load_user_data().await.unwrap();
        assert_eq!(updated_user_data.used_device_ids, vec!["device-1"]);
    }
}