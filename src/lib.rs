use ansi_colors::*;
use rustyline::error::ReadlineError::{Eof, Interrupted};
use rustyline::Editor;

mod lex;
mod parse;

pub fn run() {
    let mut coloured_prompt = ColouredStr::new(">>> ");
    let mut overflow_prompt = ColouredStr::new("... ");

    coloured_prompt.yellow();
    overflow_prompt.yellow();
    let overflow_prompt = overflow_prompt.coloured_string;

    let mut rl = Editor::<()>::new();
    loop {
        let mut prompt = &coloured_prompt.coloured_string;
        let mut input = String::new();
        loop {
            match rl.readline(prompt) {
                Err(Interrupted) | Err(Eof) => return,
                Err(e) => panic!("Error: {:?}", e),
                Ok(line) => {
                    input.push_str(&line);
                    if input.is_empty() {
                        break;
                    }
                    match parse::eval(&input) {
                        Ok(val) => {
                            rl.add_history_entry(input);
                            println!("{}", val);
                            break;
                        }
                        Err(parse::CalcErr::Lex(e)) => {
                            print_error_message(&input, e);
                            rl.add_history_entry(input);
                            break;
                        }
                        Err(parse::CalcErr::Incomplete) => prompt = &overflow_prompt,
                    };
                }
            }
        }
    }
}

fn print_error_message(input: &str, e: lex::LexErr) {
    let error_indent = 2;
    println!("\n{}", " ".repeat(error_indent) + input);

    let (pos, msg) = e;
    let x = format!("{}^ ", " ".repeat(pos + error_indent));
    let mut pointer = ColouredStr::new(&x);
    pointer.red();
    println!("{}", pointer.coloured_string + msg);
}
