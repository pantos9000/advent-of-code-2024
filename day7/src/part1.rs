#![allow(dead_code)]

pub fn run(input: &str) -> usize {
    input
        .lines()
        .filter_map(Equation::parse)
        .filter(|equation| equation.can_be_true())
        .map(|equation| equation.result())
        .sum()
}

#[derive(Debug, Clone)]
pub struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    pub fn parse(line: &str) -> Option<Self> {
        let (result, numbers) = line.trim().split_once(':')?;
        let result = result.parse().unwrap();
        let numbers = numbers
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        Some(Self { result, numbers })
    }

    pub fn result(&self) -> usize {
        self.result
    }

    pub fn numbers(&self) -> &[usize] {
        &self.numbers
    }

    pub fn can_be_true(&self) -> bool {
        let mut results = vec![0];
        for number in &self.numbers {
            let mut new_results = Vec::new();
            for result in results {
                let plus = result + number;
                let mult = result * number;
                if plus <= self.result {
                    new_results.push(plus);
                }
                if mult <= self.result {
                    new_results.push(mult);
                }
            }
            results = new_results;
        }
        results.contains(&self.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_parse() {
        let line = " 3267: 81 40 27";
        let equation = Equation::parse(line).unwrap();
        assert_eq!(equation.result, 3267);
        assert_eq!(equation.numbers, &[81, 40, 27]);
    }

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
        assert_eq!(3749, run(input));
    }
}
