#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    // Init an array with a 0 for each bit
    let bits = input.lines().next().unwrap().len();
    let mut counts = vec![0; bits];
    for number in input.lines() {
        // For each bit, find out if 1 or 0 is most common
        for (index, c) in number.chars().enumerate() {
            if c == '1' {
                counts[index] += 1;
            } else {
                counts[index] -= 1;
            }
        }
    }
    // For each bit (LSB -> MSB), gamma rate using the most common bit from all numbers
    let gamma_rate = counts
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| if *x > 0 { 1 << i } else { 0 })
        .sum::<usize>();
    // Epsilon rate uses the least common (xor with all bits set)
    let epsilon_rate = gamma_rate ^ ((1 << bits) - 1);
    // Result is the product of the two
    gamma_rate * epsilon_rate
}

fn filter(input: &str, keep_set: bool) -> usize {
    // Check how many bits each number has
    let bits = input.lines().next().unwrap().len();
    // Interoperate each line as a binary number
    let mut values = input
        .lines()
        .map(|number| usize::from_str_radix(number, 2).unwrap())
        .collect::<Vec<_>>();
    for index in 1.. {
        let bit = bits - index;
        // Count how many bits at this index are set across all numbers
        let count = values
            .iter()
            .map(|number| if number & (1 << bit) == 0 { -1 } else { 1 })
            .sum::<isize>();
        // Depending on mode, work out what we want to keep
        let keep = !((count >= 0) ^ keep_set);
        // Keep all values that have the right state for the bit at this index
        values = values
            .iter()
            .filter(|&number| (number & (1 << bit) != 0) == keep)
            .copied()
            .collect();
        if values.len() == 1 {
            // Until we only have one value left
            break;
        }
    }
    // Interpret as a binary number and return decimal value
    *values.first().unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    filter(input, true) * filter(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 198);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 230);
    }
}
