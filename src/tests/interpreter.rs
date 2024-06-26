use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Display,
    io::{self, Cursor, Write},
    rc::Rc,
};

use tracing_test::traced_test;

use crate::prelude::*;

#[test]
fn test_interpret() {
    let prog = r#"
        print "Hello, World!";
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["Hello, World!"]);
}

#[test]
fn test_fib() {
    let prog = r#"
        fun fib(n) {
            if (n <= 1) return n;
            return fib(n - 2) + fib(n - 1);
        }
        print fib(10);
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["55"]);
}

#[test]
fn test_double_define() {
    let prog = r#"
        fun bad() {
          var a = "first";
          var a = "second";
        }
        bad();
    "#;
    let err = run_prog(prog).unwrap_err();
    assert!(
        err.to_string()
            .contains("call: a binding 'a' already exists in this scope"),
        "{err}"
    );
}

#[test]
fn test_global_return() {
    let prog = r#"
        return "at top level";
    "#;
    let err = run_prog(prog).unwrap_err();
    assert!(
        err.to_string().contains("can't return from top-level code"),
        "{err}"
    );
}

#[test]
fn test_closure_binding() {
    let prog = r#"
        var a = "global";
        {
          fun showA() {
            print a;
          }
          showA();
          var a = "block";
          showA();
        }
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["global", "global"]);
}

#[test]
fn test_print_class() {
    let prog = r#"
        class DevonshireCream {
            serveOn() {
                return "Scones";
            }
        }
        print DevonshireCream;
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["DevonshireCream"]);
}

#[test]
#[traced_test]
fn test_new_bagel() {
    let prog = r#"
        class Bagel {}
        var bagel = Bagel();
        print bagel;
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["Bagel instance"]);
}

#[test]
#[traced_test]
fn test_object_properties() {
    let prog = r#"
        class Props {}
        var props = Props();
        props.x = 42;
        print props.x;
    "#;
    let run = run_prog(prog).unwrap();
    assert_eq!(run.lines(), vec!["42"]);
}

#[derive(Debug)]
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
    let mut stdout = Buffer::default();
    let mut stderr = Buffer::default();
    if let Err(err) = Lox::default()
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .run(prog.as_ref().trim())
    {
        eprintln!("{stdout}");
        return Err(err.into());
    }
    Ok(Run {
        stdout: stdout.take(),
    })
}

#[derive(Clone, Default)]
struct Buffer {
    bs: Rc<RefCell<Vec<u8>>>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.bs.as_ref().borrow().clone();
        let out = String::from_utf8_lossy(&out);
        write!(f, "{out}")
    }
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
