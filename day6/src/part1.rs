#![allow(unused)]

use crate::plane::*;

pub fn run(input: &str) -> usize {
    let mut plane: Plane<Spot> = Plane::parse(input);
    let mut guard = Guard::from_input(input).unwrap();
    let mut num = 0;
    loop {
        if guard.poop(&mut plane).is_ok() {
            num += 1;
        }
        if guard.walk(&plane).is_err() {
            break;
        }
    }
    num
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    Obstacle,
    Free,
    Pooped,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Obstacle,
            '.' => Self::Free,
            '^' => Self::Free,
            '<' => Self::Free,
            '>' => Self::Free,
            'v' => Self::Free,
            _ => panic!("unknown char {c}"),
        }
    }
}

impl Spot {
    fn is_obstacle(self) -> bool {
        match self {
            Spot::Obstacle => true,
            Spot::Free => false,
            Spot::Pooped => false,
        }
    }
}

struct Guard {
    position: Coords,
    direction: Direction,
}

impl Guard {
    fn from_input(input: &str) -> Option<Self> {
        let filter_guard = |(x, y, c): (usize, usize, char)| -> Option<Self> {
            let direction = match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => return None,
            };
            let position = Coords::new(x, y);
            let guard = Self {
                position,
                direction,
            };

            Some(guard)
        };

        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
            .filter_map(filter_guard)
            .next()
    }

    fn poop(&self, plane: &mut Plane<Spot>) -> Result<(), ()> {
        let spot = plane.get_mut(self.position).unwrap();
        let result = match spot {
            Spot::Obstacle => unreachable!(),
            Spot::Free => Ok(()),
            Spot::Pooped => Err(()),
        };
        *spot = Spot::Pooped;
        result
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.rot_right().rot_right()
    }

    /// returns `None` when end is reached
    fn walk(&mut self, plane: &Plane<Spot>) -> Result<(), ()> {
        let next_pos = self
            .position
            .move_into_direction(self.direction)
            .ok_or(())?;
        let next_spot = plane.get(next_pos).ok_or(())?;
        if next_spot.is_obstacle() {
            self.turn_right();
            self.walk(plane)
        } else {
            self.position = next_pos;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\n\
        ";
        assert_eq!(run(input), 41);
    }
}
