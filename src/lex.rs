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

pub type TokenPosition = usize;

pub struct LexResult {
    pub pos: TokenPosition,
    pub token: Result<Token, LexErr>,
}

#[derive(Debug, Clone)]
pub struct LexErr(String);

fn read_num(c: char, iter: &mut Peekable<Enumerate<impl Iterator<Item = char>>>) -> LexResult {
    let mut num = c.to_string();
    let mut found_dot = false;

    let i = 0;
    while let Some((_, c)) = iter.peek() {
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
        Ok(n) => LexResult {
            pos: i,
            token: Ok(Token::Number(n)),
        },
        Err(_) => LexResult {
            pos: i,
            token: Err(LexErr("Expected number".to_string())),
        },
    }
}

fn next_token(iter: &mut Peekable<Enumerate<impl Iterator<Item = char>>>) -> LexResult {
    use Token::*;

    let i = 0;
    while let Some((i, c)) = iter.next() {
        if c.is_whitespace() {
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
                _ => {
                    return read_num(c, iter);
                }
            };
            return LexResult {
                pos: i,
                token: Ok(token),
            };
        }
    }
    return LexResult {
        pos: i,
        token: Ok(End),
    };
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
            LexResult {
                pos: _,
                token: Ok(Token::End),
            } => None,
            x => Some(x),
        }
    }
}
