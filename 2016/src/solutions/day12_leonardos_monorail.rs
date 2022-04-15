use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ExecError {
    #[error("Cannot mutate non-register {arg:?}")]
    ArgNotARegister { arg: Arg },
    #[error("End of program")]
    EndOfProgram,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    Register(usize),
    Value(isize),
}

impl FromStr for Arg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().unwrap();
        if first.is_ascii_alphabetic() {
            Ok(Self::Register(first as usize - 'a' as usize))
        } else {
            Ok(Self::Value(s.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg),
    Out(Arg),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "cpy" => Ok(Self::Cpy(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "inc" => Ok(Self::Inc(parts[1].parse().unwrap())),
            "dec" => Ok(Self::Dec(parts[1].parse().unwrap())),
            "jnz" => Ok(Self::Jnz(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "tgl" => Ok(Self::Tgl(parts[1].parse().unwrap())),
            "out" => Ok(Self::Out(parts[1].parse().unwrap())),
            _ => unreachable!("{}", s),
        }
    }
}

impl Op {
    /// Toggles an `Op` in place
    fn toggle(&mut self) {
        *self = match self {
            Self::Inc(x) => Self::Dec(*x),
            Self::Dec(x) | Self::Tgl(x) | Self::Out(x) => Self::Inc(*x),
            Self::Cpy(x, y) => Self::Jnz(*x, *y),
            Self::Jnz(x, y) => Self::Cpy(*x, *y),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    registers: [isize; 4],
    pc: usize,
    memory: Vec<Op>,
}

impl Computer {
    /// Initialises a new computer
    pub fn new(memory: &[Op]) -> Self {
        Self {
            registers: [0; 4],
            pc: 0,
            memory: memory.to_owned(),
        }
    }

    /// Returns the value of an `Arg`
    pub const fn get(&self, arg: &Arg) -> isize {
        match arg {
            Arg::Register(r) => self.registers[*r],
            Arg::Value(v) => *v,
        }
    }

    /// Returns a mutable reference to a register
    ///
    /// Errors if `Arg` is not a register
    pub fn get_mut<'a>(&'a mut self, arg: &'a Arg) -> Result<&'a mut isize, ExecError> {
        match arg {
            Arg::Register(r) => Ok(&mut self.registers[*r]),
            Arg::Value(_) => Err(ExecError::ArgNotARegister { arg: *arg }),
        }
    }

    /// Execute the `Op` based on the current PC value
    ///
    /// Returns either:
    /// Some(value) in the case of an output instruction
    /// None for all other valid instructions
    pub fn exec(&mut self) -> Result<Option<isize>, ExecError> {
        let mut new_pc = self.pc + 1;
        let mut output = None;
        match *self.memory.get(self.pc).ok_or(ExecError::EndOfProgram)? {
            Op::Cpy(x, y) => *self.get_mut(&y)? = self.get(&x),
            Op::Inc(x) => *self.get_mut(&x)? += 1,
            Op::Dec(x) => *self.get_mut(&x)? -= 1,
            Op::Jnz(x, y) => {
                // If non-zero, jump
                if self.get(&x) != 0 {
                    //Calculate new PC value (based on original)
                    new_pc = (self.pc as isize + self.get(&y)) as usize;
                }
            }
            Op::Tgl(x) => {
                // Modify the target op in memory
                let target = self.pc as isize + self.get(&x);
                if target >= 0 && target < self.memory.len() as isize {
                    self.memory[target as usize].toggle();
                }
            }
            Op::Out(x) => {
                // Set the output value
                output = Some(self.get(&x));
            }
        }
        // Update PC
        self.pc = new_pc;
        // Return the optional output value
        Ok(output)
    }

    /// Runs until the end of the program
    pub fn run(&mut self) {
        while self.exec() != Err(ExecError::EndOfProgram) {}
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Op]) -> isize {
    let mut computer = Computer::new(input);
    computer.run();
    computer.registers[0]
}

#[aoc(day12, part2)]
fn part2(input: &[Op]) -> isize {
    let mut computer = Computer::new(input);
    computer.registers[2] = 1;
    computer.run();
    computer.registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    cpy 41 a
    inc a
    inc a
    dec a
    jnz a 2
    dec a
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 42);
    }
}
