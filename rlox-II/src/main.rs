use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

#[allow(non_snake_case)]
extern crate rlox_ii;
use rlox_ii::*;

fn main() -> Result<(), Box<dyn Error>> {
    match args().len() {
        3..=usize::MAX => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        2 => {
            let path = PathBuf::from(args().nth(1).unwrap());
            run_file(&path)?;
        }
        _ => {
            run_prompt()?;
        }
    }
    Ok(())
}
