use rox::prelude::*;

fn main() {
    let left = UnaryExpr::new(
        Token::new(TokenType::Minus, Lexeme::from("-"), None, 1),
        LiteralExpr::new(Literal::Number(123.0)),
    );
    let op = Token::new(TokenType::Star, Lexeme::from("*"), None, 1);
    let right = GroupExpr::new(LiteralExpr::new(Literal::String(String::from("foo"))));
    let expr = BinaryExpr::new(left, op, right);
    let mut printer = AstPrinter::default();
    let out = expr.accept(&mut printer);
    println!("{out}");
}
