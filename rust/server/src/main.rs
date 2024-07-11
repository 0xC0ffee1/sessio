use clap::{Parser};
mod server;

use server::Opt;

fn main() {
    let args = Opt::parse();
    server::run(args);
}
