use itertools::Itertools;
use std::iter;
use utils::grid::{Direction, Pos};

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Direction> {
    // Convert ^v>< chars into directions
    input.chars().map(|d| d.try_into().unwrap()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Direction]) -> usize {
    let start = Pos::new(0, 0);
    let mut pos = start;
    input
        .iter()
        .map(move |dir| {
            // Move based on the directions
            pos = pos.next(*dir);
            pos
        })
        .chain(iter::once(start))
        .unique()
        .count() // Result is the number of unique locations (including the start)
}

#[aoc(day3, part2)]
fn part2(input: &[Direction]) -> usize {
    let start = Pos::new(0, 0);
    let mut santa = start;
    let mut robot = start;
    input
        .iter()
        .enumerate()
        .map(move |(i, dir)| {
            // Take turns to move based on the directions
            if i % 2 == 0 {
                // Santa's turn
                santa = santa.next(*dir);
                santa
            } else {
                // Robot's turn
                robot = robot.next(*dir);
                robot
            }
        })
        .chain(iter::once(start))
        .unique()
        .count() // Result is the number of unique locations (including the start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(">")), 2);
        assert_eq!(part1(&gen("^>v<")), 4);
        assert_eq!(part1(&gen("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen("^v")), 3);
        assert_eq!(part2(&gen("^>v<")), 3);
        assert_eq!(part2(&gen("^v^v^v^v^v")), 11);
    }
}
