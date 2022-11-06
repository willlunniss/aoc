use std::{cmp::min, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum State {
    Flying(u64),
    Resting(u64),
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: u64,
    fly_time: u64,
    rest_time: u64,
    state: State,
    position: u64,
    score: u64,
}

impl FromStr for Reindeer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u64> = s
            .split_ascii_whitespace()
            .flat_map(str::parse::<u64>)
            .collect();
        Ok(Self {
            speed: values[0],
            fly_time: values[1],
            rest_time: values[2],
            state: State::Flying(values[1]),
            position: 0,
            score: 0,
        })
    }
}

impl Reindeer {
    // Calculates the distance that can be travelled over a period of time assuming currently ready to start flying
    fn dist_travelled(&self, time: u64) -> u64 {
        let cycle_time = self.fly_time + self.rest_time;
        let complete_cycles = time / cycle_time;
        let additional_time = min(time - (complete_cycles * cycle_time), self.fly_time);
        ((complete_cycles * self.fly_time) + additional_time) * self.speed
    }

    // Advanced forward one second
    fn step(&mut self) {
        match self.state {
            State::Flying(left) => {
                // Advance position
                self.position += self.speed;
                // Update state
                if left == 1 {
                    self.state = State::Resting(self.rest_time);
                } else {
                    self.state = State::Flying(left - 1);
                }
            }
            State::Resting(left) => {
                // Update state
                if left == 1 {
                    self.state = State::Flying(self.fly_time);
                } else {
                    self.state = State::Resting(left - 1);
                }
            }
        }
    }

    // Assigns one point if currently in the lead position
    fn score(&mut self, lead_position: u64) {
        if self.position == lead_position {
            self.score += 1;
        }
    }
}

// Calculates the winning score when racing over a set time period
fn calculate_score(input: &[Reindeer], time: u64) -> u64 {
    let mut input: Vec<Reindeer> = input.to_vec();
    for _ in 0..time {
        // Move each reindeer forward in time for one second
        input.iter_mut().for_each(Reindeer::step);
        // Work out how far ahead the leader is
        let current_best = input.iter().map(|r| r.position).max().unwrap();
        // Assign one point to all that are at that position
        input.iter_mut().for_each(|r| r.score(current_best));
    }

    // Return the winning score
    input.iter().map(|r| r.score).max().unwrap()
}

#[aoc_generator(day14)]
fn gen(input: &str) -> Vec<Reindeer> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> u64 {
    // Return the maximum distance travelled
    input.iter().map(|x| x.dist_travelled(2503)).max().unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &[Reindeer]) -> u64 {
    // Return the maximum score
    calculate_score(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_COMET: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";

    static EXAMPLE_INPUT: &str = indoc! {"
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "};

    #[test]
    fn test_part1_example() {
        assert_eq!(
            EXAMPLE_COMET
                .parse::<Reindeer>()
                .unwrap()
                .dist_travelled(1000),
            1120
        );
    }
    #[test]
    fn test_part2_example() {
        assert_eq!(calculate_score(&gen(EXAMPLE_INPUT), 1000), 689);
    }
}
