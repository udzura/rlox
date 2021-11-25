#![feature(box_into_inner)]

use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::rc::Rc;

pub mod ast_printer;
pub mod callable;
pub mod class;
pub mod context;
pub mod environment;
pub mod errors;
pub mod expr;
pub mod function;
pub mod instance;
pub mod interpreter;
pub mod parser;
pub mod resolver;
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
                eprintln!("{}", e);
                exit(65);
            }
            if let Some(e) = e.downcast_ref::<SemanticError>() {
                eprintln!("{}", e);
                exit(66);
            }
            if let Some(e) = e.downcast_ref::<RuntimeBreak>() {
                eprintln!("{}", e);
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

use context::Context;
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::*;

fn run(source: String) -> Result<(), Box<dyn Error>> {
    let context = Rc::new(RefCell::new(Context::default()));
    let mut scanner = Scanner::new(&source, context.clone());
    scanner.scan_tokens()?;

    let parser = Parser::new(scanner.tokens, context.clone());
    let statements = parser.parse();
    if context.borrow().had_error {
        return Err(Box::new(errors::ParseError::raise()));
    }

    // if let Some(_) = std::env::var_os("LOX_DEBUG") {
    //     println!(
    //         "[DEBUG] AST: {}",
    //         ast_printer::AstPrinter {}.print(&expression)
    //     );
    // }

    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter, context.clone());
    resolver.resolve(&statements);
    if context.borrow().had_error {
        return Err(Box::new(errors::SemanticError::raise()));
    }

    interpreter.interpret(&statements)?;

    Ok(())
}
