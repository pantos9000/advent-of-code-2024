#[allow(dead_code)]
pub fn run(input: &str) -> i64 {
    let mut list1: Vec<i64> = input
        .split_whitespace()
        .step_by(2)
        .map(|num| num.parse().unwrap())
        .collect();
    let mut list2: Vec<i64> = input
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|num| num.parse().unwrap())
        .collect();

    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2)
        .map(|(first, second)| first - second)
        .map(|diff| diff.abs())
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
        assert_eq!(run(EXAMPLE), 11);
    }
}
