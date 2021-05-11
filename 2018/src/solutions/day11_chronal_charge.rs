use digits_iterator::DigitsExtension;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::convert::TryFrom;

type Power = isize;
type Pos = (usize, usize);
type Size = usize;
const GRID_SIZE: Size = 300;

// Calculates the power level for a given fuel cell
fn power_level(x: usize, y: usize, serial_number: usize) -> Power {
    let rack_id = x + 10;
    let hundreds = (((rack_id * y) + serial_number) * rack_id)
        .digits()
        .rev()
        .nth(2)
        .unwrap_or(0);
    Power::try_from(hundreds).unwrap() - 5
}

/// Finds the grid of the specified size which has the maximum total power
///
/// Returns a tuple of (size, power, pos) of that grid
fn maximise_power(summed_levels: &[Vec<Power>], size: Size) -> Option<(Size, Power, Pos)> {
    // For all (x, y) positions that could fit a size x size sub grid calculate the total power
    (1..=GRID_SIZE - size)
        .flat_map(|y| (1..=GRID_SIZE - size).map(move |x| (x, y)))
        .map(|(x, y)| {
            // Calculate the power in a grid at (x, y) of size x size using the pre-computed
            // summed levels using Summed-area table technique
            let power = summed_levels[y - 1 + size][x - 1 + size] + summed_levels[y - 1][x - 1]
                - summed_levels[y - 1 + size][x - 1]
                - summed_levels[y - 1][x - 1 + size];
            (size, power, (x, y))
        })
        .max_by_key(|(_size, power, _pos)| *power)
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Vec<Power>> {
    let serial_number = input.parse().unwrap();
    // Calculate the summed levels (for use by Summed-area table technique)
    let mut summed_levels = vec![vec![0; GRID_SIZE + 1]; GRID_SIZE + 1];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            // Value at cell is equal to it's power level plus total of all cells above/left of it
            summed_levels[y][x] = power_level(x, y, serial_number)
                + summed_levels[y - 1][x]
                + summed_levels[y][x - 1]
                - summed_levels[y - 1][x - 1];
        }
    }
    summed_levels
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<Power>]) -> String {
    // Simply find the grid of maximum power for size of 3
    let (_, _, pos) = maximise_power(input, 3).unwrap();
    format!("{},{}", pos.0, pos.1)
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<Power>]) -> String {
    // Find the grid of size between 1 and 300 that has maximum power
    let (size, _power, pos) = (1..=GRID_SIZE)
        .into_par_iter()
        .filter_map(|size| maximise_power(input, size))
        .max_by_key(|(_size, power, _pos)| *power)
        .unwrap();
    format!("{},{},{}", pos.0, pos.1, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
