#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn count_increments(depths: &[usize], window_size: usize) -> usize {
    let mut increments = 0;
    let mut previous = 0;
    // Skipping the first measurement, loop over all values using a sliding window
    // and count how many times the sum of the values increments vs the previous
    for values in depths.windows(window_size).skip(1) { 
        let sum = values.iter().sum();
        if sum > previous {
            increments += 1;
        }
        previous = sum;
    }
    increments
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