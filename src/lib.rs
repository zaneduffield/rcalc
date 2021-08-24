use colored::Colorize;
use rustyline::error::ReadlineError::{Eof, Interrupted};
use rustyline::Editor;

mod lex;
mod parse;

pub fn run() {
    let prompt = ">>> ".yellow().to_string();
    let overflow = "... ".yellow().to_string();

    let mut rl = Editor::<()>::new();
    loop {
        if let State::Stop = process_line(&mut rl, &prompt, &overflow) {
            break;
        }
    }
}

enum State {
    Continue,
    Stop,
}

fn process_line(rl: &mut Editor<()>, start_prompt: &str, overflow: &str) -> State {
    let mut input = String::new();
    let mut prompt = start_prompt;
    loop {
        match rl.readline(prompt) {
            Err(Interrupted) | Err(Eof) => return State::Stop,
            Err(e) => panic!("Error: {:?}", e),
            Ok(line) => {
                input.push_str(&line);
                if input.is_empty() {
                    break;
                }
                match parse::eval(&input) {
                    Ok(val) => {
                        println!("{}", val);
                        break;
                    }
                    Err(parse::CalcErr::Lex(e)) => {
                        print_error_message(&input, e);
                        break;
                    }
                    Err(parse::CalcErr::Incomplete) => prompt = overflow,
                };
            }
        }
    }
    rl.add_history_entry(input);
    State::Continue
}

fn print_error_message(input: &str, e: lex::LexErr) {
    let error_indent = 2;
    println!("\n{}", " ".repeat(error_indent) + input);

    let (pos, msg) = e;
    let x = format!("{}^ ", " ".repeat(pos + error_indent));
    println!("{}{}", x.red(), msg);
}
