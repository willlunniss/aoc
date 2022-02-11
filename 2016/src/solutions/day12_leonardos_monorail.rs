use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
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
            _ => unreachable!("{}", s),
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    registers: [isize; 4],
    pc: usize,
}

impl Computer {
    const fn new() -> Self {
        Self {
            registers: [0; 4],
            pc: 0,
        }
    }

    const fn get(&self, arg: &Arg) -> isize {
        match arg {
            Arg::Register(r) => self.registers[*r],
            Arg::Value(v) => *v,
        }
    }

    fn get_mut<'a>(&'a mut self, arg: &'a Arg) -> &'a mut isize {
        match arg {
            Arg::Register(r) => &mut self.registers[*r],
            Arg::Value(_) => panic!(),
        }
    }

    /// Execute a single `Op`
    pub fn exec(&mut self, op: &Op) {
        // Assume it will fully execute
        self.pc += 1;
        match op {
            Op::Cpy(x, y) => *self.get_mut(y) = self.get(x),
            Op::Inc(x) => *self.get_mut(x) += 1,
            Op::Dec(x) => *self.get_mut(x) -= 1,
            Op::Jnz(x, y) => {
                // If non-zero, jump
                if self.get(x) != 0 {
                    // Update the PC (account for it already having been advanced by 1)
                    let jump = self.get(y);
                    if jump > 0 {
                        self.pc += jump as usize - 1;
                    } else {
                        self.pc -= isize::abs(jump) as usize + 1;
                    }
                }
            }
        }
    }

    pub fn run(&mut self, ops: &[Op]) {
        while self.pc < ops.len() {
            self.exec(&ops[self.pc]);
        }
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Op]) -> isize {
    let mut computer = Computer::new();
    computer.run(input);
    computer.registers[0]
}

#[aoc(day12, part2)]
fn part2(input: &[Op]) -> isize {
    let mut computer = Computer::new();
    computer.registers[2] = 1;
    computer.run(input);
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
