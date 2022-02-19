use itertools::Itertools;

#[aoc_generator(day20)]
fn gen(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            line.split('-')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .sorted_by(|a, b| b.0.cmp(&a.0)) // Pre-sort the ranges in reverse order by min value
        .collect()
}

/// Returns the list of IPs allowed by the pre-sorted ranges
fn allowed(ranges: &[(usize, usize)]) -> Vec<usize> {
    let mut ranges = ranges.to_vec();
    let mut allowed = Vec::new();
    let mut current = 0;
    while let Some((min, max)) = ranges.pop() {
        if current < min {
            // The next minimum is below our current best value
            // All between best and min are allowed
            allowed.extend(current..min);
            current = min;
        }
        if max >= current {
            // Advance the current value to be after the end of the range
            current = max + 1;
        }
    }
    allowed
}

#[aoc(day20, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    *allowed(input).first().unwrap()
}

#[aoc(day20, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    allowed(input).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    5-8
    0-2
    4-7
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 3);
    }
}
