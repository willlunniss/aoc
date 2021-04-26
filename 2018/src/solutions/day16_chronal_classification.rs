use crate::chronal_device::Op;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Sample {
    before: [usize; 4],
    instr: [usize; 4],
    after: [usize; 4],
}

/// Returns the ops that match the supplied sample's behaviour
fn matching_ops(sample: &Sample) -> HashSet<Op> {
    Op::iter()
        .filter(|op| {
            // Clone registers from before
            let mut after = sample.before;
            // Execute the instruction and update the registers
            after[sample.instr[3]] = op.execute(sample.instr[1], sample.instr[2], &sample.before);
            // Check it matches as expected
            after == sample.after
        })
        .collect()
}

#[aoc_generator(day16)]
fn gen(input: &str) -> (Vec<Sample>, Vec<[usize; 4]>) {
    /// Converts a split string into a fixed sized array of usize values
    fn to_num_arr<'a, const SIZE: usize>(split: impl Iterator<Item = &'a str>) -> [usize; SIZE] {
        split
            .map(|value| value.parse().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    }

    let mut lines = input.lines();
    let mut samples = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // Reached end of samples
            break;
        }
        // Read in number values from the sample e.g.
        // Before: [1, 2, 2, 0]
        // 15 0 2 1
        // After:  [1, 0, 2, 0]
        let before = to_num_arr(line[9..19].split(", "));
        let instr = to_num_arr(lines.next().unwrap().split(' '));
        let after = to_num_arr(lines.next().unwrap()[9..19].split(", "));
        samples.push(Sample {
            before,
            instr,
            after,
        });
        // Skip blank line
        lines.next();
    }

    // Read in the rest of the input in as the test program (series of instructions)
    let test_program = lines
        .skip(1)
        .map(|line| to_num_arr(line.split(' ')))
        .collect();

    (samples, test_program)
}

#[aoc(day16, part1)]
fn part1(input: &(Vec<Sample>, Vec<[usize; 4]>)) -> usize {
    let (samples, _) = input;
    // Count the number of samples that match 3 or more operations
    samples
        .iter()
        .filter(|sample| matching_ops(sample).len() >= 3)
        .count()
}

#[aoc(day16, part2)]
fn part2(input: &(Vec<Sample>, Vec<[usize; 4]>)) -> usize {
    let (samples, test_program) = input;

    // Work out which opcode maps to which op from the samples
    let mut candidates: HashMap<usize, HashSet<Op>> = HashMap::new();
    for sample in samples {
        // Find the matching ops for the opcode for each sample
        let opcode = sample.instr[0];
        let matches = matching_ops(sample);
        // Build up potential candidate ops for the opcode
        candidates
            .entry(opcode)
            .and_modify(|ops| {
                // Keep reducing the candidates for this opcode by only keeping ones that match this sample
                ops.retain(|op| matches.contains(op));
            }) // No candidates for this opcode yet, add all matches
            .or_insert(matches);
    }

    // Reduce the candidates until we have a direct 1-1 mapping for each opcode/op
    let mut mappings: HashMap<usize, Op> = HashMap::new();
    while !candidates.is_empty() {
        // Find the first opcode that has a single op that matches and add it to the mappings
        let (opcode, ops) = candidates.iter().find(|(_, ops)| ops.len() == 1).unwrap();
        let op = *ops.iter().next().unwrap();
        let opcode = *opcode;
        mappings.insert(opcode, op);
        // Remove this opcode as we have mapped it
        candidates.remove(&opcode);
        // And remove the op from the candidates for the other opcodes
        candidates
            .iter_mut()
            .for_each(|(_, ops)| ops.retain(|&x| x != op));
    }

    // Now execute the program using the mappings and return the value in register 0 at the end
    let mut registers = [0, 0, 0, 0];
    for instr in test_program {
        let op = mappings.get(&instr[0]).unwrap();
        registers[instr[3]] = op.execute(instr[1], instr[2], &registers);
    }
    registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_ops() {
        let before = [3, 2, 1, 1];
        let instr = [9, 2, 1, 2];
        let after = [3, 2, 2, 1];
        // Test the example potentials for opcode 9 give the expected result
        let matching = matching_ops(&Sample {
            before,
            instr,
            after,
        });
        assert_eq!(matching.len(), 3);
        assert_eq!(
            [Op::addi, Op::mulr, Op::seti]
                .iter()
                .all(|op| matching.contains(op)),
            true
        );
    }
}
