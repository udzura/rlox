use std::error::Error;
use std::{env::args, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    match args().len() {
        3..=usize::MAX => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        2 => {
            // run_file(args().nth(1));
            unimplemented!("file");
        }
        _ => {
            // run_prompt();
            unimplemented!("prompt");
        }
    }
}
