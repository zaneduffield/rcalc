use std::convert::TryFrom;
use std::io;
use std::io::Write;
use std::iter::Peekable;

extern crate ansi_colors;
use ansi_colors::*;

fn main() {
    println!("Evaluate math expressions using + - * / ^ ()");
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
        match Expr::parse(&input) {
            Ok(expr) => println!("{}", expr.eval()),
            Err(e) => println!("Error: {:?}", e),
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    LParen,
    RParen,
    Plus,
    Dash,
    Caret,
    Slash,
    Star,
    Number(f64),
}

fn read_num(c: char, iter: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut num = c.to_string();
    let mut found_dot = false;

    while let Some(c) = iter.peek() {
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
    num
}

fn tokenize(input: &str) -> Result<Vec<Token>, ParseErr> {
    use Token::*;

    let mut tokens = vec![];
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        } else {
            match c {
                '(' => tokens.push(LParen),
                ')' => tokens.push(RParen),
                '+' => tokens.push(Plus),
                '-' => tokens.push(Dash),
                '*' => tokens.push(Star),
                '/' => tokens.push(Slash),
                '^' => tokens.push(Caret),
                _ => {
                    let num = read_num(c, &mut iter);
                    match num.parse() {
                        Ok(n) => tokens.push(Number(n)),
                        Err(_) => return Err(ParseErr::SyntaxErr("Expected number".to_string())),
                    }
                }
            }
        }
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
}

impl TryFrom<Token> for Operator {
    type Error = &'static str;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        use Operator::*;
        match token {
            Token::Plus => Ok(Add),
            Token::Dash => Ok(Sub),
            Token::Caret => Ok(Pow),
            Token::Slash => Ok(Div),
            Token::Star => Ok(Mul),
            _ => Err("Can only convert operators"),
        }
    }
}

#[derive(Debug)]
enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Number(f64),
}

#[derive(Debug)]
enum ParseErr {
    SyntaxErr(String),
}

impl Expr {
    fn eval(self) -> f64 {
        use Expr::*;
        use Operator::*;

        match self {
            Expr::Number(x) => x,
            Unary(Neg, x) => -x.eval(),
            Binary(Add, x, y) => x.eval() + y.eval(),
            Binary(Sub, x, y) | Binary(Neg, x, y) => x.eval() - y.eval(),
            Binary(Mul, x, y) => x.eval() * y.eval(),
            Binary(Div, x, y) => x.eval() / y.eval(),
            Binary(Pow, x, y) => x.eval().powf(y.eval()),
            Unary(_, x) => x.eval(),
        }
    }

    fn _parse_expression(
        input: &mut Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<Expr, ParseErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_term(input)?;
        match input.peek() {
            Some(Token::Plus) => {
                input.next();
                Ok(Binary(
                    Add,
                    Box::new(left),
                    Box::new(Expr::_parse_term(input)?),
                ))
            }
            Some(Token::Dash) => {
                input.next();
                Ok(Binary(
                    Sub,
                    Box::new(left),
                    Box::new(Expr::_parse_term(input)?),
                ))
            }
            _ => Ok(left),
        }
    }

    fn _parse_term(input: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Expr, ParseErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_factor(input)?;
        match input.peek() {
            Some(Token::Star) => {
                input.next();
                Ok(Binary(
                    Mul,
                    Box::new(left),
                    Box::new(Expr::_parse_factor(input)?),
                ))
            }
            Some(Token::Slash) => {
                input.next();
                Ok(Binary(
                    Div,
                    Box::new(left),
                    Box::new(Expr::_parse_factor(input)?),
                ))
            }
            _ => Ok(left),
        }
    }

    fn _parse_factor(input: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Expr, ParseErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_primary(input)?;
        match input.peek() {
            Some(Token::Caret) => {
                input.next();
                Ok(Binary(
                    Pow,
                    Box::new(left),
                    Box::new(Expr::_parse_factor(input)?),
                ))
            }
            _ => Ok(left),
        }
    }

    fn _parse_primary(input: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Expr, ParseErr> {
        use Expr::*;
        use Operator::*;

        match input.next() {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::LParen) => {
                let expr = Expr::_parse_expression(input)?;
                match input.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ParseErr::SyntaxErr(format!("Expected closing parenthesis"))),
                }
            }
            Some(Token::Dash) => Ok(Unary(Neg, Box::new(Expr::_parse_expression(input)?))),
            _ => Err(ParseErr::SyntaxErr(format!("Unexpected token"))),
        }
    }

    fn parse(input: &str) -> Result<Expr, ParseErr> {
        let tokens = tokenize(input)?;
        Expr::_parse_expression(&mut tokens.iter().peekable())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_pos_num() {
        assert_eq!(1.0, Expr::parse("1.0").unwrap().eval());
    }
    #[test]
    pub fn test_paren() {
        assert_eq!(1.0, Expr::parse("(1.0)").unwrap().eval());
    }
    #[test]
    pub fn test_mul() {
        assert_eq!(15.0, Expr::parse("3*5").unwrap().eval());
    }
    #[test]
    pub fn test_add() {
        assert_eq!(9.0, Expr::parse("2 + 7").unwrap().eval());
    }
    #[test]
    pub fn test_sub() {
        assert_eq!(10.0, Expr::parse("11 - 1").unwrap().eval());
    }

    #[test]
    pub fn test_paren_add() {
        assert_eq!(2.0, Expr::parse("(1)+1").unwrap().eval());
    }

    #[test]
    pub fn test_double_neg() {
        assert_eq!(2.0, Expr::parse("1--1").unwrap().eval());
    }

    #[test]
    pub fn test_right_add() {
        assert_eq!(1.5, Expr::parse("(1+0.25)+0.25").unwrap().eval());
    }
    #[test]
    pub fn test_left_add() {
        assert_eq!(1.5, Expr::parse("1+(0.25+0.25)").unwrap().eval());
    }
    #[test]
    pub fn test_mul_before_add() {
        assert_eq!(14.0, Expr::parse("4*3+2").unwrap().eval());
        assert_eq!(14.0, Expr::parse("2+4*3").unwrap().eval());
    }
}
