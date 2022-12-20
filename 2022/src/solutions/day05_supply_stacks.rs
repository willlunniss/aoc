use reformation::Reformation;

#[derive(Debug, Clone)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn new(cargo_section: &str) -> Self {
        let lines = cargo_section.lines().collect::<Vec<_>>();
        let count = lines // Get the number of stacks
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .count();
        let mut stacks = vec![Vec::new(); count];
        // Work backwards from the bottom to the top of stack
        for level in (0..lines.len() - 1).rev() {
            for stack in 0..count {
                let pos = 1 + (stack * 4);
                if let Some(item) = lines[level].chars().nth(pos) {
                    if item != ' ' {
                        // Add valid items to the stack
                        stacks[stack].push(item);
                    }
                }
            }
        }
        Self { stacks }
    }

    /// Returns the creates at the top of each stack
    fn top_creates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }

    /// Move one crates at a time between stacks
    fn move_single(&mut self, r: &Rearrangement) {
        for _i in 0..r.count {
            let item = self.stacks[r.from - 1].pop().unwrap();
            self.stacks[r.to - 1].push(item);
        }
    }

    /// Move multiple crates in one go between stacks
    fn move_multiple(&mut self, r: &Rearrangement) {
        let remove_from = self.stacks[r.from - 1].len() - r.count;
        let items = self.stacks[r.from - 1]
            .drain(remove_from..)
            .collect::<Vec<_>>();
        self.stacks[r.to - 1].extend(items);
    }
}

#[derive(Reformation, Debug, Clone, Copy)]
#[reformation(r"move {count} from {from} to {to}", fromstr = true)]
struct Rearrangement {
    count: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
fn gen(input: &str) -> (Cargo, Vec<Rearrangement>) {
    let (cargo, rearrangements) = input.split_once("\n\n").unwrap();
    (
        Cargo::new(cargo),
        rearrangements.lines().flat_map(str::parse).collect(),
    )
}

#[aoc(day5, part1)]
fn part1(input: &(Cargo, Vec<Rearrangement>)) -> String {
    let (mut cargo, rearrangements) = input.clone();
    //CrateMover 9000 - move a single crate at once
    for r in rearrangements {
        cargo.move_single(&r);
    }
    cargo.top_creates()
}

#[aoc(day5, part2)]
fn part2(input: &(Cargo, Vec<Rearrangement>)) -> String {
    let (mut cargo, rearrangements) = input.clone();
    // CrateMover 9001 - move multiple crates at once
    for r in rearrangements {
        cargo.move_multiple(&r);
    }
    cargo.top_creates()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), "CMZ");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), "MCD");
    }
}
