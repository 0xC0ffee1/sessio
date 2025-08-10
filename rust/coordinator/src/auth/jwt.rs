use axum::http::HeaderMap;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use log::{info, warn};
use chrono::{DateTime, Utc};
use std::sync::OnceLock;

// JWT secret loaded from environment variable
static JWT_SECRET: OnceLock<Vec<u8>> = OnceLock::new();

fn get_jwt_secret() -> &'static [u8] {
    JWT_SECRET.get_or_init(|| {
        std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                warn!("JWT_SECRET not found in environment, using default (NOT FOR PRODUCTION!)");
                "your-256-bit-secret-key-change-in-production".to_string()
            })
            .into_bytes()
    })
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // account_id (subject) - now using UUID instead
    pub account_id: String, // account UUID (same as sub for consistency)
    pub exp: usize,         // expiration time
    pub iat: usize,         // issued at
}

// Device Claims for WebSocket authentication
#[derive(Serialize, Deserialize)]
pub struct DeviceClaims {
    sub: String,        // device_id (subject)
    account_id: String, // account UUID
    device_id: String,  // device identifier
    exp: usize,         // expiration time (6 months)
    iat: usize,         // issued at
}

// Token validation result with expiration info
#[derive(Debug)]
pub struct TokenValidationResult {
    pub claims: Claims,
    pub is_expired: bool,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub time_until_expiry: Option<chrono::Duration>,
}

#[derive(Debug)]
pub struct DeviceTokenValidationResult {
    pub device_id: String,
    pub account_id: String,
    pub is_expired: bool,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub time_until_expiry: Option<chrono::Duration>,
}

pub fn generate_jwt_token(account_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp() as usize;
    let exp = now + 3600; // 1 hour expiry
    
    let claims = Claims {
        sub: account_id.to_string(),        // Using account_id as subject
        account_id: account_id.to_string(), // Same value for consistency
        exp,
        iat: now,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_jwt_secret()))
}
pub fn verify_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(token, &DecodingKey::from_secret(get_jwt_secret()), &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e),
    }
}

pub fn generate_device_jwt_token(device_id: &str, account_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now().timestamp() as usize;
    let exp = now + (6 * 30 * 24 * 60 * 60); // 6 months in seconds
    
    let claims = DeviceClaims {
        sub: account_id.to_string(),
        account_id: account_id.to_string(),
        device_id: device_id.to_string(),
        exp,
        iat: now,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(get_jwt_secret()))
}
pub fn validate_jwt_token(headers: &HeaderMap) -> Option<String> {
    // Extract JWT token from headers
    let token = extract_jwt_token_from_headers(headers)?;
    
    // Validate JWT token
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(&token, &DecodingKey::from_secret(get_jwt_secret()), &validation) {
        Ok(token_data) => {
            info!("Valid JWT token for account_id: {}", token_data.claims.sub);
            Some(token_data.claims.sub)
        }
        Err(e) => {
            warn!("JWT validation failed: {}", e);
            None
        }
    }
}

pub fn validate_device_jwt_token(token: &str) -> Option<(String, String)> {
    let validation = Validation::new(Algorithm::HS256);
    match decode::<DeviceClaims>(token, &DecodingKey::from_secret(get_jwt_secret()), &validation) {
        Ok(token_data) => {
            info!("Valid device JWT token for device: {}", token_data.claims.device_id);
            Some((token_data.claims.device_id, token_data.claims.account_id))
        }
        Err(e) => {
            warn!("Device JWT validation failed: {}", e);
            None
        }
    }
}

pub fn extract_jwt_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // First try Authorization header for API calls
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }
    
    // Then try auth_token cookie for browser requests
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse cookies and find the auth_token cookie
            for cookie_part in cookie_str.split(';').map(|c| c.trim()) {
                if let Some(token) = cookie_part.strip_prefix("auth_token=") {
                    return Some(token.to_string());
                }
            }
        }
    }
    
    None
}

pub fn extract_account_from_jwt(headers: &HeaderMap) -> Option<String> {
    validate_jwt_token(headers)
}

// Extract account_id from device JWT token  
pub fn extract_account_id_from_device_jwt(headers: &HeaderMap) -> Option<String> {
    let token = extract_jwt_token_from_headers(headers)?;
    let (_, account_id) = validate_device_jwt_token(&token)?;
    Some(account_id)
}

// Enhanced JWT validation with expiration details
pub fn validate_jwt_token_with_expiration(token: &str) -> Result<TokenValidationResult, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    
    // First try to decode without expiration checking to get the claims
    let mut validation_no_exp = validation.clone();
    validation_no_exp.validate_exp = false;
    
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(get_jwt_secret()), &validation_no_exp)?;
    let claims = token_data.claims;
    
    let now = Utc::now();
    let expires_at = DateTime::from_timestamp(claims.exp as i64, 0).unwrap_or(now);
    let issued_at = DateTime::from_timestamp(claims.iat as i64, 0).unwrap_or(now);
    let is_expired = now > expires_at;
    let time_until_expiry = if !is_expired {
        Some(expires_at - now)
    } else {
        None
    };
    
    Ok(TokenValidationResult {
        claims,
        is_expired,
        expires_at,
        issued_at,
        time_until_expiry,
    })
}

// Enhanced device JWT validation with expiration details
pub fn validate_device_jwt_token_with_expiration(token: &str) -> Result<DeviceTokenValidationResult, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    
    // First try to decode without expiration checking to get the claims
    let mut validation_no_exp = validation.clone();
    validation_no_exp.validate_exp = false;
    
    let token_data = decode::<DeviceClaims>(token, &DecodingKey::from_secret(get_jwt_secret()), &validation_no_exp)?;
    let claims = token_data.claims;
    
    let now = chrono::Utc::now();
    let expires_at = DateTime::from_timestamp(claims.exp as i64, 0).unwrap_or(now);
    let issued_at = DateTime::from_timestamp(claims.iat as i64, 0).unwrap_or(now);
    let is_expired = now > expires_at;
    let time_until_expiry = if !is_expired {
        Some(expires_at - now)
    } else {
        None
    };
    
    Ok(DeviceTokenValidationResult {
        device_id: claims.device_id,
        account_id: claims.account_id,
        is_expired,
        expires_at,
        issued_at,
        time_until_expiry,
    })
}

// Helper function to create TokenExpirationInfo from validation result
pub fn create_token_expiration_info(validation_result: &TokenValidationResult) -> crate::models::TokenExpirationInfo {
    let time_until_expiry_seconds = validation_result.time_until_expiry
        .map(|duration| duration.num_seconds());
    
    let needs_refresh = validation_result.time_until_expiry
        .map(|duration| duration.num_minutes() < 5)
        .unwrap_or(true); // If no time left, definitely needs refresh
    
    crate::models::TokenExpirationInfo {
        is_expired: validation_result.is_expired,
        expires_at: validation_result.expires_at,
        time_until_expiry_seconds,
        needs_refresh,
    }
}
