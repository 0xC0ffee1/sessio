use std::{fs, net::SocketAddr, path::PathBuf};

use clap::Parser;
use serde::Deserialize;


mod coordinator;

mod db;

#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opt {
    #[clap(long = "listen", short = 'l', default_value = "0.0.0.0:2223")]
    listen: SocketAddr,

    #[clap(long, short = 'd')]
    database_url: Option<String>,

    #[clap(long)]
    coordinator_url: Option<String>,

    #[clap(long, help = "Web UI URL for WebAuthn RP ID (e.g. https://app.example.com)")]
    web_ui_url: Option<String>,

    #[clap(long, default_value = "info")]
    log_level: String,

    // TLS certificate paths
    #[clap(long, help = "Path to TLS certificate file (PEM format)")]
    cert_file: Option<PathBuf>,

    #[clap(long, help = "Path to TLS private key file (PEM format)")]
    key_file: Option<PathBuf>,

    // Allow HTTP mode (dangerous, for development only)
    #[clap(long, help = "Allow HTTP connections (dangerous, for development or proxied connections only)")]
    dangerously_use_http: bool,

    // Optional configuration file
    #[clap(long, short = 'f')]
    config: Option<PathBuf>,
}

impl Opt {
    pub fn from_args_and_file() -> Self {
        let mut opt = Opt::parse(); // Parse from command line arguments

        if let Some(config_path) = &opt.config {
            let config_content =
                fs::read_to_string(config_path).expect("Failed to read configuration file");
            let config: Opt =
                toml::from_str(&config_content).expect("Failed to parse configuration file");

            // Merge configurations
            opt.listen = config.listen;
            if config.database_url.is_some() {
                opt.database_url = config.database_url;
            }
            if config.coordinator_url.is_some() {
                opt.coordinator_url = config.coordinator_url;
            }
            if config.web_ui_url.is_some() {
                opt.web_ui_url = config.web_ui_url;
            }
            opt.log_level = config.log_level;
            if config.cert_file.is_some() {
                opt.cert_file = config.cert_file;
            }
            if config.key_file.is_some() {
                opt.key_file = config.key_file;
            }
            if config.dangerously_use_http {
                opt.dangerously_use_http = config.dangerously_use_http;
            }
        }

        // Use environment variable if database URL not provided via CLI or config
        if opt.database_url.is_none() {
            opt.database_url = Some(
                std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| {
                        eprintln!("ERROR: DATABASE_URL environment variable must be set or provided via --database-url");
                        std::process::exit(1);
                    })
            );
        }

        // Use environment variable if coordinator URL not provided via CLI or config
        if opt.coordinator_url.is_none() {
            opt.coordinator_url = Some(
                std::env::var("COORDINATOR_URL")
                    .unwrap_or_else(|_| "http://localhost:8000".to_string())
            );
        }

        opt
    }
}

fn main() {
    let options = Opt::from_args_and_file();
    coordinator::run(options);
}
