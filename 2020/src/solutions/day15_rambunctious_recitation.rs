use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn gen(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn find_nth_number(input: &Vec<usize>, n: usize) -> usize {    
    let mut last_occurances : HashMap<usize, usize> = HashMap::new();
    let mut last_number;
    // Record starting numbers
    for (index, value) in input.iter().enumerate() {
        last_occurances.insert(*value, index + 1);
    }
    // Fetch the last and then remove it (will be add at the end of the first loop itteration)
    last_number = *input.last().unwrap();
    last_occurances.remove(&last_number);
    // Now play the memory game until turn n
    let mut turn = input.len() + 1;
    while turn <= n {
        // Find when we last said the last number before the last turn
        let last_occurance = last_occurances.get(&last_number);
        let next_number;
        if last_occurance.is_some() {
            // We have seen this number before the last turn
            // New value is diff between this turn and when we saw it previously
            next_number = turn - 1 - last_occurance.unwrap();
        } else {
            // Not seen before the last turn
            // New value is 0
            next_number = 0;
        }
        // Now update the last occurance from the previous iteration
        last_occurances.insert(last_number, turn - 1);
        // And then assign next to last and increment the turn
        last_number = next_number;
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

        // Really slow test
        // assert_eq!(find_nth_number(&[0,3,6].to_vec(), 30000000), 175594);
    }
}