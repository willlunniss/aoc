use digits_iterator::DigitsExtension;
use std::collections::VecDeque;

/// AOC 2019 Intcode implementation
///
/// # Examples
/// ```
/// # use aoc_2019::intcode::Intcode;
/// // Our sample program
/// let program : Vec<isize> = [1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec();
/// // Create a new instance using the program
/// let mut computer = Intcode::new(program);
/// // Run the loaded program
/// computer.run();
/// // Get and check the result (this program stores it in address 0)
/// let result = computer.get_mem(0);
/// assert_eq!(result, 30);
/// ```
#[derive(Clone)]
pub struct Intcode {
    mem: Vec<isize>,
    ip: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
    relative_base: isize,
}

/// Result of executing an instruction
enum Result {
    SetIP(usize),
    InputRequired,
    Exit,
}

/// Type of operation and associated (static) configuration
#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

impl Op {
    /// Creates a new op from an opcode
    fn new(opcode: usize) -> Self {
        use Op::*;
        match opcode {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => AdjustRelativeBase,
            99 => Halt,
            _ => panic!("Unexpected opcode {}", opcode),
        }
    }

    /// Gets the instruction implementation
    fn instr_impl(&self) -> fn(&mut Intcode, param_addrs: &[usize]) -> Option<Result> {
        use Op::*;
        match self {
            Add => Intcode::instr_1_add,
            Multiply => Intcode::instr_2_multiply,
            Input => Intcode::instr_3_input,
            Output => Intcode::instr_4_output,
            JumpIfTrue => Intcode::instr_5_jump_if_true,
            JumpIfFalse => Intcode::instr_6_jump_if_false,
            LessThan => Intcode::instr_7_less_than,
            Equals => Intcode::instr_8_equals,
            AdjustRelativeBase => Intcode::instr_9_adjust_relative_base,
            Halt => Intcode::instr_99_halt,
        }
    }

    /// Gets the number of parameters used
    const fn params(&self) -> usize {
        use Op::*;
        match self {
            Add | Multiply | LessThan | Equals => 3,
            JumpIfTrue | JumpIfFalse => 2,
            Input | Output | AdjustRelativeBase => 1,
            Halt => 0,
        }
    }
}

/// Instruction representation including dynamic configuration
#[derive(Debug)]
struct Instr {
    // The actual operation to be performed
    op: Op,
    // Modes of the parameters to use
    param_modes: [u8; 3],
}

impl Intcode {
    /// Initialises a new Intcode computer with the supplied program
    pub fn new(program: Vec<isize>) -> Self {
        Self {
            mem: program,
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            relative_base: 0,
        }
    }

    /// Initialises a new Intcode computer with the supplied program with the specified memory size
    /// (anything beyond the length of the program will be initialised to 0)
    pub fn new_with(program: &Vec<isize>, memory_size: usize) -> Self {
        // Create a new instance with memory of the required size set to 0
        let mut result = Self::new(vec![0; memory_size]);
        // Then load our program into it
        for (id, val) in program.iter().enumerate() {
            result.mem[id] = *val;
        }
        result
    }

    /// Initialises a new Intcode computer from a comma separated string of integers
    pub fn from(program: &str) -> Self {
        Self {
            mem: program
                .split(',')
                .map(|i| i.parse::<isize>().unwrap())
                .collect(),
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            relative_base: 0,
        }
    }

    /// Initialises a new Intcode computer from a comma separated string of integers
    /// with the specified memory size (anything beyond the length of the program will be initialised to 0)
    pub fn from_with(program: &str, memory_size: usize) -> Self {
        // Create a new instance with memory of the required size set to 0
        let mut result = Self::new(vec![0; memory_size]);
        // Then load our program into it
        for (id, val) in program
            .split(',')
            .map(|i| i.parse::<isize>().unwrap())
            .enumerate()
        {
            result.mem[id] = val;
        }
        result
    }

    /// Sets the value at the supplied memory address
    pub fn set_mem(&mut self, addr: usize, value: isize) {
        self.mem[addr] = value;
    }

    /// Gets the value at the supplied memory address
    pub fn get_mem(&self, addr: usize) -> isize {
        self.mem[addr]
    }

    /// Runs the loaded program until completion (returns true)
    /// or it is blocked on an input (returns false)
    /// (In which case an input should be supplied and run should be called again to resume)
    pub fn run(&mut self) -> bool {
        loop {
            match self.execute(self.ip) {
                Result::SetIP(ip) => self.ip = ip,
                Result::InputRequired => return false,
                Result::Exit => return true,
            }
        }
    }

