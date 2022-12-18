use std::collections::HashSet;

use itertools::Itertools;

/// Returns the priority of an item
fn priority(item: char) -> usize {
    (match item {
        'a'..='z' => item as u8 - 'a' as u8 + 1,
        'A'..='Z' => item as u8 - 'A' as u8 + 27,
        _ => unreachable!(),
    }) as usize
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|rucksack| {
            // Split items into the two compartments
            let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
            let (c1, c2) = (
                c1.chars().collect::<HashSet<char>>(),
                c2.chars().collect::<HashSet<char>>(),
            );
            // Find the item that appears in both
            *c1.intersection(&c2).next().unwrap()
        })
        .map(priority) // Calculate their priorities
        .sum() // And sum them all up
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .chunks(3) // Split into each group of 3 rucksacks
        .into_iter()
        .map(|group| {
            group // Get a list of items in each rucksack
                .flat_map(|rucksack| rucksack.chars().unique())
                .counts() // Count across all rucksacks in the group
                .iter() // Find the one item in all rucksacks in the group
                .find(|(_, count)| **count == 3)
                .map(|(item, _)| *item)
                .unwrap()
        })
        .map(priority) // Calculate their priorities
        .sum() // And sum them all up
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 70);
    }
}
