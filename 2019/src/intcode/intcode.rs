use std::collections::VecDeque;
use digits_iterator::*;

/// AOC 2019 Intcode implementation
///
/// # Examples
/// ```
/// # use aoc_2019::intcode::Intcode;
/// // Our sample program
/// let program : Vec<isize> = [1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec();
/// // Create a new instance using the program
/// let mut computer = Intcode::new(&program);
/// // Run the loaded program
/// computer.run();
/// // Get and check the result (this program stores it in address 0)
/// let result = computer.get_mem(0);
/// assert_eq!(result, 30);
/// ```
pub struct Intcode {
    mem: Vec<isize>,
    ip: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
}

/// Result of executing an instruction
enum Result {
    SetIP(usize),
    Exit,
}

/// Type of operation and associated (static) configuration
#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mult,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

impl Op {
    /// Creates a new op from an opcode
    fn new(opcode: usize) -> Op {
        use Op::*;
        match opcode {
            1 => Add,
            2 => Mult,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Halt,
            _ => panic!("Unexpected opcode {}", opcode)
        }
    }

    /// Gets the instruction implementation
    fn instr_impl(&self) -> fn(&mut Intcode, ip: usize, params: Vec<isize>) -> Option<Result> {
        use Op::*;
        match self {
            Add => Intcode::instr_1_add,
            Mult => Intcode::instr_2_mult,
            Input => Intcode::instr_3_input,
            Output => Intcode::instr_4_output,
            JumpIfTrue => Intcode::instr_5_jump_if_true,
            JumpIfFalse => Intcode::instr_6_jump_if_false,
            LessThan => Intcode::instr_7_less_than,
            Equals => Intcode::instr_8_equals,
            Halt => Intcode::instr_99_halt,
        }
    }

    /// Gets the number of parameters used
    fn params(&self) -> usize {
        use Op::*;
        match self {
            Add | Mult => 3,
            Input | Output => 1,
            JumpIfTrue | JumpIfFalse => 2,
            LessThan | Equals => 3,
            Halt => 0,
        }
    }

    /// Gets the id of the output parameter, or None if there is none
    fn output_param(&self) -> Option<usize> {
        use Op::*;
        match self {
            Add | Mult => Some(2),
            Input => Some(0),
            LessThan | Equals => Some(2),
            _ => None,
        }
    }
}


/// Instruction representation including dynamic configuration
struct Instr {
    // The actual operation to be performed
    op: Op,
    // Modes of the parameters to use
    param_modes: Vec<usize>
}

impl Intcode {
    /// Initialises a new Intcode computer with the supplied program
    pub fn new(program: &Vec<isize>) -> Intcode {
        return Intcode {
            mem: program.clone(),
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        };
    }

    /// Initialises a new Intcode computer from a comma separated string of integers
    pub fn from(program: &str) -> Intcode {
        return Intcode {
            mem: program.split(',').map(|i| i.parse::<isize>().unwrap()).collect(),
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        };
    }

    /// Sets the value at the supplied memory address
    pub fn set_mem(&mut self, addr: usize, value: isize) {
        self.mem[addr] = value;
    }

    /// Gets the value at the supplied memory address
    pub fn get_mem(&self, addr: usize) -> isize {
        return self.mem[addr];
    }

    /// Runs the loaded program until completion
    pub fn run(&mut self) {
        loop {
            match self.execute(self.ip) {
                Result::Exit => return,
                Result::SetIP(ip) => self.ip = ip,
            }
        }
    }

    /// Gets the input queue
    pub fn inputs(&mut self) -> &mut VecDeque<isize> {
        return &mut self.inputs;
    }

    /// Gets the output queue
    pub fn outputs(&mut self) -> &mut VecDeque<isize> {
        return &mut self.outputs;
    }

    /// Executes a single instruction at the specified IP
    fn execute(&mut self, ip: usize) -> Result {
        // Decode the instruction
        let instr = Intcode::decode(self.mem[ip]);
        // Get the reference to the implementation
        let instr_impl = instr.op.instr_impl();
        // Get the parameters
        let params = self.params(ip, instr.op.params(), instr.param_modes);
        // Execute it
        let result = instr_impl(self, ip, params);
        if result.is_none() {
            // Simple instructions with no result
            // Just advance the instruction pointer by 1 + num params
            return Result::SetIP(ip + 1 + instr.op.params());
        }
        return result.unwrap();
    }

    /// Decodes an instruction
    fn decode(encoded: isize) -> Instr {
        // Extract the opcode and param modes
        let mut opcode = 0;
        let mut param_modes = vec![0; 3];
        for (idx, digit) in encoded.digits().collect::<Vec<_>>().iter().rev().enumerate() {
            match idx {
                0 => opcode += digit,
                1 => opcode += 10 * digit,
                _ => param_modes[idx - 2] = *digit as usize
            }
        }
        // Lookup the opcode
        let op = Op::new(opcode as usize);
        // Force params that are used to write to memory to always be in address mode even if not specified
        if let Some(output_param) = op.output_param() {
            param_modes[output_param] = 1
        }
        return Instr { op: op, param_modes: param_modes };
    }
    
