use std::io;
use std::io::Write;

extern crate ansi_colors;
use ansi_colors::*;

mod lex;
mod parse;

pub fn run() {
    let mut prompt = ColouredStr::new(">>> ");
    prompt.red();
    loop {
        let mut input = String::new();
        print!("{}", prompt.coloured_string);
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).unwrap();
        if input == "" {
            break;
        };
        match parse::eval(&input) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("Error: {:?}", e),
        };
    }
}
