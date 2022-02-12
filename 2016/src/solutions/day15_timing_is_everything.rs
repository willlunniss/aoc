#[derive(Clone)]
struct Disc {
    id: usize,
    positions: usize,
    start: usize,
}

impl Disc {
    const fn new(id: usize, positions: usize, start: usize) -> Self {
        Self {
            id,
            positions,
            start,
        }
    }

    /// Returns true if the slot on the disc will be lined up (at position 0) at a given time
    const fn lined_up(&self, time: usize) -> bool {
        (self.id + self.start + time) % self.positions == 0
    }
}

#[aoc_generator(day15)]
fn gen(input: &str) -> Vec<Disc> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts = line.split(' ').collect::<Vec<_>>();
            Disc::new(
                index + 1,
                parts[3].parse().unwrap(),
                parts[11].chars().next().unwrap().to_digit(10).unwrap() as usize,
            )
        })
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &[Disc]) -> usize {
    (0..)
        .find(|time| input.iter().all(|disc| disc.lined_up(*time)))
        .unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &[Disc]) -> usize {
    let mut discs = input.to_vec();
    discs.push(Disc::new(input.len() + 1, 11, 0));
    (0..)
        .find(|time| discs.iter().all(|disc| disc.lined_up(*time)))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    Disc #1 has 5 positions; at time=0, it is at position 4.
    Disc #2 has 2 positions; at time=0, it is at position 1
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 5);
    }
}
