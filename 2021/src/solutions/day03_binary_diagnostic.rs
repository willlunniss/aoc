#[aoc_generator(day3)]
fn gen(input: &str) -> (Vec<usize>, usize) {
    // Check how many bits a number has
    let bits = input.lines().next().unwrap().len();
    // Interpret each line as a binary number
    let values = input
        .lines()
        .map(|number| usize::from_str_radix(number, 2).unwrap())
        .collect::<Vec<_>>();
    (values, bits)
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<usize>, usize)) -> usize {
    let (values, bits) = input;
    // For each bit (LSB -> MSB), find out if 1 or 0 is most common
    let counts = (0..*bits)
        .map(|bit| {
            values
                .iter()
                .map(|number| if number & (1 << bit) == 0 { -1 } else { 1 })
                .sum::<isize>()
        })
        .collect::<Vec<_>>();
    // For each bit (LSB -> MSB) set the bit to be the most common bit from all numbers
    let gamma_rate = counts
        .iter()
        .enumerate()
        .map(|(bit, count)| if *count > 0 { 1 << bit } else { 0 })
        .sum::<usize>();
    // Epsilon rate uses the least common (so can xor with all bits set)
    let epsilon_rate = gamma_rate ^ ((1 << bits) - 1);
    // Result is the product of the two
    gamma_rate * epsilon_rate
}

fn find_rating(values: &Vec<usize>, bits: usize, keep_set: bool) -> usize {
    let mut values = values.clone();
    // For each bit (MSB -> LSB)
    for bit in (0..bits).rev() {
        // Count how of the numbers have this bit set
        let count = values
            .iter()
            .map(|number| if number & (1 << bit) == 0 { -1 } else { 1 })
            .sum::<isize>();
        // Depending on mode, work out what we want to keep
        let keep = !((count >= 0) ^ keep_set);
        // Keep all values that have the right state for the bit
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
fn part2(input: &(Vec<usize>, usize)) -> usize {
    let (values, bits) = input;
    find_rating(values, *bits, true) * find_rating(values, *bits, false)
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
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 198);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 230);
    }
}
