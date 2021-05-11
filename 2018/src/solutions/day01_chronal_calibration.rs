use std::collections::HashSet;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[isize]) -> isize {
    // Simply sum up all the changes to get the final frequency
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[isize]) -> Option<isize> {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;
    // Loop over the changes indefinitely
    for change in input.iter().cloned().cycle() {
        frequency += change;
        // Store the frequency, if we have already seen it before then return it
        if !frequencies.insert(frequency) {
            return Some(frequency);
        }
    }
    None
}
