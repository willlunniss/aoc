use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn gen(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn find_nth_number(input: &Vec<usize>, n: usize) -> usize {    
    // Use a massive vector (rather than a HashMap) to sacrifice RAM for speed
    // Use -1 indicates we haven't seen it before (faster than wrapping/unwrapping)
    let mut last_occurrences : Vec<isize> = vec![-1; n];
    let mut last_number;
    // Record starting numbers (except the last)
    for (index, value) in input.iter().take(input.len() - 1).enumerate() {
        last_occurrences[*value] = index as isize + 1;
    }
    // Get the last starting number (will be add at the end of the first loop iteration)
    last_number = *input.last().unwrap();
    // Now play the memory game until turn n
    let mut turn = input.len() + 1;
    while turn <= n  {
        // Find when we last said the last number before the last turn
        let last_occurrence = last_occurrences[last_number];
        let last_turn = (turn as isize) - 1;
        let next_number = if last_occurrence != -1 {
            // We have seen this number before the last turn
            // New value is diff between last turn and when we saw it previously
            last_turn - last_occurrence
        } else {
            // Not seen before the last turn
            // New value is 0
            0
        };
        // Now update the last occurrence from the previous turn
        last_occurrences[last_number] = last_turn;
        // And then assign next to last and increment the turn
        last_number = next_number as usize;
        turn += 1;
    }
    
    return last_number;
}

#[aoc(day15, part1)]
fn part1(input: &Vec<usize>) -> usize {
    find_nth_number(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &Vec<usize>) -> usize {
    find_nth_number(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth_number() {
        assert_eq!(find_nth_number(&[0,3,6].to_vec(), 2020), 436);
    }
}