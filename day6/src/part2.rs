use crate::part1::{Guard, PoopError, Spot};
use crate::plane::*;

pub fn run(input: &str) -> usize {
    let plane: Plane<Spot> = Plane::parse(input);
    let original_path_coords = original_path_coords(&plane);
    plane_possibilities(plane, original_path_coords.into_iter())
        .filter(contains_loop)
        .count()
}

fn contains_loop(plane: &Plane<Spot>) -> bool {
    let mut guard = Guard::from_plane(plane).unwrap();

    loop {
        if guard.walk(plane).is_err() {
            return false;
        }
        match guard.poop(plane) {
            Ok(()) => continue,
            Err(PoopError::DifferentDirection) => continue,
            Err(PoopError::SameDirection) => return true,
        }
    }
}

fn plane_possibilities(
    plane: Plane<Spot>,
    original_path: impl Iterator<Item = Coords>,
) -> impl Iterator<Item = Plane<Spot>> {
    let create_new_plane = move |coords| {
        let mut new_plane = plane.clone();
        let spot = new_plane.get_mut(coords).unwrap();
        if spot == &Spot::Free {
            *spot = Spot::Obstacle;
        }
        new_plane
    };
    original_path.map(create_new_plane)
}

fn original_path_coords(plane: &Plane<Spot>) -> impl Iterator<Item = Coords> {
    let mut guard = Guard::from_plane(plane).unwrap();
    loop {
        if guard.walk(plane).is_err() {
            break;
        }
        let _ = guard.poop(plane);
    }

    guard.into_iter_pooped().map(|(coords, _)| coords)
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
        assert_eq!(run(input), 6);
    }
}
