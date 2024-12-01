use std::collections::HashMap;

pub fn run(input: &str) -> i64 {
    fn get_num<const N: usize>(line: &str) -> i64 {
        line.split_whitespace()
            .filter(|x| !x.is_empty())
            .nth(N)
            .unwrap()
            .parse()
            .unwrap()
    }

    let mut map: HashMap<i64, (i64, i64)> = HashMap::new();

    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(get_num::<0>)
        .for_each(|num| map.entry(num).or_default().0 += 1);
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(get_num::<1>)
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
