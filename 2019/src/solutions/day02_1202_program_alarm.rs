use crate::intcode::Intcode;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<isize> {
    return input
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
}

#[aoc(day2, part1)]
fn part1(input: &Vec<isize>) -> isize {
    let mut computer = Intcode::new(input);
    computer.set_mem(1, 12);
    computer.set_mem(2, 2);
    computer.run();
    return computer.get_mem(0);
}

#[aoc(day2, part2)]
fn part2(input: &Vec<isize>) -> isize {
    // Find what inputs are needed to produce 19690720
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Intcode::new(input);
            computer.set_mem(1, noun);
            computer.set_mem(2, verb);
            computer.run();
            if computer.get_mem(0) == 19690720 {
                return (100 * noun) + verb;
            }
        }
    }
    return 0;
}
