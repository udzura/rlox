use crate::scanner::*;
use crate::vm::*;

pub fn compile(source: String) -> InterpretResult {
    let mut scanner = Scanner::new(&source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!(
            "{:12} '{:}'",
            format!("{:?}", token.token_type),
            token.strref()
        );

        if token.token_type == TokenType::EOF {
            break;
        }
    }
    Ok(())
}
