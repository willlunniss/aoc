use itertools::{Either, Itertools, MinMaxResult};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Storage {
    Output(usize),
    Bot(usize),
}

impl Storage {
    fn new(kind: &str, id: &str) -> Self {
        match kind {
            "output" => Self::Output(id.parse().unwrap()),
            "bot" => Self::Bot(id.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    lower: Storage,
    higher: Storage,
}

#[aoc_generator(day10)]
fn gen(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Instruction>) {
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut instructions = HashMap::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "value" => {
                // EXAMPLE: value 5 goes to bot 2
                bots.entry(parts[5].parse().unwrap())
                    .or_default()
                    .push(parts[1].parse().unwrap());
            }
            "bot" => {
                // EXAMPLE: bot 1 gives low to output 1 and high to bot 0
                instructions.insert(
                    parts[1].parse().unwrap(),
                    Instruction {
                        lower: Storage::new(parts[5], parts[6]),
                        higher: Storage::new(parts[10], parts[11]),
                    },
                );
            }
            _ => unreachable!(),
        }
    }
    (bots, instructions)
}

/// Simulates the bots re-distributing chips amongst themselves and the output bins
///
/// Part1: Returns Left(id of the bot that compares two specific chip values)
/// Part2: Returns Right(product of output bins 0, 1 and 2)
fn simulate(
    bots: &HashMap<usize, Vec<usize>>,
    instructions: &HashMap<usize, Instruction>,
    target_comparison: MinMaxResult<&usize>,
) -> Either<usize, usize> {
    let mut outputs = HashMap::new();
    let mut bots = bots.clone();
    // Keep going while there are bots holding 2 chips
    while let Some(bot) = bots
        .iter()
        .find_map(|(bot, chips)| (chips.len() == 2).then(|| *bot))
    {
        // Get the min/max values for the chips
        let min_max = bots.get(&bot).unwrap().iter().minmax();
        if min_max == target_comparison {
            // Part 1: Id of the bot that made the comparison
            return Either::Left(bot);
        } else if let MinMaxResult::MinMax(&min, &max) = min_max {
            // Otherwise process as per instructions
            let instruction = instructions.get(&bot).unwrap();
            for (value, storage) in [(min, &instruction.lower), (max, &instruction.higher)] {
                match storage {
                    Storage::Bot(target) => {
                        bots.entry(*target).or_default().push(value);
                    }
                    Storage::Output(target) => {
                        outputs.insert(*target, value);
                    }
                }
            }
            bots.remove(&bot);
        }
    }
    // Part 2: product of outputs 0, 1 and 2
    Either::Right((0..=2).map(|x| outputs.get(&x).unwrap()).product())
}

#[aoc(day10, part1)]
fn part1(input: &(HashMap<usize, Vec<usize>>, HashMap<usize, Instruction>)) -> usize {
    let (bots, instructions) = input;
    let target = if instructions.len() > 3 {
        MinMaxResult::MinMax(&17, &61)
    } else {
        MinMaxResult::MinMax(&2, &5) // Test mode
    };
    simulate(bots, instructions, target).left().unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &(HashMap<usize, Vec<usize>>, HashMap<usize, Instruction>)) -> usize {
    let (bots, instructions) = input;
    simulate(bots, instructions, MinMaxResult::NoElements)
        .right()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    value 5 goes to bot 2
    bot 2 gives low to bot 1 and high to bot 0
    value 3 goes to bot 1
    bot 1 gives low to output 1 and high to bot 0
    bot 0 gives low to output 2 and high to output 0
    value 2 goes to bot 2
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 30);
    }
}
