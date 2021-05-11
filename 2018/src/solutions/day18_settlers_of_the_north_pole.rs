use std::collections::HashMap;
use utils::grid::VecGrid;

/// Simulates lumber collection for `minutes` of time
///
/// Returns the resource value after `minutes`
fn simulate(input: &VecGrid<char>, minutes: usize) -> usize {
    let mut current = input.clone();
    let mut next = current.clone();
    let mut history = HashMap::new();
    let mut minute = 1;
    while minute <= minutes {
        // For each minute calculate the next states based on current
        for (pos, acre) in &current {
            next.insert(
                pos,
                match acre {
                    '.' => {
                        // An open acre will become filled with trees if three or more adjacent acres contained trees.
                        // Otherwise, nothing happens.
                        if current.neighbours8(pos).filter(|&n| n == Some('|')).count() >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
                        // Otherwise, nothing happens.
                        if current.neighbours8(pos).filter(|&n| n == Some('#')).count() >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other
                        // lumberyard and at least one acre containing trees. Otherwise, it becomes open.
                        if current.neighbours8(pos).filter(|&n| n == Some('#')).count() >= 1
                            && current.neighbours8(pos).filter(|&n| n == Some('|')).count() >= 1
                        {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => {
                        panic!("Unexpected acre {}", acre)
                    }
                },
            );
        }
        if let Some(first_seen) = history.insert(next.values().collect::<String>(), minute) {
            // Started cycling through a set of states
            // Fast forward to be the start of the last cycle before target minutes
            let period = minute - first_seen;
            let shift_by = ((minutes - minute) / period) * period;
            minute += shift_by;
        }
        // Swap over and advance time
        std::mem::swap(&mut current, &mut next);
        minute += 1;
    }
    // Calculate the resource value as woods * luberyards
    current.values().filter(|&value| *value == '|').count()
        * current.values().filter(|&value| *value == '#').count()
}

#[aoc_generator(day18)]
fn gen(input: &str) -> VecGrid<char> {
    VecGrid::from(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day18, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    simulate(input, 10)
}

#[aoc(day18, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    simulate(input, 1_000_000_000)
}
