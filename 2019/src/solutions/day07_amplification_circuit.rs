use itertools::Itertools;
use crate::intcode::Intcode;

fn run_chained_amps(program: &Vec<isize>, phases: &Vec<&usize>, feedback: bool) -> usize {
    // Create and configure the amps with their phase settings
    let mut amps = Vec::new();
    for phase in phases {
        // Create a new Amplifier instance
        let mut amp = Intcode::new(program.clone());
        // Configure with the phase
        amp.inputs().push_back(**phase as isize);
        // Add to our list of amps
        amps.push(amp);
    }
    // Keep running until all amps have finished
    let mut signal = 0;
    loop {
        // Cycle through all amps
        let mut finished = true;
        for amp in amps.iter_mut() {
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
    return signal as usize;
}

fn find_highest_phase_setting(program: &Vec<isize>, phase_settings: &Vec<usize>, feedback: bool) -> usize {
    let mut highest_signal = 0;
    // Test for all permutations of phases
    for phases in phase_settings.iter().permutations(phase_settings.len()).unique() {
        // Calculate the output signal based on using these phases
        let signal = run_chained_amps(program, &phases, feedback);
        // Found a phase combination with a higher output, use it
        if signal > highest_signal {
            highest_signal = signal;
        }
    }
    return highest_signal;
}

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<isize> {
    return input.split(',').map(|i| i.parse().unwrap()).collect();
}

#[aoc(day7, part1)]
fn part1(input: &Vec<isize>) -> usize {
    return find_highest_phase_setting(input, &[0, 1, 2, 3, 4].to_vec(), false);
}

#[aoc(day7, part2)]
fn part2(input: &Vec<isize>) -> usize {
    return find_highest_phase_setting(input, &[5, 6, 7, 8, 9].to_vec(), true);
}