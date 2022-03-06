use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPositionOfLetter(char),
    ReversePositions(usize, usize),
    Move(usize, usize),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "swap" => match parts[1] {
                "position" => Ok(Self::SwapPosition(
                    parts[2].parse().unwrap(),
                    parts[5].parse().unwrap(),
                )),
                "letter" => Ok(Self::SwapLetter(
                    parts[2].chars().next().unwrap(),
                    parts[5].chars().next().unwrap(),
                )),
                _ => Err("Not a valid swap Operation".to_owned()),
            },
            "rotate" => match parts[1] {
                "left" => Ok(Self::RotateLeft(parts[2].parse().unwrap())),
                "right" => Ok(Self::RotateRight(parts[2].parse().unwrap())),
                "based" => Ok(Self::RotateBasedOnPositionOfLetter(
                    s.chars().last().unwrap(),
                )),
                _ => Err("Not a valid rotate Operation".to_owned()),
            },
            "reverse" => Ok(Self::ReversePositions(
                parts[2].parse().unwrap(),
                parts[4].parse().unwrap(),
            )),
            "move" => Ok(Self::Move(
                parts[2].parse().unwrap(),
                parts[5].parse().unwrap(),
            )),
            _ => Err("Not a valid Operation".to_owned()),
        }
    }
}

#[aoc_generator(day21)]
fn gen(input: &str) -> Vec<Operation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn scramble(phrase: &str, ops: &[Operation]) -> String {
    let mut chars = phrase.chars().collect::<VecDeque<_>>();
    for op in ops {
        match op {
            Operation::SwapPosition(x, y) => {
                chars.swap(*x, *y);
            }
            Operation::SwapLetter(a, b) => {
                let (x, y) = (
                    chars.iter().position(|l| l == a).unwrap(),
                    chars.iter().position(|l| l == b).unwrap(),
                );
                chars.swap(x, y);
            }
            Operation::RotateLeft(steps) => chars.rotate_left(*steps),
            Operation::RotateRight(steps) => chars.rotate_right(*steps),
            Operation::RotateBasedOnPositionOfLetter(a) => {
                let mut steps = chars.iter().position(|l| l == a).unwrap();
                if steps >= 4 {
                    steps += 1;
                }
                chars.rotate_right((1 + steps) % chars.len());
            }
            Operation::ReversePositions(x, y) => {
                for (i, c) in (*x..=*y)
                    .rev()
                    .map(|i| chars[i])
                    .collect::<Vec<_>>()
                    .iter()
                    .enumerate()
                {
                    chars[*x + i] = *c;
                }
            }
            Operation::Move(x, y) => {
                let value = chars.remove(*x).unwrap();
                chars.insert(*y, value);
            }
        }
    }
    chars.iter().collect()
}

#[aoc(day21, part1)]
fn part1(input: &[Operation]) -> String {
    scramble(
        if input.len() < 10 {
            "abcde"
        } else {
            "abcdefgh"
        },
        input,
    )
}

#[aoc(day21, part2)]
fn part2(input: &[Operation]) -> Option<String> {
    let target;
    let letters;
    if input.len() < 10 {
        letters = "abcde";
        target = "decab";
    } else {
        letters = "abcdefgh";
        target = "fbgdceah";
    }
    for password in letters
        .chars()
        .permutations(letters.len())
        .map(|chars| chars.iter().collect::<String>())
    {
        if target == scramble(&password, input) {
            return Some(password);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    swap position 4 with position 0
    swap letter d with letter b
    reverse positions 0 through 4
    rotate left 1 step
    move position 1 to position 4
    move position 3 to position 0
    rotate based on position of letter b
    rotate based on position of letter d
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), "decab");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), Some("abcde".to_owned()));
    }
}
