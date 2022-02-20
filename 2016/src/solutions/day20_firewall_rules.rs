use itertools::Itertools;

/// Returns the allowed IPs based on the supplied ranges
fn allowed(input: &str) -> impl Iterator<Item = usize> + '_ {
    let mut current = 0;
    input
        .lines()
        .map(|line| {
            // Read in the ranges
            line.split('-')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .sorted_by(|a, b| a.0.cmp(&b.0)) // Sort from lowest to highest
        .flat_map(move |(min, max)| {
            // Allow all IPs between current and next min. If current >= min then this range will be empty
            let allowed = current..min;
            if max >= current {
                // Advance the current value to be after the end of the range
                current = max + 1;
            }
            allowed
        })
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    allowed(input).next().unwrap()
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    allowed(input).count()
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
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }
}
