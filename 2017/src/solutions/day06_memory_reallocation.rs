use std::collections::HashMap;

fn find_repeated_configuration(input: &[usize]) -> Option<(usize, usize)> {
    let mut history = HashMap::new();
    let mut banks = input.to_vec();
    for cycle in 1.. {
        // Find the index of the first bank with the most blocks
        let mut selected = 0;
        let mut max = 0;
        for (index, &bank) in banks.iter().enumerate() {
            if bank > max {
                max = bank;
                selected = index;
            }
        }
        // Remove blocks from the select index
        let blocks = banks[selected];
        banks[selected] = 0;
        // Redistribute them
        for allocate in 1..=blocks {
            banks[(selected + allocate) % input.len()] += 1;
        }
        // Compute a signature of blocks in each bank
        let sig = banks.iter().map(ToString::to_string).collect::<String>();
        if let Some(first_seen) = history.insert(sig, cycle) {
            // Have seen this configuration before
            return Some((cycle, first_seen));
        }
    }
    None
}

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| {
            line.split('\t')
                .map(|value| value.parse::<usize>().unwrap())
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[usize]) -> usize {
    // Want to know how many cycles before we see a configuration again
    let (cycles, _) = find_repeated_configuration(input).unwrap();
    cycles
}

#[aoc(day6, part2)]
fn part2(input: &[usize]) -> usize {
    // Want to know the size of the loop (cycles between when the configuration occurred)
    let (cycles, first_seen) = find_repeated_configuration(input).unwrap();
    cycles - first_seen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&[0, 2, 7, 0]), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&[0, 2, 7, 0]), 4);
    }
}
