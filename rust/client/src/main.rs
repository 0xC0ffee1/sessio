use std::{net::SocketAddr, sync::Arc};

use log::error;
use clap::{Parser, Subcommand};

mod client;
use client::Opt;

pub mod platform;
pub mod ipc;


fn main() {
    let args = Opt::parse();
    let err = client::run(args);
}
