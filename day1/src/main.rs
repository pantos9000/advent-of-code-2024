mod part1;
mod part2;

pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn main() {
    let input = read_input();
    let result = part2::run(&input);
    println!("Result is {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input();
        assert_eq!(2086478, part1::run(&input));
    }
}