    /// Gets the input queue
    pub fn inputs(&mut self) -> &mut VecDeque<isize> {
        &mut self.inputs
    }

    /// Inputs the supplied line of text
    ///
    /// Inputs the line as ASCII chars and terminates with a new line
    pub fn inputln(&mut self, line: &str) {
        for c in line.chars() {
            self.inputs.push_back(c as isize);
        }
        self.inputs.push_back(10); // New line
    }

    /// Prints outputs as ASCII chars
    pub fn print_outputs_as_ascii(&mut self) {
        self.outputs()
            .drain(0..) // Consume all outputs and treat as chars
            .for_each(|c| print!("{}", c as u8 as char));
    }

    /// Gets the output queue
    pub fn outputs(&mut self) -> &mut VecDeque<isize> {
        &mut self.outputs
    }

    /// Executes a single instruction at the specified IP
    fn execute(&mut self, ip: usize) -> Result {
        // Decode the instruction
        let instr = Self::decode(self.mem[ip]);
        // Get the reference to the implementation
        let instr_impl = instr.op.instr_impl();
        // Get the parameters
        let param_addrs = self.param_addrs(ip, instr.op.params(), &instr.param_modes);
        // Execute it
        let result = instr_impl(self, &param_addrs);
        if result.is_none() {
            // Simple instructions with no result
            // Just advance the instruction pointer by 1 + num params
            return Result::SetIP(ip + 1 + instr.op.params());
        }
        result.unwrap()
    }

    /// Decodes an instruction
    fn decode(encoded: isize) -> Instr {
        // Extract the opcode and param modes
        let mut opcode = 0;
        let mut param_modes = [0, 0, 0];
        for (idx, digit) in encoded
            .digits()
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .enumerate()
        {
            match idx {
                0 => opcode += digit,
                1 => opcode += 10 * digit,
                _ => param_modes[idx - 2] = *digit as u8,
            }
        }
        // Lookup the opcode
        let op = Op::new(opcode as usize);
        Instr { op, param_modes }
    }

    /// Gets the address of parameters for an instruction taking into account the different parameter modes
    fn param_addrs(&self, ip: usize, count: usize, modes: &[u8]) -> Vec<usize> {
        (0..count)
            .map(|param| {
                let addr = ip + 1 + param;
                match modes[param] {
                    0 => {
                        // Position mode - return the value at the address
                        self.mem[addr] as usize
                    }
                    1 => {
                        // Immediate mode - return the address directly
                        addr
                    }
                    2 => {
                        // Relative mode - return the relative base + the value at the address
                        (self.relative_base + self.mem[addr]) as usize
                    }
                    _ => {
                        panic!("Unexpected parameter mode {}", modes[param])
                    }
                }
            })
            .collect()
    }

    /// Gets the next available input
    fn input(&mut self) -> Option<isize> {
        self.inputs.pop_front()
    }

    /// Outputs the supplied value
    fn output(&mut self, value: isize) {
        self.outputs.push_back(value);
    }

    /// Sets the 3rd parameter to the 1st plus the 2nd
    fn instr_1_add(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.mem[param_addrs[2]] = self.mem[param_addrs[0]] + self.mem[param_addrs[1]];
        None
    }

    /// Sets the 3rd parameter to the 1st multiplied by the 2nd
    fn instr_2_multiply(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.mem[param_addrs[2]] = self.mem[param_addrs[0]] * self.mem[param_addrs[1]];
        None
    }

    /// Fetches an input and stores it in the 1st parameter
    /// If no input is available returns `InputRequired so that one can
    /// be provided before resuming
    fn instr_3_input(&mut self, param_addrs: &[usize]) -> Option<Result> {
        if let Some(value) = self.input() {
            // Input value is available, write it to memory
            self.mem[param_addrs[0]] = value;
            None
        } else {
            // No input available
            // The IP will not get updated so that next time run() is called
            // the program will be resumed and this instruction will be re-tried
            Some(Result::InputRequired)
        }
    }

    /// Outputs the 1st parameter
    fn instr_4_output(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.output(self.mem[param_addrs[0]]);
        None
    }

    /// Sets the IP to the value of the 2nd parameter if the 1st is not-equal to 0
    fn instr_5_jump_if_true(&mut self, param_addrs: &[usize]) -> Option<Result> {
        if self.mem[param_addrs[0]] != 0 {
            Some(Result::SetIP(self.mem[param_addrs[1]] as usize))
        } else {
            None
        }
    }

