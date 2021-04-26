use crate::chronal_device::{Instr, Op};
use std::collections::HashSet;

#[aoc_generator(day21)]
fn gen(input: &str) -> (usize, Vec<Instr>) {
    let mut lines = input.lines();
    let ip_register = lines.next().unwrap()[4..].parse().unwrap();
    let instrs = lines.map(|line| line.parse().unwrap()).collect();
    (ip_register, instrs)
}

#[aoc(day21, part1)]
fn part1(input: &(usize, Vec<Instr>)) -> usize {
    let (ip_register, instrs) = input;
    // Start with registers initialised to 0
    let mut registers = [0, 0, 0, 0, 0, 0];
    // Keep executing so long as the IP is within the bounds of the valid instructions
    while let Some(instr) = instrs.get(registers[*ip_register]) {
        if instr.op == Op::eqrr && instr.b == 0 {
            // Wait until we find the first (and only) instruction that reads the value from register 0
            // Set register 0 to be equal to the value in the other register so that this instruction
            // results in the IP being advanced and then the program to halt
            registers[0] = registers[instr.a];
        }
        // Execute the instruction
        instr.execute(&mut registers);
        // Advance the IP
        registers[*ip_register] += 1;
    }
    // Must have successfully halted, return the value we used for register 0
    registers[0]
}

#[aoc(day21, part2)]
fn part2(input: &(usize, Vec<Instr>)) -> usize {
    let (ip_register, instrs) = input;
    // FIXME: Part 2 is far too slow from fully executing the program
    // Start with registers initialised to 0
    let mut registers = [0, 0, 0, 0, 0, 0];
    let mut history = HashSet::new();
    let mut last = 0;
    // Keep executing so long as the IP is within the bounds of the valid instructions
    while let Some(instr) = instrs.get(registers[*ip_register]) {
        if instr.op == Op::eqrr && instr.b == 0 {
            // Wait until we find the first (and only) instruction that reads the value from register 0
            // Make a note of the value in the other register
            // Once we see it again assume we have seen them all then return the one we saw last
            // as being the one that will cause the program to run the longest
            if !history.insert(registers[instr.a]) {
                return last;
            }
            last = registers[instr.a];
        }
        // Execute the instruction
        instr.execute(&mut registers);
        // Advance the IP
        registers[*ip_register] += 1;
    }
    // Don't expect to get here
    0
}
