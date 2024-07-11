use std::{net::SocketAddr, sync::Arc};

use log::error;
use clap::{Parser, Subcommand};

mod coordinator;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opt {
    #[clap(long = "listen", short = 'l', default_value = "0.0.0.0:2223")]
    listen: SocketAddr,
}

fn main() {
    let args = Opt::parse();
    coordinator::run(args.listen);
}
