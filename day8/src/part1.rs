#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::plane::*;

pub fn run(input: &str) -> usize {
    let plane: Plane<Spot> = Plane::parse(input);

    let mut antennas: HashMap<Spot, Vec<Coords>> =
        HashMap::with_capacity(plane.width() * plane.height());

    for (coords, spot) in plane.iter().filter(|(_, &spot)| spot != Spot::Empty) {
        antennas.entry(*spot).or_default().push(coords);
    }

    let antinodes: HashSet<Coords> = antennas
        .into_values()
        .map(|v| v.into_iter())
        .flat_map(combinations)
        .flat_map(|(c1, c2)| c1.move_by_diff(c1.diff(c2)))
        .collect();

    antinodes
        .into_iter()
        .filter_map(|coord| plane.get(coord).copied())
        .count()
}

fn combinations<I>(coords: I) -> impl Iterator<Item = (Coords, Coords)>
where
    I: Iterator<Item = Coords> + Clone,
{
    coords
        .clone()
        .flat_map(move |c1| coords.clone().map(move |c2| (c1, c2)))
        .filter(|(c1, c2)| c1 != c2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spot {
    Antenna(char),
    Empty,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            ' ' => panic!("no spaces allowed"),
            '.' => Self::Empty,
            '#' => Self::Empty,
            '0'..='9' => Self::Antenna(c),
            'a'..='z' => Self::Antenna(c),
            'A'..='Z' => Self::Antenna(c),
            _ => panic!("Unknown char {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let c1 = Coords::new(1, 1);
        let c2 = Coords::new(2, 2);
        let c3 = Coords::new(3, 3);
        let coords = vec![c1, c2, c3];
        let mut combis = combinations(coords.into_iter());
        assert_eq!(combis.next(), Some((c1, c2)));
        assert_eq!(combis.next(), Some((c1, c3)));
        assert_eq!(combis.next(), Some((c2, c1)));
        assert_eq!(combis.next(), Some((c2, c3)));
        assert_eq!(combis.next(), Some((c3, c1)));
        assert_eq!(combis.next(), Some((c3, c2)));
        assert!(combis.next().is_none());
    }

    #[test]
    fn test_example() {
        let input = "\
            ......#....#\n\
            ...#....0...\n\
            ....#0....#.\n\
            ..#....0....\n\
            ....0....#..\n\
            .#....A.....\n\
            ...#........\n\
            #......#....\n\
            ........A...\n\
            .........A..\n\
            ..........#.\n\
            ..........#.\n\
        ";
        assert_eq!(14, run(input));
    }
}
