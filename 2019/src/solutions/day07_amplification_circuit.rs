use crate::intcode::Intcode;
use itertools::Itertools;

fn run_chained_amps(program: &Vec<isize>, phases: &[&isize], feedback: bool) -> isize {
    // Create and configure the amps with their phase settings
    let mut amps = Vec::new();
    for phase in phases {
        // Create a new Amplifier instance
        let mut amp = Intcode::new(program.clone());
        // Configure with the phase
        amp.inputs().push_back(**phase);
        // Add to our list of amps
        amps.push(amp);
    }
    // Keep running until all amps have finished
    let mut signal = 0;
    loop {
        // Cycle through all amps
        let mut finished = true;
        for amp in &mut amps {
            // Give it the signal from the last amp (or 0 for the first)
            amp.inputs().push_back(signal);
            if !amp.run() {
                // Amp hasn't finished, must be waiting on an input
                finished = false;
            }
            // Store it's output to be the next signal to propagate
            signal = amp.outputs().pop_front().unwrap();
        }
        if finished || !feedback {
            // All amps have finished or we aren't running in feedback mode (which loops last back to first)
            break;
        }
    }
    // Return the signal value (output from last amp)
    signal
}

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<isize> {
    return input.split(',').map(|i| i.parse().unwrap()).collect();
}

#[aoc(day7, part1)]
fn part1(input: &Vec<isize>) -> isize {
    // Test for all permutations of phases returning the one that generated the highest output
    return [0, 1, 2, 3, 4]
        .to_vec()
        .iter()
        .permutations(5)
        .unique()
        .map(|phases| run_chained_amps(input, &phases, false))
        .max()
        .unwrap();
}

#[aoc(day7, part2)]
fn part2(input: &Vec<isize>) -> isize {
    // Test for all permutations of the new phases returning the one that generated the highest output in loop mode
    return [5, 6, 7, 8, 9]
        .to_vec()
        .iter()
        .permutations(5)
        .unique()
        .map(|phases| run_chained_amps(input, &phases, true))
        .max()
        .unwrap();
}
