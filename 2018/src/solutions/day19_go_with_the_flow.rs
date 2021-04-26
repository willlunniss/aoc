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
    0
}
