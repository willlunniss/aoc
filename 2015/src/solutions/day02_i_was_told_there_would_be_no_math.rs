use itertools::Itertools;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<[usize; 3]> {
    input
        .lines()
        .map(|line| {
            // Transform LxWxH into [L, W, H]
            line.split('x')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[[usize; 3]]) -> usize {
    input
        .iter()
        .map(|dims| {
            // Calculate the 3 different side areas
            let areas = dims
                .iter()
                .combinations(2)
                .map(|side| side[0] * side[1])
                .collect::<Vec<_>>();
            // Need enough to cover all sides (2 of each side area) + the spare based on the smallest
            (2 * areas.iter().sum::<usize>()) + *areas.iter().min().unwrap()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[[usize; 3]]) -> usize {
    input
        .iter()
        .map(|dims| {
            // Wrap the ribbon around the smallest two dims
            let wrap = dims
                .iter()
                .combinations(2)
                .map(|side| (side[0] + side[1]) * 2)
                .min()
                .unwrap();
            // Bow size is simply the product of all dims
            let bow = dims.iter().product::<usize>();
            wrap + bow
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    2x3x4
    1x1x10
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 101);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 48);
    }
}
