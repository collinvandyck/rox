#![allow(unused)]

use rox::prelude::*;

#[derive(Debug, clap::Parser)]
struct Args {
    script: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt().init();
    let ok = if let Some(script) = args.script {
        run_file(&script)?
    } else {
        run_prompt()?
    };
    if !ok {
        std::process::exit(1);
    }
    Ok(())
}

fn run_file(script: &Path) -> Result<bool> {
    let mut lox = Lox::new();
    let bs = fs::read(script)?;
    let prog = String::from_utf8(bs).context("script to utf8")?;
    lox.run(prog);
    Ok(!lox.had_error())
}

fn run_prompt() -> Result<bool> {
    let mut lox = Lox::new();
    for line in stdin().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        lox.clear_err();
        lox.run(line);
    }
    Ok(!lox.had_error())
}

fn run(prog: String) -> Result<()> {
    Ok(())
}
