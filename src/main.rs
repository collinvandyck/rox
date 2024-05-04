#![allow(unused)]

use std::path::PathBuf;

use clap::Parser;
use tracing::info;

#[derive(Debug, clap::Parser)]
struct Args {
    script: PathBuf,
}

fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    info!("Hello, world!");
}
