use cached::proc_macro::cached;
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

    fn play_turn(&mut self, spaces: u32, requires: u32) -> bool {
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

lazy_static! {
    /// Possible outcomes of rolling dirac die 3 times
    static ref DIRAC_ROLL_3_SUMS: Vec<u8> = [1, 2, 3, 1, 2, 3, 1, 2, 3]
        .iter()
        .combinations(3)
        .unique()
        .map(|x| { x.iter().copied().sum() })
        .collect();
}

#[cached]
fn dirac_winner(p1_pos: u8, p1_score: u8, p2_pos: u8, p2_score: u8) -> (u128, u128) {
    let mut wins = (0, 0);
    // For each possible outcome of player 1 rolling
    for p1_roll in DIRAC_ROLL_3_SUMS.iter() {
        // For each possible outcome of player 2 rolling
        for p2_roll in DIRAC_ROLL_3_SUMS.iter() {
            // Create new parallel players for each outcome by purposefully re-defining passed in values
            // Move players based on their rolls
            let mut p1_pos = p1_pos + p1_roll;
            if p1_pos > 10 {
                p1_pos -= 10;
            }
            let mut p2_pos = p2_pos + p2_roll;
            if p2_pos > 10 {
                p2_pos -= 10;
            }

            // Calculate scores
            let p1_score = p1_score + p1_pos;
            let p2_score = p2_score + p2_pos;

            // See if someone won
            if p1_score >= 21 {
                wins.0 += 1;
                // This game and all parallel games for this roll of player 1 end
                break;
            } else if p2_score >= 21 {
                wins.1 += 1;
            } else {
                // No one won - game continues
                let new_wins = dirac_winner(p1_pos, p1_score, p2_pos, p2_score);
                wins.0 += new_wins.0;
                wins.1 += new_wins.1;
            }
        }
    }
    wins
}

#[aoc(day21, part1)]
fn part1(input: &[Player]) -> u32 {
    let (mut player1, mut player2) = (input[0].clone(), input[1].clone());
    let mut die = DeterministicDice::new();
    // Play with deterministic die until someone wins
    loop {
        if player1.play_turn(die.roll3sum(), 1000) {
            return player2.score * die.rolls;
        }
        if player2.play_turn(die.roll3sum(), 1000) {
            return player1.score * die.rolls;
        }
    }
}

#[aoc(day21, part2)]
fn part2(input: &[Player]) -> u128 {
    // Find out who wins most for the given starting position using dirac die
    let wins = dirac_winner(
        input[0].space.try_into().unwrap(),
        0,
        input[1].space.try_into().unwrap(),
        0,
    );
    u128::max(wins.0, wins.1)
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
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 739_785);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 444_356_092_776_315);
    }
}
