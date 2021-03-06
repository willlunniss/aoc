use crate::intcode::Intcode;

#[aoc(day5, part1)]
fn part1(input: &str) -> isize {
    let mut computer = Intcode::from(input);
    computer.inputs().push_back(1);
    computer.run();
    return *computer.outputs().iter().last().unwrap();
}

#[aoc(day5, part2)]
fn part2(input: &str) -> isize {
    let mut computer = Intcode::from(input);
    computer.inputs().push_back(5);
    computer.run();
    return *computer.outputs().iter().last().unwrap();
}