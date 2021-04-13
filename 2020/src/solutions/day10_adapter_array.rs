use std::collections::HashMap;

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<usize>) -> usize {
    let mut sorted = input.clone();
    sorted.push(0); // Add outlet which is 0
    sorted.sort_unstable(); // Sort smallest to largest
    sorted.push(sorted.last().unwrap() + 3); // Add built in adapter is always +3 more than last
    let mut differences = vec![0; 4]; // Expect no more than +3 difference
    for i in 1..sorted.len() {
        // For each adapter, record the difference between it and the previous
        differences[sorted[i] - sorted[i - 1]] += 1;
    }
    differences[1] * differences[3]
}

/// Calculates the number of combinations that adapters could be arranged
/// given the supplied contiguous len of adapters
fn combinations(contiguous_len: usize) -> usize {
    // TODO: Work out the formula for this...
    match contiguous_len {
        3 => 2,
        4 => 4,
        5 => 7,
        _ => panic!(
            "Don't know how many combinations there are for {}",
            contiguous_len
        ),
    }
}

#[aoc(day10, part2)]
fn part2(input: &Vec<usize>) -> usize {
    let mut sorted = input.clone();
    sorted.push(0); // Add outlet which is 0
    sorted.sort(); // Sort smallest to largest
    sorted.push(sorted.last().unwrap() + 3); // Add built in adapter is always +3 more than last
    let mut groups: HashMap<usize, usize> = HashMap::new();
    let mut contiguous_len = 1;
    for i in 1..sorted.len() {
        // We are interested in contiguous (diff of 1 between each) groups of 3 or more adapters
        // as they have options to remove some adapters for different combinations
        if sorted[i] - sorted[i - 1] == 1 {
            // Adapters are contiguous, increment len
            contiguous_len += 1;
        } else {
            // End of sequence, note down how long it was if >= 3 and reset
            if contiguous_len >= 3 {
                let count = groups.entry(contiguous_len).or_default();
                *count += 1;
            }
            contiguous_len = 1;
        }
    }
    let mut arrangements = 1;
    for (len, count) in &groups {
        // Work out the total number of arrangements
        arrangements *= usize::pow(combinations(*len), *count as u32);
    }
    arrangements
}
