use crate::solutions::day18_duet::{Arg, Machine, Op};
use rayon::prelude::*;

#[aoc_generator(day23)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day23, part1)]
fn part1(input: &[Op]) -> usize {
    // Initialise the machine and then run until completion counting
    // the number of multiply operations that are executed
    let mut machine = Machine::new(0);
    let mut mul_count = 0;
    while machine.pc() < input.len() {
        let op = &input[machine.pc()];
        machine.exec(op);
        if let Op::Mul(_, _) = op {
            mul_count += 1;
        }
    }
    mul_count
}

#[aoc(day23, part2)]
fn part2(input: &[Op]) -> usize {
    // Through revere engineering it was found that the input program is trying to find all the
    // numbers in a range with a step that have factors excluding 1 or themselves.
    // The implementation is very slow so we instead execute it enough to get the parameters of
    // the calculation and then natively determine how many numbers in the range have any factors.

    // Init the machine and set register a(0) to 1;
    let mut machine = Machine::new(0);
    machine.registers[0] = 1;
    // Execute the first 8 instructions to calculate the parameters
    while machine.pc() < 8 {
        let op = &input[machine.pc()];
        machine.exec(op);
    }
    // Get the step size from the input instructions
    if let Op::Sub(_, Arg::Value(istep)) = input[30] {
        let step = isize::abs(istep).try_into().unwrap();
        // Get the range from the registers
        let from = machine.registers[1];
        let to = machine.registers[2];
        // Know we have all the parameters, we can check for factors
        return (from..to + 1)
            .into_par_iter()
            .step_by(step)
            .filter(|&n| {
                // For all numbers in the range, check to see if they have any factors (excluding 1 or themselves)
                (2..n).into_iter().any(|x| n % x == 0)
            })
            .count();
    }
    0
}
