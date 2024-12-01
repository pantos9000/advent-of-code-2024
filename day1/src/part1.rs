pub fn run(input: &str) -> i64 {
    fn get_num<const N: usize>(line: &str) -> i64 {
        line.split_whitespace()
            .filter(|x| !x.is_empty())
            .nth(N)
            .unwrap()
            .parse()
            .unwrap()
    }

    let mut list1: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(get_num::<0>)
        .collect();
    let mut list2: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(get_num::<1>)
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
