use itertools::Itertools;
use std::collections::HashSet;

/// Balances packages between the specified number of groups finding the lowest
/// Quantum Entanglement (QE) of the first group
fn balance(packages: &Vec<usize>, groups: usize, target_weight: Option<usize>) -> Option<usize> {
    for split in 1..packages.len() - groups {
        // Get all possible selections of split size for this group
        let mut selections = packages.iter().combinations(split).collect::<Vec<_>>();
        if target_weight.is_none() {
            // First group - we need to find the one with the lowest QE
            // Sort selections from lowest to highest QE
            selections.sort_by_cached_key(|p| p.iter().copied().product::<usize>());
        }

        // Go through each to find the first one that balances
        for selected in &selections {
            let mut target_weight = target_weight;
            // Check if this group is balanced
            let weight = selected.iter().copied().sum::<usize>();
            if target_weight.is_none() {
                // First group, balance against it's weight
                target_weight = Some(weight);
            } else if Some(weight) != target_weight {
                // This non-first group does not match the target weight so not balanced
                continue;
            }

            // Work out what is left
            let removed = selected.iter().collect::<HashSet<_>>();
            let remaining = packages
                .iter()
                .filter(|p| !removed.contains(p))
                .copied()
                .collect::<Vec<_>>();

            // Check it can be balanced between the remaining groups
            let remaining_groups = groups - 1;
            if remaining.iter().sum::<usize>() != weight * remaining_groups {
                continue;
            }

            if remaining_groups > 1 {
                // Recursively attempt to balance remaining groups
                if balance(&remaining, remaining_groups, target_weight).is_none() {
                    // Cannot balance them
                    continue;
                }
            }

            // Balanced this and all remaining groups!
            return Some(selected.iter().copied().product());
        }
    }
    // Failed to balance
    None
}

#[aoc_generator(day24)]
fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day24, part1)]
fn part1(input: &Vec<usize>) -> Option<usize> {
    balance(input, 3, None)
}

#[aoc(day24, part2)]
fn part2(input: &Vec<usize>) -> Option<usize> {
    balance(input, 4, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    1
    2
    3
    4
    5
    7
    8
    9
    10
    11
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), Some(99));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), Some(44));
    }
}
