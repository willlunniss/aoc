use strum_macros::EnumString;

#[derive(Debug, Clone, Copy)]
enum Param {
    Variable(usize),
    Number(isize),
}

impl Param {
    fn from(s: &str) -> Self {
        match s {
            "w" => Self::Variable(0),
            "x" => Self::Variable(1),
            "y" => Self::Variable(2),
            "z" => Self::Variable(3),
            _ => Self::Number(s.parse().unwrap()),
        }
    }

    fn reg(&self) -> usize {
        match self {
            Self::Variable(r) => *r,
            Self::Number(_) => panic!("Not a variable"),
        }
    }

    const fn value(&self, registers: &[isize; 4]) -> isize {
        match self {
            Self::Variable(r) => registers[*r],
            Self::Number(n) => *n,
        }
    }
}

#[derive(Debug, EnumString, Clone, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone)]
struct Instr {
    op: Op,
    a: Param,
    b: Option<Param>,
}

impl Instr {
    fn from(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        Self {
            op: parts[0].parse().unwrap(),
            a: Param::from(parts[1]),
            b: if parts.len() == 3 {
                Some(Param::from(parts[2]))
            } else {
                None
            },
        }
    }
}

struct Alu {
    registers: [isize; 4],
    input: Vec<isize>,
}

impl Alu {
    fn new(input: Vec<isize>) -> Self {
        Self {
            registers: [0, 0, 0, 0],
            input,
        }
    }

    fn value(&self, register: &str) -> isize {
        Param::from(register).value(&self.registers)
    }

    fn execute(&mut self, instr: &Instr) {
        match instr.op {
            Op::Inp => self.registers[instr.a.reg()] = self.input.pop().unwrap(),
            Op::Add => self.registers[instr.a.reg()] += instr.b.unwrap().value(&self.registers),
            Op::Mul => self.registers[instr.a.reg()] *= instr.b.unwrap().value(&self.registers),
            Op::Div => self.registers[instr.a.reg()] /= instr.b.unwrap().value(&self.registers),
            Op::Mod => {
                self.registers[instr.a.reg()] =
                    self.registers[instr.a.reg()] % instr.b.unwrap().value(&self.registers);
            }
            Op::Eql => {
                self.registers[instr.a.reg()] =
                    if self.registers[instr.a.reg()] == instr.b.unwrap().value(&self.registers) {
                        1
                    } else {
                        0
                    }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum StackMode {
    Push,
    Pop,
}

#[derive(Debug)]
struct DigitOp {
    mode: StackMode,
    a: isize,
    b: isize,
}

impl DigitOp {
    fn new(instrs: &[Instr]) -> Self {
        assert_eq!(instrs.len(), 18);
        assert_eq!(instrs[4].op, Op::Div);
        Self {
            mode: if instrs[4].b.unwrap().value(&[0; 4]) == 26 {
                StackMode::Pop
            } else {
                StackMode::Push
            },
            a: instrs[5].b.unwrap().value(&[0; 4]),
            b: instrs[15].b.unwrap().value(&[0; 4]),
        }
    }
}

/// Validates a model number against MONAD program
fn validate(monad: &[Instr], model: Vec<isize>) -> bool {
    let mut input = model;
    input.reverse(); // Send in MSB to LSB
    let mut alu = Alu::new(input);
    for instr in monad.iter() {
        alu.execute(instr);
    }
    alu.value("z") == 0
}

/// Finds the maximum or minimum model number
fn find_model(monad: &[Instr], maximise: bool) -> Option<usize> {
    // Group into the different operations that applied to the digits
    // MONAD applies a series of stack push and pop
    // Need to find values such that the matching:
    //  push[i] + a[i] + b[j] = pop[j]
    // Which can be turned into
    //         push[i] + diff = pop[j]
    //               z + diff = y
    //         digit[i] = z
    //         digit[j] = y
    let grouped = monad
        .chunks(18)
        .map(|x| DigitOp::new(x))
        .collect::<Vec<_>>();

    let mut diffs = Vec::new();
    let mut digits = vec![0; 14];
    for (j, x) in grouped.iter().enumerate() {
        if x.mode == StackMode::Push {
            // Add digit to the stack
            diffs.push((j, x.b));
        } else {
            // Remove a digit from the stack
            // Can now work out optimum value for the digits
            let (i, v) = diffs.pop().unwrap();
            let diff = v + x.a;
            // i will always be more significant than j
            // so always prioritise getting i as close to min/max as possible
            let z = if maximise {
                // Try to make i the highest value
                if diff > 0 {
                    9 - diff
                } else {
                    9
                }
            } else {
                // Try to make i the lowest value
                if diff < 0 {
                    1 - diff
                } else {
                    1
                }
            };
            let y = z + diff;
            digits[i] = z;
            digits[j] = y;
        }
    }
    // Just to be sure, validate the result
    let result = digits
        .iter()
        .fold(0, |acc, x| (acc * 10) + x)
        .try_into()
        .unwrap();
    if validate(monad, digits) {
        Some(result)
    } else {
        None
    }
}

#[aoc_generator(day24)]
fn gen(input: &str) -> Vec<Instr> {
    input.lines().map(|line| Instr::from(line)).collect()
}

#[aoc(day24, part1)]
fn part1(instrs: &[Instr]) -> Option<usize> {
    find_model(instrs, true)
}

#[aoc(day24, part2)]
fn part2(instrs: &[Instr]) -> Option<usize> {
    find_model(instrs, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example_1() {
        let mut alu = Alu::new([56].to_vec());
        alu.execute(&Instr::from("inp x"));
        alu.execute(&Instr::from("mul x -1"));
        assert_eq!(alu.value("x"), -56);
    }

    #[test]
    fn test_part1_example_2() {
        let mut alu = Alu::new([9, 3].to_vec());
        alu.execute(&Instr::from("inp z"));
        alu.execute(&Instr::from("inp x"));
        alu.execute(&Instr::from("mul z 3"));
        alu.execute(&Instr::from("eql z x"));
        assert_eq!(alu.value("z"), 1);

        let mut alu = Alu::new([4, 3].to_vec());
        alu.execute(&Instr::from("inp z"));
        alu.execute(&Instr::from("inp x"));
        alu.execute(&Instr::from("mul z 3"));
        alu.execute(&Instr::from("eql z x"));
        assert_eq!(alu.value("z"), 0);
    }

    #[test]
    fn test_part1_example_3() {
        let mut alu = Alu::new([5].to_vec());
        alu.execute(&Instr::from("inp w"));
        alu.execute(&Instr::from("add z w"));
        alu.execute(&Instr::from("mod z 2"));
        alu.execute(&Instr::from("div w 2"));
        alu.execute(&Instr::from("add y w"));
        alu.execute(&Instr::from("mod y 2"));
        alu.execute(&Instr::from("div w 2"));
        alu.execute(&Instr::from("add x w"));
        alu.execute(&Instr::from("mod x 2"));
        alu.execute(&Instr::from("div w 2"));
        alu.execute(&Instr::from("mod w 2"));
        assert_eq!(alu.value("z"), 1);
        assert_eq!(alu.value("y"), 0);
        assert_eq!(alu.value("x"), 1);
        assert_eq!(alu.value("w"), 0);
    }
}
