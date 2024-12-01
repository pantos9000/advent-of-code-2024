use std::collections::HashMap;

pub fn run(input: &str) -> i64 {
    let mut map: HashMap<i64, (i64, i64)> = HashMap::new();

    input
        .split_whitespace()
        .step_by(2)
        .map(|num| num.parse().unwrap())
        .for_each(|num| map.entry(num).or_default().0 += 1);
    input
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|num| num.parse().unwrap())
        .for_each(|num| map.entry(num).or_default().1 += 1);

    map.into_iter()
        .map(|(num, (count1, count2))| num * count1 * count2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 31);
    }
}
