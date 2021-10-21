extern crate rlox_part1;
use rlox_part1::ast_printer::AstPrinter;
use rlox_part1::expr::*;
use rlox_part1::token::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let expression = Expr::binary(
    //     Expr::literal(Literal::Num(1.0)),
    //     Token::new(TokenType::PLUS, "+", Literal::Nil, 1),
    //     Expr::literal(Literal::Num(3.0)),
    // );
    let nil = Literal::Nil;

    let expression = Expr::binary(
        Expr::unary(
            Token::new(TokenType::MINUS, "-", nil.clone(), 1),
            Expr::literal(Literal::Num(123.0)),
        ),
        Token::new(TokenType::STAR, "*", nil.clone(), 1),
        Expr::grouping(Expr::literal(Literal::Num(45.67))),
    );

    println!("{}", AstPrinter {}.print(&expression));
    Ok(())
}
