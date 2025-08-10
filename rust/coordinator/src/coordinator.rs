use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;

use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use tower_http::cors::CorsLayer;
use crate::Opt;
use sessio_coordinator::db::DatabaseRepository;

// Import our modules
use sessio_coordinator::auth::*;
use sessio_coordinator::handlers::*;
use sessio_coordinator::models::*;
use sessio_coordinator::middleware::auth_middleware;

#[tokio::main]
pub async fn run(options: Opt) {
    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        // Debug mode
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    // Initialize database connection
    let database_url = options.database_url.as_ref()
        .expect("Database URL must be provided");
    
    let db = Arc::new(
        DatabaseRepository::new(database_url)
            .await
            .expect("Failed to connect to database"),
    );

    // Initialize database schema
    db.initialize_schema()
        .await
        .expect("Failed to initialize database schema");

    let server = Server {
        sessions: HashMap::new(),
        clients: HashMap::new(),
        db: db.clone(),
        coordinator_url: options.coordinator_url.clone().unwrap_or_else(|| "http://localhost:8000".to_string()),
        web_ui_url: options.web_ui_url.unwrap_or_else(|| options.coordinator_url.unwrap_or_else(|| "http://localhost:8000".to_string())),
    };
    let app_state = Arc::new(Mutex::new(server));

    let app = Router::new()
        // Account creation now only through passkey registration
        .route("/device", post(device_handler).delete(delete_device_handler).patch(update_device_handler))
        .route("/devices", post(devices_handler))
        .route("/install", post(install_handler))
        .route("/auth", post(auth_handler))
        
        // WebAuthn routes for passkey authentication
        // New single-step passkey registration
        .route("/passkey/register/start", post(passkey_register_start_handler))
        .route("/passkey/register/finish", post(passkey_register_finish_handler))
        .route("/webauthn/auth/start", post(webauthn_auth_start_handler))
        .route("/webauthn/auth/finish", post(webauthn_auth_finish_handler))
        .route("/device/sign/start", post(sign_device_start_handler))
        .route("/device/sign/finish", post(sign_device_finish_handler))
        .route("/authorized-keys", post(authorized_keys_handler))
        .route("/ws", get(ws_handler))
        .route("/health", get(health_handler))
        .layer(middleware::from_fn(auth_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
        )
        .with_state(app_state);

    // Start server based on TLS configuration  
    if options.dangerously_use_http {
        println!("Started HTTP/WebSocket server on {}!", options.listen);
        let listener = tokio::net::TcpListener::bind(options.listen).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        use axum_server::tls_rustls::RustlsConfig;
        
        let cert_file = options.cert_file.as_ref()
            .expect("TLS certificate file path must be provided for HTTPS mode");
        let key_file = options.key_file.as_ref()
            .expect("TLS private key file path must be provided for HTTPS mode");
            
        let config = RustlsConfig::from_pem_file(cert_file, key_file)
            .await
            .expect("Failed to load TLS configuration");
            
        println!("Started HTTPS/WebSocket server on {}!", options.listen);
        axum_server::bind_rustls(options.listen, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}