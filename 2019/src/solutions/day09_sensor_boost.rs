use crate::intcode::Intcode;

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    let mut boost = Intcode::from_with(input, 1024 * 1024);
    boost.inputs().push_back(1);
    boost.run();
    return boost.outputs().pop_front().unwrap();
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
    let mut boost = Intcode::from_with(input, 1024 * 1024);
    boost.inputs().push_back(2);
    boost.run();
    return boost.outputs().pop_front().unwrap();
}
