use std::{convert::Infallible, str::FromStr};
use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumString)]
enum Op {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl Op {
    /// Executes an instruction returning the calculated result (that should be stored in register c)
    const fn execute<const SIZE: usize>(self, a: usize, b: usize, registers: &[usize; SIZE]) -> usize {
        match self {
            Self::addr => registers[a] + registers[b],
            Self::addi => registers[a] + b,
            Self::mulr => registers[a] * registers[b],
            Self::muli => registers[a] * b,
            Self::banr => registers[a] & registers[b],
            Self::bani => registers[a] & b,
            Self::borr => registers[a] | registers[b],
            Self::bori => registers[a] | b,
            Self::setr => registers[a],
            Self::seti => a,
            Self::gtir => (a > registers[b]) as usize,
            Self::gtri => (registers[a] > b) as usize,
            Self::gtrr => (registers[a] > registers[b]) as usize,
            Self::eqir => (a == registers[b]) as usize,
            Self::eqri => (registers[a] == b) as usize,
            Self::eqrr => (registers[a] == registers[b]) as usize,
        }
    }
}

#[derive(Debug)]
struct Instr {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instr {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse instruction details as an OP and 3 numbers e.g. seti 5 0 1
        let parts = s.split(' ').collect::<Vec<_>>();
        Ok(Self {
            op: Op::from_str(parts[0]).unwrap(),
            a: parts[1].parse().unwrap(),
            b: parts[2].parse().unwrap(),
            c: parts[3].parse().unwrap(),
        })
    }
}

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
    while registers[*ip_register] < instrs.len() {
        // Get the instruction to execute
        let instr = instrs.get(registers[*ip_register]).unwrap();
        // Execute the instruction, updating output register
        registers[instr.c] = instr.op.execute(instr.a, instr.b, &registers);
        // Advance the IP
        registers[*ip_register] += 1;
    }
    registers[0]
}

#[aoc(day19, part2)]
fn part2(input: &(usize, Vec<Instr>)) -> usize {
    0
}
