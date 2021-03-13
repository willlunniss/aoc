use digits_iterator::*;
use itertools::Itertools;

/// Validates a password according to the rules in part 1
fn validate1(password: usize) -> bool {
    let mut last = 0;
    let mut has_adjacent = false;
    for digit in password.digits() {
        if digit < last {
            // Each digit must be equal or greater than the previous
            return false;
        }
        if digit == last {
            // Need to have at least two adjacent digits that are the same
            has_adjacent = true;
        }
        last = digit;
    }
    has_adjacent
}

/// Validates a password according to the rules in part 2
fn validate2(password: usize) -> bool {
    let mut last = 0;
    let mut digit_count = vec![0; 10];
    for digit in password.digits() {
        if digit < last {
            // Each digit must be equal or greater than the previous
            return false;
        }
        // Count digits (due to previous rule any value seen more than once will always be adjacent)
        digit_count[digit as usize] += 1;
        last = digit;
    }
    // Password is valid if there are 1 or more instances of 2 adjacent numbers
    // (123444 does not count, but 111122 is OK)
    return digit_count.iter().any(|count| *count == 2);
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<usize> {
    // Get the start and end of the range
    let (start, end) = input
        .split('-')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    // Create a new candidate for all possible values in the range
    (start..end).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[usize]) -> usize {
    return input
        .iter()
        .filter(|password| validate1(**password))
        .count();
}

#[aoc(day4, part2)]
fn part2(input: &[usize]) -> usize {
    return input
        .iter()
        .filter(|password| validate2(**password))
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate1() {
        assert!(validate1(111111));
        assert!(!validate1(223450));
        assert!(!validate1(123789));
    }

    #[test]
    fn test_validate2() {
        assert!(validate2(112233));
        assert!(!validate2(123444));
        assert!(validate2(111122));
    }
}
