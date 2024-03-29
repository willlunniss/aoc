use std::collections::HashSet;

#[aoc_generator(day1)]
fn gen(input: &str) -> HashSet<i64> {
    // Parse each line into a number and store in a HashSet
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &HashSet<i64>) -> i64 {
    // Loop through all keys, seeing if 2020 - key = x exists
    for key in input.iter() {
        let target = 2020 - key;
        if input.contains(&target) {
            // Found one - calculate multiple of the numbers
            let multiplied = key * target;
            return multiplied;
        }
    }
    0
}

#[aoc(day1, part2)]
fn part2(input: &HashSet<i64>) -> i64 {
    // Loop through all keys, seeing if 2020 - key1 - key2 = x exists
    for key1 in input.iter() {
        let target1 = 2020 - key1;
        for key2 in input.iter() {
            let target = target1 - key2;
            if input.contains(&target) {
                // Found one - calculate multiple of the  numbers
                let multiplied = key1 * key2 * target;
                return multiplied;
            }
        }
    }
    0
}
