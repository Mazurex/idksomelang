pub mod error;
mod lexer;

use crate::lexer::lexer::Lexer;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid usage! Correct usage is: ./purrin myfile.prn");
        process::exit(1);
    }

    let filepath = &args[1];

    let src = fs::read_to_string(filepath).expect("File doesn't exist lmao");

    let mut lexer = Lexer::new(src, filepath.to_string());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            e.display();
            process::exit(1);
        }
    };

    for token in &tokens {
        let is_empty = token.v.is_some();

        println!(
            "{:?}{}{}{}",
            token.t,
            if is_empty { ' ' } else { '(' },
            token.v.clone().unwrap_or(String::new()),
            if is_empty { ' ' } else { ')' }
        );
    }
}
