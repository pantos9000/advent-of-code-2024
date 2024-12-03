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

    fn dampened(&self) -> impl Iterator<Item = DampenedReport> {
        (0..self.0.len()).map(|i| DampenedReport::new(&self.0, i))
    }

    fn is_safe(&self) -> bool {
        for dampened_report in self.dampened() {
            if dampened_report.is_safe() {
                return true;
            }
        }
        false
    }
}

struct DampenedReport<'a> {
    report: &'a [i32],
    ignored_index: usize,
}

impl<'a> DampenedReport<'a> {
    fn new(report: &'a [i32], ignored_index: usize) -> Self {
        Self {
            report,
            ignored_index,
        }
    }

    fn is_safe(&self) -> bool {
        if self.report.len() < 3 {
            return false;
        }

        let sign = match self.ignored_index {
            0 => self.report[1] - self.report[2],
            1 => self.report[0] - self.report[2],
            _ => self.report[0] - self.report[1],
        }
        .signum();

        use itertools::Itertools;
        for diff in self
            .report
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if i == self.ignored_index {
                    None
                } else {
                    Some(val)
                }
            })
            .tuple_windows()
            .map(|(x, y)| x - y)
        {
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
        assert_eq!(run(EXAMPLE), 4);
    }
}
