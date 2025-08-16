use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use serde::Deserialize;
use url::Url;
use dirs;
mod server;
mod sftp;
mod config_manager;

use config_manager::ServerConfigManager;

#[derive(Parser, Debug)]
#[clap(name = "sessio-server")]
pub struct Opt {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the server with the specified configuration
    Run {
        #[clap(long, short = 'c')]
        coordinator: Option<Url>,
        
        // The identifier of this machine
        #[clap(long)]
        id: Option<String>,
        
        // The path to your private key
        #[clap(long, short = 'p')]
        private_key: Option<PathBuf>,
        
        // Optional configuration file
        #[clap(long, short = 'f')]
        config: Option<PathBuf>,
    },
    /// Install the server with an install key
    Install {
        /// The install key provided by the coordinator
        #[clap(long, short = 'k')]
        install_key: String,
        
        /// The coordinator URL
        #[clap(long, short = 'c')]
        coordinator: Option<Url>,
        
        /// The identifier of this machine
        #[clap(long)]
        id: Option<String>,
        
        /// Optional configuration file
        #[clap(long, short = 'f')]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    match opt.command {
        Commands::Run { .. } => {
            server::run().await;
        }
        Commands::Install { install_key, coordinator, id, config } => {
            let mut config_manager = ServerConfigManager::new()
                .expect("Failed to initialize configuration manager");
            
            // Use default coordinator URL if not provided
            let coordinator_url = coordinator.unwrap_or_else(|| "https://127.0.0.1:2223".parse().unwrap());
            let device_id = id.unwrap_or_else(|| "id_not_set".to_string());
            
            if let Err(e) = install_server(install_key, coordinator_url, device_id, config, &mut config_manager).await {
                eprintln!("Install failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn install_server(install_key: String, coordinator: Url, id: String, config: Option<PathBuf>, config_manager: &mut ServerConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    use serde_json::json;
    use std::path::Path;
    
    println!("Installing server with install key...");
    
    // Check if using HTTP coordinator is allowed
    if coordinator.scheme() == "http" {
        eprintln!("WARNING: Using HTTP coordinator connection is dangerous and should only be used for development!");
        eprintln!("For production use, please use HTTPS coordinator URLs.");
    }
    
    // Generate or load SSH key pair for this device
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let private_key_path = home_dir.join(".sessio/keys/ssh_host_ed25519_key");
    let public_key = generate_or_load_public_key(&private_key_path)?;
    
    // Create device metadata
    let os_name = std::env::consts::OS;
    let metadata = json!({
        "os_name": os_name,
    });
    
    // Create install request
    let install_request = json!({
        "install_key": install_key,
        "public_key": public_key,
        "metadata": metadata,
    });
    
    // Send install request to coordinator
    let client = reqwest::Client::new();
    let install_url = coordinator.join("install").map_err(|e| format!("Invalid coordinator URL: {}", e))?;
    
    println!("Sending request to: {}", install_url);
    println!("Request body: {}", serde_json::to_string_pretty(&install_request).unwrap_or_else(|_| "Failed to serialize".to_string()));
    
    match client
        .post(install_url)
        .json(&install_request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(install_response) => {
                        println!("Installation successful!");
                        println!("Device ID: {}", install_response["device_id"]);
                        
                        // Store JWT token and device data in settings
                        let jwt_token = install_response["jwt_token"].as_str().unwrap_or("");
                        if jwt_token.is_empty() {
                            eprintln!("Warning: No JWT token received from install response");
                        }
                        
                        // Extract passkey public key and credential ID if available
                        let passkey_public_key = install_response["passkey_public_key"].as_str().map(|s| s.to_string());
                        let passkey_credential_id = install_response["passkey_credential_id"].as_str().map(|s| s.to_string());
                        
                        if passkey_public_key.is_some() && passkey_credential_id.is_some() {
                            println!("Received passkey public key for signature verification");
                        } else {
                            println!("Warning: No passkey public key received - device signature verification will not be available");
                        }
                        
                        if let Err(e) = config_manager.update_account_registration(
                            jwt_token,
                            &install_response["device_id"].as_str().unwrap_or(""),
                            &coordinator.to_string(),
                            passkey_public_key,
                            passkey_credential_id,
                        ).await {
                            eprintln!("Warning: Failed to save account data: {}", e);
                        }
                        
                        println!("Server is now registered and ready to use.");
                    }
                    Err(e) => {
                        eprintln!("Failed to parse install response: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Install request failed with status: {}", response.status());
                if let Ok(error_text) = response.text().await {
                    eprintln!("Error: {}", error_text);
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to send install request: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}


fn generate_or_load_public_key(private_key_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    use common::utils::keygen::generate_keypair;
    use russh::keys::load_secret_key;
    use russh::keys::ssh_key::Algorithm;
    
    // Ensure the keys directory exists
    if let Some(parent_dir) = private_key_path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }
    
    // If the private key doesn't exist, generate it
    if !private_key_path.exists() {
        println!("Generating new SSH key pair...");
        generate_keypair(
            private_key_path.parent().unwrap_or_else(|| std::path::Path::new("keys/")),
            Algorithm::Ed25519,
            private_key_path.file_name().unwrap().to_str().unwrap(),
        )?;
    }
    
    // Load the private key and extract the public key
    let private_key = load_secret_key(private_key_path.to_str().unwrap(), None)?;
    let public_key_ssh = private_key.public_key();
    
    // Convert to openssh format string  
    use russh::keys::PublicKeyBase64;
    let public_key_string = public_key_ssh.public_key_base64();
    
    Ok(public_key_string)
}
