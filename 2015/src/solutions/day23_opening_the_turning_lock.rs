use std::collections::HashMap;

use reformation::Reformation;

type Register = char;
type Offset = i32;
type Value = u32;

#[derive(Reformation, Eq, PartialEq, Debug, Copy, Clone)]
enum Instr {
    #[reformation(r"hlf {}")]
    Half(Register),
    #[reformation(r"tpl {}")]
    Triple(Register),
    #[reformation(r"inc {}")]
    Increment(Register),
    #[reformation(r"jmp {}")]
    Jump(Offset),
    #[reformation(r"jie {}, {}")]
    JumpIfEven(Register, Offset),
    #[reformation(r"jio {}, {}")]
    JumpIfOne(Register, Offset),
}

struct Computer {
    program: Vec<Instr>,
    registers: HashMap<Register, Value>,
    ip: i32,
}

impl Computer {
    fn new(program: Vec<Instr>) -> Self {
        Self {
            program,
            registers: [('a', 0), ('b', 0)].iter().copied().collect(),
            ip: 0,
        }
    }

    /// Run the program to completion
    fn run(&mut self) {
        // Keep running while pointing at a valid instruction
        while let Some(instr) = self.program.get(self.ip as usize) {
            let mut next_ip = self.ip + 1;
            match instr {
                Instr::Half(r) => *self.registers.get_mut(r).unwrap() /= 2,
                Instr::Triple(r) => *self.registers.get_mut(r).unwrap() *= 3,
                Instr::Increment(r) => *self.registers.get_mut(r).unwrap() += 1,
                Instr::Jump(offset) => next_ip = self.ip + offset,
                Instr::JumpIfEven(r, offset) => {
                    if self.registers.get(r).unwrap() % 2 == 0 {
                        next_ip = self.ip + offset;
                    }
                }
                Instr::JumpIfOne(r, offset) => {
                    if *self.registers.get(r).unwrap() == 1 {
                        next_ip = self.ip + offset;
                    }
                }
            };
            self.ip = next_ip;
        }
    }
}

#[aoc_generator(day23)]
fn gen(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| Instr::parse(line).unwrap())
        .collect()
}

#[aoc(day23, part1)]
fn part1(input: &Vec<Instr>) -> Value {
    let mut computer = Computer::new(input.clone());
    computer.run();
    computer.registers[&'b']
}

#[aoc(day23, part2)]
fn part2(input: &Vec<Instr>) -> Value {
    let mut computer = Computer::new(input.clone());
    computer.registers.insert('a', 1);
    computer.run();
    computer.registers[&'b']
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    inc b
    jio b, +2
    tpl b
    inc b
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 2);
    }
}
