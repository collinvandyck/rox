use crate::Lox;

#[test]
fn test_interpret() {
    let prog = r#"print "Hello""#;
}

struct Run {
    stdout: Vec<u8>,
}

fn run_prog(prog: impl AsRef<str>) -> anyhow::Result<()> {
    let mut stdout = vec![];
    Lox::default().stdout(Box::new(stdout)).run(prog)?;
    Ok(())
}