    /// Sets the IP to the value of the 2nd parameter if the 1st is equal to 0
    fn instr_6_jump_if_false(&mut self, param_addrs: &[usize]) -> Option<Result> {
        if self.mem[param_addrs[0]] == 0 {
            Some(Result::SetIP(self.mem[param_addrs[1]] as usize))
        } else {
            None
        }
    }

    /// Sets the 3rd parameter to 1 if the 1st is less than the second, else sets to 0
    fn instr_7_less_than(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.mem[param_addrs[2]] = if self.mem[param_addrs[0]] < self.mem[param_addrs[1]] {
            1
        } else {
            0
        };
        None
    }

    /// Sets the 3rd parameter to 1 if the 1st and second are equal, else sets to 0
    fn instr_8_equals(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.mem[param_addrs[2]] = if self.mem[param_addrs[0]] == self.mem[param_addrs[1]] {
            1
        } else {
            0
        };
        None
    }

    /// Adjusts the relative base by the amount in the 1st parameter
    fn instr_9_adjust_relative_base(&mut self, param_addrs: &[usize]) -> Option<Result> {
        self.relative_base += self.mem[param_addrs[0]];
        None
    }

    /// Unconditionally causes the program to exit
    fn instr_99_halt(&mut self, _: &[usize]) -> Option<Result> {
        Some(Result::Exit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instr_1_add() {
        let mut computer = Intcode::new([1, 3, 3, 3].to_vec());
        computer.execute(0);
        assert_eq!(computer.get_mem(3), 6);
    }

    #[test]
    fn test_instr_2_multi() {
        let mut computer = Intcode::new([2, 3, 3, 3].to_vec());
        computer.execute(0);
        assert_eq!(computer.get_mem(3), 9);
    }

    #[test]
    fn test_instr_9_relative_base() {
        // Test program that
        // Sets relative base to -2
        // Stores the input in the location specified by 7 - 2 = 5
        let mut computer = Intcode::new([109, -2, 203, 7, 99, 0].to_vec());
        computer.inputs().push_back(22);
        computer.run();
        assert_eq!(computer.relative_base, -2);
        println!("{:?}", computer.mem);
        assert_eq!(computer.get_mem(5), 22);
    }

    #[test]
    fn test_run_basic_day2_part1_1() {
        // Tests a sample program from day 2 part 1
        let mut computer = Intcode::new([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec());
        computer.run();
        assert_eq!(computer.get_mem(0), 3500);
        assert_eq!(computer.get_mem(3), 70);
    }

    #[test]
    fn test_run_basic_day2_part1_2() {
        // Tests a sample program from day 2 part 1
        let mut computer = Intcode::new([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec());
        computer.run();
        assert_eq!(computer.get_mem(0), 30);
        assert_eq!(computer.get_mem(4), 2);
    }

    #[test]
    fn test_decode() {
        assert_eq!(Intcode::decode(2).param_modes, [0, 0, 0]);
        assert_eq!(Intcode::decode(99).param_modes, [0, 0, 0]);
        assert_eq!(Intcode::decode(1002).param_modes, [0, 1, 0]);
        assert_eq!(Intcode::decode(1002).op, Op::Multiply);
        assert_eq!(Intcode::decode(1202).param_modes, [2, 1, 0]);
    }

    #[test]
    fn test_params() {
        let computer = Intcode::new([1002, 4, 3, 4, 33].to_vec());
        let instr = Intcode::decode(1002);
        assert_eq!(
            computer.param_addrs(0, instr.op.params(), &instr.param_modes),
            [4, 2, 4].to_vec()
        );
    }

    #[test]
    fn test_jump_pos_mode() {
        // Tests some samples from day 5
        let program = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec();
        let mut computer = Intcode::new(program.clone());
        computer.inputs().push_back(0);
        computer.run();
        assert_eq!(*computer.outputs().iter().last().unwrap(), 0);
        let mut computer = Intcode::new(program);
        computer.inputs().push_back(72);
        computer.run();
        assert_eq!(*computer.outputs().iter().last().unwrap(), 1);
    }

    #[test]
    fn test_advanced_day_9_self_replicating() {
        let program = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .to_vec();
        let mut computer = Intcode::new_with(&program, 1024);
        computer.run();
        assert_eq!(
            computer.outputs().iter().copied().collect::<Vec<_>>(),
            program
        );
    }

    #[test]
    fn test_advanced_day_9_self_large_numbers() {
        let program = [104, 1125899906842624, 99].to_vec();
        let mut computer = Intcode::new_with(&program, 1024);
        computer.run();
        assert_eq!(computer.outputs().pop_front().unwrap(), 1125899906842624);
    }
}
