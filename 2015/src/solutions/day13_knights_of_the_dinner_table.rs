use std::{collections::HashMap, iter::once, str::FromStr};

use itertools::Itertools;

type Person = char;
type Happiness = i64;

#[derive(Debug)]
struct Preference {
    person: Person,
    next_to: Person,
    happiness: Happiness,
}

impl FromStr for Preference {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        Ok(Self {
            person: parts[0].chars().next().unwrap(),
            next_to: parts[10].chars().next().unwrap(),
            happiness: parts[3].parse::<i64>().unwrap()
                * (if parts[2].starts_with('g') { 1 } else { -1 }),
        })
    }
}

#[aoc_generator(day13)]
fn gen(input: &str) -> HashMap<Person, HashMap<Person, Happiness>> {
    let mut pairings: HashMap<Person, HashMap<Person, Happiness>> = HashMap::new();
    for pref in input
        .lines()
        .map(|line| line.parse::<Preference>().unwrap())
    {
        // Calculate the happiness achieved by seating two people next to each other
        *pairings
            .entry(pref.person)
            .or_default()
            .entry(pref.next_to)
            .or_default() += pref.happiness;
        *pairings
            .entry(pref.next_to)
            .or_default()
            .entry(pref.person)
            .or_default() += pref.happiness;
    }
    pairings
}

// Calculates the maximum happiness that is achievable when assigning people to a circular table
fn calculate_happiness(pairings: &HashMap<Person, HashMap<Person, Happiness>>) -> Happiness {
    pairings
        .keys()
        .permutations(pairings.len()) // Consider all permutations of orderings of people
        .map(|order| {
            order
                .iter()
                .tuple_windows() // For every pair of people
                .chain(once((order.first().unwrap(), order.last().unwrap()))) // Including wrapping around from last to first (as it's a circular table)
                .map(|(a, b)| pairings.get(a).and_then(|x| x.get(b)).unwrap_or(&0)) // Happiness for this pairing (if there was a preference)
                .sum() // Calculate total happiness of everyone based on the order
        })
        .max()
        .unwrap() // and return the maximum
}

#[aoc(day13, part1)]
fn part1(input: &HashMap<Person, HashMap<Person, Happiness>>) -> Happiness {
    calculate_happiness(input)
}

#[aoc(day13, part2)]
fn part2(input: &HashMap<Person, HashMap<Person, Happiness>>) -> Happiness {
    let mut input = input.clone();
    // Add self with no happiness rating regardless of where sat
    input.insert('x', HashMap::new());
    calculate_happiness(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7 happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 330);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 286);
    }
}
