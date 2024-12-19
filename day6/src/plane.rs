#![allow(dead_code)]

pub struct Plane<T>(Vec<Vec<T>>);

impl<T> Plane<T>
where
    T: From<char>,
{
    pub fn parse(input: &str) -> Self {
        let parse_line =
            |line: &str| -> Vec<_> { line.trim().chars().map(|c| T::from(c)).collect() };
        let inner = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(parse_line)
            .collect();
        Self(inner)
    }

    pub fn width(&self) -> usize {
        self.0.first().map(|x| x.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, coords: Coords) -> Option<&T> {
        self.0.get(coords.y)?.get(coords.x)
    }

    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        self.0.get_mut(coords.y)?.get_mut(coords.x)
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coords> + use<'_, T> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, vx)| vx.iter().enumerate().map(move |(x, _)| Coords::new(x, y)))
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flat_map(|vx| vx.iter())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coords {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_up(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }

    fn move_down(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_add(1)?,
        })
    }

    fn move_left(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }

    fn move_right(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(1)?,
            y: self.y,
        })
    }

    pub fn move_into_direction(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.move_up(),
            Direction::UpRight => self.move_up()?.move_right(),
            Direction::Right => self.move_right(),
            Direction::DownRight => self.move_down()?.move_right(),
            Direction::Down => self.move_down(),
            Direction::DownLeft => self.move_down()?.move_left(),
            Direction::Left => self.move_left(),
            Direction::UpLeft => self.move_up()?.move_left(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        self.opposite() == other
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::UpRight => Self::DownLeft,
            Self::Right => Self::Left,
            Self::DownRight => Self::UpLeft,
            Self::Down => Self::Up,
            Self::DownLeft => Self::UpRight,
            Self::Left => Self::Right,
            Self::UpLeft => Self::DownRight,
        }
    }

    pub fn rot_left(&self) -> Self {
        match self {
            Self::Up => Self::UpLeft,
            Self::UpRight => Self::Up,
            Self::Right => Self::UpRight,
            Self::DownRight => Self::Right,
            Self::Down => Self::DownRight,
            Self::DownLeft => Self::Down,
            Self::Left => Self::DownLeft,
            Self::UpLeft => Self::Left,
        }
    }

    pub fn rot_right(&self) -> Self {
        match self {
            Self::Up => Self::UpRight,
            Self::UpRight => Self::Right,
            Self::Right => Self::DownRight,
            Self::DownRight => Self::Down,
            Self::Down => Self::DownLeft,
            Self::DownLeft => Self::Left,
            Self::Left => Self::UpLeft,
            Self::UpLeft => Self::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_size() {
        let input = "abc\n123";
        let plane: Plane<char> = Plane::parse(input);
        assert_eq!(plane.width(), 3);
        assert_eq!(plane.height(), 2);
    }

    #[test]
    fn test_iter_coords() {
        let input = "ab\n12";
        let expected: Vec<Coords> =
            vec![(0, 0).into(), (1, 0).into(), (0, 1).into(), (1, 1).into()];
        let plane: Plane<char> = Plane::parse(input);
        let coords: Vec<_> = plane.iter_coords().collect();
        assert_eq!(coords, expected);
    }

    #[test]
    fn test_iter_entries() {
        let input = "ab\n12";
        let expected: Vec<&char> = vec![&'a', &'b', &'1', &'2'];
        let plane: Plane<char> = Plane::parse(input);
        let chars: Vec<_> = plane.iter_entries().collect();
        assert_eq!(chars, expected);
    }

    #[test]
    fn test_get_miss() {
        let input = "ab\n12";
        let plane: Plane<char> = Plane::parse(input);
        let coords_x = Coords::new(2, 0);
        let coords_y = Coords::new(0, 2);
        assert!(plane.get(coords_x).is_none());
        assert!(plane.get(coords_y).is_none());
    }

    #[test]
    fn test_get_hit() {
        let input = "ab\n12";
        let plane: Plane<char> = Plane::parse(input);
        let coords_x = Coords::new(1, 0);
        let coords_y = Coords::new(0, 1);
        assert_eq!(plane.get(coords_x).unwrap(), &'b');
        assert_eq!(plane.get(coords_y).unwrap(), &'1');
    }
}
