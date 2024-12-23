#![allow(unused)]

use crate::plane::*;

pub fn run(input: &str) -> usize {
    let mut plane: Plane<Spot> = Plane::parse(input);
    let mut guard = Guard::from_plane(&plane).unwrap();
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
pub enum Spot {
    Obstacle,
    Free,
    Pooped,
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
            Spot::Pooped => false,
            Spot::Start(_) => false,
        }
    }
}

pub struct Guard {
    position: Coords,
    direction: Direction,
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
            });
        }
        None
    }

    pub fn poop(&self, plane: &mut Plane<Spot>) -> Result<(), ()> {
        let spot = plane.get_mut(self.position).unwrap();
        let result = match spot {
            Spot::Obstacle => unreachable!(),
            Spot::Free => Ok(()),
            Spot::Start(_) => Ok(()),
            Spot::Pooped => Err(()),
        };
        *spot = Spot::Pooped;
        result
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
