use itertools::Itertools;
use std::collections::HashSet;

fn gen(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    gen(input)
        .iter()
        .filter(|&passphrase| {
            // Passphrases are valid if all words are unique
            let mut words = HashSet::new();
            for word in passphrase {
                if words.contains(word) {
                    return false;
                }
                words.insert(word);
            }
            // No duplicates - it's valid
            true
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    gen(input)
        .iter()
        .filter(|&passphrase| {
            // Passphrases are valid if all words are unique anagrams
            let mut words = HashSet::new();
            for word in passphrase {
                // Sort all chars in the word so that we can easily test for anagrams
                let sorted_chars = word.chars().sorted().collect::<String>();
                if words.contains(&sorted_chars) {
                    return false;
                }
                words.insert(sorted_chars);
            }
            // No anagrams - it's valid
            true
        })
        .count()
}
