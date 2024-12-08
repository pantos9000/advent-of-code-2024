pub fn run(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let example = r#"
            ..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....
        "#;
        assert_eq!(4, run(example));
    }

    #[test]
    fn test_2() {
        let example = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#;
        assert_eq!(18, run(example));
    }
}
