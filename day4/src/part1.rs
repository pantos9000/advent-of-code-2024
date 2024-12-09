use strum::IntoEnumIterator;

use crate::plane::{self, Plane};

pub fn run(input: &str) -> usize {
    let plane = Plane::parse(input);

    plane
        .iter_coords()
        .flat_map(|coords| plane::Direction::iter().map(move |direction| (coords, direction)))
        .filter(|(coords, direction)| has_str(&plane, "XMAS", *coords, *direction))
        .count()
}

fn has_str(
    plane: &Plane<char>,
    s: &str,
    coords: plane::Coords,
    direction: plane::Direction,
) -> bool {
    let mut coords = coords;
    let mut chars_iter = s.chars();

    let c = chars_iter.next().unwrap();
    let Some(contained) = plane.get(coords) else {
        return false;
    };
    if *contained != c {
        return false;
    }

    for c in chars_iter {
        let Some(new_coords) = coords.move_into_direction(direction) else {
            return false;
        };
        coords = new_coords;
        let Some(contained) = plane.get(coords) else {
            return false;
        };
        if *contained != c {
            return false;
        }
    }

    true
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

    #[test]
    fn test_3() {
        let example = "SAMX";
        assert_eq!(1, run(example));
    }
}
