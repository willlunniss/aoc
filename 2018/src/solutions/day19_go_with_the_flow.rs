use crate::chronal_device::Instr;

#[aoc_generator(day19)]
fn gen(input: &str) -> (usize, Vec<Instr>) {
    let mut lines = input.lines();
    let ip_register = lines.next().unwrap()[4..].parse().unwrap();
    let instrs = lines.map(|line| line.parse().unwrap()).collect();
    (ip_register, instrs)
}

#[aoc(day19, part1)]
fn part1(input: &(usize, Vec<Instr>)) -> usize {
    let (ip_register, instrs) = input;
    // Start with registers initialised to 0
    let mut registers = [0, 0, 0, 0, 0, 0];
    // Keep executing so long as the IP is within the bounds of the valid instructions
    while let Some(instr) = instrs.get(registers[*ip_register]) {
        // Execute the instruction
        instr.execute(&mut registers);
        // Advance the IP
        registers[*ip_register] += 1;
    }
    registers[0]
}

#[aoc(day19, part2)]
fn part2(input: &(usize, Vec<Instr>)) -> usize {
    // Part 2 would take far too long to run to completion if we just run it
    // so had to inspect it to work out what it was trying to do
    //
    // The input program is doing two things:
    // * Setup phase: 0,17-25
    //   Calculate a target number (quick). This number is much larger for part 2
    //   than it is for part 1 which is what makes the second phase so slow
    //
    // * Nested loop phase: 1-16
    //   Find all factors of the target number and sum them up (very slow)
    //
    // If we run just the setup phase we can get the target number and then
    // sum up all it's factors in significantly less time

    // Run the setup phase of the program to find the target number

    let (ip_register, instrs) = input;
    // r0 is set to 1 for part 2, all others still at 0
    let mut registers = [1, 0, 0, 0, 0, 0];
    // Keep executing so long as the IP is within the bounds of the valid instructions
    while let Some(instr) = instrs.get(registers[*ip_register]) {
        // Execute the instruction
        instr.execute(&mut registers);
        // When we execute instr 33 (addr a b c), the target number will end up in c
        if registers[*ip_register] == 33 {
            let target = registers[instr.c];
            // Quickly sum up all numbers that are a factor of the target number
            return (1..=target).filter(|&x| target % x == 0).sum();
        }
        // Advance the IP
        registers[*ip_register] += 1;
    }
    0
}
