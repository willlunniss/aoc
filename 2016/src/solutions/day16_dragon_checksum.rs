use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::iter::once;

/// Performs one expansion pass over the data
fn expand(data: &[bool]) -> impl Iterator<Item = bool> + '_ {
    data.iter()
        .copied()
        .chain(once(false))
        .chain(data.iter().rev().map(|x| !x))
}

/// Expands the data until it fills the disk
fn fill_disk(data: Vec<bool>, target: usize) -> Vec<bool> {
    (0..)
        .fold_while(data, |acc, _| {
            // Keep expanding until we have the target amount of data
            let expanded = expand(&acc).take(target).collect::<Vec<_>>();
            if expanded.len() == target {
                Done(expanded)
            } else {
                Continue(expanded)
            }
        })
        .into_inner()
}

/// Performs one reduction pass over the data
fn reduce(data: &[bool]) -> Vec<bool> {
    data.iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| chunk.next() == chunk.next())
        .collect()
}

/// Generates the checksum of the data
fn checksum(data: Vec<bool>) -> Vec<bool> {
    (0..)
        .fold_while(data, |acc, _| {
            // Keep reducing until the data has an odd length
            let reduced = reduce(&acc);
            if reduced.len() % 2 == 1 {
                Done(reduced)
            } else {
                Continue(reduced)
            }
        })
        .into_inner()
}

/// Generates enough data to fill the disk and then returns the checksum of it
fn generate_and_checksum(initial_data: &str, disk_size: usize) -> String {
    checksum(fill_disk(
        initial_data.chars().map(|x| x == '1').collect(),
        disk_size,
    ))
    .iter()
    .map(|x| if *x { '1' } else { '0' })
    .collect()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> String {
    generate_and_checksum(input, 272)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> String {
    generate_and_checksum(input, 35_651_584)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "10000";

    #[test]
    fn test_part1_fill_disk() {
        assert_eq!(fill_disk([true].to_vec(), 3), [true, false, false]);
        assert_eq!(
            fill_disk([true].to_vec(), 6),
            [true, false, false, false, true, true]
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(generate_and_checksum(EXAMPLE_INPUT, 20), "01100");
    }
}
