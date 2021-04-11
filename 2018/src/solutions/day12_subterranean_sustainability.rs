use std::collections::{BTreeSet, HashSet};

type Arrangement = u8;

/// Converts the contents of pots (true = plant, false = empty) into a single
/// value by representing each pot using a single bit
fn arrangement(pots: impl IntoIterator<Item = bool>) -> Arrangement {
    pots.into_iter()
        .enumerate()
        .map(|(index, contents)| if contents { 1 << index } else { 0 })
        .sum()
}

/// Simulates plants spreading between pots
///
/// Returns the sum of the values of the pots that contain plants after the number of generations
fn simulate(
    potted: &BTreeSet<isize>,
    spread_notes: &HashSet<Arrangement>,
    generations: usize,
) -> isize {
    // Create a mutable copy of the set of pots with plants in them
    let mut potted = potted.clone();
    // Need to see if plants stay consistently spaced between generations
    let mut previous_spacing = Vec::new();
    for generation in 0..generations {
        // Check from pot min-2 to max+2 to see if they will have a pot in next generation
        let mut next = BTreeSet::new();
        let mut iter = potted.iter();
        for pot in iter.next().unwrap() - 2..=iter.last().unwrap() + 2 {
            // Get arrangement of plants in pots -2 to + 2 of the current pot
            let arrangement = arrangement((pot - 2..=pot + 2).map(|pot| potted.contains(&pot)));
            // Check the notes to see if it will result in a plant in this pot in the next generation
            if spread_notes.contains(&arrangement) {
                // Yes it will, add the pot
                next.insert(pot);
            }
        }
        // Work out the spacing between pots with plants in them
        let spacing = potted
            .iter()
            .zip(potted.iter().skip(1))
            .map(|(prev, current)| current - prev)
            .collect();
        if spacing == previous_spacing {
            // Have reached a stable state with consistent spacing between plants in pots, will only shift either left or right now
            // Work out which way we are going
            let generation_shift = next.iter().next().unwrap() - potted.iter().next().unwrap();
            // Then fast forward to the target number of generations by adding remaining generations * how much we move per generation
            let remaining = generations - generation;
            let total_shift = generation_shift * (remaining as isize);
            // Move all the plants along
            let shifted = potted
                .iter()
                .map(|pot| pot + total_shift)
                .collect::<Vec<isize>>();
            // Return the sum of the pots that contain plants
            return shifted.iter().sum::<isize>();
        }
        previous_spacing = spacing;
        std::mem::swap(&mut potted, &mut next);
    }
    // Reached the target number of generations without converging
    // Return the sum of the pots that contain plants
    potted.iter().sum::<isize>()
}

#[aoc_generator(day12)]
fn gen(input: &str) -> (BTreeSet<isize>, HashSet<Arrangement>) {
    let mut lines = input.lines();

    // Create an ordered set of the pots that contain plants (after skipping past the heading)
    let potted = lines
        .next()
        .unwrap()
        .chars()
        .skip("initial state: ".len())
        .enumerate()
        .filter_map(|(pot, contents)| {
            if contents == '#' {
                Some(pot as isize)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();

    // Read in the ways that plants can spread e.g. .###. => # keeping only the ones that result in a plant next generation
    // Encode the arrangement of plants in pots into a single value for easy lookup later
    let spread_notes = lines
        .skip(1)
        .filter_map(|line| {
            if line.chars().nth(9).unwrap() == '#' {
                Some(arrangement(line.chars().take(5).map(|c| c == '#')))
            } else {
                None
            }
        })
        .collect();

    (potted, spread_notes)
}

#[aoc(day12, part1)]
fn part1(input: &(BTreeSet<isize>, HashSet<Arrangement>)) -> isize {
    let (potted, spread_notes) = input;
    simulate(potted, spread_notes, 20)
}

#[aoc(day12, part2)]
fn part2(input: &(BTreeSet<isize>, HashSet<Arrangement>)) -> isize {
    let (potted, spread_notes) = input;
    simulate(potted, spread_notes, 50_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_arrangement(pots: &str) -> Arrangement {
        arrangement(pots.chars().map(|c| c == '#'))
    }

    #[test]
    fn test_encode_arrangement() {
        assert_eq!(encode_arrangement("....."), 0);
        assert_eq!(encode_arrangement("#...."), 1);
        assert_eq!(encode_arrangement("##..."), 3);
        assert_eq!(encode_arrangement("..#.."), 4);
        assert_eq!(encode_arrangement("...#."), 8);
        assert_eq!(encode_arrangement("....#"), 16);
        assert_eq!(encode_arrangement("#####"), 31);
    }
}
