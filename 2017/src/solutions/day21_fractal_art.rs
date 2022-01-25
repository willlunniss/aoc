use indoc::indoc;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use utils::grid::{Pos, VecGrid};

static START_PATTERN: &str = indoc!(
    "
    .#.
    ..#
    ###
    "
);

fn gen(input: &str) -> HashMap<String, String> {
    let mut rules = HashMap::new();
    for (from, to) in input.lines().map(|line| {
        line.split(" => ")
            .map(|x| x.replace("/", "\n"))
            .collect_tuple()
            .unwrap()
    }) {
        // Expands the rules into the various rotated and mirrored forms
        let mut normal = from.parse::<VecGrid<char>>().unwrap();
        let mut mirrored = normal.mirror();
        for _ in 0..4 {
            rules.insert(normal.values().collect(), to.clone());
            normal = normal.rotate();

            rules.insert(mirrored.values().collect(), to.clone());
            mirrored = mirrored.rotate();
        }
    }
    rules
}

/// Enhances the `grid` using the supplied `rules` `iterations` times
fn enhance(
    grid: &VecGrid<char>,
    rules: &HashMap<String, String>,
    iterations: usize,
) -> VecGrid<char> {
    let mut grid = grid.clone();
    for _iteration in 0..iterations {
        let size = grid.width();
        let sub_size = if size % 2 == 0 { 2 } else { 3 };
        let new_sub_size = sub_size + 1;
        let new_size = size * (sub_size + 1) / sub_size;
        let mut next = VecGrid::new_sized('?', new_size, new_size);
        for (ys, xs) in iproduct!(0..size / sub_size, 0..size / sub_size) {
            // Get the sub grid
            let sub_grid = &iproduct!(0..sub_size, 0..sub_size)
                .map(|(y, x)| Pos::new((xs * sub_size) + x, (ys * sub_size) + y))
                .map(|pos| grid.get(pos).unwrap())
                .collect::<String>();
            // Look up what it is enhanced to
            let new_sub_grid = rules
                .get(sub_grid)
                .unwrap()
                .parse::<VecGrid<char>>()
                .unwrap();
            // Add new sub grid to the next overall grid
            for (y, x) in iproduct!(0..new_sub_size, 0..new_sub_size) {
                next[Pos::new((xs * new_sub_size) + x, (ys * new_sub_size) + y)] =
                    new_sub_grid[Pos::new(x, y)];
            }
        }
        grid = next;
    }
    grid
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let rules = gen(input);
    let iterations = if rules.len() == 12 { 2 } else { 5 };
    enhance(&START_PATTERN.parse().unwrap(), &rules, iterations)
        .values()
        .filter(|x| **x == '#')
        .count()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    enhance(&START_PATTERN.parse().unwrap(), &gen(input), 18)
        .values()
        .filter(|x| **x == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = indoc!(
        "
        ../.# => ##./#../...
        .#./..#/### => #..#/..../..../#..#
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 12);
    }
}
