use crate::plane::{self, Plane};

pub fn run(input: &str) -> usize {
    let plane = Plane::parse(input);

    plane
        .iter_coords()
        .filter(|coords| is_x(&plane, *coords))
        .count()
}

fn is_x(plane: &Plane<char>, coords: plane::Coords) -> bool {
    // first check 'A' in the middle
    let Some(contained) = plane.get(coords) else {
        return false;
    };
    if *contained != 'A' {
        return false;
    }

    // now get the surrounding chars
    let directions = [
        plane::Direction::UpLeft,
        plane::Direction::UpRight,
        plane::Direction::DownRight,
        plane::Direction::DownLeft,
    ];
    let x: String = directions
        .into_iter()
        .filter_map(|direction| coords.move_into_direction(direction))
        .filter_map(|coords| plane.get(coords))
        .collect();
    if x.len() < 4 {
        return false;
    }

    // now check the surrounding chars
    matches!(x.as_ref(), "MMSS" | "SMMS" | "SSMM" | "MSSM")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
            .M.S...... \n\
            ..A..MSMS. \n\
            .M.S.MAA.. \n\
            ..A.ASMSM. \n\
            .M.S.M.... \n\
            .......... \n\
            S.S.S.S.S. \n\
            .A.A.A.A.. \n\
            M.M.M.M.M. \n\
            .......... \n\
            ";
        assert_eq!(9, run(example));
    }
}
