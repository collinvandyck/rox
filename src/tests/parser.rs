use tracing_test::traced_test;

use crate::prelude::*;

#[test]
fn assign() {
    let prog = "x=42;";
    let scanner = Scanner::new(&prog);
    let tokens = scanner.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    assert_eq!(
        stmts,
        vec![Stmt::Expr(ExprStmt {
            expr: Expr::Assign(AssignExpr {
                name: Token {
                    typ: TokenType::Identifier,
                    lexeme: Lexeme::from("x"),
                    literal: None,
                    line: 1
                },
                value: Box::new(Expr::Literal(LiteralExpr {
                    value: Value::Number(42.0)
                }))
            }),
        })]
    );
}

#[traced_test]
#[test]
fn set() {
    let prog = "foo.x=42;";
    let scanner = Scanner::new(&prog);
    let tokens = scanner.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    // i know that this is wrong
    assert_eq!(
        stmts,
        vec![Stmt::Expr(ExprStmt {
            expr: Expr::Assign(AssignExpr {
                name: Token {
                    typ: TokenType::Identifier,
                    lexeme: Lexeme::from("x"),
                    literal: None,
                    line: 1
                },
                value: Box::new(Expr::Literal(LiteralExpr {
                    value: Value::Number(42.0)
                }))
            }),
        })]
    );
}
