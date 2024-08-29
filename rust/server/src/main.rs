use std::{fs, path::PathBuf};

use clap::Parser;
use serde::Deserialize;
use url::Url;
mod server;
mod sftp;

#[derive(Parser, Debug, Deserialize)]
#[clap(name = "client")]
pub struct Opt {
    #[clap(long, short = 'c', default_value = "quic://example.com:2223")]
    coordinator: Url,

    // The identifier of this machine
    #[clap(long, default_value = "id_not_set")]
    id: String,

    // The path to your private key
    #[clap(long, short = 'p', default_value = "keys/ssh_host_ed25519_key")]
    private_key: PathBuf,

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
            opt.coordinator = config.coordinator;
            opt.id = if opt.id == "id_not_set" {
                config.id
            } else {
                opt.id
            };
            opt.private_key = if opt.private_key == PathBuf::from("keys/ssh_host_ed25519_key") {
                config.private_key
            } else {
                opt.private_key
            };
        }
        opt
    }
}

fn main() {
    server::run(Opt::from_args_and_file());
}
