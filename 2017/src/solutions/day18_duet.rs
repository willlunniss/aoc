use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Arg {
    Register(usize),
    Value(isize),
}

impl FromStr for Arg {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().unwrap();
        if first.is_ascii_alphabetic() {
            Ok(Self::Register(first as usize - 'a' as usize))
        } else {
            Ok(Self::Value(s.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Snd(Arg),
    Set(Arg, Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Mod(Arg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg),
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "snd" => Ok(Self::Snd(parts[1].parse().unwrap())),
            "set" => Ok(Self::Set(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "add" => Ok(Self::Add(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "mul" => Ok(Self::Mul(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "mod" => Ok(Self::Mod(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            "rcv" => Ok(Self::Rcv(parts[1].parse().unwrap())),
            "jgz" => Ok(Self::Jgz(
                parts[1].parse().unwrap(),
                parts[2].parse().unwrap(),
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum ExecResult {
    ReceiveBlock,
    Send(isize),
    End,
}

#[derive(Debug)]
struct Machine {
    registers: [isize; 26],
    pc: usize,
    queue: VecDeque<isize>,
}

impl Machine {
    fn new(id: usize) -> Self {
        let mut m = Self {
            registers: vec![0; 26].try_into().unwrap(),
            pc: 0,
            queue: VecDeque::new(),
        };
        *m.get_mut(&"p".parse().unwrap()) = id.try_into().unwrap();
        m
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

    /// Queue a value (to be used by `Op::Rcv`)
    fn queue(&mut self, value: isize) {
        self.queue.push_back(value);
    }

    /// Execute a single `Op`
    fn exec(&mut self, op: &Op) -> Option<ExecResult> {
        // Assume it will fully execute
        self.pc += 1;
        match op {
            Op::Snd(x) => return Some(ExecResult::Send(self.get(x))),
            Op::Set(x, y) => *self.get_mut(x) = self.get(y),
            Op::Add(x, y) => *self.get_mut(x) += self.get(y),
            Op::Mul(x, y) => *self.get_mut(x) *= self.get(y),
            Op::Mod(x, y) => *self.get_mut(x) %= self.get(y),
            Op::Rcv(x) => {
                if let Some(v) = self.queue.pop_front() {
                    *self.get_mut(x) = v;
                } else {
                    // Blocked on needing a value, put the PC back
                    self.pc -= 1;
                    return Some(ExecResult::ReceiveBlock);
                }
            }
            Op::Jgz(x, y) => {
                // If non-zero, jump
                if self.get(x) > 0 {
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
        None
    }

    /// Track snd values and return the most recent one on the first rcv `Op`
    fn recover(&mut self, ops: &[Op]) -> Option<isize> {
        let mut last_frequency = 0;
        while self.pc < ops.len() {
            match self.exec(&ops[self.pc]) {
                Some(ExecResult::Send(freq)) => last_frequency = freq,
                Some(ExecResult::ReceiveBlock) => return Some(last_frequency),
                Some(ExecResult::End) => return None,
                None => {}
            }
        }
        None
    }

    /// Run until either a:
    /// * A value is sent
    /// * Blocked waiting to receive a value
    /// * Reached the end
    fn run(&mut self, ops: &[Op]) -> ExecResult {
        while self.pc < ops.len() {
            if let Some(result) = self.exec(&ops[self.pc]) {
                return result;
            }
        }
        ExecResult::End
    }
}

#[aoc_generator(day18)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Op]) -> isize {
    Machine::new(0).recover(input).unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &[Op]) -> usize {
    // Init two copies with different ids
    let (mut m0, mut m1) = (Machine::new(0), Machine::new(1));
    let mut m1_send_count = 0;
    loop {
        // Keep running them until they both stop
        let mut stopped = 0;
        match m0.run(input) {
            ExecResult::Send(v) => m1.queue(v),
            ExecResult::End | ExecResult::ReceiveBlock => {
                stopped += 1;
            }
        }
        match m1.run(input) {
            ExecResult::Send(v) => {
                m1_send_count += 1;
                m0.queue(v);
            }
            ExecResult::End | ExecResult::ReceiveBlock => {
                stopped += 1;
            }
        }
        // When they have both stopped, result is the number of m1 sends
        if stopped == 2 {
            return m1_send_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_1: &str = indoc!(
        "
        set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2
        "
    );

    static EXAMPLE_INPUT_2: &str = indoc!(
        "
        snd 1
        snd 2
        snd p
        rcv a
        rcv b
        rcv c
        rcv d
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT_1)), 4);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT_2)), 3);
    }
}
