use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

// Returns the next ascii char
const fn next_char(c: char) -> char {
    (c as u8 + 1) as char
}

// Increments a password to the next possible (but not necessarily valid) value
fn increment(password: Vec<char>) -> Vec<char> {
    let mut password = password;
    for x in (0..password.len()).rev() {
        if password[x] == 'z' {
            password[x] = 'a';
        } else {
            password[x] = next_char(password[x]);
            break;
        }
    }
    password
}

// Checks requirement 1: must include one increasing straight of at least three letters
fn has_increasing(password: &[char]) -> bool {
    password
        .iter()
        .tuple_windows()
        .any(|(&a, &b, &c)| next_char(a) == b && next_char(b) == c)
}

// Checks requirement 2:  may not contain the letters i, o, or l
fn no_banned(password: &[char]) -> bool {
    !password.iter().any(|&x| x == 'i' || x == 'o' || x == 'l')
}

// Checks requirement 3: must contain at least two different, non-overlapping pairs of letters
fn has_pairs(password: &[char]) -> bool {
    password
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| *a)
        .unique()
        .count()
        >= 2
}

// Generates the next valid password
fn next_password(password: Vec<char>) -> Vec<char> {
    (0..)
        .fold_while(increment(password), |password, _| {
            // Keep incrementing until all requirements pass
            if has_increasing(&password) && no_banned(&password) && has_pairs(&password) {
                Done(password)
            } else {
                Continue(increment(password))
            }
        })
        .into_inner()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    next_password(input.chars().collect()).iter().collect()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    next_password(next_password(input.chars().collect()))
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}
