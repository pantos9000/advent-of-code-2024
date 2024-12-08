use std::{cell::Cell, rc::Rc};

use chumsky::prelude::*;

use crate::part1::Expr;

pub fn run(input: &str) -> i32 {
    parser()
        .parse(input)
        .unwrap()
        .unwrap_or(Expr::Num(0))
        .eval()
}

fn parser() -> impl Parser<char, Option<Expr>, Error = Simple<char>> {
    let parse_crap = any().ignored().map(|_| None);
    let parse_int = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));

    let ignore = Rc::new(Cell::new(false));
    let ignore_start = Rc::clone(&ignore);
    let ignore_end = Rc::clone(&ignore);

    let parse_ignore_start = just("don't()").ignored().map(move |_| {
        ignore_start.set(true);
        None
    });

    let parse_ignore_end = just("do()").ignored().map(move |_| {
        ignore_end.set(false);
        None
    });

    let parse_mul = just("mul(")
        .ignore_then(parse_int)
        .then_ignore(just(","))
        .then(parse_int)
        .then_ignore(just(")"))
        .map(move |(a, b)| {
            if ignore.get() {
                None
            } else {
                Some(Expr::Mul(Box::new(a), Box::new(b)))
            }
        });

    let parse_some = parse_ignore_start
        .or(parse_ignore_end)
        .or(parse_mul)
        .or(parse_crap);

    let fold_func = move |a, b| match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(Expr::Add(Box::new(a), Box::new(b))),
    };

    parse_some
        .clone()
        .then(parse_some.repeated())
        .foldl(fold_func)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn easy_test() {
        assert_eq!(0, run("asdon't()mul(2,3)"));
    }

    #[test]
    fn test_example() {
        assert_eq!(48, run(EXAMPLE));
    }
}
