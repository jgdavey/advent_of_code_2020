use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::{all_consuming, cut, map, map_res},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Term {
    Number(isize),
    Add,
    Multiply,
    SubExpr(Expr),
}

impl Term {
    fn solve_simple(&self) -> isize {
        match self {
            Term::Number(i) => *i,
            Term::SubExpr(e) => e.solve_simple(),
            _ => unimplemented!(),
        }
    }

    fn addition_precedence(&self) -> Self {
        match self {
            Term::SubExpr(expr) => Term::SubExpr(expr.addition_precedence()),
            e => e.clone(),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Term::*;
        match self {
            Number(n) => write!(f, "{}", n),
            Add => write!(f, "+"),
            Multiply => write!(f, "*"),
            SubExpr(e) => write!(f, "({})", e),
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

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, term) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", term)?;
            } else {
                write!(f, " {}", term)?;
            }
        }
        Ok(())
    }
}

impl Expr {
    fn solve_simple(&self) -> isize {
        let mut terms = self.0.iter();

        let mut acc = terms.next().map(|t| t.solve_simple()).unwrap_or(0);

        while let Some(op) = terms.next() {
            if let Some(term) = terms.next() {
                let number = term.solve_simple();
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

    fn addition_precedence(&self) -> Expr {
        let mut exprs = vec![];
        let mut i = 0;
        while i < self.0.len() {
            match &self.0[i] {
                Term::Add => {
                    let prev = exprs.pop().unwrap();
                    let add = Term::SubExpr(Expr(vec![
                        prev,
                        Term::Add,
                        self.0[i + 1].addition_precedence(),
                    ]));
                    exprs.push(add);
                    i += 1;
                }
                expr => exprs.push(expr.addition_precedence()),
            }
            i += 1;
        }
        Expr(exprs)
    }

    fn solve_advanced(&self) -> isize {
        let rewritten = self.addition_precedence();
        rewritten.solve_simple()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let expr: Expr = line.parse().unwrap();
        sum += expr.solve_simple();
    }
    println!("Part 1 total sum: {}", sum);

    let mut sum = 0;
    for line in input.lines() {
        let expr: Expr = line.parse().unwrap();
        sum += expr.solve_advanced();
    }
    println!("Part 2 total sum: {}", sum);
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
        assert_eq!(expr.solve_simple(), 26);

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let expr: Expr = input.parse().unwrap();
        assert_eq!(expr.solve_simple(), 437);

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let expr: Expr = input.parse().unwrap();
        assert_eq!(expr.solve_simple(), 13632);
    }

    #[test]
    fn test_solve_advanced() {
        let expr: Expr = "2 * 3 + (4 * 5)".parse().unwrap();
        assert_eq!(expr.solve_advanced(), 46);

        let expr: Expr = "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse().unwrap();
        assert_eq!(expr.solve_advanced(), 1445);

        let expr: Expr = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".parse().unwrap();
        assert_eq!(expr.solve_advanced(), 669060);
        //((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
    }

    #[test]
    fn test_addition_rewrite() {
        let expr: Expr = "2 * 3 + 4 * 5".parse().unwrap();
        assert_eq!(format!("{}", expr.addition_precedence()), "2 * (3 + 4) * 5");

        let expr: Expr = "2 * 3 + (4 * 5)".parse().unwrap();
        assert_eq!(
            format!("{}", expr.addition_precedence()),
            "2 * (3 + (4 * 5))"
        );

        let expr: Expr = "2 * 3 + (4 * 6 + 5)".parse().unwrap();
        assert_eq!(
            format!("{}", expr.addition_precedence()),
            "2 * (3 + (4 * (6 + 5)))"
        );
    }
}
