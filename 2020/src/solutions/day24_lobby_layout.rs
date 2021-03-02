use std::collections::HashMap;
use std::str::FromStr;
use std::convert::Infallible;
use lazy_static::lazy_static;

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

lazy_static! {
    static ref NEIGHBOURS: Vec<CubeCoordinate> = [Direction::E,Direction::SE,Direction::SW,Direction::W,Direction::NW,Direction::NE]
        .to_vec().iter().map(|d| CubeCoordinate::new(d)).collect();
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

    pub fn neighbours(&self) -> Vec<CubeCoordinate> {
        return NEIGHBOURS.iter().map(|n| CubeCoordinate{x: self.x + n.x, y: self.y + n.y, z: self.z + n.z}).collect();
    }

    pub fn origin() -> CubeCoordinate {
        CubeCoordinate{x: 0, y: 0, z: 0}
    }

    /// Resolves a list of coordinates into a single one
    pub fn resolve(coordinates: Vec<CubeCoordinate>) -> CubeCoordinate {
        coordinates.iter().fold(CubeCoordinate::origin(), | acc, c| CubeCoordinate{x: acc.x + c.x, y: acc.y + c.y, z: acc.z + c.z})
    }
}

/// Gets the next state for a tile
pub fn next_state(tiles: &HashMap<CubeCoordinate, bool>, tile: &CubeCoordinate) -> bool {
    let mut flipped_neighbours = 0;
    for n in tile.neighbours() {
        if *tiles.get(&n).unwrap_or(&false) {
            flipped_neighbours += 1;
        }
    }
    return if *tiles.get(&tile).unwrap_or(&false) {
        // Leave flipped if only 1/2 neighbour is flipped
        flipped_neighbours == 1 || flipped_neighbours == 2
    } else {
        // Flip if 2 neighbours are flipped
        flipped_neighbours == 2
    };
}

#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<CubeCoordinate> {
    return input.lines().map(|s| s.parse::<DirectionList>().unwrap().resolve()).collect();
}

#[aoc(day24, part1)]
fn part1(input: &Vec<CubeCoordinate>) -> usize {
    let mut tiles : HashMap<CubeCoordinate, bool> = HashMap::new();
    for tile in input {
        // For each referenced tile, get it's state (defaulting to not flipped)
        let flipped = tiles.entry(tile.clone()).or_default();
        // and then flip it
        *flipped ^= true;
    }
    // Count tiles which end in the flipped state
    return tiles.values().filter(|flipped| **flipped).collect::<Vec<_>>().len();
}

#[aoc(day24, part2)]
fn part2(input: &Vec<CubeCoordinate>) -> usize {    
    let mut tiles : HashMap<CubeCoordinate, bool> = HashMap::new();
    for tile in input {
        // For each referenced tile, get it's state (defaulting to not flipped)
        let flipped = tiles.entry(tile.clone()).or_default();
        // and then flip it
        *flipped ^= true;
    }
    // Now do 100 passes flipping the tiles according to the rules
    for _ in 1..=100 {
        let mut next : HashMap<CubeCoordinate, bool> = HashMap::new();
        for tile in tiles.keys() {
            next.insert(tile.clone(), next_state(&tiles, &tile));
            // Now check it's neighbours (as the grid will keep expanding)
            for neighbour in tile.neighbours() {
                // If we haven't already computed the next state for this neighbour then do it now
                if !next.contains_key(&neighbour) {
                    next.insert(neighbour.clone(), next_state(&tiles, &neighbour));
                }
            }
        }
        std::mem::swap(&mut next, &mut tiles);
    }
    // Count tiles which end in the flipped state
    return tiles.values().filter(|flipped| **flipped).collect::<Vec<_>>().len();
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