use rox::prelude::*;

fn main() {
    let left = Expr::unary(
        Token::new(TokenType::Minus, Lexeme::from("-"), None, 1),
        Expr::literal(Literal::Number(123.0)),
    );
    let op = Token::new(TokenType::Star, Lexeme::from("*"), None, 1);
    let right = Expr::group(Expr::literal(Literal::String(String::from("foo"))));
    let expr = Expr::binary(left, op, right);
    let mut printer = AstPrinter::default();
    let out = expr.accept(&mut printer);
    println!("{out}");
}
