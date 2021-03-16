use crate::utils::grid::VecGrid;
use std::collections::HashSet;

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
            // and then work out next state
            next.set(pos, next_state(*value, adjacent_bugs));
        }
        std::mem::swap(&mut input, &mut next);
    }
}

#[aoc(day24, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    0
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
