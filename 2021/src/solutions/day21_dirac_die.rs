use itertools::Itertools;

#[aoc_generator(day21)]
fn gen(input: &str) -> Vec<Player> {
    input.lines().map(Player::new).collect()
}

#[derive(Debug, Clone)]
struct Player {
    id: u32,
    space: u32,
    score: u32,
}

impl Player {
    fn new(s: &str) -> Self {
        Self {
            id: s.chars().nth(7).unwrap().to_digit(10).unwrap(),
            space: s.chars().last().unwrap().to_digit(10).unwrap(),
            score: 0,
        }
    }

    const fn has_won(&self, requires: u32) -> bool {
        self.score >= requires
    }

    fn play(&mut self, spaces: u32, requires: u32) -> bool {
        self.space += spaces;
        while self.space > 10 {
            self.space -= 10;
        }
        self.score += self.space as u32;
        self.has_won(requires)
    }
}

#[derive(Debug)]
struct DeterministicDice {
    value: u32,
    rolls: u32,
}

impl DeterministicDice {
    const fn new() -> Self {
        Self {
            value: 100,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        self.rolls += 1;
        self.value += 1;
        if self.value > 100 {
            self.value = 1;
        }
        self.value
    }

    fn roll3sum(&mut self) -> u32 {
        self.roll() + self.roll() + self.roll()
    }
}

#[aoc(day21, part1)]
fn part1(input: &[Player]) -> u32 {
    let (mut player1, mut player2) = (input[0].clone(), input[1].clone());
    let mut die = DeterministicDice::new();
    loop {
        if player1.play(die.roll3sum(), 1000) {
            return player2.score * die.rolls;
        }
        if player2.play(die.roll3sum(), 1000) {
            return player1.score * die.rolls;
        }
    }
}

#[aoc(day21, part2)]
fn part2(input: &[Player]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    Player 1 starting position: 4
    Player 2 starting position: 8
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 739785);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 0);
    }
}
