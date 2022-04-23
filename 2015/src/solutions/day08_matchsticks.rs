/// Calculates the length of the string after decoding all escape chars
fn decoded_length(s: &str) -> usize {
    let mut iter = s.chars();
    let mut length = 0;
    while let Some(c) = iter.next() {
        match c {
            '\\' => {
                // Escape sequence results in one char
                length += 1;
                if Some('x') == iter.next() {
                    // \xFF sequence
                    iter.nth(1);
                } // else \Z sequence
            }
            '"' => {}         // Quote takes up no space
            _ => length += 1, // All other chars result in one char
        }
    }
    length
}

/// Calculates the length of the string after escaping all chars
fn encoded_length(s: &str) -> usize {
    let mut length = 2; // Additional two for the start/end quotes
    for c in s.chars() {
        length += 1;
        match c {
            '\\' | '"' => {
                // Char needs escaping, increase length
                length += 1;
            }
            _ => {}
        }
    }
    length
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(|s| (s.len() - decoded_length(s))).sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input.lines().map(|s| (encoded_length(s) - s.len())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        assert_eq!(part1(r#""""#), 2);
        assert_eq!(part1(r#""\x27""#), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(r#""""#), 4);
        assert_eq!(part2(r#""aaa\"aaa""#), 6);
        assert_eq!(part2(r#""\x27""#), 5);
    }
}