    /// Gets the parameters for an instruction taking into account the different parameter modes
    fn params(&self, ip: usize, count: usize, modes: Vec<usize>) -> Vec<isize> {
        return (0..count).map(|param| {
            let value = self.mem[ip + 1 + param as usize];
            if modes[param] == 0 {
                // Address mode - the real value is at the address of the value
                self.mem[value as usize]
            } else {
                // Immediate mode - return the value directly
                value
            }
        }).collect();
    }

    /// Gets the next available input
    fn input(&mut self) -> isize {
        return self.inputs.pop_front().unwrap();
    }

    /// Outputs the supplied value
    fn output(&mut self, value: isize) {
        self.outputs.push_back(value);
    }

    /// Sets the 3rd parameter to the 1st plus the 2nd
    fn instr_1_add(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.mem[params[2] as usize] = params[0] + params[1];
        return None;
    }

    /// Sets the 3rd parameter to the 1st multiplied by the 2nd
    fn instr_2_mult(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.mem[params[2] as usize] = params[0] * params[1];
        return None;
    }

    /// Fetches an input and stores it in the 1st parameter
    fn instr_3_input(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.mem[params[0] as usize] = self.input();
        return None;
    }

    /// Outputs the 1st parameter
    fn instr_4_output(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.output(params[0]);
        return None;
    }

    /// Sets the IP to the value of the 2nd parameter if the 1st is not-equal to 0
    fn instr_5_jump_if_true(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        if params[0] != 0 {
            return Some(Result::SetIP(params[1] as usize));
        } else {
            return None;
        }
    }

    /// Sets the IP to the value of the 2nd parameter if the 1st is equal to 0
    fn instr_6_jump_if_false(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        if params[0] == 0 {
            return Some(Result::SetIP(params[1] as usize));
        } else {
            return None;
        }
    }

    /// Sets the 3rd parameter to 1 if the 1st is less than the second, else sets to 0
    fn instr_7_less_than(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.mem[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 };
        return None;
    }

    /// Sets the 3rd parameter to 1 if the 1st and second are equal, else sets to 0
    fn instr_8_equals(&mut self, _: usize, params: Vec<isize>) -> Option<Result> {
        self.mem[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 };
        return None;
    }

    /// Unconditionally causes the program to exit
    fn instr_99_halt(&mut self, _: usize, _: Vec<isize>) -> Option<Result> {
        return Some(Result::Exit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instr_1_add() {
        let mut computer = Intcode::new(&[1, 3, 3, 3].to_vec());
        computer.execute(0);
        assert_eq!(computer.get_mem(3), 6);
    }

    #[test]
    fn test_instr_2_multi() {
        let mut computer = Intcode::new(&[2, 3, 3, 3].to_vec());
        computer.execute(0);
        assert_eq!(computer.get_mem(3), 9);
    }

    #[test]
    fn test_run_basic_day2_part1_1() {
        // Tests a sample program from day 2 part 1
        let mut computer = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec());
        computer.run();
        assert_eq!(computer.get_mem(0), 3500);
        assert_eq!(computer.get_mem(3), 70);
    }

    #[test]
    fn test_run_basic_day2_part1_2() {
        // Tests a sample program from day 2 part 1
        let mut computer = Intcode::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec());
        computer.run();
        assert_eq!(computer.get_mem(0), 30);
        assert_eq!(computer.get_mem(4), 2);
    }

    #[test]
    fn test_decode() {
        assert_eq!(Intcode::decode(2).param_modes, [0, 0, 1]);
        assert_eq!(Intcode::decode(99).param_modes, [0, 0, 0]);
        assert_eq!(Intcode::decode(1002).param_modes, [0, 1, 1]);
        assert_eq!(Intcode::decode(1002).op, Op::Mult);
    }

    #[test]
    fn test_params() {
        let computer = Intcode::new(&[1002,4,3,4,33].to_vec());
        let instr = Intcode::decode(1002);
        assert_eq!(computer.params(0, instr.op.params(), instr.param_modes), [33, 3, 4].to_vec());
    }

    #[test]
    fn test_jump_pos_mode() {
        // Tests some samples from day 5
        let program = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9].to_vec();
        let mut computer = Intcode::new(&program);
        computer.inputs().push_back(0);
        computer.run();
        assert_eq!(*computer.outputs().iter().last().unwrap(), 0);
        let mut computer = Intcode::new(&program);
        computer.inputs().push_back(72);
        computer.run();
        assert_eq!(*computer.outputs().iter().last().unwrap(), 1);
    }
}
