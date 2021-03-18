use crate::utils::grid::{Direction, Pos, VecGrid};
use std::collections::{HashMap, HashSet};

/// Calculates a layouts biodiversity rating
fn rating(values: &[char]) -> usize {
    let mut points = 1;
    let mut rating = 0;
    for value in values {
        if *value == '#' {
            rating += points;
        }
        points *= 2;
    }
    rating
}

/// Calculates the next state for a cell
const fn next_state(current: char, adjacent_bugs: usize) -> char {
    // New state is based on adjacent bugs and current state
    if adjacent_bugs == 1 || (adjacent_bugs == 2 && current == '.') {
        '#'
    } else {
        '.'
    }
}

#[aoc_generator(day24)]
fn gen(input: &str) -> VecGrid<char> {
    VecGrid::from(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day24, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    let mut input = input.clone();
    let mut next = input.clone();
    let mut layouts: HashSet<Vec<char>> = HashSet::new();
    loop {
        let values = input.values();
        if layouts.contains(&values) {
            // If we have seen this layout before return the rating
            return rating(&values);
        }
        layouts.insert(values);
        for (pos, value) in &input {
            // For each cell count adjacent bugs
            let adjacent_bugs = input
                .neighbours(pos)
                .iter()
                .filter(|n| **n == Some('#'))
                .count();
            // and then work out the next state
            next.set(pos, next_state(*value, adjacent_bugs));
        }
        std::mem::swap(&mut input, &mut next);
    }
}

#[aoc(day24, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    // Create a default (empty) grid for use when moving into new levels
    let mut default = VecGrid::from(vec![vec!['.'; 5]; 5]);

    // Mark the center tile in the default and our initial input as recursing
    let center = Pos::new(2, 2);
    default.set(center, '?');
    let mut input = input.clone();
    input.set(center, '?');

    // Store the current levels
    // Lower levels are inside and higher levels are outside
    let mut current = HashMap::new();
    current.insert(0, input);
    // And a place to store the next states
    let mut next = HashMap::new();

    for minutes in 1..=200 {
        // We will immediately expand and can then expand into a new level every 2 minutes (two steps out from the center)
        let growth = 1 + (minutes / 2);
        for level in -growth..=growth {
            // Get the grids that we will need to check to evaluate this level
            let up = current.get(&(level + 1));
            let down = current.get(&(level - 1));
            let grid = current.get(&level).unwrap_or(&default);
            // Get (or create if needed) somewhere to store the next state for this level
            let result = next.entry(level).or_insert_with(|| default.clone());
            // Loop over this level's grid
            for (pos, value) in grid {
                if *value == '?' {
                    continue; // Recursive level, skip over - will handle through next loop up
                }
                // Count all adjacent bugs, checking other levels as needed
                let neighbours = grid.neighbours_ex(pos);
                let mut adjacent_bugs = 0;
                for (direction, _, neighbour) in neighbours {
                    if let Some(n) = neighbour {
                        // Have a neighbour within this grid
                        if n == '#' {
                            // Normal bug cell
                            adjacent_bugs += 1;
                        } else if n == '?' {
                            // At the center, need to check down 1 level (at the 5 neighbours)
                            if let Some(inner) = down {
                                // And down is a level we have evaluated before (so may have bugs in it)
                                let inner_neighbours: Vec<Pos> = match direction {
                                    Direction::Up => {
                                        let y = 4; // Check the bottom row of the inner
                                        (0..5).into_iter().map(|x| Pos::new(x, y)).collect()
                                    }
                                    Direction::Down => {
                                        let y = 0; // Check the upper row of the inner
                                        (0..5).into_iter().map(|x| Pos::new(x, y)).collect()
                                    }
                                    Direction::Left => {
                                        let x = 4; // Check the right side of the inner
                                        (0..5).into_iter().map(|y| Pos::new(x, y)).collect()
                                    }
                                    Direction::Right => {
                                        let x = 0; // Check the left side of the inner
                                        (0..5).into_iter().map(|y| Pos::new(x, y)).collect()
                                    }
                                };
                                // Add all inner adjacent bugs
                                adjacent_bugs += inner_neighbours
                                    .iter()
                                    .map(|position| inner.get(*position))
                                    .filter(|n| *n == Some('#'))
                                    .count();
                            }
                        }
                    } else if let Some(outer) = up {
                        // At the edge and up is a level we have evaluated before (so may have bugs in it)
                        // Check the outer grid by looking outwards from it's center
                        if let Some('#') = outer.get(center.next(direction)) {
                            adjacent_bugs += 1;
                        }
                    }
                }
                // and then work out the next state as before
                result.set(pos, next_state(*value, adjacent_bugs));
            }
        }
        std::mem::swap(&mut current, &mut next);
    }
    // Return total number of bugs after 200 minutes
    current
        .values()
        .flat_map(|grid| grid.into_iter().filter(|(_, c)| **c == '#'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating() {
        let values = [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.', '#', '.', '.', '.',
        ];
        assert_eq!(rating(&values), 2_129_920);
    }
}
