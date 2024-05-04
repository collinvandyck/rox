#![allow(unused)]

use rox::prelude::*;

#[derive(Debug, clap::Parser)]
struct Args {
    script: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    if let Some(script) = args.script {
        run_file(&script)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(script: &Path) -> Result<()> {
    let bs = fs::read(script)?;
    let prog = String::from_utf8(bs).context("script to utf8")?;
    run(prog)
}

fn run_prompt() -> Result<()> {
    for line in stdin().lines() {
        let line = line?;
        run(line)?;
    }
    Ok(())
}

fn run(prog: String) -> Result<()> {
    Ok(())
}
