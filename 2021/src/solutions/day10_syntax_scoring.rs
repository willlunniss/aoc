use itertools::Itertools;
use std::collections::VecDeque;

/// Returns the opening/closing bracket that matches the supplied char
fn bracket_pair(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut chunks = VecDeque::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        // Open bracket - add to chunks
                        chunks.push_front(c);
                    }
                    ')' | ']' | '}' | '>' => {
                        // Closing bracket - check it closes the last opened bracket
                        if bracket_pair(c) != chunks.pop_front().unwrap() {
                            // Doesn't match - line is corrupted
                            // Score based on illegal bracket's value
                            return match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            };
                        }
                    }
                    _ => unreachable!(),
                }
            }
            // Not corrupted, 0 score
            0
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let scored = input
        .lines()
        .filter_map(|line| {
            let mut chunks = VecDeque::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        // Open bracket - add to chunks
                        chunks.push_front(c);
                    }
                    ')' | ']' | '}' | '>' => {
                        // Closing bracket - check it closes the last opened bracket
                        if bracket_pair(c) != chunks.pop_front().unwrap() {
                            // Doesn't match - line is corrupted
                            // Discard line
                            return None;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            // Line is not corrupted, score based on incomplete brackets
            Some(
                chunks
                    .iter()
                    .map(|&c| match bracket_pair(c) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, x| (acc * 5) + x),
            )
        })
        .sorted()
        .collect::<Vec<_>>();

    // Final result is the value of the middle score (will always be an odd number)
    *scored.get((scored.len() - 1) / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 26397);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 288_957);
    }
}
