use std::collections::HashMap;

type Pos3 = [isize; 3];

#[aoc_generator(day23)]
fn gen(input: &str) -> HashMap<Pos3, usize> {
    input
        .lines()
        .map(|line| {
            let parts = line
                .split(&['<', ' ', ',', '>', '='][..])
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();
            (
                [
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                    parts[3].parse().unwrap(),
                ],
                parts[5].parse().unwrap(),
            )
        })
        .collect()
}

/// Calculates the Manhattan distance for two positions
fn manhattan_distance(a: &Pos3, b: &Pos3) -> usize {
    (0..3)
        .map(|index| (a[index] - b[index]).abs() as usize)
        .sum()
}

#[aoc(day23, part1)]
fn part1(input: &HashMap<Pos3, usize>) -> usize {
    // Find the strongest nanobot as the one with the max range
    let (strongest, range) = input.iter().max_by_key(|(_, range)| *range).unwrap();
    // Count all the nanobots that are in range of the strongest
    input
        .keys()
        .filter(|pos| manhattan_distance(strongest, pos) <= *range)
        .count()
}

#[aoc(day23, part2)]
fn part2(input: &HashMap<Pos3, usize>) -> usize {
    0
}
