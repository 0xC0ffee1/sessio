use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;

mod coordinator;
mod common;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opt {
    #[clap(long = "listen", short = 'l', default_value = "0.0.0.0:2223")]
    listen: SocketAddr,
    #[clap(short = 'k', long = "key", requires = "cert")]
    key: PathBuf,
    /// TLS certificate in PEM format
    #[clap(short = 'c', long = "cert", requires = "key")]
    cert: PathBuf,
}

fn main() {
    let options = Opt::parse();
    coordinator::run(options);
}
