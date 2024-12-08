use chumsky::prelude::*;

pub fn run(input: &str) -> i32 {
    parser()
        .parse(input)
        .unwrap()
        .unwrap_or(Expr::Num(0))
        .eval()
}

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Mul(x, y) => x.eval() * y.eval(),
            Expr::Num(x) => *x,
            Expr::Add(x, y) => x.eval() + y.eval(),
        }
    }
}

fn parser() -> impl Parser<char, Option<Expr>, Error = Simple<char>> {
    let parse_crap = any().ignored().map(|_| None);
    let parse_int = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));

    let parse_mul = parse_int
        .then_ignore(just(','))
        .then(parse_int)
        .delimited_by(just("mul("), just(')'))
        .map(|(a, b)| Some(Expr::Mul(Box::new(a), Box::new(b))));

    let parse_some = parse_mul.or(parse_crap);

    let fold_func = |a, b| match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(Expr::Add(Box::new(a), Box::new(b))),
    };

    parse_some.then(parse_some.repeated()).foldl(fold_func)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    #[test]
    fn test_part1() {
        assert_eq!(161, run(EXAMPLE));
    }

    #[test]
    fn test_good() {
        assert_eq!(44 * 46, run("mul(44,46)"));
        assert_eq!(123 * 4, run("mul(123,4)"));
    }

    #[test]
    fn test_bad() {
        assert_eq!(0, run("mul(4*"));
        assert_eq!(0, run("mul(6,9!"));
        assert_eq!(0, run("?(12,34)"));
        assert_eq!(0, run("mul ( 2 , 4 )"));
    }
}
