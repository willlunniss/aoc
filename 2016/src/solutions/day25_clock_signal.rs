use crate::solutions::day12_leonardos_monorail::{Arg, Computer, Op};

#[aoc_generator(day25)]
fn gen(input: &str) -> Vec<Op> {
    input.lines().flat_map(str::parse).collect()
}

#[aoc(day25, part1)]
fn part1(ops: &[Op]) -> Option<isize> {
    'guess: for input in 0.. {
        // Find the lowest input that gives the expected output clock signal
        let mut computer = Computer::new(ops);
        *computer.get_mut(&Arg::Register(0)).unwrap() = input;
        // Expect to see 0, 1, 0, 1, 0, 1 ...
        let mut expected = 0;
        let mut match_count = 0;
        // Run until we see the target signal for 100 cycles
        while match_count < 100 {
            if let Ok(Some(output)) = computer.exec() {
                if output != expected {
                    // Didn't match what we expected
                    continue 'guess;
                }
                // Update the value that we expect to see by toggling the 1st bit
                expected ^= 1;
                match_count += 1;
            }
        }
        // Output matched the target clock signal for 100 cycles
        return Some(input);
    }
    None
}
