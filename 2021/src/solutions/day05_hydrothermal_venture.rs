use itertools::Itertools;
use utils::grid::{MapGrid, Pos};

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<(Pos, Pos)> {
    // Parse each line as a pair of positions split by ->
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pos| pos.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[(Pos, Pos)]) -> usize {
    let mut grid = MapGrid::new();
    for (from, to) in input
        .iter()
        .filter(|(from, to)| from.x == to.x || from.y == to.y)
    {
        // For all horizontal or vertical lines between positions
        for pos in from.positions_inclusive(to) {
            // Increment count for all positions between the two (inclusive of them)
            grid.entry(pos).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    // Find how many positions have 2 or more lines crossing them
    grid.values().filter(|x| **x > 1).count()
}

#[aoc(day5, part2)]
fn part2(input: &[(Pos, Pos)]) -> usize {
    let mut grid = MapGrid::new();
    for (from, to) in input {
        // For all lines between positions
        for pos in from.positions_inclusive(to) {
            // Increment count for all positions between the two (inclusive of them)
            grid.entry(pos).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    // Find how many positions have 2 or more lines crossing them
    grid.values().filter(|x| **x > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 12);
    }
}
