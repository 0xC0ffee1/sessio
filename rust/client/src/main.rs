use std::{net::SocketAddr, sync::Arc};

use log::error;
use clap::{Parser, Subcommand};

mod client;
use client::Opt;
pub mod ipc;
use homedir::my_home;


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
    
    #[cfg(windows)]
    ipc::start_grpc_server("127.0.0.1:53051").await;
    #[cfg(unix)]
    ipc::start_grpc_server(&format!("{}/.sessio", my_home().unwrap().unwrap().to_str().unwrap())).await;
}
