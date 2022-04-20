use itertools::Itertools;

/// Returns the number of vowels in the string
fn count_vowels(string: &str) -> usize {
    string
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

/// Returns true if the string contains a double char (e.g. bb in abbac)
fn has_double_char(string: &str) -> bool {
    string.chars().tuple_windows().any(|(a, b)| a == b)
}

/// Returns true if the string contains any of the banned sub-strings
fn has_banned_strings(string: &str) -> bool {
    string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy")
}

/// Returns true if the string contains non-overlapping pairs of chars
///
/// It contains a pair of any two letters that appears at least twice in the string without overlapping,
/// like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps)
fn has_non_overlapping_pairs(string: &str) -> bool {
    string
        .chars()
        .tuple_windows::<(_, _)>() // Get all pairs of chars
        .counts() // Count occurrences
        .iter()
        .filter(|(_, count)| **count >= 2) // Consider just ones where we see the pair multiple times
        .any(|((c1, c2), _)| {
            // For all multiple occurring pairs
            string
                .match_indices(&[*c1, *c2].iter().collect::<String>()) // Get the indexes where they occur
                .map(|(i, _)| i)
                .tuple_combinations()
                .any(|(i1, i2)| i2 - i1 >= 2) // Check that at least one pair is 2 chars apart (i.e. not overlapping)
        })
}

/// Returns true if the string contains repeating char X in the form XYX
fn has_repeat_char_with_gap(string: &str) -> bool {
    string.chars().tuple_windows().any(|(a, _, c)| a == c)
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    // Count nice strings using rules
    input
        .lines()
        .filter(|s| count_vowels(s) >= 3 && has_double_char(s) && !has_banned_strings(s))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    // Count nice strings using rules
    input
        .lines()
        .filter(|s| has_non_overlapping_pairs(s) && has_repeat_char_with_gap(s))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_1: &str = indoc! {"
    ugknbfddgicrmopn
    aaa
    jchzalrnumimnmhp
    haegwjzuvuyypxyu
    dvszwmarrgswjxmb
"};

    static EXAMPLE_INPUT_2: &str = indoc! {"
    qjhvhtzxzqqjkmpb
    xxyxx
    uurcxstgmygtbstg
    ieodomkazucvgmuy
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 2);
    }

    #[test]
    fn test_has_non_overlapping_pairs() {
        assert!(has_non_overlapping_pairs("aabcdefgaa"));
        assert!(!has_non_overlapping_pairs("aaa"));
    }
}
