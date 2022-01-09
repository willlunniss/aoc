use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, EnumString)]
#[allow(non_camel_case_types)]
enum Direction {
    n,
    ne,
    se,
    s,
    sw,
    nw,
}

/// Use cub coordinates to represent hex grid
#[derive(Debug)]
struct CubeCoordinate {
    q: isize,
    r: isize,
    s: isize,
}

impl From<&Direction> for CubeCoordinate {
    fn from(d: &Direction) -> Self {
        use Direction::{n, ne, nw, s, se, sw};
        match d {
            n => Self::new(0, -1, 1),
            ne => Self::new(1, -1, 0),
            se => Self::new(1, 0, -1),
            s => Self::new(0, 1, -1),
            sw => Self::new(-1, 1, 0),
            nw => Self::new(-1, 0, 1),
        }
    }
}

impl CubeCoordinate {
    const fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    const fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    /// Move a single step in the supplied direction
    fn step(&self, direction: &Direction) -> Self {
        let shift: Self = direction.into();
        Self {
            q: self.q + shift.q,
            r: self.r + shift.r,
            s: self.s + shift.s,
        }
    }

    /// Returns the manhattan distance to the origin
    const fn manhattan_distance(&self) -> isize {
        (isize::abs(self.q) + isize::abs(self.r) + isize::abs(self.s)) / 2
    }
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Direction> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Direction]) -> isize {
    // Trace path that has been taken and then calculate min number of steps
    // back to the origin (manhattan distance)
    input
        .iter()
        .fold(CubeCoordinate::origin(), |acc, x| acc.step(x))
        .manhattan_distance()
}

#[aoc(day11, part2)]
fn part2(input: &[Direction]) -> isize {
    // Find the maximum number of steps from the origin while tracing the path
    let (_pos, max) = input.iter().fold((CubeCoordinate::origin(), 0), |acc, x| {
        let pos = acc.0.step(x);
        let dist = pos.manhattan_distance();
        (pos, isize::max(acc.1, dist))
    });
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen("ne,ne,ne")), 3);
        assert_eq!(part1(&gen("ne,ne,sw,sw")), 0);
        assert_eq!(part1(&gen("ne,ne,s,s")), 2);
        assert_eq!(part1(&gen("se,sw,se,sw,sw")), 3);
    }
}
