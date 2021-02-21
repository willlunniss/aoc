use std::collections::HashMap;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

pub struct DirectionList {
    directions: Vec<Direction>
}

impl DirectionList {
    pub fn new(directions: Vec<Direction>) -> DirectionList {
        return DirectionList { directions: directions};
    }

    pub fn resolve(&self) -> CubeCoordinate {
        CubeCoordinate::resolve(self.directions.iter().map(|d| CubeCoordinate::new(d)).collect())
    }
}

impl FromStr for DirectionList {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        let mut directions = Vec::new();
        let mut iter = s.chars();
        while let Some(c) = iter.next() {
            match Some(c) {
                Some('e') => directions.push(E),
                Some('w') => directions.push(W),
                Some('s') => {
                    match iter.next().unwrap() {
                        'e' => directions.push(SE),
                        'w' => directions.push(SW),
                        _ => panic!("Unexpected sequence")
                    }
                },
                Some('n') => {
                    match iter.next().unwrap() {
                        'e' => directions.push(NE),
                        'w' => directions.push(NW),
                        _ => panic!("Unexpected sequence")
                    }
                }
                _ => { break; },
            }
        }
        return Ok(DirectionList{directions: directions});
    }
}

// Use 3 dimensions to represent cubs on the hex grid
// https://www.redblobgames.com/grids/hexagons/
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct CubeCoordinate {
    x: isize,
    y: isize,
    z: isize
}

impl CubeCoordinate {
    /// Creates a new coordinate from a direction
    pub fn new(direction: &Direction) -> CubeCoordinate {
        use Direction::*;
        match direction {
            E => CubeCoordinate{x: 1, y: -1, z: 0},
            SE => CubeCoordinate{x: 0, y: -1, z: 1},
            SW => CubeCoordinate{x: -1, y: 0, z: 1},
            W => CubeCoordinate{x: -1, y: 1, z: 0},
            NW => CubeCoordinate{x: 0, y: 1, z: -1},
            NE => CubeCoordinate{x: 1, y: 0, z: -1},
        }
    }

    pub fn origin() -> CubeCoordinate {
        CubeCoordinate{x: 0, y: 0, z: 0}
    }

    /// Resolves a list of coordinates into a single one
    pub fn resolve(coordinates: Vec<CubeCoordinate>) -> CubeCoordinate {
        coordinates.iter().fold(CubeCoordinate::origin(), | acc, c| CubeCoordinate{x: acc.x + c.x, y: acc.y + c.y, z: acc.z + c.z})
    }
}

#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<CubeCoordinate> {
    return input.lines().map(|s| s.parse::<DirectionList>().unwrap().resolve()).collect();
}

#[aoc(day24, part1)]
fn part1(input: &Vec<CubeCoordinate>) -> usize {
    let mut states : HashMap<&CubeCoordinate, bool> = HashMap::new();
    for tile in input {
        // For each referenced tile, get it's state (defaulting to not flipped)
        let state = states.entry(tile).or_default();
        // and then flip it
        *state ^= true;
    }
    // Count tiles which end in the flipped state
    return states.values().filter(|state| **state).collect::<Vec<_>>().len();
}

#[aoc(day24, part2)]
fn part2(input: &Vec<CubeCoordinate>) -> usize {
    
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions_to_coordinate() {
        use Direction::*;
        assert_eq!(DirectionList::new([E, SE, W].to_vec()).resolve(), CubeCoordinate::new(&SE));
        assert_eq!(DirectionList::new([NW, W, SW, E, E].to_vec()).resolve(), CubeCoordinate::origin());
    }
}