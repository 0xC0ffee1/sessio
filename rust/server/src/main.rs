use clap::{Parser};
mod server;
mod sftp;

use server::Opt;

fn main() {
    let args = Opt::parse();
    server::run(args);
}
