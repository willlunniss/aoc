use crate::solutions::day12_leonardos_monorail::{Arg, Computer, Op};

#[aoc_generator(day23)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day23, part1)]
fn part1(input: &[Op]) -> isize {
    let mut computer = Computer::new(input);
    *computer.get_mut(&Arg::Register(0)).unwrap() = 7;
    computer.run();
    computer.get(&Arg::Register(0))
}

#[aoc(day23, part2)]
fn part2(input: &[Op]) -> isize {
    let mut computer = Computer::new(input);
    *computer.get_mut(&Arg::Register(0)).unwrap() = 12;
    computer.run();
    computer.get(&Arg::Register(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    cpy 2 a
    tgl a
    tgl a
    tgl a
    cpy 1 a
    dec a
    dec a
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 3);
    }
}
