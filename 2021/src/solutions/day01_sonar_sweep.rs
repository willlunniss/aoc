use itertools::Itertools;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increments(depths: &[usize], window_size: usize) -> usize {
    // Loop over all values using a sliding window and count how many times
    // the sum of the values increments vs the previous
    depths
        .windows(window_size)
        .map(|values| values.iter().sum::<usize>())
        .tuple_windows()
        .filter(|&(previous, current)| current > previous)
        .count()
}

#[aoc(day1, part1)]
fn part1(input: &[usize]) -> usize {
    // Count increments looking at each value individually
    count_increments(input, 1)
}

#[aoc(day1, part2)]
fn part2(input: &[usize]) -> usize {
    // Count increments using a sliding window of 3 values
    count_increments(input, 3)
}
