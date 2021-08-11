use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Dash,
    Caret,
    Slash,
    Star,
    Number(f64),
    End,
}

pub const UNKNOWN_SYMBOL: &str = "unknown symbol";

pub type TokenPosition = usize;
pub type LexErr = (TokenPosition, &'static str);
pub type LexResult = Result<(TokenPosition, Token), LexErr>;

fn read_num(iter: &mut Peekable<Enumerate<impl Iterator<Item = char>>>) -> LexResult {
    let mut num = String::new();
    let mut found_dot = false;

    let mut pos = 0;
    while let Some((i, c)) = iter.peek() {
        pos = *i;
        if *c == '.' {
            if found_dot {
                break;
            } else {
                found_dot = true;
            }
        }
        if c.is_ascii_digit() || *c == '.' {
            num.push(*c);
            iter.next();
        } else {
            break;
        }
    }
    match num.parse() {
        Ok(n) => Ok((pos, Token::Number(n))),
        Err(_) => Err((pos, UNKNOWN_SYMBOL)),
    }
}

fn next_token(iter: &mut Peekable<Enumerate<impl Iterator<Item = char>>>) -> LexResult {
    use Token::*;

    let mut pos = 0;
    while let Some((i, c)) = iter.peek() {
        pos = *i;
        if c.is_whitespace() {
            iter.next();
            continue;
        } else {
            let token = match c {
                '(' => LParen,
                ')' => RParen,
                '+' => Plus,
                '-' => Dash,
                '*' => Star,
                '/' => Slash,
                '^' => Caret,
                _ => return read_num(iter),
            };
            iter.next();
            return Ok((pos, token));
        }
    }
    return Ok((pos, End));
}

pub struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().enumerate().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;
    fn next(&mut self) -> Option<Self::Item> {
        match next_token(&mut self.chars) {
            Ok((_, Token::End)) => None,
            x => Some(x),
        }
    }
}
