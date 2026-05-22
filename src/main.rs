mod lexer;

use std::{env, fs, process};
use crate::lexer::lexer::Lexer;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid usage! Correct usage is: ./idksomelang myfile.idk");
        process::exit(1);
    }

    let filepath = &args[1];

    let src = fs::read_to_string(filepath)
        .expect("File doesn't exist lmao");


    let mut lexer = Lexer::init(src);
    let tokens = lexer.tokenize();

    for token in &tokens {
        println!("[ {:?}({}) ]", token.t, token.stringify_value());
    }
}