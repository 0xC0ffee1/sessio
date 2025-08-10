use anyhow::{Context, Result};
use rand::Rng;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

use super::models::*;
use super::models::DeviceWithCategories;

pub struct DatabaseRepository {
    pool: PgPool,
}

impl DatabaseRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .context("Failed to connect to database")?;

        Ok(Self { pool })
    }

    pub async fn initialize_schema(&self) -> Result<()> {
        // Embed the schema.sql file into the binary at compile time
        const SCHEMA_SQL: &str = include_str!("schema.sql");
        
        log::info!("Initializing database schema from embedded schema.sql");
        
        // Execute the complete schema
        sqlx::raw_sql(SCHEMA_SQL)
            .execute(&self.pool)
            .await
            .context("Failed to execute embedded schema.sql")?;
        
        log::info!("Database schema initialization completed");
        
        Ok(())
    }


    // Generate a secure install key
    fn generate_install_key() -> String {
        use rand::distributions::Alphanumeric;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    // Create account only (without install key) 
    pub async fn create_account_only(&self) -> Result<Account> {
        // Create account with auto-generated UUID
        let account: Account = sqlx::query_as(
            "INSERT INTO accounts DEFAULT VALUES RETURNING *",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }


    // Get account by ID
    pub async fn get_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>> {
        let account: Option<Account> = sqlx::query_as(
            "SELECT * FROM accounts WHERE id = $1"
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }


    // Create install key for specific account and device (15 minutes expiry)
    pub async fn create_device_install_key(&self, account_id: Uuid, device_id: &str, categories: Option<&[String]>) -> Result<InstallKey> {
        let install_key_str = Self::generate_install_key();
        
        // Convert categories to JSON value
        let categories_json = if let Some(cats) = categories {
            serde_json::to_value(cats).unwrap_or(serde_json::Value::Array(vec![]))
        } else {
            serde_json::Value::Array(vec![])
        };
        
        let install_key: InstallKey = sqlx::query_as(
            "INSERT INTO install_keys (account_id, install_key, device_id, categories, expires_at) VALUES ($1, $2, $3, $4, NOW() + INTERVAL '15 minutes') RETURNING *",
        )
        .bind(account_id)
        .bind(&install_key_str)
        .bind(device_id)
        .bind(&categories_json)
        .fetch_one(&self.pool)
        .await?;

        Ok(install_key)
    }

    // Install key operations
    pub async fn validate_install_key(&self, key: &str) -> Result<Option<InstallKey>> {
        let install_key: Option<InstallKey> = sqlx::query_as(
            "SELECT * FROM install_keys WHERE install_key = $1 AND used_at IS NULL AND expires_at > NOW()",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(install_key)
    }

    pub async fn mark_install_key_used(&self, key_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE install_keys SET used_at = NOW() WHERE id = $1")
            .bind(key_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    
    pub async fn mark_install_key_used_with_device(&self, key_id: Uuid, device_id: &str) -> Result<()> {
        sqlx::query("UPDATE install_keys SET used_at = NOW(), device_id = $2 WHERE id = $1")
            .bind(key_id)
            .bind(device_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Device operations
    pub async fn create_or_update_device(
        &self,
        account_id: Uuid,
        device_id: &str,
        os_name: Option<&str>,
        public_key: Option<&str>,
        metadata: serde_json::Value,
        version: Option<&str>,
    ) -> Result<Device> {
        let device: Device = sqlx::query_as(
            r#"
            INSERT INTO devices (account_id, device_id, os_name, public_key, metadata, version)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (device_id, account_id) DO UPDATE SET
                os_name = EXCLUDED.os_name,
                public_key = EXCLUDED.public_key,
                metadata = EXCLUDED.metadata,
                version = EXCLUDED.version,
                last_seen_at = NOW()
            RETURNING *
            "#,
        )
        .bind(account_id)
        .bind(device_id)
        .bind(os_name)
        .bind(public_key)
        .bind(metadata)
        .bind(version)
        .fetch_one(&self.pool)
        .await?;

        Ok(device)
    }

    pub async fn create_or_update_device_with_category(
        &self,
        account_id: Uuid,
        device_id: &str,
        os_name: Option<&str>,
        public_key: Option<&str>,
        metadata: serde_json::Value,
        category_id: Option<Uuid>,
    ) -> Result<Device> {
        // First create/update the device without category_id
        let device = self.create_or_update_device(
            account_id,
            device_id,
            os_name,
            public_key,
            metadata,
            None, // version not provided in this method
        ).await?;
        
        // Then add the category relationship if provided
        if let Some(cat_id) = category_id {
            // Clear existing categories and add the new one
            self.set_device_categories(device.id, &[cat_id]).await?;
        }

        Ok(device)
    }

    // New method to create/update device with multiple categories
    pub async fn create_or_update_device_with_categories(
        &self,
        account_id: Uuid,
        device_id: &str,
        os_name: Option<&str>,
        public_key: Option<&str>,
        metadata: serde_json::Value,
        category_ids: &[Uuid],
    ) -> Result<Device> {
        // First create/update the device
        let device = self.create_or_update_device(
            account_id,
            device_id,
            os_name,
            public_key,
            metadata,
            None, // version not provided in this method
        ).await?;
        
        // Then set the device categories in the junction table
        if !category_ids.is_empty() {
            self.set_device_categories(device.id, category_ids).await?;
        }
        
        Ok(device)
    }

    pub async fn get_device_by_id(&self, device_id: &str, account_id: Uuid) -> Result<Option<Device>> {
        let device: Option<Device> = sqlx::query_as(
            "SELECT * FROM devices WHERE device_id = $1 AND account_id = $2",
        )
        .bind(device_id)
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device)
    }

    pub async fn update_device_last_seen(&self, device_id: &str, account_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE devices SET last_seen_at = NOW() WHERE device_id = $1 AND account_id = $2")
            .bind(device_id)
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_device_heartbeat(&self, device_id: &str, account_id: Uuid, version: Option<String>) -> Result<()> {
        if let Some(version) = version {
            sqlx::query("UPDATE devices SET last_seen_at = NOW(), version = $3 WHERE device_id = $1 AND account_id = $2")
                .bind(device_id)
                .bind(account_id)
                .bind(version)
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query("UPDATE devices SET last_seen_at = NOW() WHERE device_id = $1 AND account_id = $2")
                .bind(device_id)
                .bind(account_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    pub async fn delete_device(&self, device_id: &str, account_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM devices WHERE device_id = $1 AND account_id = $2"
        )
        .bind(device_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Authentication challenge operations
    pub async fn create_auth_challenge(&self, device_public_key: &str, challenge_data: &str) -> Result<AuthChallenge> {
        let challenge: AuthChallenge = sqlx::query_as(
            "INSERT INTO auth_challenges (challenge_data, device_public_key) VALUES ($1, $2) RETURNING *"
        )
        .bind(challenge_data)
        .bind(device_public_key)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(challenge)
    }

    pub async fn get_auth_challenge(&self, challenge_id: Uuid) -> Result<Option<AuthChallenge>> {
        let challenge: Option<AuthChallenge> = sqlx::query_as(
            "SELECT * FROM auth_challenges WHERE id = $1 AND expires_at > NOW() AND used = FALSE"
        )
        .bind(challenge_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(challenge)
    }

    pub async fn mark_challenge_used(&self, challenge_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE auth_challenges SET used = TRUE WHERE id = $1")
            .bind(challenge_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn cleanup_expired_challenges(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM auth_challenges WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }

    pub async fn get_device_by_public_key(&self, public_key: &str) -> Result<Option<Device>> {
        let device: Option<Device> = sqlx::query_as(
            "SELECT * FROM devices WHERE public_key = $1"
        )
        .bind(public_key)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(device)
    }

    // Note: Sessions are now memory-only for ephemeral hole-punching coordination

    pub async fn get_account_by_device_id(&self, device_id: &str) -> Result<Option<Account>> {
        let account: Option<Account> = sqlx::query_as(
            r#"
            SELECT a.* FROM accounts a
            JOIN devices d ON d.account_id = a.id
            WHERE d.device_id = $1
            "#,
        )
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    pub async fn verify_device_account(&self, device_id: &str, account_id: Uuid) -> Result<bool> {
        let exists: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM devices
                WHERE device_id = $1 AND account_id = $2
            )
            "#,
        )
        .bind(device_id)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }


    pub async fn get_devices_by_account_id(&self, account_id: Uuid) -> Result<Vec<Device>> {
        let devices: Vec<Device> = sqlx::query_as(
            r#"
            SELECT d.* FROM devices d
            WHERE d.account_id = $1
            ORDER BY d.created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(devices)
    }


    pub async fn get_authorized_keys_by_account_id(&self, account_id: Uuid) -> Result<Vec<(String, String, String, Option<String>, Option<String>, Option<String>)>> {
        let keys: Vec<(String, String, String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
            r#"
            SELECT 
                d.device_id, 
                d.public_key, 
                COALESCE(d.os_name, 'unknown') as os_name,
                d.signature,
                d.signed_at::text,
                d.signer_credential_id
            FROM devices d
            WHERE d.account_id = $1 
              AND d.public_key IS NOT NULL 
              AND d.signature IS NOT NULL
            ORDER BY d.signed_at DESC, d.device_id
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(keys)
    }

    // WebAuthn operations for frontend passkey authentication
    
    pub async fn create_webauthn_registration_session(
        &self, 
        account_id: Uuid, 
        session_data: &str
    ) -> Result<WebauthnRegistrationSession> {
        let session: WebauthnRegistrationSession = sqlx::query_as(
            "INSERT INTO webauthn_registration_sessions (account_id, session_data) VALUES ($1, $2) RETURNING *"
        )
        .bind(account_id)
        .bind(session_data)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    pub async fn get_webauthn_registration_session(
        &self, 
        session_id: Uuid
    ) -> Result<Option<WebauthnRegistrationSession>> {
        let session: Option<WebauthnRegistrationSession> = sqlx::query_as(
            "SELECT * FROM webauthn_registration_sessions WHERE id = $1 AND expires_at > NOW()"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    pub async fn delete_webauthn_registration_session(&self, session_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM webauthn_registration_sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn create_webauthn_credential(
        &self,
        account_id: Uuid,
        credential_id: &str,
        public_key: &[u8],
        counter: i32,
        user_handle: Option<&[u8]>,
        backup_eligible: Option<bool>,
        backup_state: Option<bool>,
        attestation_type: Option<&str>,
        user_verified: Option<bool>,
    ) -> Result<WebauthnCredential> {
        let credential: WebauthnCredential = sqlx::query_as(
            r#"
            INSERT INTO webauthn_credentials 
            (account_id, credential_id, public_key, counter, user_handle, backup_eligible, backup_state, attestation_type, user_verified) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
            RETURNING *
            "#
        )
        .bind(account_id)
        .bind(credential_id)
        .bind(public_key)
        .bind(counter)
        .bind(user_handle)
        .bind(backup_eligible)
        .bind(backup_state)
        .bind(attestation_type)
        .bind(user_verified)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(credential)
    }
    
    pub async fn create_webauthn_authentication_session(
        &self, 
        session_data: &str
    ) -> Result<WebauthnAuthenticationSession> {
        let session: WebauthnAuthenticationSession = sqlx::query_as(
            "INSERT INTO webauthn_authentication_sessions (session_data) VALUES ($1) RETURNING *"
        )
        .bind(session_data)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    pub async fn get_webauthn_authentication_session(
        &self, 
        session_id: Uuid
    ) -> Result<Option<WebauthnAuthenticationSession>> {
        let session: Option<WebauthnAuthenticationSession> = sqlx::query_as(
            "SELECT * FROM webauthn_authentication_sessions WHERE id = $1 AND expires_at > NOW()"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    pub async fn delete_webauthn_authentication_session(&self, session_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM webauthn_authentication_sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_webauthn_credentials_by_account(
        &self, 
        account_id: Uuid
    ) -> Result<Vec<WebauthnCredential>> {
        let credentials: Vec<WebauthnCredential> = sqlx::query_as(
            "SELECT * FROM webauthn_credentials WHERE account_id = $1"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(credentials)
    }
    
    pub async fn get_webauthn_credential_by_id(
        &self, 
        credential_id: &str
    ) -> Result<Option<WebauthnCredential>> {
        let credential: Option<WebauthnCredential> = sqlx::query_as(
            "SELECT * FROM webauthn_credentials WHERE credential_id = $1"
        )
        .bind(credential_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(credential)
    }
    
    pub async fn get_all_webauthn_credentials(&self) -> Result<Vec<WebauthnCredential>> {
        let credentials: Vec<WebauthnCredential> = sqlx::query_as(
            "SELECT * FROM webauthn_credentials ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(credentials)
    }
    
    pub async fn update_webauthn_credential_counter(
        &self, 
        credential_id: &str, 
        new_counter: i32
    ) -> Result<()> {
        sqlx::query(
            "UPDATE webauthn_credentials SET counter = $1, last_used_at = NOW() WHERE credential_id = $2"
        )
        .bind(new_counter)
        .bind(credential_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn cleanup_expired_webauthn_sessions(&self) -> Result<(u64, u64)> {
        let reg_deleted = sqlx::query("DELETE FROM webauthn_registration_sessions WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await?
            .rows_affected();
            
        let auth_deleted = sqlx::query("DELETE FROM webauthn_authentication_sessions WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await?
            .rows_affected();
        
        Ok((reg_deleted, auth_deleted))
    }

    // Update account with passkey public key
    pub async fn update_account_passkey_info(
        &self,
        account_id: Uuid,
        public_key: &[u8],
        credential_id: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE accounts SET passkey_public_key = $1, passkey_credential_id = $2 WHERE id = $3"
        )
        .bind(public_key)
        .bind(credential_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    // Sign a device with passkey credential
    pub async fn sign_device(
        &self,
        device_id: &str,
        account_id: Uuid,
        signature: &str,
        credential_id: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE devices SET signature = $1, signed_at = NOW(), signer_credential_id = $2 
             WHERE device_id = $3 AND account_id = $4"
        )
        .bind(signature)
        .bind(credential_id)
        .bind(device_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    // Category management methods
    pub async fn create_category(&self, account_id: Uuid, name: &str) -> Result<Category> {
        let category: Category = sqlx::query_as(
            "INSERT INTO categories (account_id, name) VALUES ($1, $2) 
             ON CONFLICT (account_id, name) DO UPDATE SET updated_at = NOW() 
             RETURNING *"
        )
        .bind(account_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(category)
    }

    pub async fn get_categories_by_account_id(&self, account_id: Uuid) -> Result<Vec<Category>> {
        let categories: Vec<Category> = sqlx::query_as(
            "SELECT * FROM categories WHERE account_id = $1 ORDER BY name ASC"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(categories)
    }

    pub async fn get_category_by_name(&self, account_id: Uuid, name: &str) -> Result<Option<Category>> {
        let category: Option<Category> = sqlx::query_as(
            "SELECT * FROM categories WHERE account_id = $1 AND name = $2"
        )
        .bind(account_id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(category)
    }

    // Device update methods
    pub async fn update_device(
        &self,
        device_id: &str,
        account_id: Uuid,
        os_name: Option<&str>,
        category_id: Option<Uuid>,
        update_category: bool,
    ) -> Result<Device> {
        // First update the device (without category_id as it's not in the devices table)
        let device: Device = sqlx::query_as(
            "UPDATE devices SET 
                os_name = COALESCE($3, os_name),
                updated_at = NOW()
             WHERE device_id = $1 AND account_id = $2 
             RETURNING *"
        )
        .bind(device_id)
        .bind(account_id)
        .bind(os_name)
        .fetch_one(&self.pool)
        .await?;
        
        // Then update the category if requested
        if update_category {
            if let Some(cat_id) = category_id {
                // Clear existing categories and set the new one
                self.set_device_categories(device.id, &[cat_id]).await?;
            } else {
                // Clear all categories if None provided
                self.set_device_categories(device.id, &[]).await?;
            }
        }
        
        Ok(device)
    }

    // Get devices with category information
    // Add/remove categories for a device
    pub async fn add_device_to_category(&self, device_id: Uuid, category_id: Uuid) -> Result<()> {
        sqlx::query(
            "INSERT INTO device_categories (device_id, category_id) VALUES ($1, $2) ON CONFLICT (device_id, category_id) DO NOTHING"
        )
        .bind(device_id)
        .bind(category_id)
        .execute(&self.pool)
        .await
        .context("Failed to add device to category")?;
        
        Ok(())
    }

    pub async fn remove_device_from_category(&self, device_id: Uuid, category_id: Uuid) -> Result<()> {
        sqlx::query(
            "DELETE FROM device_categories WHERE device_id = $1 AND category_id = $2"
        )
        .bind(device_id)
        .bind(category_id)
        .execute(&self.pool)
        .await
        .context("Failed to remove device from category")?;
        
        Ok(())
    }

    pub async fn set_device_categories(&self, device_id: Uuid, category_ids: &[Uuid]) -> Result<()> {
        // First remove all existing category assignments for this device
        sqlx::query("DELETE FROM device_categories WHERE device_id = $1")
            .bind(device_id)
            .execute(&self.pool)
            .await
            .context("Failed to clear device categories")?;
        
        // Then add the new category assignments
        for category_id in category_ids {
            self.add_device_to_category(device_id, *category_id).await?;
        }
        
        Ok(())
    }

    pub async fn get_device_categories(&self, device_id: Uuid) -> Result<Vec<Category>> {
        let categories: Vec<Category> = sqlx::query_as(
            r#"
            SELECT c.id, c.account_id, c.name, c.created_at, c.updated_at
            FROM categories c
            INNER JOIN device_categories dc ON c.id = dc.category_id
            WHERE dc.device_id = $1
            ORDER BY c.name
            "#
        )
        .bind(device_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to get device categories")?;
        
        Ok(categories)
    }

    // Updated method to return devices with multiple categories
    pub async fn get_devices_with_categories(&self, account_id: Uuid) -> Result<Vec<DeviceWithCategories>> {
        // First get all devices for this account
        let devices: Vec<Device> = self.get_devices_by_account_id(account_id).await?;
        
        // Then for each device, get its categories
        let mut devices_with_categories = Vec::new();
        
        for device in devices {
            let categories = self.get_device_categories(device.id).await?;
            devices_with_categories.push(DeviceWithCategories {
                device,
                categories,
            });
        }
        
        Ok(devices_with_categories)
    }

    // Keep the old method for backward compatibility, but mark as deprecated  
    pub async fn get_devices_with_category_legacy(&self, account_id: Uuid) -> Result<Vec<DeviceWithCategory>> {
        let devices: Vec<Device> = self.get_devices_by_account_id(account_id).await?;
        
        let mut devices_with_categories = Vec::new();
        
        for device in devices {
            // Get the first category for this device (for backward compatibility)
            let categories = self.get_device_categories(device.id).await?;
            let category_name = categories.first().map(|c| c.name.clone());
            
            devices_with_categories.push(DeviceWithCategory {
                device,
                category_name,
            });
        }
        
        Ok(devices_with_categories)
    }

    // Get passkey credential by credential ID and deserialize it
    // Update jwt_token_issued_at when a new device JWT token is generated
    pub async fn update_device_jwt_token_issued_at(&self, device_id: &str, account_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE devices SET jwt_token_issued_at = NOW() WHERE device_id = $1 AND account_id = $2"
        )
        .bind(device_id)
        .bind(account_id)
        .execute(&self.pool)
        .await
        .context("Failed to update device jwt_token_issued_at")?;
        
        Ok(())
    }

    pub async fn get_passkey_by_credential_id(&self, credential_id: &str) -> Result<Option<webauthn_rs::prelude::DiscoverableKey>> {
        let credential = match self.get_webauthn_credential_by_id(credential_id).await? {
            Some(cred) => cred,
            None => return Ok(None),
        };
        
        // Deserialize the stored passkey JSON back to a Passkey object
        let passkey: webauthn_rs::prelude::DiscoverableKey = serde_json::from_slice(&credential.public_key)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize stored passkey: {}", e))?;
        
        Ok(Some(passkey))
    }


}