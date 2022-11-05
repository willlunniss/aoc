// Performs a single look and say step
fn step(input: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut current = *input.iter().next().unwrap();
    let mut count = 0;
    for &c in input.iter() {
        if c == current {
            count += 1;
        } else {
            // End of current sequence, output count and the digit
            result.push(count);
            result.push(current);
            // Move to new current digit
            current = c;
            count = 1;
        }
    }
    // End of the sequence, output count and digit for the last one
    result.push(count);
    result.push(current);
    result
}

// Generates a look and say sequence
fn look_and_say(input: &str, passes: usize) -> Vec<u32> {
    // Convert to vec of digits and then apply the requested number of
    // passes to get the final sequence
    (0..passes).fold(
        input
            .chars()
            .map(|x| char::to_digit(x, 10).unwrap())
            .collect(),
        |seq, _| step(&seq),
    )
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    look_and_say(input, 40).len()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    look_and_say(input, 50).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(look_and_say("1211", 1).len(), 6);
    }
}
