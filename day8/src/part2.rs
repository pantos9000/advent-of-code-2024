#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::part1::*;
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
        .flat_map(|(c1, c2)| echo(plane.width(), plane.height(), c1, c2))
        .collect();

    antinodes
        .into_iter()
        .filter_map(|coord| plane.get(coord).copied())
        .count()
}

fn echo(width: usize, height: usize, c1: Coords, c2: Coords) -> impl Iterator<Item = Coords> {
    let diff = c2.diff(c1);
    let mut next_coord = Some(c1);
    let next = move || -> Option<Coords> {
        let old = next_coord?;
        next_coord = old.move_by_diff(diff);
        if let Some(c) = next_coord {
            if c.x >= width || c.y >= height {
                next_coord = None;
            }
        }
        next_coord
    };
    std::iter::repeat_with(next)
        .take_while(Option::is_some)
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let c1 = Coords::new(1, 2);
        let c2 = Coords::new(2, 4);
        let mut it = echo(6, 10, c1, c2);
        assert_eq!(it.next(), Some(Coords { x: 2, y: 4 }));
        assert_eq!(it.next(), Some(Coords { x: 3, y: 6 }));
        assert_eq!(it.next(), Some(Coords { x: 4, y: 8 }));
        assert!(it.next().is_none());
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
        assert_eq!(34, run(input));
    }
}
