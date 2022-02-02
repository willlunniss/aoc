use itertools::Itertools;
use std::collections::HashSet;

/// Computes the hash for this `door_id` and `index`.
///
/// If valid, returns the 6/7th chars as u8s
fn compute(door_id: &str, index: usize) -> Option<(u8, u8)> {
    let values: [u8; 16] = md5::compute(format!("{door_id}{index}")).into();
    // Each value contains two chars, test that the first 5 are 0
    if values[0] == 0 && values[1] == 0 && (values[2] & 0xF0) == 0 {
        // 6th char is the second nibble of the 3rd value and the 7th is the first nibble of the 4th
        Some((values[2] & 0xF, (values[3] & 0xF0) >> 4))
    } else {
        None
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    // Build the password from the first 8 indexes that give a valid char
    (0..)
        .filter_map(|index| compute(input, index))
        .take(8)
        .map(|(c, _)| char::from_digit(c as u32, 16).unwrap())
        .collect::<String>()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    // Get the first 8 pairs where the first char is position (keeping the first found)
    // Then port by position and build the password from the second char
    let mut found = HashSet::new();
    (0..)
        .filter_map(|index| compute(input, index))
        .filter(|(pos, _)| matches!(pos, 0..=7) && found.insert(*pos))
        .take(8)
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, c)| char::from_digit(c as u32, 16).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "abc";

    #[test]
    fn test_compute() {
        assert_eq!(compute(EXAMPLE_INPUT, 3_231_929), Some((0x1, 0x5)));
        assert_eq!(compute(EXAMPLE_INPUT, 5_017_308), Some((0x8, 0xf)));
        assert_eq!(compute(EXAMPLE_INPUT, 5_278_568), Some((0xf, 0x9)));
        assert_eq!(compute(EXAMPLE_INPUT, 5_357_525), Some((0x4, 0xe)));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), "18f47a30");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "05ace8e3");
    }
}
