use itertools::Itertools;

#[derive(Debug, Clone)]
struct Generator {
    factor: usize,
    value: usize,
    strict_multiple: usize,
}

impl Generator {
    /// Initialises a new generator
    fn new(name: char, value: usize) -> Self {
        // Factor and multiple are different for A vs B
        let factor = match name {
            'A' => 16807,
            'B' => 48271,
            _ => unreachable!(),
        };
        let strict_multiple = match name {
            'A' => 4,
            'B' => 8,
            _ => unreachable!(),
        };
        Self {
            factor,
            value,
            strict_multiple,
        }
    }

    /// Returns the next value
    fn next(&mut self) -> usize {
        self.value = (self.value * self.factor) % 2_147_483_647;
        self.value
    }

    /// Returns the next value applying this generators criteria
    fn next_strict(&mut self) -> usize {
        loop {
            // Keep generating next values until it's a multiple of this generators specific value
            self.value = (self.value * self.factor) % 2_147_483_647;
            if self.value % self.strict_multiple == 0 {
                return self.value;
            }
        }
    }
}

#[aoc_generator(day15)]
fn gen(input: &str) -> (Generator, Generator) {
    input
        .lines()
        .map(|line| {
            // Parse lines such as: Generator A starts with 883
            let (name, value) = line.split_once(" starts with ").unwrap();
            Generator::new(name.chars().last().unwrap(), value.parse().unwrap())
        })
        .collect_tuple()
        .unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &(Generator, Generator)) -> usize {
    let (mut a, mut b) = input.clone();
    let mut pairs = 0;
    for _ in 0..40_000_000 {
        // Count the number of times the lower 16-bits match for each generators next values
        if (a.next() & 0xFFFF) == (b.next() & 0xFFFF) {
            pairs += 1;
        }
    }
    pairs
}

#[aoc(day15, part2)]
fn part2(input: &(Generator, Generator)) -> usize {
    let (mut a, mut b) = input.clone();
    let mut pairs = 0;
    for _ in 0..5_000_000 {
        // Count the number of times the lower 16-bits match for each generators next values
        // based on their strict criteria
        if (a.next_strict() & 0xFFFF) == (b.next_strict() & 0xFFFF) {
            pairs += 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
    Generator A starts with 65
    Generator B starts with 8921
    "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 588);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 309);
    }
}
