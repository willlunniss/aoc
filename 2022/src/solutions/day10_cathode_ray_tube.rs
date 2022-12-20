use reformation::Reformation;
use utils::{
    grid::{MapGrid, Pos},
    ocr::OcrString,
};

#[derive(Reformation, Debug, Clone, Copy)]
enum Instr {
    #[reformation(r"noop")]
    Noop, // Takes 1 cycle
    #[reformation(r"addx {}")]
    AddX(isize), // Takes 2 cycles
}

struct Cpu {
    instructions: Vec<Instr>,
    pipeline: Option<Instr>,
    cycle: isize,
    ip: usize,
    register: isize,
}

impl Cpu {
    fn new(instructions: Vec<Instr>) -> Self {
        Self {
            instructions,
            pipeline: None,
            cycle: 1,
            ip: 0,
            register: 1,
        }
    }

    /// Executes a single cycle
    ///
    /// Returns false if the program has ended
    fn step(&mut self) -> bool {
        self.cycle += 1;
        // Check the pipeline first
        if let Some(instr) = self.pipeline {
            // Pipeline contains an instruction, execute it
            let Instr::AddX(value) = instr else {
                panic!("Pipeline should only contain AddX instructions");
            };
            self.register += value;
            self.pipeline = None;
            return true;
        }
        // Nothing in the pipeline, get the next instruction
        let Some(instr) = self.instructions.get(self.ip) else {
            return false; // End of instructions
        };
        self.ip += 1;
        match instr {
            Instr::Noop => {}                               // Do nothing
            Instr::AddX(_) => self.pipeline = Some(*instr), // Add to pipeline to complete next cycle
        }
        true // More to do
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<Instr> {
    input.lines().flat_map(Instr::parse).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Instr>) -> isize {
    let mut cpu = Cpu::new(input.clone());
    let mut signal_strength = Vec::new();
    while cpu.step() {
        if cpu.cycle >= 20 && (cpu.cycle - 20) % 40 == 0 {
            // Record the signal strength at specific cycles
            signal_strength.push(cpu.cycle * cpu.register);
        }
    }
    signal_strength.iter().sum()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Instr>) -> OcrString {
    let mut cpu = Cpu::new(input.clone());
    let mut crt = MapGrid::new();
    let mut crt_row: isize = 0;
    let mut crt_column: isize = 0;

    while cpu.step() {
        // Advance position
        crt_column += 1;
        if crt_column == 40 {
            // End of column, move to start of next row
            crt_column = 0;
            crt_row += 1;
            if crt_row == 6 {
                // End of rows, move back to start row
                crt_row = 0;
            }
        }
        // Check to see if the sprite aligns (+/- 1 row)
        if crt_column >= cpu.register - 1 && crt_column <= cpu.register + 1 {
            // Draw!
            crt.insert(Pos::from((crt_column, crt_row)), '#');
        }
    }
    crt.keys().map(|pos| (pos.x, pos.y)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 13140);
    }
}
