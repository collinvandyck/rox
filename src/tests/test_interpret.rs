use std::{
    borrow::BorrowMut,
    cell::RefCell,
    io::{self, Cursor, Write},
    rc::Rc,
};

use crate::prelude::*;

#[test]
fn test_interpret() {
    let prog = r#"
        print "Hello, World!";
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["Hello, World!"]);
}

struct Run {
    stdout: Vec<u8>,
}

impl Run {
    fn stdout_to_string(&self) -> String {
        std::str::from_utf8(&self.stdout).unwrap().to_string()
    }
    fn lines(&self) -> Vec<String> {
        self.stdout_to_string()
            .lines()
            .map(ToString::to_string)
            .collect()
    }
}

fn run_prog(prog: impl AsRef<str>) -> Result<Run, Box<dyn std::error::Error>> {
    let mut buf = Buffer::default();
    Lox::default().stdout(buf.clone()).run(&prog)?;
    Ok(Run { stdout: buf.take() })
}

#[derive(Clone, Default)]
struct Buffer {
    bs: Rc<RefCell<Vec<u8>>>,
}

impl Buffer {
    fn take(mut self) -> Vec<u8> {
        self.bs.as_ref().replace(vec![])
    }
}

impl std::io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.bs.as_ref().borrow_mut().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl From<Buffer> for Box<dyn io::Write> {
    fn from(value: Buffer) -> Self {
        Box::new(value)
    }
}
