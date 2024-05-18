use crate::prelude::*;

#[test]
fn test_tokens() {
    for (prog, ex) in [
        (
            "",
            vec![Token {
                typ: TokenType::Eof,
                lexeme: Lexeme::default(),
                literal: None,
                line: 1,
            }],
        ),
        (
            "3",
            vec![
                Token {
                    typ: TokenType::Number,
                    lexeme: Lexeme::from("3"),
                    literal: Some(Value::Number(3.0)),
                    line: 1,
                },
                Token {
                    typ: TokenType::Eof,
                    lexeme: Lexeme::default(),
                    literal: None,
                    line: 1,
                },
            ],
        ),
        (
            r#""foo""#,
            vec![
                Token {
                    typ: TokenType::String,
                    lexeme: Lexeme::from("foo"),
                    literal: Some(Value::String("foo".into())),
                    line: 1,
                },
                Token {
                    typ: TokenType::Eof,
                    lexeme: Lexeme::default(),
                    literal: None,
                    line: 1,
                },
            ],
        ),
    ] {
        let mut s = Scanner::new(prog);
        let toks = s.scan_tokens().unwrap();
        assert_eq!(toks, ex, "prog '{prog}' produced {toks:#?}");
    }
}
