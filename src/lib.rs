use std::io;
use std::io::Write;

extern crate ansi_colors;
use ansi_colors::*;

mod lex;
mod parse;

pub fn run() {
    let mut prompt = ColouredStr::new(">>> ");
    let mut overflow_prompt = ColouredStr::new("....");
    let prompt_len = prompt.string.len();

    prompt.yellow();
    overflow_prompt.yellow();
    let overflow_prompt = overflow_prompt.coloured_string;
    let prompt = prompt.coloured_string;

    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        loop {
            io::stdin().read_line(&mut input).unwrap();
            if input.is_empty() {
                return;
            } else if input.trim().is_empty() {
                break;
            }
            match parse::eval(&input) {
                Ok(val) => {
                    println!("{}", val);
                    break;
                }
                Err(parse::CalcErr::Lex(e)) => {
                    print_error_message(&input, prompt_len, e);
                    break;
                }
                Err(parse::CalcErr::Incomplete) => {
                    print!("{} ", overflow_prompt);
                    io::stdout().flush().unwrap();
                }
            };
        }
    }
}

fn print_error_message(input: &str, prompt_len: usize, e: lex::LexErr) {
    if input.trim().chars().any(|c| c == '\n') {
        println!(
            "\n{:pad$}{}",
            "",
            input.replace('\n', " "),
            pad = prompt_len
        );
    }
    println!("{:pad$}{}", "", format_error_message(e), pad = prompt_len);
}

fn format_error_message(err: lex::LexErr) -> String {
    let (pos, msg) = err;
    let x = format!("{:pad$}^ ", "", pad = pos);
    let mut pointer = ColouredStr::new(&x);
    pointer.red();
    pointer.coloured_string + msg
}
