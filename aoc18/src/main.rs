use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::{all_consuming, cut, map, map_res},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Term {
    Number(isize),
    Add,
    Multiply,
    SubExpr(Expr),
}

impl Term {
    fn solve(&self) -> isize {
        match self {
            Term::Number(i) => *i,
            Term::SubExpr(e) => e.solve(),
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Term {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or("Can't parse empty")? {
            '*' => Ok(Term::Multiply),
            '+' => Ok(Term::Add),
            '(' => unimplemented!(),
            _ => Ok(Term::Number(s.parse().map_err(|_| "Illegal number")?)),
        }
    }
}

fn parse_integer(i: &str) -> IResult<&str, Term> {
    alt((
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<isize>().map(Term::Number)
        }),
        map(preceded(char('-'), digit1), |digit_str: &str| {
            Term::Number(-digit_str.parse::<isize>().unwrap())
        }),
    ))(i)
}

fn parse_op(i: &str) -> IResult<&str, Term> {
    alt((
        map(tag("+"), |_| Term::Add),
        map(tag("*"), |_| Term::Multiply),
    ))(i)
}

fn parse_list(i: &str) -> IResult<&str, Term> {
    delimited(
        char('('),
        map(parse_expr, Term::SubExpr),
        cut(preceded(space0, char(')'))),
    )(i)
}

fn parse_term(i: &str) -> IResult<&str, Term> {
    let p = alt((parse_integer, parse_op, parse_list));

    delimited(space0, p, space0)(i)
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    map(many1(parse_term), Expr)(i)
}

#[derive(Debug, PartialEq, Eq)]
struct Expr(Vec<Term>);

impl FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_expr)(s) {
            Ok((_, expr)) => Ok(expr),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Expr {
    fn solve(&self) -> isize {
        let mut terms = self.0.iter();

        let mut acc = terms.next().map(|t| t.solve()).unwrap_or(0);

        while let Some(op) = terms.next() {
            if let Some(term) = terms.next() {
                let number = term.solve();
                match op {
                    Term::Add => acc += number,
                    Term::Multiply => acc *= number,
                    _ => panic!("Illegal sequence of terms"),
                }
            } else {
                panic!("Operator with no term after")
            }
        }
        acc
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let expr: Expr = line.parse().unwrap();
        sum += expr.solve();
    }
    println!("Total sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Term::*;
        let input = "2 * 13 + 1";
        let parsed: Expr = input.parse().unwrap();
        assert_eq!(
            parsed,
            Expr(vec![Number(2), Multiply, Number(13), Add, Number(1)])
        );

        let input = "2 * (13 + 1)";
        let parsed: Expr = input.parse().unwrap();
        assert_eq!(
            parsed,
            Expr(vec![
                Number(2),
                Multiply,
                SubExpr(Expr(vec![Number(13), Add, Number(1)]))
            ])
        );

        let input = "(1 + 2) * (3 + (4 * 5))";
        let parsed: Expr = input.parse().unwrap();
        assert_eq!(
            parsed,
            Expr(vec![
                SubExpr(Expr(vec![Number(1), Add, Number(2)])),
                Multiply,
                SubExpr(Expr(vec![
                    Number(3),
                    Add,
                    SubExpr(Expr(vec![Number(4), Multiply, Number(5)]))
                ]))
            ])
        );
    }

    #[test]
    fn test_solve_expression() {
        let input = "2 * 3 + (4 * 5)";
        let expr: Expr = input.parse().unwrap();
        assert_eq!(expr.solve(), 26);

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let expr: Expr = input.parse().unwrap();
        assert_eq!(expr.solve(), 437);

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let expr: Expr = input.parse().unwrap();
        assert_eq!(expr.solve(), 13632);
    }
}
