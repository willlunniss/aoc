use pathfinding::prelude::dijkstra;
use utils::grid::{Pos, VecGrid};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: Pos,
    visited: usize,
}

impl State {
    /// Updates the state based on moving to `pos` containing `item`
    fn next(&self, pos: Pos, item: char) -> Self {
        // Calculate new visited value
        // Specifically need to set the bit field corresponding to an item rather than just
        // adding it in case we visit the same item multiple times
        let visited = item
            .to_digit(10)
            .map_or(self.visited, |value| self.visited | (1 << value));
        Self { pos, visited }
    }
}

fn visit_all_points(grid: &VecGrid<char>, return_to_start: bool) -> usize {
    // Find all the points of interest
    let points = grid
        .into_iter()
        .filter(|(_, c)| c.is_digit(10))
        .collect::<Vec<_>>();
    // Starting from 0
    let start = points.iter().find(|(_, c)| **c == '0').unwrap().0;
    // Represent each one as a bit field which will be set when visited
    let target = points
        .iter()
        .map(|(_, c)| 1 << c.to_digit(10).unwrap())
        .sum::<usize>();
    // Find the shortest path to visit all points at least once
    dijkstra(
        &State {
            pos: start,
            visited: 0,
        },
        |state| {
            grid.neighbours_ex(state.pos)
                .filter(|(_, _, x)| x != &Some('#'))
                .map(|(_, pos, x)| (state.next(pos, x.unwrap()), 1))
                .collect::<Vec<_>>()
        },
        |state| state.visited == target && (!return_to_start || state.pos == start),
    )
    .unwrap()
    .1
}

#[aoc_generator(day24)]
fn gen(input: &str) -> VecGrid<char> {
    input.parse().unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    visit_all_points(input, false)
}

#[aoc(day24, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    visit_all_points(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    ###########
    #0.1.....2#
    #.#######.#
    #4.......3#
    ###########
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 14);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 20);
    }
}
