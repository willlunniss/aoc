use std::{convert::Infallible, str::FromStr};
use strum_macros::{EnumIter, EnumString};

/// Operation for the chronal wrist device as specified in <https://adventofcode.com/2018/day/16>
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumString, EnumIter)]
pub enum Op {
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
    #[must_use]
    pub const fn execute<const SIZE: usize>(
        self,
        a: usize,
        b: usize,
        registers: &[usize; SIZE],
    ) -> usize {
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

/// Chronal wrist device instruction consisting of an operation, two inputs and one output
#[derive(Debug)]
pub struct Instr {
    pub op: Op,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Instr {
    /// Executes the instruction given the supplied `registers` (updating them with the result)
    pub fn execute<const SIZE: usize>(&self, registers: &mut [usize; SIZE]) {
        registers[self.c] = self.op.execute(self.a, self.b, registers)
    }
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
