use std::{net::SocketAddr, sync::Arc};

use clap::{Parser, Subcommand};
use log::error;

mod client;
pub mod ipc;
use homedir::my_home;

#[derive(Parser, Debug)]
#[clap(name = "client")]
pub struct Opt {
    config_path: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        // Debug mode
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let opt = Opt::parse();

    let path = match opt.config_path {
        Some(path) => path,
        None => format!("{}/.sessio", my_home().unwrap().unwrap().to_str().unwrap()),
    };
    ipc::start_grpc_server(&path).await;
}
