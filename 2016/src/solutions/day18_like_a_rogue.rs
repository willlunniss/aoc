use itertools::Itertools;
use std::iter::once;

/// Generates rows based on the rules and returns the number of safe tiles
fn count_safe_tiles(first_row: Vec<char>, rows: usize) -> usize {
    (1..rows)
        .fold(
            (first_row.iter().filter(|x| **x == '.').count(), first_row),
            |acc, _| {
                // Iterate over triples of the previous row to calculate the new values for the next row according to rules
                // (with an extra '.' added before and after to represent the edges being safe)
                let row = once(&'.')
                    .chain(acc.1.iter())
                    .chain(once(&'.'))
                    .tuple_windows()
                    .map(|(l, c, r)| match (l, c, r) {
                        ('^', '^', '.') | ('.', '^', '^') | ('^', '.', '.') | ('.', '.', '^') => {
                            '^'
                        }
                        _ => '.',
                    })
                    .collect::<Vec<_>>();
                (acc.0 + row.iter().filter(|x| **x == '.').count(), row)
            },
        )
        .0
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    count_safe_tiles(input.chars().collect(), 40)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    count_safe_tiles(input.chars().collect(), 400_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(count_safe_tiles(".^^.^.^^^^".chars().collect(), 10), 38);
    }
}
