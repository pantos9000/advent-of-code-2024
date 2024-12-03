#[allow(dead_code)]
pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(Report::from_line)
        .filter(|report| report.is_safe())
        .count()
}

struct Report(Vec<i32>);

impl Report {
    fn from_line(line: &str) -> Self {
        let x = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Self(x)
    }

    fn is_safe(&self) -> bool {
        if self.0.len() < 2 {
            return false;
        }
        let sign = (self.0[0] - self.0[1]).signum();
        for diff in self.0.windows(2).map(|w| w[0] - w[1]) {
            if diff.signum() != sign {
                return false;
            }
            if !(1..=3).contains(&diff.abs()) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#;

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 2);
    }
}
