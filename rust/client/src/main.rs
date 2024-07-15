use std::{net::SocketAddr, sync::Arc};

use log::error;
use clap::{Parser, Subcommand};

mod client;
use client::Opt;

pub mod ipc;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    ipc::start_grpc_server("127.0.0.1:53051").await;
}
