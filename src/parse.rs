use std::convert::{From, TryFrom};
use std::iter::Peekable;

use crate::lex;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
}

impl TryFrom<lex::Token> for Operator {
    type Error = &'static str;

    fn try_from(token: lex::Token) -> Result<Self, Self::Error> {
        use Operator::*;
        match token {
            lex::Token::Plus => Ok(Add),
            lex::Token::Dash => Ok(Sub),
            lex::Token::Caret => Ok(Pow),
            lex::Token::Slash => Ok(Div),
            lex::Token::Star => Ok(Mul),
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
pub enum CalcErr {
    Lex(lex::LexErr),
    Syntax(String),
}

impl From<lex::LexErr> for CalcErr {
    fn from(e: lex::LexErr) -> CalcErr {
        CalcErr::Lex(e)
    }
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

    fn _parse_expression(input: &mut Peekable<lex::Lexer>) -> Result<Expr, CalcErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_term(input)?;
        match input.peek() {
            None => Ok(left),
            Some(result) => match &result.token {
                Ok(lex::Token::Plus) => {
                    input.next();
                    Ok(Binary(
                        Add,
                        Box::new(left),
                        Box::new(Expr::_parse_term(input)?),
                    ))
                }
                Ok(lex::Token::Dash) => {
                    input.next();
                    Ok(Binary(
                        Sub,
                        Box::new(left),
                        Box::new(Expr::_parse_term(input)?),
                    ))
                }
                Ok(_) => Ok(left),
                Err(e) => Err(CalcErr::from(e.clone())),
            },
        }
    }

    fn _parse_term(input: &mut Peekable<lex::Lexer>) -> Result<Expr, CalcErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_factor(input)?;
        match input.peek() {
            None => Ok(left),
            Some(result) => match &result.token {
                Ok(lex::Token::Star) => {
                    input.next();
                    Ok(Binary(
                        Mul,
                        Box::new(left),
                        Box::new(Expr::_parse_factor(input)?),
                    ))
                }
                Ok(lex::Token::Slash) => {
                    input.next();
                    Ok(Binary(
                        Div,
                        Box::new(left),
                        Box::new(Expr::_parse_factor(input)?),
                    ))
                }
                Ok(_) => Ok(left),
                Err(e) => Err(CalcErr::from(e.clone())),
            },
        }
    }

    fn _parse_factor(input: &mut Peekable<lex::Lexer>) -> Result<Expr, CalcErr> {
        use Expr::*;
        use Operator::*;

        let left = Expr::_parse_primary(input)?;
        match input.peek() {
            None => Ok(left),
            Some(result) => match &result.token {
                Ok(lex::Token::Caret) => {
                    input.next();
                    Ok(Binary(
                        Pow,
                        Box::new(left),
                        Box::new(Expr::_parse_factor(input)?),
                    ))
                }
                Ok(_) => Ok(left),
                Err(e) => Err(CalcErr::from(e.clone())),
            },
        }
    }

    fn _parse_primary(input: &mut Peekable<lex::Lexer>) -> Result<Expr, CalcErr> {
        use Expr::*;
        use Operator::*;

        match input.next() {
            None => Err(CalcErr::Syntax(format!("Expected more!"))),
            Some(result) => match result.token {
                Ok(lex::Token::Number(n)) => Ok(Expr::Number(n)),
                Ok(lex::Token::LParen) => {
                    let expr = Expr::_parse_expression(input)?;
                    if let Some(lex::LexResult {
                        pos: _,
                        token: Ok(lex::Token::RParen),
                    }) = input.next()
                    {
                        Ok(expr)
                    } else {
                        Err(CalcErr::Syntax(format!("Expected closing parenthesis")))
                    }
                }
                Ok(lex::Token::Dash) => Ok(Unary(Neg, Box::new(Expr::_parse_term(input)?))),
                _ => Err(CalcErr::Syntax(format!("Unexpected token"))),
            },
        }
    }

    fn parse(input: &str) -> Result<Expr, CalcErr> {
        Expr::_parse_expression(&mut lex::Lexer::new(input).peekable())
    }
}

pub fn eval(input: &str) -> Result<f64, CalcErr> {
    Ok(Expr::parse(input)?.eval())
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_num_num() {
        assert_eq!(1.0, eval("1.0").unwrap());
    }
    #[test]
    pub fn test_mul() {
        assert_eq!(15.0, eval("3 * 5").unwrap());
    }
    #[test]
    pub fn test_add() {
        assert_eq!(9.0, eval("2 + 7").unwrap());
    }
    #[test]
    pub fn test_sub() {
        assert_eq!(10.0, eval("11 - 1").unwrap());
    }

    #[test]
    pub fn test_po() {
        assert_eq!(25.0, eval("5^2").unwrap());
        assert_eq!(3.0, eval("9^0.5").unwrap());
    }

    #[test]
    pub fn test_double_neg() {
        assert_eq!(2.0, eval("1--1").unwrap());
    }

    #[test]
    pub fn test_right_add() {
        assert_eq!(1.5, eval("(1+0.25)+0.25").unwrap());
    }

    #[test]
    pub fn test_left_add() {
        assert_eq!(1.5, eval("1+(0.25+0.25)").unwrap());
    }

    #[test]
    pub fn test_mul_before_add() {
        assert_eq!(14.0, eval("4 * 3 + 2").unwrap());
        assert_eq!(14.0, eval("2 + 4 * 3").unwrap());
    }

    #[test]
    pub fn test_neg_before_add() {
        assert_eq!(0.0, eval("-5 + 5").unwrap());
        assert_eq!(-10.0, eval("-(5 + 5)").unwrap());
    }

    #[test]
    pub fn test_pow_before_all() {
        assert_eq!(-25.0, eval("-5^2").unwrap());
        assert_eq!(37.0, eval("6^2+1").unwrap());
        assert_eq!(200.0, eval("2*10^2").unwrap());
        assert_eq!(2.0, eval("2^2/2").unwrap());
    }

    #[test]
    pub fn test_paren() {
        assert_eq!(12.0, eval("2 * (5 + 1)").unwrap());
        assert_eq!(11.0, eval("(2 * 5) + 1").unwrap());
    }
}
