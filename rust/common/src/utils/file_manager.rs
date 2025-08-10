use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Robust file manager with atomic writes, backups, and validation
pub struct FileManager {
    base_path: PathBuf,
}

impl FileManager {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Load JSON data from file with type safety and backup fallback
    pub async fn load_json<T>(&self, filename: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de> + Default,
    {
        let file_path = self.base_path.join(filename);
        let backup_path = self.base_path.join("backups").join(format!("{}.backup", filename));

        // Try to read the main file first
        match self.read_and_parse_json(&file_path).await {
            Ok(data) => Ok(data),
            Err(main_error) => {
                log::warn!("Failed to read main file {}: {}", filename, main_error);
                
                // Try backup file
                match self.read_and_parse_json(&backup_path).await {
                    Ok(data) => {
                        log::info!("Successfully recovered from backup: {}", filename);
                        // Restore the main file from backup
                        if let Err(e) = self.copy_file(&backup_path, &file_path).await {
                            log::error!("Failed to restore main file from backup: {}", e);
                        }
                        Ok(data)
                    }
                    Err(backup_error) => {
                        log::warn!("Failed to read backup file: {}", backup_error);
                        // Return default if both files fail
                        log::info!("Using default values for {}", filename);
                        Ok(T::default())
                    }
                }
            }
        }
    }

    /// Save JSON data to file with atomic write and backup
    pub async fn save_json<T>(&self, filename: &str, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let file_path = self.base_path.join(filename);
        let backup_path = self.base_path.join("backups").join(format!("{}.backup", filename));
        let temp_path = self.base_path.join(format!("{}.tmp", filename));

        // Ensure directories exist
        self.ensure_directories().await?;

        // Create backup if main file exists
        if file_path.exists() {
            self.copy_file(&file_path, &backup_path).await
                .context("Failed to create backup")?;
        }

        // Write to temporary file first (atomic write)
        let json_content = serde_json::to_string_pretty(data)
            .context("Failed to serialize data to JSON")?;

        fs::write(&temp_path, json_content)
            .await
            .context("Failed to write temporary file")?;

        // Atomic rename
        fs::rename(&temp_path, &file_path)
            .await
            .context("Failed to rename temporary file")?;

        // Set proper permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).await?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&file_path, perms).await?;
        }

        log::debug!("Successfully saved {}", filename);
        Ok(())
    }

    /// Update a specific field in a JSON file
    pub async fn update_json_field<T, U>(&self, filename: &str, field_name: &str, value: &U) -> Result<()>
    where
        T: for<'de> Deserialize<'de> + Serialize + Default,
        U: Serialize,
    {
        let mut data: T = self.load_json(filename).await?;
        
        // Convert to serde_json::Value for field manipulation
        let mut json_value = serde_json::to_value(&data)?;
        
        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(field_name.to_string(), serde_json::to_value(value)?);
        }
        
        let updated_data: T = serde_json::from_value(json_value)?;
        self.save_json(filename, &updated_data).await
    }

    /// Validate JSON file integrity
    pub async fn validate_json_file(&self, filename: &str) -> Result<bool> {
        let file_path = self.base_path.join(filename);
        
        if !file_path.exists() {
            return Ok(false);
        }

        match fs::read_to_string(&file_path).await {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            Err(_) => Ok(false),
        }
    }

    /// Get file modification time for change detection
    pub async fn get_file_modified_time(&self, filename: &str) -> Result<Option<std::time::SystemTime>> {
        let file_path = self.base_path.join(filename);
        
        if !file_path.exists() {
            return Ok(None);
        }

        let metadata = fs::metadata(&file_path).await?;
        Ok(Some(metadata.modified()?))
    }

    /// Private helper methods
    async fn read_and_parse_json<T>(&self, file_path: &Path) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let content = fs::read_to_string(file_path)
            .await
            .context("Failed to read file")?;

        if content.trim().is_empty() {
            return Err(anyhow::anyhow!("File is empty"));
        }

        serde_json::from_str(&content)
            .context("Failed to parse JSON")
    }

    async fn copy_file(&self, source: &Path, dest: &Path) -> Result<()> {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = fs::read(source).await?;
        fs::write(dest, content).await?;
        Ok(())
    }

    async fn ensure_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.base_path).await?;
        fs::create_dir_all(self.base_path.join("backups")).await?;
        fs::create_dir_all(self.base_path.join("keys")).await?;

        // Set proper permissions for directories
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for dir in [&self.base_path, &self.base_path.join("backups"), &self.base_path.join("keys")] {
                let mut perms = fs::metadata(dir).await?.permissions();
                perms.set_mode(0o700);
                fs::set_permissions(dir, perms).await?;
            }
        }

        Ok(())
    }

    /// Cleanup old backup files (keep last N backups)
    pub async fn cleanup_old_backups(&self, keep_count: usize) -> Result<()> {
        let backup_dir = self.base_path.join("backups");
        
        if !backup_dir.exists() {
            return Ok(());
        }

        let mut entries = fs::read_dir(&backup_dir).await?;
        let mut files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_file() {
                    if let Ok(modified) = metadata.modified() {
                        files.push((entry.path(), modified));
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        files.sort_by(|a, b| b.1.cmp(&a.1));

        // Remove old files
        for (path, _) in files.into_iter().skip(keep_count) {
            if let Err(e) = fs::remove_file(&path).await {
                log::warn!("Failed to remove old backup {:?}: {}", path, e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[tokio::test]
    async fn test_save_and_load_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_manager = FileManager::new(temp_dir.path());

        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Save data
        file_manager.save_json("test.json", &test_data).await.unwrap();

        // Load data
        let loaded_data: TestData = file_manager.load_json("test.json").await.unwrap();
        assert_eq!(test_data, loaded_data);
    }

    #[tokio::test]
    async fn test_backup_recovery() {
        let temp_dir = TempDir::new().unwrap();
        let file_manager = FileManager::new(temp_dir.path());

        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Save data (creates backup)
        file_manager.save_json("test.json", &test_data).await.unwrap();

        // Corrupt main file
        let main_file = temp_dir.path().join("test.json");
        fs::write(&main_file, "invalid json").await.unwrap();

        // Load should recover from backup
        let loaded_data: TestData = file_manager.load_json("test.json").await.unwrap();
        assert_eq!(test_data, loaded_data);
    }

    #[tokio::test]
    async fn test_validate_json_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_manager = FileManager::new(temp_dir.path());

        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Save valid data
        file_manager.save_json("test.json", &test_data).await.unwrap();
        assert!(file_manager.validate_json_file("test.json").await.unwrap());

        // Create invalid JSON
        let invalid_file = temp_dir.path().join("invalid.json");
        fs::write(&invalid_file, "invalid json").await.unwrap();
        assert!(!file_manager.validate_json_file("invalid.json").await.unwrap());
    }
}