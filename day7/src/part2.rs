use crate::part1::Equation;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .filter_map(Equation::parse)
        .filter(|equation| equation.can_be_true_part2())
        .map(|equation| equation.result())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20\n\
        ";
        assert_eq!(11387, run(input));
    }
}
