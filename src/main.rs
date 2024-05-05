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
        run_file(&script)?
    } else {
        run_prompt()?
    }
    Ok(())
}

fn run_file(script: &Path) -> Result<()> {
    let mut lox = Lox::new();
    let bs = fs::read(script)?;
    let prog = String::from_utf8(bs).context("script to utf8")?;
    lox.run(prog)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    let mut lox = Lox::new();
    for line in stdin().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if let Err(err) = lox.run(line) {
            eprintln!("{err}");
        };
    }
    Ok(())
}

fn run(prog: String) -> Result<()> {
    Ok(())
}
