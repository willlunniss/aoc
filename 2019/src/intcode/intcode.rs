pub struct Intcode {
    mem: Vec<usize>,
    ip: usize,
}

impl Intcode {
    /// Initialises a new Intcode computer with the supplied program data
    pub fn new(data: &Vec<usize>) -> Intcode {
        return Intcode {
            mem: data.clone(),
            ip: 0,
        };
    }

    /// Sets the value at the supplied memory address
    pub fn set_mem(&mut self, addr: usize, value: usize) {
        self.mem[addr] = value;
    }

    /// Gets the value at the supplied memory address
    pub fn get_mem(&self, addr: usize) -> usize {
        return self.mem[addr];
    }

    /// Runs the loaded program until completion
    pub fn run(&mut self) {
        while self.execute(self.ip) {
            self.ip += 4;
        }
    }

    /// Executes a single instruction at ip 0
    ///
    /// Returns false if the program exists, true otherwise
    pub fn execute(&mut self, ip: usize) -> bool {
        match self.mem[ip] {
            1 => self.instr_1_add(ip),
            2 => self.instr_2_mult(ip),
            99 => return false,
            _ => {
                panic!("Unexpected opt {}", self.mem[ip])
            }
        };
        return true;
    }

    /// Returns the three parameters for a instruction
    fn params(&self, ip: usize) -> (usize, usize, usize) {
        return (self.mem[ip + 1], self.mem[ip + 2], self.mem[ip + 3]);
    }

    fn instr_1_add(&mut self, ip: usize) {
        let (input1, input2, output) = self.params(ip);
        self.mem[output] = self.mem[input1] + self.mem[input2];
    }

    fn instr_2_mult(&mut self, ip: usize) {
        let (input1, input2, output) = self.params(ip);
        self.mem[output] = self.mem[input1] * self.mem[input2];
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
}
