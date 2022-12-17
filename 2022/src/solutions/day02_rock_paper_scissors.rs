#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            (
                usize::from(line.as_bytes()[0] - b'A'),
                usize::from(line.as_bytes()[2] - b'X'),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    // Opponent/Player: 0=Rock, 1=Paper, 2=Scissors
    // Calculate play score + outcome score
    input
        .iter()
        .map(|(opponent, player)| (player + 1) + ((((player + 3) - opponent) % 3) * 3))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    // Opponent: 0=Rock, 1=Paper, 2=Scissors
    // Outcome: 0=Loose, 1=Draw, 2=Win
    // Calculate play score based on desired outcome + outcome score
    input
        .iter()
        .map(|(opponent, outcome)| (opponent + outcome + 3) % 3 + (outcome * 3))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    A Y
    B X
    C Z
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 15);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 12);
    }
}
