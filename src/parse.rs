use std::convert::From;
use std::iter::Peekable;

use crate::lex;
use lex::Token::*;
use Operator::*;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
}

#[derive(Debug)]
enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Number(f64),
}

const UNEXPECTED_TOKEN: &str = "not expected here";

#[derive(Debug, PartialEq, Eq)]
pub enum CalcErr {
    Lex(lex::LexErr),
    Incomplete,
}

impl From<lex::LexErr> for CalcErr {
    fn from(e: lex::LexErr) -> Self {
        CalcErr::Lex(e)
    }
}

type ExprResult = Result<Expr, CalcErr>;

impl Expr {
    fn eval(self) -> f64 {
        use Expr::*;

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

    fn _parse(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        let expr = Expr::_parse_expression(input)?;
        match input.next() {
            None => Ok(expr),
            Some(x) => {
                let (pos, _) = x?;
                Err(CalcErr::Lex((pos, UNEXPECTED_TOKEN)))
            },
        }
    }

    fn _parse_expression(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        use Expr::*;

        let mut expr = Expr::_parse_term(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(x) => match x {
                    Ok((_, Plus)) => {
                        input.next();
                        expr = Binary(Add, Box::new(expr), Box::new(Expr::_parse_term(input)?))
                    }
                    Ok((_, Dash)) => {
                        input.next();
                        expr = Binary(Sub, Box::new(expr), Box::new(Expr::_parse_term(input)?))
                    }
                    _ => return Ok(expr),
                },
            }
        }
    }

    fn _parse_term(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        use Expr::*;

        let mut expr = Expr::_parse_factor(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(x) => match x {
                    Ok((_, Star)) => {
                        input.next();
                        expr = Binary(Mul, Box::new(expr), Box::new(Expr::_parse_factor(input)?))
                    }
                    Ok((_, Slash)) => {
                        input.next();
                        expr = Binary(Div, Box::new(expr), Box::new(Expr::_parse_factor(input)?))
                    }
                    _ => return Ok(expr),
                },
            }
        }
    }

    fn _parse_factor(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        use Expr::*;

        let mut expr = Expr::_parse_primary(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(Ok((_, Caret))) => {
                    input.next();
                    expr = Binary(Pow, Box::new(expr), Box::new(Expr::_parse_factor(input)?))
                }
                _ => return Ok(expr),
            }
        }
    }

    fn _parse_primary(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        use Expr::*;

        match input.next() {
            None => Err(CalcErr::Incomplete),
            Some(x) => match x? {
                (_, lex::Token::Number(n)) => Ok(Expr::Number(n)),
                (_, LParen) => Expr::_parse_paren(input),
                (_, Dash) => Ok(Unary(Neg, Box::new(Expr::_parse_term(input)?))),
                (pos, _) => Err(CalcErr::Lex((pos, UNEXPECTED_TOKEN))),
            },
        }
    }

    fn _parse_paren(input: &mut Peekable<lex::Lexer>) -> ExprResult {
        let expr = Expr::_parse_expression(input)?;
        if let Some(x) = input.next() {
            if let (_, RParen) = x? {
                return Ok(expr)
            }
        }
        Err(CalcErr::Incomplete)
    }

    fn parse(input: &str) -> ExprResult {
        Expr::_parse(&mut lex::Lexer::new(input).peekable())
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
    pub fn test_pow() {
        assert_eq!(25.0, eval("5^2").unwrap());
        assert_eq!(3.0, eval("9^0.5").unwrap());
    }

    #[test]
    pub fn test_double_neg() {
        assert_eq!(2.0, eval("1--1").unwrap());
    }

    #[test]
    pub fn test_chained_add() {
        assert_eq!(3.0, eval("1+1+1").unwrap());
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

    #[test]
    pub fn test_unexpected_token() {
        assert_eq!(Err(CalcErr::Lex((7, UNEXPECTED_TOKEN))), eval("1 - 5 */ 5"));
        assert_eq!(Err(CalcErr::Lex((1, UNEXPECTED_TOKEN))), eval("2()"));
        assert_eq!(Err(CalcErr::Lex((3, UNEXPECTED_TOKEN))), eval("2*()"));
    }

    #[test]
    pub fn test_incomplete() {
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * "));
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * ("));
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * (5+2"));
        assert_eq!(Ok(14.0), eval("2 * (5+2)"));
    }

    #[test]
    pub fn test_unknown_symbol() {
        assert_eq!(Err(CalcErr::Lex((4, lex::UNKNOWN_SYMBOL))), eval("2 * &"));
        assert_eq!(Err(CalcErr::Lex((6, lex::UNKNOWN_SYMBOL))), eval("2 * (1a"));
    }
}
