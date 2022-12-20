use itertools::Itertools;

/// Returns the index into the message where the first sub-sequence of
/// distinct characters ends
fn start_of_seq(message: &str, distinct_chars: usize) -> usize {
    let chars = message.chars().collect::<Vec<char>>();
    chars
        .windows(distinct_chars) // For all complete windows of the target size
        .enumerate() // Find the first where all chars are unique
        .find(|(_, w)| w.iter().unique().count() == distinct_chars)
        .map(|(index, _)| index + distinct_chars) // Return the index of where it ends
        .unwrap()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    start_of_seq(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    start_of_seq(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
