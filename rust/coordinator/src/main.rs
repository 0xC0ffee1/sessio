use std::{fs, net::SocketAddr, path::PathBuf};

use clap::Parser;
use serde::Deserialize;

mod common;
mod coordinator;

#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opt {
    #[clap(long = "listen", short = 'l', default_value = "0.0.0.0:2223")]
    listen: SocketAddr,
    #[clap(
        short = 'k',
        long = "key",
        requires = "cert",
        default_value = "not_set"
    )]
    key: PathBuf,
    /// TLS certificate in PEM format
    #[clap(
        short = 'c',
        long = "cert",
        requires = "key",
        default_value = "not_set"
    )]
    cert: PathBuf,

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
            opt.key = if opt.key == PathBuf::from("not_set") {
                config.key
            } else {
                opt.key
            };
            opt.cert = if opt.cert == PathBuf::from("not_set") {
                config.cert
            } else {
                opt.cert
            };
        }
        opt
    }
}

fn main() {
    let options = Opt::from_args_and_file();
    coordinator::run(options);
}
