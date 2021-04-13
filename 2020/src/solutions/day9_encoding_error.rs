use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn find_invalid(input: &[usize], preamble: usize) -> usize {
    for i in preamble..input.len() {
        let value = input[i];
        // Build a HashSet of valid values based on the different combinations of the previous N numbers
        let valid: HashSet<usize> = input[i - preamble..i]
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a + b)
            .collect();
        if !valid.contains(&value) {
            // Found an invalid value!
            return value;
        }
    }
    0
}

#[aoc(day9, part1)]
fn part1(input: &[usize]) -> usize {
    find_invalid(input, 25)
}

#[aoc(day9, part2)]
fn part2(input: &[usize]) -> usize {
    // Find the invalid value
    let preamble = 25;
    let target = find_invalid(input, preamble);

    // Now find a contiguous set of 2+ numbers that adds up to the target
    for i in 0..input.len() {
        let mut total = 0;
        for n in i..input.len() {
            total += input[n];
            if total == target {
                // Found it, return the sum of smallest and largest
                let smallest = input[i..n].iter().min().unwrap();
                let largest = input[i..n].iter().max().unwrap();
                return smallest + largest;
            }
            if total > target {
                // Added too much and missed the target, try next starting point
                break;
            }
        }
    }
    // Failed to find the answer
    0
}
