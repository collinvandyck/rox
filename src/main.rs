#![allow(unused)]

use anyhow::Result as AnyRes;
use clap::Parser;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, clap::Parser)]
struct Args {
    script: Option<PathBuf>,
}

fn main() -> AnyRes<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    if let Some(script) = args.script {
        run_file(&script)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(script: &Path) -> AnyRes<()> {
    Ok(())
}

fn run_prompt() -> AnyRes<()> {
    Ok(())
}
