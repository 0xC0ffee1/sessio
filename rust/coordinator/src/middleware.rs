use axum::{
    middleware::Next,
    http::{Request, StatusCode},
    response::Response,
    body::Body,
};
use log::{info, warn};
use crate::auth::jwt::{validate_jwt_token, validate_jwt_token_with_expiration, extract_jwt_token_from_headers};

// Authentication middleware to protect sensitive endpoints
pub async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let headers = request.headers().clone();
    let path = request.uri().path();
    
    // Allow endpoints without forcing passkey auth
    let public_endpoints = [
        // Coordinator client authentication endpoints (use sessio public key auth)
        "/auth/challenge",
        "/auth/verify",
        // WebAuthn endpoints for frontend passkey authentication only
        "/webauthn/register/start",
        "/webauthn/register/finish", 
        "/webauthn/auth/start",
        "/webauthn/auth/finish",
        // Single-step passkey registration endpoints
        "/passkey/register/start",
        "/passkey/register/finish",
        // System endpoints
        "/health",
        "/auth",
        "/install",
        "/heartbeat",
        "/devices",
        "/ws"
    ];

    if public_endpoints.contains(&path) {
        return Ok(next.run(request).await);
    }

    // For frontend endpoints (device, devices), check for JWT authentication
    if let Some(token) = extract_jwt_token_from_headers(&headers) {
        match validate_jwt_token_with_expiration(&token) {
            Ok(validation_result) => {
                if validation_result.is_expired {
                    warn!("JWT token expired for path: {}", path);
                    // Return a custom status that the frontend can detect
                    return Err(StatusCode::UNAUTHORIZED);
                } else {
                    info!("JWT authentication successful for: {}", path);
                    return Ok(next.run(request).await);
                }
            }
            Err(e) => {
                warn!("JWT validation failed for path {}: {}", path, e);
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    
    warn!("JWT authentication failed: no token provided for {}", path);
    Err(StatusCode::UNAUTHORIZED)
}
