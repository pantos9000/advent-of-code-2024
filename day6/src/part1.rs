#![allow(unused)]

use std::collections::HashMap;

use crate::plane::*;

pub fn run(input: &str) -> usize {
    let plane: Plane<Spot> = Plane::parse(input);
    let mut guard = Guard::from_plane(&plane).unwrap();
    let mut num = 0;
    loop {
        if guard.poop(&plane).is_ok() {
            num += 1;
        }
        if guard.walk(&plane).is_err() {
            break;
        }
    }
    num
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot {
    Obstacle,
    Free,
    Start(Direction),
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Obstacle,
            '.' => Self::Free,
            '^' => Self::Start(Direction::Up),
            '<' => Self::Start(Direction::Left),
            '>' => Self::Start(Direction::Right),
            'v' => Self::Start(Direction::Down),
            _ => panic!("unknown char {c}"),
        }
    }
}

impl Spot {
    fn is_obstacle(self) -> bool {
        match self {
            Spot::Obstacle => true,
            Spot::Free => false,
            Spot::Start(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoopError {
    DifferentDirection,
    SameDirection,
}

pub struct Guard {
    position: Coords,
    direction: Direction,
    pooped: HashMap<Coords, Direction>,
}

impl Guard {
    pub fn from_plane(plane: &Plane<Spot>) -> Option<Self> {
        for position in plane.iter_coords() {
            let direction = match plane.get(position) {
                None => continue,
                Some(Spot::Start(direction)) => *direction,
                Some(_) => continue,
            };
            return Some(Self {
                position,
                direction,
                pooped: HashMap::new(),
            });
        }
        None
    }

    /// returns Ok if spot was free, else an error with info if the direction was faced before
    pub fn poop(&mut self, plane: &Plane<Spot>) -> Result<(), PoopError> {
        if let Some(direction) = self.pooped.get(&self.position) {
            if *direction == self.direction {
                return Err(PoopError::SameDirection);
            }
            return Err(PoopError::DifferentDirection);
        }

        self.pooped.insert(self.position, self.direction);
        Ok(())
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.rot_right().rot_right()
    }

    /// returns `None` when end is reached
    pub fn walk(&mut self, plane: &Plane<Spot>) -> Result<(), ()> {
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

    pub fn iter_pooped(&self) -> impl Iterator<Item = (Coords, Direction)> + use<'_> {
        self.pooped.iter().map(|(c, d)| (*c, *d))
    }

    pub fn into_iter_pooped(self) -> impl Iterator<Item = (Coords, Direction)> {
        self.pooped.into_iter()
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
