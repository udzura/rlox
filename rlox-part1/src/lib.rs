use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod errors;
pub mod scanner;

pub fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes)?;
    let code = String::from_utf8(bytes).expect("invalid string");
    match run(code) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            exit(65);
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

use scanner::*;
fn run(source: String) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens()?;
    for token in scanner.tokens.iter() {
        println!("{}", token);
    }

    eprintln!("parse op: {}", source);
    Ok(())
}
