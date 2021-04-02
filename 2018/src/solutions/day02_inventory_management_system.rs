use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut matches_twice = 0;
    let mut matches_thrice = 0;
    for line in input.lines() {
        // Count occurrences of each char
        let mut counts = HashMap::new();
        for c in line.chars() {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        // Record a match if we see any char exactly two times
        if counts.values().any(|&x| x == 2) {
            matches_twice += 1;
        }
        // And also record a match if we see any char exactly three times
        if counts.values().any(|&x| x == 3) {
            matches_thrice += 1;
        }
    }
    matches_twice * matches_thrice
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    'outer: for ids in input.lines().combinations(2) {
        // For each pair of ids compare all chars
        let (id1, id2) = (ids[0], ids[1]);
        let mut difference = None;
        for (index, (c1, c2)) in id1.chars().zip(id2.chars()).enumerate() {
            // We want to find the pair of ids with exactly one difference
            if c1 != c2 {
                // Found a difference
                if difference.is_some() {
                    // More than one difference, move on to next pair
                    continue 'outer;
                }
                difference = Some(index);
            }
        }
        if let Some(index) = difference {
            // If we got here then we found a single difference, return the chars excluding the one that differed
            return [&id1[0..index], &id1[index + 1..]].concat();
        }
    }
    "".to_owned()
}
