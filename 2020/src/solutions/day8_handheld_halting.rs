use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style = "lowercase")]
pub enum Op {
    Acc,
    Jmp,
    Nop,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{op} {value}")]
pub struct Instr {
    op: Op,
    value: isize,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{pc} {acc}")]
struct State {
    pc: usize,
    acc: isize,
}

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<Instr> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn execute_instr(program: &Vec<Instr>, state: &mut State) {
    let instr = program[state.pc];
    //println!("{:?}: Executing {:?}", state, instr);
    match instr.op {
        Op::Acc => {
            // Increment accumulator by instruction value
            state.acc += instr.value;
            // Advance PC by one
            state.pc += 1;
        },
        Op::Jmp => {
            // Jump by updating the PC by the instruction value
            state.pc = (state.pc as isize + instr.value) as usize;
        },
        Op::Nop => {
            // Do nothing, just advance the PC by one
            state.pc += 1;
        },
    }
}

/// Executes the supplied program until it ends or hits and infinite loop
/// Returns the final program state
fn execute(program: &Vec<Instr>) -> State {    
    let mut state = State{pc: 0, acc: 0};
    let mut executed_instrs = vec![0; program.len()];
    // Run until we hit a loop (try to execute the same instruction again)
    // or reach the end of the program
    while state.pc < program.len() && executed_instrs[state.pc] == 0 {
        executed_instrs[state.pc] = 1;
        execute_instr(&program, &mut state);
    }
    return state;
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Instr>) -> isize {
    let state = execute(&input);
    return state.acc;
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Instr>) -> isize {
    // Create a copy of the program so that we can try to correct it
    let mut program = input.to_vec();
    for idx in 0..program.len() {
        // Loop through everything instruction and see if it's one we can swap (nop/jmp)
        let instr = program[idx];        
        match instr.op {
            Op::Jmp => program[idx] = Instr{op: Op::Nop, value: instr.value},
            Op::Nop => program[idx] = Instr{op: Op::Jmp, value: instr.value},
            _ => continue // Can't swap this one, move on to the next instr
        }
        // Now execute the modified program
        let state = execute(&program);
        //println!("Trying with {} swapped (={:?}) -> {:?}", idx, program[idx], state);
        if state.pc >= program.len() {
            // We reach the end of the program!
            return state.acc;
        }
        // That didn't work, put it back and we will try again...
        program[idx] = instr;
    }
    // Something went wrong, nothing worked :(
    return -1;    
}
