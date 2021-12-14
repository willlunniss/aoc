use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PolymerFormula<'a> {
    polymer_template: &'a str,
    insertions: HashMap<(char, char), char>,
}

fn gen(input: &str) -> PolymerFormula {
    let mut iter = input.lines();
    let polymer_template = iter.next().unwrap();

    let insertions = iter
        .skip(1)
        .map(|line| {
            (
                // Split CH -> B into a pair of chars and a char
                (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap()),
                line.chars().nth(6).unwrap(),
            )
        })
        .collect();

    PolymerFormula {
        polymer_template,
        insertions,
    }
}

/// Simulates the formula by stepping through each insertion
fn simulate(formula: &PolymerFormula, steps: usize) -> usize {
    let mut polymer = formula.polymer_template.chars().collect::<Vec<char>>();
    for _step in 1..=steps {
        let mut generates = Vec::new();
        // For each pair, replace (e1, e3) with (e1, e2), (e2, e3)
        for (&e1, &e3) in polymer.iter().tuple_windows() {
            let e2 = *formula.insertions.get(&(e1, e3)).unwrap();
            generates.push(e1);
            generates.push(e2);
        }
        // Need to include the final element of the polymer
        generates.push(polymer[polymer.len() - 1]);
        polymer = generates;
    }

    // Count frequency of all elements
    let mut frequency = HashMap::new();
    for c in polymer {
        frequency.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    // Answer is most common element quantity - least common element quantity
    let counts = frequency.values().sorted().collect::<Vec<_>>();
    counts[counts.len() - 1] - counts[0]
}

/// Calculates the formula by manipulating the pairs' counts
fn calculate(formula: &PolymerFormula, steps: usize) -> usize {
    let polymer = formula.polymer_template.chars().collect::<Vec<char>>();

    // Get frequency of initial pairs
    let mut pairs = HashMap::new();
    for (&e1, &e2) in polymer.iter().tuple_windows() {
        pairs.entry((e1, e2)).and_modify(|e| *e += 1).or_insert(1);
    }

    for _step in 1..=steps {
        // Count how many times each pair occurs
        let counts = pairs
            .iter()
            .map(|((e1, e2), v)| ((*e1, *e2), *v))
            .collect::<HashMap<_, _>>();
        for ((e1, e3), count) in counts {
            // Decrement the count for pair (e1, e3)
            // and increment for pair (e1, e2) and (e2, e3)
            let e2 = *formula.insertions.get(&(e1, e3)).unwrap();
            pairs.entry((e1, e3)).and_modify(|c| *c -= count);
            pairs
                .entry((e1, e2))
                .and_modify(|c| *c += count)
                .or_insert(count);
            pairs
                .entry((e2, e3))
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }

    // Translate the counts for each pair into counts for each element
    // As part of this we will be doubly counting all but the end most elements
    let mut frequency = HashMap::new();
    for ((e1, e2), &count) in &pairs {
        frequency
            .entry(e1)
            .and_modify(|c| *c += count)
            .or_insert(count);
        frequency
            .entry(e2)
            .and_modify(|c| *c += count)
            .or_insert(count);
    }

    // Account for the fact doubly counted the elements that are shared by pairs
    // i.e. in ABCD we count 1221
    // All even ones should be divided by two
    // The two odds (the end ones) should be divided by two and then +1
    for count in frequency.values_mut() {
        let end_element = *count % 2 == 1;
        *count /= 2;
        if end_element {
            *count += 1;
        }
    }

    // Answer is most common element quantity - least common element quantity
    let counts = frequency.values().sorted().collect::<Vec<_>>();
    counts[counts.len() - 1] - counts[0]
}

#[aoc(day14, part1, simulate)]
fn part1_simulate(input: &str) -> usize {
    simulate(&gen(input), 10)
}

#[aoc(day14, part1, calculate)]
fn part1_calculate(input: &str) -> usize {
    calculate(&gen(input), 10)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    calculate(&gen(input), 40)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    NNCB

    CH -> Bs
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
"};

    #[test]
    fn test_part1_example_simulate() {
        assert_eq!(part1_simulate(EXAMPLE_INPUT), 1588);
    }

    #[test]
    fn test_part1_example_calculate() {
        assert_eq!(part1_calculate(EXAMPLE_INPUT), 1588);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 2_188_189_693_529);
    }
}
