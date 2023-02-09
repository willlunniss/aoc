use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;
use reformation::Reformation;

#[derive(Reformation, Debug, Clone, Copy)]
enum Operation {
    #[reformation(r"old \+ old")]
    Double,
    #[reformation(r"old \* old")]
    Square,
    #[reformation(r"old \+ {}")]
    Add(u64),
    #[reformation(r"old \* {}")]
    Mul(u64),
}

impl Operation {
    /// Applies the operation to an item, returning it's new worry level
    const fn apply(&self, item: u64) -> u64 {
        match self {
            Self::Double => item * 2,
            Self::Square => item * item,
            Self::Add(value) => item + *value,
            Self::Mul(value) => item * *value,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    destination: [usize; 2],
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .lines()
            .map(|line| line.trim().split(&[' ', ','][..]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self {
            items: parts[1][2..]
                .iter()
                .flat_map(|item| item.parse::<u64>())
                .collect(),
            operation: Operation::parse(&parts[2][3..].join(" ")).unwrap(),
            test: parts[3][3].parse().unwrap(),
            destination: [parts[4][5].parse().unwrap(), parts[5][5].parse().unwrap()],
        })
    }
}

impl Monkey {
    /// Inspects the all items returning (destination, item)
    fn inspect<const RELIEF_FACTOR: u64>(&mut self, worry_limit: u64) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|item| {
                // Calculate the new worry level based on the operation, the amount of relief we
                // get after each item is inspected, and then warp it if it goes over the max limit
                let worry_level = (self.operation.apply(item) / RELIEF_FACTOR) % worry_limit;
                // Which monkey gets it next depends on the outcome of the test
                (
                    self.destination[usize::from(worry_level % self.test != 0)],
                    worry_level,
                )
            })
            .collect_vec()
    }
}

/// Simulates the monkeys playing keep away for a set number of rounds
fn keep_away<const RELIEF_FACTOR: u64>(monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut monkeys = monkeys;
    // If we are too worried and go for long enough then worried level will become unmanageable
    // Calculate the maximum worry that we need to track an item as having
    // while ensuring that each monkey's test's still apply
    let worry_limit = monkeys.iter().map(|m| m.test).product();
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            // Get all items that this monkey throws after inspecting them
            let thrown = monkeys[m].inspect::<RELIEF_FACTOR>(worry_limit);
            inspections[m] += thrown.len();
            // Re-distribute them to the other monkeys
            for (dest, item) in &thrown {
                monkeys[*dest].items.push_back(*item);
            }
        }
    }
    // Calculate the monkey business based on the two most active monkeys
    inspections.iter().sorted().rev().take(2).product()
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Monkey> {
    input.split("\n\n").flat_map(str::parse).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> usize {
    keep_away::<3>(input.to_vec(), 20)
}

#[aoc(day11, part2)]
fn part2(input: &[Monkey]) -> usize {
    keep_away::<1>(input.to_vec(), 10000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
  Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 10605);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 2_713_310_158);
    }
}
