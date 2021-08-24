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
    Mod,
    Pow,
    Neg,
}

#[derive(Debug)]
enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Num(f64),
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
            Num(x) => x,
            Unary(Neg, x) => -x.eval(),
            Binary(Add, x, y) => x.eval() + y.eval(),
            Binary(Sub, x, y) | Binary(Neg, x, y) => x.eval() - y.eval(),
            Binary(Mul, x, y) => x.eval() * y.eval(),
            Binary(Div, x, y) => x.eval() / y.eval(),
            Binary(Mod, x, y) => x.eval() % y.eval(),
            Binary(Pow, x, y) => x.eval().powf(y.eval()),
            Unary(_, x) => x.eval(),
        }
    }
}

mod recursive_descent_parse {
    use super::*;
    use Expr::*;
    type Lexer<'a> = Peekable<lex::Lexer<'a>>;

    fn parse_complete_expr(input: &mut Lexer) -> ExprResult {
        let expr = parse_expr(input)?;
        match input.next() {
            None => Ok(expr),
            Some(x) => {
                let (pos, _) = x?;
                Err(CalcErr::Lex((pos, UNEXPECTED_TOKEN)))
            }
        }
    }

    fn parse_expr(input: &mut Lexer) -> ExprResult {
        let mut expr = parse_term(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(x) => match x {
                    Ok((_, Plus)) => {
                        input.next();
                        expr = Binary(Add, Box::new(expr), Box::new(parse_term(input)?))
                    }
                    Ok((_, Dash)) => {
                        input.next();
                        expr = Binary(Sub, Box::new(expr), Box::new(parse_term(input)?))
                    }
                    _ => return Ok(expr),
                },
            }
        }
    }

    fn parse_term(input: &mut Lexer) -> ExprResult {
        let mut expr = parse_factor(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(x) => match x {
                    Ok((_, Star)) => {
                        input.next();
                        expr = Binary(Mul, Box::new(expr), Box::new(parse_factor(input)?))
                    }
                    Ok((_, Slash)) => {
                        input.next();
                        expr = Binary(Div, Box::new(expr), Box::new(parse_factor(input)?))
                    }
                    Ok((_, Percent)) => {
                        input.next();
                        expr = Binary(Mod, Box::new(expr), Box::new(parse_factor(input)?))
                    }
                    _ => return Ok(expr),
                },
            }
        }
    }

    fn parse_factor(input: &mut Lexer) -> ExprResult {
        let mut expr = parse_primary(input)?;
        loop {
            match input.peek() {
                None => return Ok(expr),
                Some(Ok((_, Caret))) => {
                    input.next();
                    expr = Binary(Pow, Box::new(expr), Box::new(parse_factor(input)?))
                }
                _ => return Ok(expr),
            }
        }
    }

    fn parse_primary(input: &mut Lexer) -> ExprResult {
        match input.next() {
            None => Err(CalcErr::Incomplete),
            Some(x) => match x? {
                (_, Number(n)) => Ok(Num(n)),
                (_, LParen) => parse_parenthesised(input),
                (_, Dash) => Ok(Unary(Neg, Box::new(parse_factor(input)?))),
                (pos, _) => Err(CalcErr::Lex((pos, UNEXPECTED_TOKEN))),
            },
        }
    }

    fn parse_parenthesised(input: &mut Lexer) -> ExprResult {
        let expr = parse_expr(input)?;
        if let Some(x) = input.next() {
            if let (_, RParen) = x? {
                return Ok(expr);
            }
        }
        Err(CalcErr::Incomplete)
    }

    pub(super) fn parse(input: &str) -> ExprResult {
        parse_complete_expr(&mut lex::Lexer::new(input).peekable())
    }
}

pub fn eval(input: &str) -> Result<f64, CalcErr> {
    Ok(recursive_descent_parse::parse(input)?.eval())
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn num_is_parsed() {
        assert_eq!(1.0, eval("1.0").unwrap());
    }

    #[test]
    pub fn mul() {
        assert_eq!(15.0, eval("3 * 5").unwrap());
    }

    #[test]
    pub fn modulus() {
        assert_eq!(1.0, eval("1 % 2").unwrap());
        assert_eq!(0.0, eval("4 % 2").unwrap());
        assert_eq!(2.0, eval("8 % 3").unwrap());
        assert_eq!(4.0, eval("11 % 7").unwrap());
        assert_eq!(2.0, eval("8 % 3").unwrap());
    }

    #[test]
    pub fn add() {
        assert_eq!(9.0, eval("2 + 7").unwrap());
    }

    #[test]
    pub fn sub() {
        assert_eq!(10.0, eval("11 - 1").unwrap());
    }

    #[test]
    pub fn pow() {
        assert_eq!(25.0, eval("5^2").unwrap());
        assert_eq!(3.0, eval("9^0.5").unwrap());
    }

    #[test]
    pub fn double_neg() {
        assert_eq!(2.0, eval("1--1").unwrap());
    }

    #[test]
    pub fn chained_add() {
        assert_eq!(3.0, eval("1+1+1").unwrap());
    }

    #[test]
    pub fn paren_before_add() {
        assert_eq!(0.3125, eval("(1+0.25)*0.25").unwrap());
        assert_eq!(0.66, eval("1.2*(0.3+0.25)").unwrap());
    }

    #[test]
    pub fn paren_before_mul() {
        assert_eq!(12.0, eval("2 * (5 + 1)").unwrap());
        assert_eq!(11.0, eval("(2 * 5) + 1").unwrap());
    }

    #[test]
    pub fn mul_before_add() {
        assert_eq!(14.0, eval("4 * 3 + 2").unwrap());
        assert_eq!(14.0, eval("2 + 4 * 3").unwrap());
    }

    #[test]
    pub fn neg_before_add() {
        assert_eq!(0.0, eval("-5 + 5").unwrap());
        assert_eq!(-10.0, eval("-(5 + 5)").unwrap());
    }

    #[test]
    pub fn pow_before_all() {
        assert_eq!(-25.0, eval("-5^2").unwrap());
        assert_eq!(37.0, eval("6^2+1").unwrap());
        assert_eq!(200.0, eval("2*10^2").unwrap());
        assert_eq!(2.0, eval("2^2/2").unwrap());
    }
    #[test]
    pub fn is_left_associative() {
        assert_eq!(1.0, eval("5 * 2 % 3").unwrap());
        assert_eq!(9.0, eval("6 / 2 * 3").unwrap())
    }

    #[test]
    pub fn unexpected_token_is_rejected() {
        assert_eq!(Err(CalcErr::Lex((7, UNEXPECTED_TOKEN))), eval("1 - 5 */ 5"));
        assert_eq!(Err(CalcErr::Lex((1, UNEXPECTED_TOKEN))), eval("2()"));
        assert_eq!(Err(CalcErr::Lex((3, UNEXPECTED_TOKEN))), eval("2*()"));
    }

    #[test]
    pub fn incomplete_expr_is_identified() {
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * "));
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * ("));
        assert_eq!(Err(CalcErr::Incomplete), eval("2 * (5+2"));
        assert_eq!(Ok(14.0), eval("2 * (5+2)"));
    }

    #[test]
    pub fn unknown_symbol_is_rejected() {
        assert_eq!(Err(CalcErr::Lex((4, lex::UNKNOWN_SYMBOL))), eval("2 * &"));
        assert_eq!(Err(CalcErr::Lex((6, lex::UNKNOWN_SYMBOL))), eval("2 * (1a"));
    }
}
