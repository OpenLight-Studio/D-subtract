use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut parser = parser::Parser::new(contents);
    let code = parser.generate_code();

    fs::write("output.bin", &code).expect("Failed to write output");
    println!("Compiled successfully to output.bin");
}
