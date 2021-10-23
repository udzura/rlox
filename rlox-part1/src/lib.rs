use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod ast_printer;
pub mod environment;
pub mod errors;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
pub mod value;
pub mod visitor;

pub fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    use errors::*;

    let mut f = File::open(path)?;
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes)?;
    let code = String::from_utf8(bytes).expect("invalid string");
    match run(code) {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<ParseError>() {
                eprintln!("{:?}", e);
                exit(65);
            }
            if let Some(e) = e.downcast_ref::<RuntimeError>() {
                eprintln!("{:?}", e);
                exit(70);
            }
            Ok(())
        }
    }
}

pub fn run_prompt() -> Result<(), IoError> {
    let mut reader = BufReader::new(std::io::stdin());

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }
        match run(line) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    println!();
    println!("Adiós");
    Ok(())
}

use interpreter::Interpreter;
use parser::Parser;
use scanner::*;

fn run(source: String) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens()?;

    let parser = Parser::new(scanner.tokens);
    if let Ok(statements) = parser.parse() {
        // if let Some(_) = std::env::var_os("LOX_DEBUG") {
        //     println!(
        //         "[DEBUG] AST: {}",
        //         ast_printer::AstPrinter {}.print(&expression)
        //     );
        // }

        Interpreter::new().interpret(&statements)?;
    }

    Ok(())
}
