use std::ops::RangeInclusive;

use itertools::Itertools;

type Pair = (RangeInclusive<usize>, RangeInclusive<usize>);

/// Returns True if a fully contains b
fn contains(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

/// Returns True if a does not include any values from b
fn disjoint(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    !a.contains(b.start()) && !a.contains(b.end())
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let (from, to) = range.split_once('-').unwrap();
                    from.parse().unwrap()..=to.parse().unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Pair]) -> usize {
    input // Count the number of pair where one is fully contained by the over
        .iter()
        .filter(|pair| contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[Pair]) -> usize {
    input // Count the number of pairs where there is some overlap (not disjoint)
        .iter()
        .filter(|pair| !disjoint(&pair.0, &pair.1) || !disjoint(&pair.1, &pair.0))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 4);
    }
}
